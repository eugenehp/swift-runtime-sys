use std::collections::HashMap;
use std::process::Command;

/// Thread-unsafe cache for demangled symbol names.
/// In production, use parking_lot::Mutex or std::sync::Mutex for thread safety.
pub struct SymbolDemangler {
    cache: HashMap<String, String>,
    swift_demangle_path: String,
}

impl SymbolDemangler {
    /// Create a new demangler with the system swift-demangle tool.
    pub fn new() -> Result<Self, String> {
        let path = "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swift-demangle";
        
        // Verify the tool exists
        if !std::path::Path::new(path).exists() {
            return Err("swift-demangle tool not found at standard Xcode location".to_string());
        }
        
        Ok(SymbolDemangler {
            cache: HashMap::new(),
            swift_demangle_path: path.to_string(),
        })
    }

    /// Demangle a single Swift symbol name.
    /// Returns the demangled form, or the original symbol if demangling fails.
    pub fn demangle(&mut self, mangled: &str) -> String {
        // Check cache first
        if let Some(cached) = self.cache.get(mangled) {
            return cached.clone();
        }

        // Call swift-demangle subprocess
        let result = Command::new(&self.swift_demangle_path)
            .arg(mangled)
            .output();

        let demangled = match result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                // swift-demangle outputs: "mangled_name ---> Module.DemangleName(...)"
                // We extract the part after the arrow
                if let Some(arrow_pos) = stdout.find("--->") {
                    let after_arrow = stdout[arrow_pos + 4..].trim();
                    
                    // Extract the meaningful part (skip module prefix if present)
                    // Format is often: "ModuleName.functionName(...)" or just "functionName(...)"
                    // We want to extract just the readable part
                    if !after_arrow.is_empty() {
                        after_arrow.to_string()
                    } else {
                        mangled.to_string()
                    }
                } else {
                    // No arrow found, return original or parsed output
                    stdout.trim().to_string()
                }
            }
            Err(_) => mangled.to_string(),
        };

        self.cache.insert(mangled.to_string(), demangled.clone());
        demangled
    }

    /// Batch demangle multiple symbols.
    pub fn demangle_batch(&mut self, mangled_syms: &[&str]) -> Vec<String> {
        mangled_syms.iter().map(|m| self.demangle(m)).collect()
    }

    /// Get cache hit count for diagnostics.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Clear the cache if needed.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demangle_any_wrap() {
        let mut demangler = SymbolDemangler::new().expect("failed to create demangler");
        let mangled = "_$s10RustBridge23swift_contract_any_wrapySvSgs5Int32V_ACtF";
        let demangled = demangler.demangle(mangled);
        
        // Should contain readable parts like "any_wrap" and "Int32"
        assert!(demangled.contains("any_wrap"), "demangled: {}", demangled);
        assert!(demangled.contains("Int32"), "demangled: {}", demangled);
    }

    #[test]
    fn test_cache() {
        let mut demangler = SymbolDemangler::new().expect("failed to create demangler");
        let mangled = "_$s10RustBridge23swift_contract_any_wrapySvSgs5Int32V_ACtF";
        
        // First call (cache miss)
        let result1 = demangler.demangle(mangled);
        assert_eq!(demangler.cache_size(), 1);
        
        // Second call (cache hit)
        let result2 = demangler.demangle(mangled);
        assert_eq!(result1, result2);
        assert_eq!(demangler.cache_size(), 1); // Still 1, was cached
    }
}
