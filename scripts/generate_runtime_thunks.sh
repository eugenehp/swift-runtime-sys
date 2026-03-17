#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SPEC_FILE="$ROOT/examples/runtime_thunk_methods.txt"
OUT_FILE="$ROOT/examples/runtime_swiftcall_thunks.generated.c"

if [[ ! -f "$SPEC_FILE" ]]; then
  echo "Spec file not found: $SPEC_FILE" >&2
  exit 1
fi

cat > "$OUT_FILE" <<'HEADER'
#include <stdint.h>
#include <stddef.h>
#include <dlfcn.h>

static void *resolve_symbol(const char *name) {
    void *sym = dlsym(RTLD_DEFAULT, name);
    if (sym != NULL) {
        return sym;
    }

    char prefixed[512] = {0};
    prefixed[0] = '_';
    for (int i = 0; i < 510 && name[i] != '\0'; i++) {
        prefixed[i + 1] = name[i];
        prefixed[i + 2] = '\0';
    }
    return dlsym(RTLD_DEFAULT, prefixed);
}

HEADER

while IFS='|' read -r thunk_name mangled_symbol self_register signature; do
  [[ -z "${thunk_name}" ]] && continue
  [[ "${thunk_name}" =~ ^# ]] && continue

  if [[ -z "${mangled_symbol}" || -z "${self_register}" || -z "${signature}" ]]; then
    echo "Invalid spec line for thunk '${thunk_name}'" >&2
    exit 1
  fi
  case "$signature" in
    self_i32_to_i32)
      cat >> "$OUT_FILE" <<THUNK
int32_t ${thunk_name}(void *obj, int32_t arg0) {
    void *raw = resolve_symbol("${mangled_symbol}");
    if (raw == NULL) {
        return INT32_MIN;
    }

#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov ${self_register}, %[self]\\n"
        "mov w0, %w[arg0]\\n"
        "blr %[fn]\\n"
        "mov %w[out], w0\\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [arg0] "r"(arg0), [fn] "r"(raw)
        : "x0", "${self_register}", "x30", "memory");
    return result;
#else
    (void)obj;
    (void)arg0;
    return INT32_MIN;
#endif
}

THUNK
      ;;
    self_to_i32)
      cat >> "$OUT_FILE" <<THUNK
int32_t ${thunk_name}(void *obj) {
    void *raw = resolve_symbol("${mangled_symbol}");
    if (raw == NULL) {
        return INT32_MIN;
    }

#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov ${self_register}, %[self]\\n"
        "blr %[fn]\\n"
        "mov %w[out], w0\\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [fn] "r"(raw)
        : "x0", "${self_register}", "x30", "memory");
    return result;
#else
    (void)obj;
    return INT32_MIN;
#endif
}

THUNK
      ;;
    self_i32_to_void)
      cat >> "$OUT_FILE" <<THUNK
int32_t ${thunk_name}(void *obj, int32_t arg0) {
    void *raw = resolve_symbol("${mangled_symbol}");
    if (raw == NULL) {
        return INT32_MIN;
    }

#if defined(__aarch64__)
    __asm__ volatile(
        "mov ${self_register}, %[self]\\n"
        "mov w0, %w[arg0]\\n"
        "blr %[fn]\\n"
        :
        : [self] "r"(obj), [arg0] "r"(arg0), [fn] "r"(raw)
        : "x0", "${self_register}", "x30", "memory");
    return 0;
#else
    (void)obj;
    (void)arg0;
    return INT32_MIN;
#endif
}

THUNK
      ;;
    self_i32_i32_to_i32)
      cat >> "$OUT_FILE" <<THUNK
int32_t ${thunk_name}(void *obj, int32_t arg0, int32_t arg1) {
    void *raw = resolve_symbol("${mangled_symbol}");
    if (raw == NULL) {
        return INT32_MIN;
    }

#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov ${self_register}, %[self]\\n"
        "mov w0, %w[arg0]\\n"
        "mov w1, %w[arg1]\\n"
        "blr %[fn]\\n"
        "mov %w[out], w0\\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [arg0] "r"(arg0), [arg1] "r"(arg1), [fn] "r"(raw)
        : "x0", "x1", "${self_register}", "x30", "memory");
    return result;
#else
    (void)obj;
    (void)arg0;
    (void)arg1;
    return INT32_MIN;
#endif
}

THUNK
      ;;
    self_to_void)
      cat >> "$OUT_FILE" <<THUNK
int32_t ${thunk_name}(void *obj) {
    void *raw = resolve_symbol("${mangled_symbol}");
    if (raw == NULL) {
        return INT32_MIN;
    }

#if defined(__aarch64__)
    __asm__ volatile(
        "mov ${self_register}, %[self]\\n"
        "blr %[fn]\\n"
        :
        : [self] "r"(obj), [fn] "r"(raw)
        : "x0", "${self_register}", "x30", "memory");
    return 0;
#else
    (void)obj;
    return INT32_MIN;
#endif
}

THUNK
      ;;
    *)
      echo "Unsupported signature '${signature}' for thunk '${thunk_name}'" >&2
      exit 1
      ;;
  esac
done < "$SPEC_FILE"

echo "Generated $OUT_FILE"
