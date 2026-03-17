#!/usr/bin/env python3
"""
ABI layout inspector using the lldb Python API.

Usage:
    PYTHONPATH="$(xcrun lldb --python-path)" python3 scripts/abi_lldb_inspect.py \
        <dylib_path> [type_name ...] > abi-dynamic.json

If no type names are given, all Swift types found in the module are dumped.
The output is a JSON array of type records, each with name, kind, size, and fields
with byte offsets taken directly from DWARF debug info.

This requires the dylib to have been compiled with -g.
"""

import json
import sys
import os


def _find_lldb_python():
    import subprocess
    try:
        result = subprocess.run(
            ["xcrun", "lldb", "--python-path"],
            capture_output=True, text=True, timeout=10
        )
        path = result.stdout.strip()
        if path and os.path.isdir(path):
            return path
    except Exception:
        pass
    # Fallback: well-known system location
    fallback = (
        "/Library/Developer/CommandLineTools/Library/"
        "PrivateFrameworks/LLDB.framework/Resources/Python"
    )
    if os.path.isdir(fallback):
        return fallback
    return None


def _ensure_lldb():
    try:
        import lldb  # noqa: F401
        return True
    except ImportError:
        path = _find_lldb_python()
        if path and path not in sys.path:
            sys.path.insert(0, path)
        try:
            import lldb  # noqa: F401
            return True
        except ImportError:
            return False


_TYPE_CLASS_NAMES = {
    0x0000: "unknown",
    0x0001: "array",
    0x0002: "block_pointer",
    0x0004: "builtin",
    0x0008: "class",
    0x0010: "complex_int",
    0x0020: "complex_float",
    0x0040: "enum",
    0x0080: "function",
    0x0100: "member_pointer",
    0x0200: "obj_class",
    0x0400: "pointer",
    0x0800: "reference",
    0x1000: "struct",
    0x2000: "typedef",
    0x4000: "union",
    0x8000: "vector",
}


def _type_class_str(raw):
    return _TYPE_CLASS_NAMES.get(int(raw), "other")


def inspect_dylib(dylib_path, type_names=None):
    """Return a list of type layout records from the given Swift dylib."""
    import lldb

    debugger = lldb.SBDebugger.Create()
    debugger.SetAsync(False)
    target = debugger.CreateTargetWithFileAndArch(dylib_path, "arm64-apple-macosx")

    if not target.IsValid():
        raise RuntimeError(f"Could not create lldb target for {dylib_path}")

    results = []
    seen = set()

    def _record_type(sbtype):
        tname = sbtype.GetName() or ""
        if not tname or tname in seen:
            return
        seen.add(tname)

        type_class = _type_class_str(sbtype.GetTypeClass())
        entry = {
            "name": tname,
            "kind": type_class,
            "size": sbtype.GetByteSize(),
            "fields": [],
        }

        n_fields = sbtype.GetNumberOfFields()
        for k in range(n_fields):
            f = sbtype.GetFieldAtIndex(k)
            ft = f.GetType()
            entry["fields"].append({
                "name": f.GetName() or f"_field{k}",
                "offset": f.GetOffsetInBytes(),
                "type": ft.GetName() or "?",
                "size": ft.GetByteSize(),
            })

        # For classes/structs without DWARF fields we still emit the record; the
        # caller can supplement with MemoryLayout data from the Swift probe.
        results.append(entry)

    if type_names:
        for name in type_names:
            tlist = target.FindTypes(name)
            for j in range(tlist.GetSize()):
                _record_type(tlist.GetTypeAtIndex(j))
    else:
        # Collect candidate names from exported symbols, then look up each type.
        # Swift metadata accessor symbols have the form $s<module><len><name>VMa
        # or $s<module><len><name>CMa — strip to bare type names.
        import re
        candidate_names = set()
        for mod_idx in range(target.GetNumModules()):
            mod = target.GetModuleAtIndex(mod_idx)
            for sym_idx in range(mod.GetNumSymbols()):
                sym = mod.GetSymbolAtIndex(sym_idx)
                sname = sym.GetName() or ""
                # Match Swift metadata accessor: $s + module + digits + TypeName + (CMa|VMa|OMa)
                m = re.search(r'\$s\w+?([A-Z][A-Za-z0-9_]*?)(?:CMa|VMa|OMa)$', sname)
                if m:
                    candidate_names.add(m.group(1))
                # Simpler fallback: any symbol segment that looks like a type name
                for part in re.findall(r'([A-Z][A-Za-z0-9]{2,})', sname):
                    candidate_names.add(part)
        for name in sorted(candidate_names):
            tlist = target.FindTypes(name)
            for j in range(tlist.GetSize()):
                _record_type(tlist.GetTypeAtIndex(j))

    lldb.SBDebugger.Destroy(debugger)
    return results


def main():
    if len(sys.argv) < 2:
        print(
            f"usage: {sys.argv[0]} <dylib_path> [TypeName ...]",
            file=sys.stderr,
        )
        sys.exit(1)

    if not _ensure_lldb():
        print(
            "error: lldb Python module not found. "
            "Run: xcrun lldb --python-path  to locate it.",
            file=sys.stderr,
        )
        sys.exit(1)

    dylib_path = sys.argv[1]
    type_names = sys.argv[2:] if len(sys.argv) > 2 else None

    records = inspect_dylib(dylib_path, type_names)
    print(json.dumps(records, indent=2))


if __name__ == "__main__":
    main()
