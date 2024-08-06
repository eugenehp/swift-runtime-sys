#[repr(C)]
pub struct _Tp {
    pub _address: u8,
}

pub type _Pred = ();

#[allow(improper_ctypes, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[repr(C)]
    #[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct __BindgenBitfieldUnit<Storage> {
        storage: Storage,
    }
    impl<Storage> __BindgenBitfieldUnit<Storage> {
        #[inline]
        pub const fn new(storage: Storage) -> Self {
            Self { storage }
        }
    }
    impl<Storage> __BindgenBitfieldUnit<Storage>
    where
        Storage: AsRef<[u8]> + AsMut<[u8]>,
    {
        #[inline]
        pub fn get_bit(&self, index: usize) -> bool {
            debug_assert!(index / 8 < self.storage.as_ref().len());
            let byte_index = index / 8;
            let byte = self.storage.as_ref()[byte_index];
            let bit_index = if cfg!(target_endian = "big") {
                7 - (index % 8)
            } else {
                index % 8
            };
            let mask = 1 << bit_index;
            byte & mask == mask
        }
        #[inline]
        pub fn set_bit(&mut self, index: usize, val: bool) {
            debug_assert!(index / 8 < self.storage.as_ref().len());
            let byte_index = index / 8;
            let byte = &mut self.storage.as_mut()[byte_index];
            let bit_index = if cfg!(target_endian = "big") {
                7 - (index % 8)
            } else {
                index % 8
            };
            let mask = 1 << bit_index;
            if val {
                *byte |= mask;
            } else {
                *byte &= !mask;
            }
        }
        #[inline]
        pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
            debug_assert!(bit_width <= 64);
            debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
            debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
            let mut val = 0;
            for i in 0..(bit_width as usize) {
                if self.get_bit(i + bit_offset) {
                    let index = if cfg!(target_endian = "big") {
                        bit_width as usize - 1 - i
                    } else {
                        i
                    };
                    val |= 1 << index;
                }
            }
            val
        }
        #[inline]
        pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
            debug_assert!(bit_width <= 64);
            debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
            debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
            for i in 0..(bit_width as usize) {
                let mask = 1 << i;
                let val_bit_is_set = val & mask == mask;
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                self.set_bit(index + bit_offset, val_bit_is_set);
            }
        }
    }
    #[allow(unused_imports)]
    use self::super::root;
    pub const _LIBCPP_ABI_VERSION: u32 = 1;
    pub const _LIBCPP_ENABLE_ASSERTIONS_DEFAULT: u32 = 0;
    pub const _LIBCPP_ENABLE_HARDENED_MODE_DEFAULT: u32 = 0;
    pub const _LIBCPP_ENABLE_DEBUG_MODE_DEFAULT: u32 = 0;
    pub const _LIBCPP_VERSION: u32 = 170006;
    pub const _LIBCPP_STD_VER: u32 = 14;
    pub const _LIBCPP_OBJECT_FORMAT_MACHO: u32 = 1;
    pub const _LIBCPP_ENABLE_ASSERTIONS: u32 = 0;
    pub const _LIBCPP_ENABLE_HARDENED_MODE: u32 = 0;
    pub const _LIBCPP_ENABLE_DEBUG_MODE: u32 = 0;
    pub const _LIBCPP_LOCALE__L_EXTENSIONS: u32 = 1;
    pub const SWIFT_COMPILER_IS_MSVC: u32 = 0;
    pub const SWIFT_BUG_REPORT_URL: &[u8; 47] = b"https://swift.org/contributing/#reporting-bugs\0";
    pub const SWIFT_BUG_REPORT_MESSAGE_BASE: &[u8; 69] =
        b"submit a bug report (https://swift.org/contributing/#reporting-bugs)\0";
    pub const SWIFT_BUG_REPORT_MESSAGE: &[u8; 76] =
        b"please submit a bug report (https://swift.org/contributing/#reporting-bugs)\0";
    pub const SWIFT_CRASH_BUG_REPORT_MESSAGE : & [u8 ; 109] = b"Please submit a bug report (https://swift.org/contributing/#reporting-bugs) and include the crash backtrace.\0" ;
    pub const SWIFT_POINTER_IS_8_BYTES: u32 = 1;
    pub const SWIFT_POINTER_IS_4_BYTES: u32 = 0;
    pub const SWIFT_OBJC_INTEROP: u32 = 1;
    pub const SWIFT_HAS_ISA_MASKING: u32 = 1;
    pub const SWIFT_HAS_OPAQUE_ISAS: u32 = 0;
    pub const SWIFT_STDLIB_HAS_MALLOC_TYPE: u32 = 0;
    pub const SWIFT_CLASS_IS_SWIFT_MASK: u32 = 2;
    pub const SWIFT_IMAGE_EXPORTS_swiftCore: u32 = 0;
    pub const SWIFT_IMAGE_EXPORTS_swift_Concurrency: u32 = 0;
    pub const SWIFT_IMAGE_EXPORTS_swiftDistributed: u32 = 0;
    pub const SWIFT_IMAGE_EXPORTS_swift_Differentiation: u32 = 0;
    pub const SWIFT_COMPACT_ABSOLUTE_FUNCTION_POINTER: u32 = 0;
    pub const SWIFT_PTRAUTH: u32 = 0;
    pub const DYNAMIC_TARGETS_ENABLED: u32 = 0;
    pub const TARGET_OS_MAC: u32 = 1;
    pub const TARGET_OS_OSX: u32 = 1;
    pub const TARGET_OS_IPHONE: u32 = 0;
    pub const TARGET_OS_IOS: u32 = 0;
    pub const TARGET_OS_WATCH: u32 = 0;
    pub const TARGET_OS_TV: u32 = 0;
    pub const TARGET_OS_MACCATALYST: u32 = 0;
    pub const TARGET_OS_UIKITFORMAC: u32 = 0;
    pub const TARGET_OS_SIMULATOR: u32 = 0;
    pub const TARGET_OS_EMBEDDED: u32 = 0;
    pub const TARGET_OS_UNIX: u32 = 0;
    pub const TARGET_OS_RTKIT: u32 = 0;
    pub const TARGET_RT_LITTLE_ENDIAN: u32 = 1;
    pub const TARGET_RT_BIG_ENDIAN: u32 = 0;
    pub const TARGET_RT_64_BIT: u32 = 1;
    pub const TARGET_RT_MAC_CFM: u32 = 0;
    pub const TARGET_RT_MAC_MACHO: u32 = 1;
    pub const TARGET_CPU_ARM64: u32 = 1;
    pub const TARGET_OS_VISION: u32 = 0;
    pub const TARGET_OS_DRIVERKIT: u32 = 0;
    pub const TARGET_OS_WIN32: u32 = 0;
    pub const TARGET_OS_WINDOWS: u32 = 0;
    pub const TARGET_OS_LINUX: u32 = 0;
    pub const TARGET_CPU_PPC: u32 = 0;
    pub const TARGET_CPU_PPC64: u32 = 0;
    pub const TARGET_CPU_68K: u32 = 0;
    pub const TARGET_CPU_X86: u32 = 0;
    pub const TARGET_CPU_X86_64: u32 = 0;
    pub const TARGET_CPU_ARM: u32 = 0;
    pub const TARGET_CPU_MIPS: u32 = 0;
    pub const TARGET_CPU_SPARC: u32 = 0;
    pub const TARGET_CPU_ALPHA: u32 = 0;
    pub const TARGET_IPHONE_SIMULATOR: u32 = 0;
    pub const TARGET_OS_NANO: u32 = 0;
    pub const SWIFT_BACKTRACE_ON_CRASH_SUPPORTED: u32 = 1;
    pub const SWIFT_BACKTRACE_SECTION: &[u8; 24] = b"__DATA,swift5_backtrace\0";
    pub const SWIFT_PAGE_SIZE: u32 = 16384;
    pub const __has_safe_buffers: u32 = 1;
    pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 1;
    pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
    pub const __DARWIN_ONLY_VERS_1050: u32 = 1;
    pub const __DARWIN_UNIX03: u32 = 1;
    pub const __DARWIN_64_BIT_INO_T: u32 = 1;
    pub const __DARWIN_VERS_1050: u32 = 1;
    pub const __DARWIN_NON_CANCELABLE: u32 = 0;
    pub const __DARWIN_SUF_EXTSN: &[u8; 14] = b"$DARWIN_EXTSN\0";
    pub const __DARWIN_C_ANSI: u32 = 4096;
    pub const __DARWIN_C_FULL: u32 = 900000;
    pub const __DARWIN_C_LEVEL: u32 = 900000;
    pub const __DARWIN_NO_LONG_LONG: u32 = 0;
    pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
    pub const _DARWIN_FEATURE_ONLY_64_BIT_INODE: u32 = 1;
    pub const _DARWIN_FEATURE_ONLY_VERS_1050: u32 = 1;
    pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
    pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
    pub const __has_ptrcheck: u32 = 0;
    pub const __API_TO_BE_DEPRECATED: u32 = 100000;
    pub const __API_TO_BE_DEPRECATED_MACOS: u32 = 100000;
    pub const __API_TO_BE_DEPRECATED_IOS: u32 = 100000;
    pub const __API_TO_BE_DEPRECATED_MACCATALYST: u32 = 100000;
    pub const __API_TO_BE_DEPRECATED_WATCHOS: u32 = 100000;
    pub const __API_TO_BE_DEPRECATED_TVOS: u32 = 100000;
    pub const __API_TO_BE_DEPRECATED_DRIVERKIT: u32 = 100000;
    pub const __API_TO_BE_DEPRECATED_VISIONOS: u32 = 100000;
    pub const __MAC_10_0: u32 = 1000;
    pub const __MAC_10_1: u32 = 1010;
    pub const __MAC_10_2: u32 = 1020;
    pub const __MAC_10_3: u32 = 1030;
    pub const __MAC_10_4: u32 = 1040;
    pub const __MAC_10_5: u32 = 1050;
    pub const __MAC_10_6: u32 = 1060;
    pub const __MAC_10_7: u32 = 1070;
    pub const __MAC_10_8: u32 = 1080;
    pub const __MAC_10_9: u32 = 1090;
    pub const __MAC_10_10: u32 = 101000;
    pub const __MAC_10_10_2: u32 = 101002;
    pub const __MAC_10_10_3: u32 = 101003;
    pub const __MAC_10_11: u32 = 101100;
    pub const __MAC_10_11_2: u32 = 101102;
    pub const __MAC_10_11_3: u32 = 101103;
    pub const __MAC_10_11_4: u32 = 101104;
    pub const __MAC_10_12: u32 = 101200;
    pub const __MAC_10_12_1: u32 = 101201;
    pub const __MAC_10_12_2: u32 = 101202;
    pub const __MAC_10_12_4: u32 = 101204;
    pub const __MAC_10_13: u32 = 101300;
    pub const __MAC_10_13_1: u32 = 101301;
    pub const __MAC_10_13_2: u32 = 101302;
    pub const __MAC_10_13_4: u32 = 101304;
    pub const __MAC_10_14: u32 = 101400;
    pub const __MAC_10_14_1: u32 = 101401;
    pub const __MAC_10_14_4: u32 = 101404;
    pub const __MAC_10_14_5: u32 = 101405;
    pub const __MAC_10_14_6: u32 = 101406;
    pub const __MAC_10_15: u32 = 101500;
    pub const __MAC_10_15_1: u32 = 101501;
    pub const __MAC_10_15_4: u32 = 101504;
    pub const __MAC_10_16: u32 = 101600;
    pub const __MAC_11_0: u32 = 110000;
    pub const __MAC_11_1: u32 = 110100;
    pub const __MAC_11_3: u32 = 110300;
    pub const __MAC_11_4: u32 = 110400;
    pub const __MAC_11_5: u32 = 110500;
    pub const __MAC_11_6: u32 = 110600;
    pub const __MAC_12_0: u32 = 120000;
    pub const __MAC_12_1: u32 = 120100;
    pub const __MAC_12_2: u32 = 120200;
    pub const __MAC_12_3: u32 = 120300;
    pub const __MAC_12_4: u32 = 120400;
    pub const __MAC_12_5: u32 = 120500;
    pub const __MAC_12_6: u32 = 120600;
    pub const __MAC_12_7: u32 = 120700;
    pub const __MAC_13_0: u32 = 130000;
    pub const __MAC_13_1: u32 = 130100;
    pub const __MAC_13_2: u32 = 130200;
    pub const __MAC_13_3: u32 = 130300;
    pub const __MAC_13_4: u32 = 130400;
    pub const __MAC_13_5: u32 = 130500;
    pub const __MAC_13_6: u32 = 130600;
    pub const __MAC_14_0: u32 = 140000;
    pub const __MAC_14_1: u32 = 140100;
    pub const __MAC_14_2: u32 = 140200;
    pub const __MAC_14_3: u32 = 140300;
    pub const __MAC_14_4: u32 = 140400;
    pub const __MAC_14_5: u32 = 140500;
    pub const __IPHONE_2_0: u32 = 20000;
    pub const __IPHONE_2_1: u32 = 20100;
    pub const __IPHONE_2_2: u32 = 20200;
    pub const __IPHONE_3_0: u32 = 30000;
    pub const __IPHONE_3_1: u32 = 30100;
    pub const __IPHONE_3_2: u32 = 30200;
    pub const __IPHONE_4_0: u32 = 40000;
    pub const __IPHONE_4_1: u32 = 40100;
    pub const __IPHONE_4_2: u32 = 40200;
    pub const __IPHONE_4_3: u32 = 40300;
    pub const __IPHONE_5_0: u32 = 50000;
    pub const __IPHONE_5_1: u32 = 50100;
    pub const __IPHONE_6_0: u32 = 60000;
    pub const __IPHONE_6_1: u32 = 60100;
    pub const __IPHONE_7_0: u32 = 70000;
    pub const __IPHONE_7_1: u32 = 70100;
    pub const __IPHONE_8_0: u32 = 80000;
    pub const __IPHONE_8_1: u32 = 80100;
    pub const __IPHONE_8_2: u32 = 80200;
    pub const __IPHONE_8_3: u32 = 80300;
    pub const __IPHONE_8_4: u32 = 80400;
    pub const __IPHONE_9_0: u32 = 90000;
    pub const __IPHONE_9_1: u32 = 90100;
    pub const __IPHONE_9_2: u32 = 90200;
    pub const __IPHONE_9_3: u32 = 90300;
    pub const __IPHONE_10_0: u32 = 100000;
    pub const __IPHONE_10_1: u32 = 100100;
    pub const __IPHONE_10_2: u32 = 100200;
    pub const __IPHONE_10_3: u32 = 100300;
    pub const __IPHONE_11_0: u32 = 110000;
    pub const __IPHONE_11_1: u32 = 110100;
    pub const __IPHONE_11_2: u32 = 110200;
    pub const __IPHONE_11_3: u32 = 110300;
    pub const __IPHONE_11_4: u32 = 110400;
    pub const __IPHONE_12_0: u32 = 120000;
    pub const __IPHONE_12_1: u32 = 120100;
    pub const __IPHONE_12_2: u32 = 120200;
    pub const __IPHONE_12_3: u32 = 120300;
    pub const __IPHONE_12_4: u32 = 120400;
    pub const __IPHONE_13_0: u32 = 130000;
    pub const __IPHONE_13_1: u32 = 130100;
    pub const __IPHONE_13_2: u32 = 130200;
    pub const __IPHONE_13_3: u32 = 130300;
    pub const __IPHONE_13_4: u32 = 130400;
    pub const __IPHONE_13_5: u32 = 130500;
    pub const __IPHONE_13_6: u32 = 130600;
    pub const __IPHONE_13_7: u32 = 130700;
    pub const __IPHONE_14_0: u32 = 140000;
    pub const __IPHONE_14_1: u32 = 140100;
    pub const __IPHONE_14_2: u32 = 140200;
    pub const __IPHONE_14_3: u32 = 140300;
    pub const __IPHONE_14_5: u32 = 140500;
    pub const __IPHONE_14_4: u32 = 140400;
    pub const __IPHONE_14_6: u32 = 140600;
    pub const __IPHONE_14_7: u32 = 140700;
    pub const __IPHONE_14_8: u32 = 140800;
    pub const __IPHONE_15_0: u32 = 150000;
    pub const __IPHONE_15_1: u32 = 150100;
    pub const __IPHONE_15_2: u32 = 150200;
    pub const __IPHONE_15_3: u32 = 150300;
    pub const __IPHONE_15_4: u32 = 150400;
    pub const __IPHONE_15_5: u32 = 150500;
    pub const __IPHONE_15_6: u32 = 150600;
    pub const __IPHONE_15_7: u32 = 150700;
    pub const __IPHONE_15_8: u32 = 150800;
    pub const __IPHONE_16_0: u32 = 160000;
    pub const __IPHONE_16_1: u32 = 160100;
    pub const __IPHONE_16_2: u32 = 160200;
    pub const __IPHONE_16_3: u32 = 160300;
    pub const __IPHONE_16_4: u32 = 160400;
    pub const __IPHONE_16_5: u32 = 160500;
    pub const __IPHONE_16_6: u32 = 160600;
    pub const __IPHONE_16_7: u32 = 160700;
    pub const __IPHONE_17_0: u32 = 170000;
    pub const __IPHONE_17_1: u32 = 170100;
    pub const __IPHONE_17_2: u32 = 170200;
    pub const __IPHONE_17_3: u32 = 170300;
    pub const __IPHONE_17_4: u32 = 170400;
    pub const __IPHONE_17_5: u32 = 170500;
    pub const __WATCHOS_1_0: u32 = 10000;
    pub const __WATCHOS_2_0: u32 = 20000;
    pub const __WATCHOS_2_1: u32 = 20100;
    pub const __WATCHOS_2_2: u32 = 20200;
    pub const __WATCHOS_3_0: u32 = 30000;
    pub const __WATCHOS_3_1: u32 = 30100;
    pub const __WATCHOS_3_1_1: u32 = 30101;
    pub const __WATCHOS_3_2: u32 = 30200;
    pub const __WATCHOS_4_0: u32 = 40000;
    pub const __WATCHOS_4_1: u32 = 40100;
    pub const __WATCHOS_4_2: u32 = 40200;
    pub const __WATCHOS_4_3: u32 = 40300;
    pub const __WATCHOS_5_0: u32 = 50000;
    pub const __WATCHOS_5_1: u32 = 50100;
    pub const __WATCHOS_5_2: u32 = 50200;
    pub const __WATCHOS_5_3: u32 = 50300;
    pub const __WATCHOS_6_0: u32 = 60000;
    pub const __WATCHOS_6_1: u32 = 60100;
    pub const __WATCHOS_6_2: u32 = 60200;
    pub const __WATCHOS_7_0: u32 = 70000;
    pub const __WATCHOS_7_1: u32 = 70100;
    pub const __WATCHOS_7_2: u32 = 70200;
    pub const __WATCHOS_7_3: u32 = 70300;
    pub const __WATCHOS_7_4: u32 = 70400;
    pub const __WATCHOS_7_5: u32 = 70500;
    pub const __WATCHOS_7_6: u32 = 70600;
    pub const __WATCHOS_8_0: u32 = 80000;
    pub const __WATCHOS_8_1: u32 = 80100;
    pub const __WATCHOS_8_3: u32 = 80300;
    pub const __WATCHOS_8_4: u32 = 80400;
    pub const __WATCHOS_8_5: u32 = 80500;
    pub const __WATCHOS_8_6: u32 = 80600;
    pub const __WATCHOS_8_7: u32 = 80700;
    pub const __WATCHOS_8_8: u32 = 80800;
    pub const __WATCHOS_9_0: u32 = 90000;
    pub const __WATCHOS_9_1: u32 = 90100;
    pub const __WATCHOS_9_2: u32 = 90200;
    pub const __WATCHOS_9_3: u32 = 90300;
    pub const __WATCHOS_9_4: u32 = 90400;
    pub const __WATCHOS_9_5: u32 = 90500;
    pub const __WATCHOS_9_6: u32 = 90600;
    pub const __WATCHOS_10_0: u32 = 100000;
    pub const __WATCHOS_10_1: u32 = 100100;
    pub const __WATCHOS_10_2: u32 = 100200;
    pub const __WATCHOS_10_3: u32 = 100300;
    pub const __WATCHOS_10_4: u32 = 100400;
    pub const __WATCHOS_10_5: u32 = 100500;
    pub const __TVOS_9_0: u32 = 90000;
    pub const __TVOS_9_1: u32 = 90100;
    pub const __TVOS_9_2: u32 = 90200;
    pub const __TVOS_10_0: u32 = 100000;
    pub const __TVOS_10_0_1: u32 = 100001;
    pub const __TVOS_10_1: u32 = 100100;
    pub const __TVOS_10_2: u32 = 100200;
    pub const __TVOS_11_0: u32 = 110000;
    pub const __TVOS_11_1: u32 = 110100;
    pub const __TVOS_11_2: u32 = 110200;
    pub const __TVOS_11_3: u32 = 110300;
    pub const __TVOS_11_4: u32 = 110400;
    pub const __TVOS_12_0: u32 = 120000;
    pub const __TVOS_12_1: u32 = 120100;
    pub const __TVOS_12_2: u32 = 120200;
    pub const __TVOS_12_3: u32 = 120300;
    pub const __TVOS_12_4: u32 = 120400;
    pub const __TVOS_13_0: u32 = 130000;
    pub const __TVOS_13_2: u32 = 130200;
    pub const __TVOS_13_3: u32 = 130300;
    pub const __TVOS_13_4: u32 = 130400;
    pub const __TVOS_14_0: u32 = 140000;
    pub const __TVOS_14_1: u32 = 140100;
    pub const __TVOS_14_2: u32 = 140200;
    pub const __TVOS_14_3: u32 = 140300;
    pub const __TVOS_14_5: u32 = 140500;
    pub const __TVOS_14_6: u32 = 140600;
    pub const __TVOS_14_7: u32 = 140700;
    pub const __TVOS_15_0: u32 = 150000;
    pub const __TVOS_15_1: u32 = 150100;
    pub const __TVOS_15_2: u32 = 150200;
    pub const __TVOS_15_3: u32 = 150300;
    pub const __TVOS_15_4: u32 = 150400;
    pub const __TVOS_15_5: u32 = 150500;
    pub const __TVOS_15_6: u32 = 150600;
    pub const __TVOS_16_0: u32 = 160000;
    pub const __TVOS_16_1: u32 = 160100;
    pub const __TVOS_16_2: u32 = 160200;
    pub const __TVOS_16_3: u32 = 160300;
    pub const __TVOS_16_4: u32 = 160400;
    pub const __TVOS_16_5: u32 = 160500;
    pub const __TVOS_16_6: u32 = 160600;
    pub const __TVOS_17_0: u32 = 170000;
    pub const __TVOS_17_1: u32 = 170100;
    pub const __TVOS_17_2: u32 = 170200;
    pub const __TVOS_17_3: u32 = 170300;
    pub const __TVOS_17_4: u32 = 170400;
    pub const __TVOS_17_5: u32 = 170500;
    pub const __BRIDGEOS_2_0: u32 = 20000;
    pub const __BRIDGEOS_3_0: u32 = 30000;
    pub const __BRIDGEOS_3_1: u32 = 30100;
    pub const __BRIDGEOS_3_4: u32 = 30400;
    pub const __BRIDGEOS_4_0: u32 = 40000;
    pub const __BRIDGEOS_4_1: u32 = 40100;
    pub const __BRIDGEOS_5_0: u32 = 50000;
    pub const __BRIDGEOS_5_1: u32 = 50100;
    pub const __BRIDGEOS_5_3: u32 = 50300;
    pub const __BRIDGEOS_6_0: u32 = 60000;
    pub const __BRIDGEOS_6_2: u32 = 60200;
    pub const __BRIDGEOS_6_4: u32 = 60400;
    pub const __BRIDGEOS_6_5: u32 = 60500;
    pub const __BRIDGEOS_6_6: u32 = 60600;
    pub const __BRIDGEOS_7_0: u32 = 70000;
    pub const __BRIDGEOS_7_1: u32 = 70100;
    pub const __BRIDGEOS_7_2: u32 = 70200;
    pub const __BRIDGEOS_7_3: u32 = 70300;
    pub const __BRIDGEOS_7_4: u32 = 70400;
    pub const __BRIDGEOS_7_6: u32 = 70600;
    pub const __BRIDGEOS_8_0: u32 = 80000;
    pub const __BRIDGEOS_8_1: u32 = 80100;
    pub const __BRIDGEOS_8_2: u32 = 80200;
    pub const __BRIDGEOS_8_3: u32 = 80300;
    pub const __BRIDGEOS_8_4: u32 = 80400;
    pub const __BRIDGEOS_8_5: u32 = 80500;
    pub const __DRIVERKIT_19_0: u32 = 190000;
    pub const __DRIVERKIT_20_0: u32 = 200000;
    pub const __DRIVERKIT_21_0: u32 = 210000;
    pub const __DRIVERKIT_22_0: u32 = 220000;
    pub const __DRIVERKIT_22_4: u32 = 220400;
    pub const __DRIVERKIT_22_5: u32 = 220500;
    pub const __DRIVERKIT_22_6: u32 = 220600;
    pub const __DRIVERKIT_23_0: u32 = 230000;
    pub const __DRIVERKIT_23_1: u32 = 230100;
    pub const __DRIVERKIT_23_2: u32 = 230200;
    pub const __DRIVERKIT_23_3: u32 = 230300;
    pub const __DRIVERKIT_23_4: u32 = 230400;
    pub const __DRIVERKIT_23_5: u32 = 230500;
    pub const __VISIONOS_1_0: u32 = 10000;
    pub const __VISIONOS_1_1: u32 = 10100;
    pub const __VISIONOS_1_2: u32 = 10200;
    pub const MAC_OS_X_VERSION_10_0: u32 = 1000;
    pub const MAC_OS_X_VERSION_10_1: u32 = 1010;
    pub const MAC_OS_X_VERSION_10_2: u32 = 1020;
    pub const MAC_OS_X_VERSION_10_3: u32 = 1030;
    pub const MAC_OS_X_VERSION_10_4: u32 = 1040;
    pub const MAC_OS_X_VERSION_10_5: u32 = 1050;
    pub const MAC_OS_X_VERSION_10_6: u32 = 1060;
    pub const MAC_OS_X_VERSION_10_7: u32 = 1070;
    pub const MAC_OS_X_VERSION_10_8: u32 = 1080;
    pub const MAC_OS_X_VERSION_10_9: u32 = 1090;
    pub const MAC_OS_X_VERSION_10_10: u32 = 101000;
    pub const MAC_OS_X_VERSION_10_10_2: u32 = 101002;
    pub const MAC_OS_X_VERSION_10_10_3: u32 = 101003;
    pub const MAC_OS_X_VERSION_10_11: u32 = 101100;
    pub const MAC_OS_X_VERSION_10_11_2: u32 = 101102;
    pub const MAC_OS_X_VERSION_10_11_3: u32 = 101103;
    pub const MAC_OS_X_VERSION_10_11_4: u32 = 101104;
    pub const MAC_OS_X_VERSION_10_12: u32 = 101200;
    pub const MAC_OS_X_VERSION_10_12_1: u32 = 101201;
    pub const MAC_OS_X_VERSION_10_12_2: u32 = 101202;
    pub const MAC_OS_X_VERSION_10_12_4: u32 = 101204;
    pub const MAC_OS_X_VERSION_10_13: u32 = 101300;
    pub const MAC_OS_X_VERSION_10_13_1: u32 = 101301;
    pub const MAC_OS_X_VERSION_10_13_2: u32 = 101302;
    pub const MAC_OS_X_VERSION_10_13_4: u32 = 101304;
    pub const MAC_OS_X_VERSION_10_14: u32 = 101400;
    pub const MAC_OS_X_VERSION_10_14_1: u32 = 101401;
    pub const MAC_OS_X_VERSION_10_14_4: u32 = 101404;
    pub const MAC_OS_X_VERSION_10_14_5: u32 = 101405;
    pub const MAC_OS_X_VERSION_10_14_6: u32 = 101406;
    pub const MAC_OS_X_VERSION_10_15: u32 = 101500;
    pub const MAC_OS_X_VERSION_10_15_1: u32 = 101501;
    pub const MAC_OS_X_VERSION_10_15_4: u32 = 101504;
    pub const MAC_OS_X_VERSION_10_16: u32 = 101600;
    pub const MAC_OS_VERSION_11_0: u32 = 110000;
    pub const MAC_OS_VERSION_11_1: u32 = 110100;
    pub const MAC_OS_VERSION_11_3: u32 = 110300;
    pub const MAC_OS_VERSION_11_4: u32 = 110400;
    pub const MAC_OS_VERSION_11_5: u32 = 110500;
    pub const MAC_OS_VERSION_11_6: u32 = 110600;
    pub const MAC_OS_VERSION_12_0: u32 = 120000;
    pub const MAC_OS_VERSION_12_1: u32 = 120100;
    pub const MAC_OS_VERSION_12_2: u32 = 120200;
    pub const MAC_OS_VERSION_12_3: u32 = 120300;
    pub const MAC_OS_VERSION_12_4: u32 = 120400;
    pub const MAC_OS_VERSION_12_5: u32 = 120500;
    pub const MAC_OS_VERSION_12_6: u32 = 120600;
    pub const MAC_OS_VERSION_12_7: u32 = 120700;
    pub const MAC_OS_VERSION_13_0: u32 = 130000;
    pub const MAC_OS_VERSION_13_1: u32 = 130100;
    pub const MAC_OS_VERSION_13_2: u32 = 130200;
    pub const MAC_OS_VERSION_13_3: u32 = 130300;
    pub const MAC_OS_VERSION_13_4: u32 = 130400;
    pub const MAC_OS_VERSION_13_5: u32 = 130500;
    pub const MAC_OS_VERSION_13_6: u32 = 130600;
    pub const MAC_OS_VERSION_14_0: u32 = 140000;
    pub const MAC_OS_VERSION_14_1: u32 = 140100;
    pub const MAC_OS_VERSION_14_2: u32 = 140200;
    pub const MAC_OS_VERSION_14_3: u32 = 140300;
    pub const MAC_OS_VERSION_14_4: u32 = 140400;
    pub const MAC_OS_VERSION_14_5: u32 = 140500;
    pub const __MAC_OS_X_VERSION_MAX_ALLOWED: u32 = 140500;
    pub const __ENABLE_LEGACY_MAC_AVAILABILITY: u32 = 1;
    pub const __PTHREAD_SIZE__: u32 = 8176;
    pub const __PTHREAD_ATTR_SIZE__: u32 = 56;
    pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
    pub const __PTHREAD_MUTEX_SIZE__: u32 = 56;
    pub const __PTHREAD_CONDATTR_SIZE__: u32 = 8;
    pub const __PTHREAD_COND_SIZE__: u32 = 40;
    pub const __PTHREAD_ONCE_SIZE__: u32 = 8;
    pub const __PTHREAD_RWLOCK_SIZE__: u32 = 192;
    pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 16;
    pub const __DARWIN_WCHAR_MIN: i32 = -2147483648;
    pub const _FORTIFY_SOURCE: u32 = 2;
    pub const __DARWIN_NSIG: u32 = 32;
    pub const NSIG: u32 = 32;
    pub const _ARM_SIGNAL_: u32 = 1;
    pub const SIGHUP: u32 = 1;
    pub const SIGINT: u32 = 2;
    pub const SIGQUIT: u32 = 3;
    pub const SIGILL: u32 = 4;
    pub const SIGTRAP: u32 = 5;
    pub const SIGABRT: u32 = 6;
    pub const SIGIOT: u32 = 6;
    pub const SIGEMT: u32 = 7;
    pub const SIGFPE: u32 = 8;
    pub const SIGKILL: u32 = 9;
    pub const SIGBUS: u32 = 10;
    pub const SIGSEGV: u32 = 11;
    pub const SIGSYS: u32 = 12;
    pub const SIGPIPE: u32 = 13;
    pub const SIGALRM: u32 = 14;
    pub const SIGTERM: u32 = 15;
    pub const SIGURG: u32 = 16;
    pub const SIGSTOP: u32 = 17;
    pub const SIGTSTP: u32 = 18;
    pub const SIGCONT: u32 = 19;
    pub const SIGCHLD: u32 = 20;
    pub const SIGTTIN: u32 = 21;
    pub const SIGTTOU: u32 = 22;
    pub const SIGIO: u32 = 23;
    pub const SIGXCPU: u32 = 24;
    pub const SIGXFSZ: u32 = 25;
    pub const SIGVTALRM: u32 = 26;
    pub const SIGPROF: u32 = 27;
    pub const SIGWINCH: u32 = 28;
    pub const SIGINFO: u32 = 29;
    pub const SIGUSR1: u32 = 30;
    pub const SIGUSR2: u32 = 31;
    pub const __DARWIN_OPAQUE_ARM_THREAD_STATE64: u32 = 0;
    pub const SIGEV_NONE: u32 = 0;
    pub const SIGEV_SIGNAL: u32 = 1;
    pub const SIGEV_THREAD: u32 = 3;
    pub const ILL_NOOP: u32 = 0;
    pub const ILL_ILLOPC: u32 = 1;
    pub const ILL_ILLTRP: u32 = 2;
    pub const ILL_PRVOPC: u32 = 3;
    pub const ILL_ILLOPN: u32 = 4;
    pub const ILL_ILLADR: u32 = 5;
    pub const ILL_PRVREG: u32 = 6;
    pub const ILL_COPROC: u32 = 7;
    pub const ILL_BADSTK: u32 = 8;
    pub const FPE_NOOP: u32 = 0;
    pub const FPE_FLTDIV: u32 = 1;
    pub const FPE_FLTOVF: u32 = 2;
    pub const FPE_FLTUND: u32 = 3;
    pub const FPE_FLTRES: u32 = 4;
    pub const FPE_FLTINV: u32 = 5;
    pub const FPE_FLTSUB: u32 = 6;
    pub const FPE_INTDIV: u32 = 7;
    pub const FPE_INTOVF: u32 = 8;
    pub const SEGV_NOOP: u32 = 0;
    pub const SEGV_MAPERR: u32 = 1;
    pub const SEGV_ACCERR: u32 = 2;
    pub const BUS_NOOP: u32 = 0;
    pub const BUS_ADRALN: u32 = 1;
    pub const BUS_ADRERR: u32 = 2;
    pub const BUS_OBJERR: u32 = 3;
    pub const TRAP_BRKPT: u32 = 1;
    pub const TRAP_TRACE: u32 = 2;
    pub const CLD_NOOP: u32 = 0;
    pub const CLD_EXITED: u32 = 1;
    pub const CLD_KILLED: u32 = 2;
    pub const CLD_DUMPED: u32 = 3;
    pub const CLD_TRAPPED: u32 = 4;
    pub const CLD_STOPPED: u32 = 5;
    pub const CLD_CONTINUED: u32 = 6;
    pub const POLL_IN: u32 = 1;
    pub const POLL_OUT: u32 = 2;
    pub const POLL_MSG: u32 = 3;
    pub const POLL_ERR: u32 = 4;
    pub const POLL_PRI: u32 = 5;
    pub const POLL_HUP: u32 = 6;
    pub const SA_ONSTACK: u32 = 1;
    pub const SA_RESTART: u32 = 2;
    pub const SA_RESETHAND: u32 = 4;
    pub const SA_NOCLDSTOP: u32 = 8;
    pub const SA_NODEFER: u32 = 16;
    pub const SA_NOCLDWAIT: u32 = 32;
    pub const SA_SIGINFO: u32 = 64;
    pub const SA_USERTRAMP: u32 = 256;
    pub const SA_64REGSET: u32 = 512;
    pub const SA_USERSPACE_MASK: u32 = 127;
    pub const SIG_BLOCK: u32 = 1;
    pub const SIG_UNBLOCK: u32 = 2;
    pub const SIG_SETMASK: u32 = 3;
    pub const SI_USER: u32 = 65537;
    pub const SI_QUEUE: u32 = 65538;
    pub const SI_TIMER: u32 = 65539;
    pub const SI_ASYNCIO: u32 = 65540;
    pub const SI_MESGQ: u32 = 65541;
    pub const SS_ONSTACK: u32 = 1;
    pub const SS_DISABLE: u32 = 4;
    pub const MINSIGSTKSZ: u32 = 32768;
    pub const SIGSTKSZ: u32 = 131072;
    pub const SV_ONSTACK: u32 = 1;
    pub const SV_INTERRUPT: u32 = 2;
    pub const SV_RESETHAND: u32 = 4;
    pub const SV_NODEFER: u32 = 16;
    pub const SV_NOCLDSTOP: u32 = 8;
    pub const SV_SIGINFO: u32 = 64;
    pub const __WORDSIZE: u32 = 64;
    pub const INT8_MAX: u32 = 127;
    pub const INT16_MAX: u32 = 32767;
    pub const INT32_MAX: u32 = 2147483647;
    pub const INT64_MAX: u64 = 9223372036854775807;
    pub const INT8_MIN: i32 = -128;
    pub const INT16_MIN: i32 = -32768;
    pub const INT32_MIN: i32 = -2147483648;
    pub const INT64_MIN: i64 = -9223372036854775808;
    pub const UINT8_MAX: u32 = 255;
    pub const UINT16_MAX: u32 = 65535;
    pub const UINT32_MAX: u32 = 4294967295;
    pub const UINT64_MAX: i32 = -1;
    pub const INT_LEAST8_MIN: i32 = -128;
    pub const INT_LEAST16_MIN: i32 = -32768;
    pub const INT_LEAST32_MIN: i32 = -2147483648;
    pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
    pub const INT_LEAST8_MAX: u32 = 127;
    pub const INT_LEAST16_MAX: u32 = 32767;
    pub const INT_LEAST32_MAX: u32 = 2147483647;
    pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
    pub const UINT_LEAST8_MAX: u32 = 255;
    pub const UINT_LEAST16_MAX: u32 = 65535;
    pub const UINT_LEAST32_MAX: u32 = 4294967295;
    pub const UINT_LEAST64_MAX: i32 = -1;
    pub const INT_FAST8_MIN: i32 = -128;
    pub const INT_FAST16_MIN: i32 = -32768;
    pub const INT_FAST32_MIN: i32 = -2147483648;
    pub const INT_FAST64_MIN: i64 = -9223372036854775808;
    pub const INT_FAST8_MAX: u32 = 127;
    pub const INT_FAST16_MAX: u32 = 32767;
    pub const INT_FAST32_MAX: u32 = 2147483647;
    pub const INT_FAST64_MAX: u64 = 9223372036854775807;
    pub const UINT_FAST8_MAX: u32 = 255;
    pub const UINT_FAST16_MAX: u32 = 65535;
    pub const UINT_FAST32_MAX: u32 = 4294967295;
    pub const UINT_FAST64_MAX: i32 = -1;
    pub const INTPTR_MAX: u64 = 9223372036854775807;
    pub const INTPTR_MIN: i64 = -9223372036854775808;
    pub const UINTPTR_MAX: i32 = -1;
    pub const SIZE_MAX: i32 = -1;
    pub const WINT_MIN: i32 = -2147483648;
    pub const WINT_MAX: u32 = 2147483647;
    pub const SIG_ATOMIC_MIN: i32 = -2147483648;
    pub const SIG_ATOMIC_MAX: u32 = 2147483647;
    pub const PRIO_PROCESS: u32 = 0;
    pub const PRIO_PGRP: u32 = 1;
    pub const PRIO_USER: u32 = 2;
    pub const PRIO_DARWIN_THREAD: u32 = 3;
    pub const PRIO_DARWIN_PROCESS: u32 = 4;
    pub const PRIO_MIN: i32 = -20;
    pub const PRIO_MAX: u32 = 20;
    pub const PRIO_DARWIN_BG: u32 = 4096;
    pub const PRIO_DARWIN_NONUI: u32 = 4097;
    pub const RUSAGE_SELF: u32 = 0;
    pub const RUSAGE_CHILDREN: i32 = -1;
    pub const RUSAGE_INFO_V0: u32 = 0;
    pub const RUSAGE_INFO_V1: u32 = 1;
    pub const RUSAGE_INFO_V2: u32 = 2;
    pub const RUSAGE_INFO_V3: u32 = 3;
    pub const RUSAGE_INFO_V4: u32 = 4;
    pub const RUSAGE_INFO_V5: u32 = 5;
    pub const RUSAGE_INFO_V6: u32 = 6;
    pub const RUSAGE_INFO_CURRENT: u32 = 6;
    pub const RU_PROC_RUNS_RESLIDE: u32 = 1;
    pub const RLIMIT_CPU: u32 = 0;
    pub const RLIMIT_FSIZE: u32 = 1;
    pub const RLIMIT_DATA: u32 = 2;
    pub const RLIMIT_STACK: u32 = 3;
    pub const RLIMIT_CORE: u32 = 4;
    pub const RLIMIT_AS: u32 = 5;
    pub const RLIMIT_RSS: u32 = 5;
    pub const RLIMIT_MEMLOCK: u32 = 6;
    pub const RLIMIT_NPROC: u32 = 7;
    pub const RLIMIT_NOFILE: u32 = 8;
    pub const RLIM_NLIMITS: u32 = 9;
    pub const _RLIMIT_POSIX_FLAG: u32 = 4096;
    pub const RLIMIT_WAKEUPS_MONITOR: u32 = 1;
    pub const RLIMIT_CPU_USAGE_MONITOR: u32 = 2;
    pub const RLIMIT_THREAD_CPULIMITS: u32 = 3;
    pub const RLIMIT_FOOTPRINT_INTERVAL: u32 = 4;
    pub const WAKEMON_ENABLE: u32 = 1;
    pub const WAKEMON_DISABLE: u32 = 2;
    pub const WAKEMON_GET_PARAMS: u32 = 4;
    pub const WAKEMON_SET_DEFAULTS: u32 = 8;
    pub const WAKEMON_MAKE_FATAL: u32 = 16;
    pub const CPUMON_MAKE_FATAL: u32 = 4096;
    pub const FOOTPRINT_INTERVAL_RESET: u32 = 1;
    pub const IOPOL_TYPE_DISK: u32 = 0;
    pub const IOPOL_TYPE_VFS_ATIME_UPDATES: u32 = 2;
    pub const IOPOL_TYPE_VFS_MATERIALIZE_DATALESS_FILES: u32 = 3;
    pub const IOPOL_TYPE_VFS_STATFS_NO_DATA_VOLUME: u32 = 4;
    pub const IOPOL_TYPE_VFS_TRIGGER_RESOLVE: u32 = 5;
    pub const IOPOL_TYPE_VFS_IGNORE_CONTENT_PROTECTION: u32 = 6;
    pub const IOPOL_TYPE_VFS_IGNORE_PERMISSIONS: u32 = 7;
    pub const IOPOL_TYPE_VFS_SKIP_MTIME_UPDATE: u32 = 8;
    pub const IOPOL_TYPE_VFS_ALLOW_LOW_SPACE_WRITES: u32 = 9;
    pub const IOPOL_TYPE_VFS_DISALLOW_RW_FOR_O_EVTONLY: u32 = 10;
    pub const IOPOL_SCOPE_PROCESS: u32 = 0;
    pub const IOPOL_SCOPE_THREAD: u32 = 1;
    pub const IOPOL_SCOPE_DARWIN_BG: u32 = 2;
    pub const IOPOL_DEFAULT: u32 = 0;
    pub const IOPOL_IMPORTANT: u32 = 1;
    pub const IOPOL_PASSIVE: u32 = 2;
    pub const IOPOL_THROTTLE: u32 = 3;
    pub const IOPOL_UTILITY: u32 = 4;
    pub const IOPOL_STANDARD: u32 = 5;
    pub const IOPOL_APPLICATION: u32 = 5;
    pub const IOPOL_NORMAL: u32 = 1;
    pub const IOPOL_ATIME_UPDATES_DEFAULT: u32 = 0;
    pub const IOPOL_ATIME_UPDATES_OFF: u32 = 1;
    pub const IOPOL_MATERIALIZE_DATALESS_FILES_DEFAULT: u32 = 0;
    pub const IOPOL_MATERIALIZE_DATALESS_FILES_OFF: u32 = 1;
    pub const IOPOL_MATERIALIZE_DATALESS_FILES_ON: u32 = 2;
    pub const IOPOL_VFS_STATFS_NO_DATA_VOLUME_DEFAULT: u32 = 0;
    pub const IOPOL_VFS_STATFS_FORCE_NO_DATA_VOLUME: u32 = 1;
    pub const IOPOL_VFS_TRIGGER_RESOLVE_DEFAULT: u32 = 0;
    pub const IOPOL_VFS_TRIGGER_RESOLVE_OFF: u32 = 1;
    pub const IOPOL_VFS_CONTENT_PROTECTION_DEFAULT: u32 = 0;
    pub const IOPOL_VFS_CONTENT_PROTECTION_IGNORE: u32 = 1;
    pub const IOPOL_VFS_IGNORE_PERMISSIONS_OFF: u32 = 0;
    pub const IOPOL_VFS_IGNORE_PERMISSIONS_ON: u32 = 1;
    pub const IOPOL_VFS_SKIP_MTIME_UPDATE_OFF: u32 = 0;
    pub const IOPOL_VFS_SKIP_MTIME_UPDATE_ON: u32 = 1;
    pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_OFF: u32 = 0;
    pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_ON: u32 = 1;
    pub const IOPOL_VFS_DISALLOW_RW_FOR_O_EVTONLY_DEFAULT: u32 = 0;
    pub const IOPOL_VFS_DISALLOW_RW_FOR_O_EVTONLY_ON: u32 = 1;
    pub const IOPOL_VFS_NOCACHE_WRITE_FS_BLKSIZE_DEFAULT: u32 = 0;
    pub const IOPOL_VFS_NOCACHE_WRITE_FS_BLKSIZE_ON: u32 = 1;
    pub const WNOHANG: u32 = 1;
    pub const WUNTRACED: u32 = 2;
    pub const WCOREFLAG: u32 = 128;
    pub const _WSTOPPED: u32 = 127;
    pub const WEXITED: u32 = 4;
    pub const WSTOPPED: u32 = 8;
    pub const WCONTINUED: u32 = 16;
    pub const WNOWAIT: u32 = 32;
    pub const WAIT_ANY: i32 = -1;
    pub const WAIT_MYPGRP: u32 = 0;
    pub const _QUAD_HIGHWORD: u32 = 1;
    pub const _QUAD_LOWWORD: u32 = 0;
    pub const __DARWIN_LITTLE_ENDIAN: u32 = 1234;
    pub const __DARWIN_BIG_ENDIAN: u32 = 4321;
    pub const __DARWIN_PDP_ENDIAN: u32 = 3412;
    pub const __DARWIN_BYTE_ORDER: u32 = 1234;
    pub const LITTLE_ENDIAN: u32 = 1234;
    pub const BIG_ENDIAN: u32 = 4321;
    pub const PDP_ENDIAN: u32 = 3412;
    pub const BYTE_ORDER: u32 = 1234;
    pub const EXIT_FAILURE: u32 = 1;
    pub const EXIT_SUCCESS: u32 = 0;
    pub const RAND_MAX: u32 = 2147483647;
    pub const __cpp_lib_chrono_udls: u32 = 201304;
    pub const __cpp_lib_complex_udls: u32 = 201309;
    pub const __cpp_lib_exchange_function: u32 = 201304;
    pub const __cpp_lib_generic_associative_lookup: u32 = 201304;
    pub const __cpp_lib_integer_sequence: u32 = 201304;
    pub const __cpp_lib_integral_constant_callable: u32 = 201304;
    pub const __cpp_lib_is_final: u32 = 201402;
    pub const __cpp_lib_is_null_pointer: u32 = 201309;
    pub const __cpp_lib_make_reverse_iterator: u32 = 201402;
    pub const __cpp_lib_make_unique: u32 = 201304;
    pub const __cpp_lib_null_iterators: u32 = 201304;
    pub const __cpp_lib_quoted_string_io: u32 = 201304;
    pub const __cpp_lib_result_of_sfinae: u32 = 201210;
    pub const __cpp_lib_robust_nonmodifying_seq_ops: u32 = 201304;
    pub const __cpp_lib_shared_timed_mutex: u32 = 201402;
    pub const __cpp_lib_string_udls: u32 = 201304;
    pub const __cpp_lib_transformation_trait_aliases: u32 = 201304;
    pub const __cpp_lib_transparent_operators: u32 = 201210;
    pub const __cpp_lib_tuple_element_t: u32 = 201402;
    pub const __cpp_lib_tuples_by_type: u32 = 201304;
    pub const __DARWIN_CLK_TCK: u32 = 100;
    pub const MB_LEN_MAX: u32 = 6;
    pub const CLK_TCK: u32 = 100;
    pub const CHAR_BIT: u32 = 8;
    pub const SCHAR_MAX: u32 = 127;
    pub const SCHAR_MIN: i32 = -128;
    pub const UCHAR_MAX: u32 = 255;
    pub const CHAR_MAX: u32 = 127;
    pub const CHAR_MIN: i32 = -128;
    pub const USHRT_MAX: u32 = 65535;
    pub const SHRT_MAX: u32 = 32767;
    pub const SHRT_MIN: i32 = -32768;
    pub const UINT_MAX: u32 = 4294967295;
    pub const INT_MAX: u32 = 2147483647;
    pub const INT_MIN: i32 = -2147483648;
    pub const ULONG_MAX: i32 = -1;
    pub const LONG_MAX: u64 = 9223372036854775807;
    pub const LONG_MIN: i64 = -9223372036854775808;
    pub const ULLONG_MAX: i32 = -1;
    pub const LLONG_MAX: u64 = 9223372036854775807;
    pub const LLONG_MIN: i64 = -9223372036854775808;
    pub const LONG_BIT: u32 = 64;
    pub const SSIZE_MAX: u64 = 9223372036854775807;
    pub const WORD_BIT: u32 = 32;
    pub const SIZE_T_MAX: i32 = -1;
    pub const UQUAD_MAX: i32 = -1;
    pub const QUAD_MAX: u64 = 9223372036854775807;
    pub const QUAD_MIN: i64 = -9223372036854775808;
    pub const ARG_MAX: u32 = 1048576;
    pub const CHILD_MAX: u32 = 266;
    pub const GID_MAX: u32 = 2147483647;
    pub const LINK_MAX: u32 = 32767;
    pub const MAX_CANON: u32 = 1024;
    pub const MAX_INPUT: u32 = 1024;
    pub const NAME_MAX: u32 = 255;
    pub const NGROUPS_MAX: u32 = 16;
    pub const UID_MAX: u32 = 2147483647;
    pub const OPEN_MAX: u32 = 10240;
    pub const PATH_MAX: u32 = 1024;
    pub const PIPE_BUF: u32 = 512;
    pub const BC_BASE_MAX: u32 = 99;
    pub const BC_DIM_MAX: u32 = 2048;
    pub const BC_SCALE_MAX: u32 = 99;
    pub const BC_STRING_MAX: u32 = 1000;
    pub const CHARCLASS_NAME_MAX: u32 = 14;
    pub const COLL_WEIGHTS_MAX: u32 = 2;
    pub const EQUIV_CLASS_MAX: u32 = 2;
    pub const EXPR_NEST_MAX: u32 = 32;
    pub const LINE_MAX: u32 = 2048;
    pub const RE_DUP_MAX: u32 = 255;
    pub const NZERO: u32 = 20;
    pub const _POSIX_ARG_MAX: u32 = 4096;
    pub const _POSIX_CHILD_MAX: u32 = 25;
    pub const _POSIX_LINK_MAX: u32 = 8;
    pub const _POSIX_MAX_CANON: u32 = 255;
    pub const _POSIX_MAX_INPUT: u32 = 255;
    pub const _POSIX_NAME_MAX: u32 = 14;
    pub const _POSIX_NGROUPS_MAX: u32 = 8;
    pub const _POSIX_OPEN_MAX: u32 = 20;
    pub const _POSIX_PATH_MAX: u32 = 256;
    pub const _POSIX_PIPE_BUF: u32 = 512;
    pub const _POSIX_SSIZE_MAX: u32 = 32767;
    pub const _POSIX_STREAM_MAX: u32 = 8;
    pub const _POSIX_TZNAME_MAX: u32 = 6;
    pub const _POSIX2_BC_BASE_MAX: u32 = 99;
    pub const _POSIX2_BC_DIM_MAX: u32 = 2048;
    pub const _POSIX2_BC_SCALE_MAX: u32 = 99;
    pub const _POSIX2_BC_STRING_MAX: u32 = 1000;
    pub const _POSIX2_EQUIV_CLASS_MAX: u32 = 2;
    pub const _POSIX2_EXPR_NEST_MAX: u32 = 32;
    pub const _POSIX2_LINE_MAX: u32 = 2048;
    pub const _POSIX2_RE_DUP_MAX: u32 = 255;
    pub const _POSIX_AIO_LISTIO_MAX: u32 = 2;
    pub const _POSIX_AIO_MAX: u32 = 1;
    pub const _POSIX_DELAYTIMER_MAX: u32 = 32;
    pub const _POSIX_MQ_OPEN_MAX: u32 = 8;
    pub const _POSIX_MQ_PRIO_MAX: u32 = 32;
    pub const _POSIX_RTSIG_MAX: u32 = 8;
    pub const _POSIX_SEM_NSEMS_MAX: u32 = 256;
    pub const _POSIX_SEM_VALUE_MAX: u32 = 32767;
    pub const _POSIX_SIGQUEUE_MAX: u32 = 32;
    pub const _POSIX_TIMER_MAX: u32 = 32;
    pub const _POSIX_CLOCKRES_MIN: u32 = 20000000;
    pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
    pub const _POSIX_THREAD_KEYS_MAX: u32 = 128;
    pub const _POSIX_THREAD_THREADS_MAX: u32 = 64;
    pub const PTHREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
    pub const PTHREAD_KEYS_MAX: u32 = 512;
    pub const PTHREAD_STACK_MIN: u32 = 16384;
    pub const _POSIX_HOST_NAME_MAX: u32 = 255;
    pub const _POSIX_LOGIN_NAME_MAX: u32 = 9;
    pub const _POSIX_SS_REPL_MAX: u32 = 4;
    pub const _POSIX_SYMLINK_MAX: u32 = 255;
    pub const _POSIX_SYMLOOP_MAX: u32 = 8;
    pub const _POSIX_TRACE_EVENT_NAME_MAX: u32 = 30;
    pub const _POSIX_TRACE_NAME_MAX: u32 = 8;
    pub const _POSIX_TRACE_SYS_MAX: u32 = 8;
    pub const _POSIX_TRACE_USER_EVENT_MAX: u32 = 32;
    pub const _POSIX_TTY_NAME_MAX: u32 = 9;
    pub const _POSIX2_CHARCLASS_NAME_MAX: u32 = 14;
    pub const _POSIX2_COLL_WEIGHTS_MAX: u32 = 2;
    pub const _POSIX_RE_DUP_MAX: u32 = 255;
    pub const OFF_MIN: i64 = -9223372036854775808;
    pub const OFF_MAX: u64 = 9223372036854775807;
    pub const PASS_MAX: u32 = 128;
    pub const NL_ARGMAX: u32 = 9;
    pub const NL_LANGMAX: u32 = 14;
    pub const NL_MSGMAX: u32 = 32767;
    pub const NL_NMAX: u32 = 1;
    pub const NL_SETMAX: u32 = 255;
    pub const NL_TEXTMAX: u32 = 2048;
    pub const _XOPEN_IOV_MAX: u32 = 16;
    pub const IOV_MAX: u32 = 1024;
    pub const _XOPEN_NAME_MAX: u32 = 255;
    pub const _XOPEN_PATH_MAX: u32 = 1024;
    pub const TIME_UTC: u32 = 1;
    pub const EPERM: u32 = 1;
    pub const ENOENT: u32 = 2;
    pub const ESRCH: u32 = 3;
    pub const EINTR: u32 = 4;
    pub const EIO: u32 = 5;
    pub const ENXIO: u32 = 6;
    pub const E2BIG: u32 = 7;
    pub const ENOEXEC: u32 = 8;
    pub const EBADF: u32 = 9;
    pub const ECHILD: u32 = 10;
    pub const EDEADLK: u32 = 11;
    pub const ENOMEM: u32 = 12;
    pub const EACCES: u32 = 13;
    pub const EFAULT: u32 = 14;
    pub const ENOTBLK: u32 = 15;
    pub const EBUSY: u32 = 16;
    pub const EEXIST: u32 = 17;
    pub const EXDEV: u32 = 18;
    pub const ENODEV: u32 = 19;
    pub const ENOTDIR: u32 = 20;
    pub const EISDIR: u32 = 21;
    pub const EINVAL: u32 = 22;
    pub const ENFILE: u32 = 23;
    pub const EMFILE: u32 = 24;
    pub const ENOTTY: u32 = 25;
    pub const ETXTBSY: u32 = 26;
    pub const EFBIG: u32 = 27;
    pub const ENOSPC: u32 = 28;
    pub const ESPIPE: u32 = 29;
    pub const EROFS: u32 = 30;
    pub const EMLINK: u32 = 31;
    pub const EPIPE: u32 = 32;
    pub const EDOM: u32 = 33;
    pub const ERANGE: u32 = 34;
    pub const EAGAIN: u32 = 35;
    pub const EWOULDBLOCK: u32 = 35;
    pub const EINPROGRESS: u32 = 36;
    pub const EALREADY: u32 = 37;
    pub const ENOTSOCK: u32 = 38;
    pub const EDESTADDRREQ: u32 = 39;
    pub const EMSGSIZE: u32 = 40;
    pub const EPROTOTYPE: u32 = 41;
    pub const ENOPROTOOPT: u32 = 42;
    pub const EPROTONOSUPPORT: u32 = 43;
    pub const ESOCKTNOSUPPORT: u32 = 44;
    pub const ENOTSUP: u32 = 45;
    pub const EPFNOSUPPORT: u32 = 46;
    pub const EAFNOSUPPORT: u32 = 47;
    pub const EADDRINUSE: u32 = 48;
    pub const EADDRNOTAVAIL: u32 = 49;
    pub const ENETDOWN: u32 = 50;
    pub const ENETUNREACH: u32 = 51;
    pub const ENETRESET: u32 = 52;
    pub const ECONNABORTED: u32 = 53;
    pub const ECONNRESET: u32 = 54;
    pub const ENOBUFS: u32 = 55;
    pub const EISCONN: u32 = 56;
    pub const ENOTCONN: u32 = 57;
    pub const ESHUTDOWN: u32 = 58;
    pub const ETOOMANYREFS: u32 = 59;
    pub const ETIMEDOUT: u32 = 60;
    pub const ECONNREFUSED: u32 = 61;
    pub const ELOOP: u32 = 62;
    pub const ENAMETOOLONG: u32 = 63;
    pub const EHOSTDOWN: u32 = 64;
    pub const EHOSTUNREACH: u32 = 65;
    pub const ENOTEMPTY: u32 = 66;
    pub const EPROCLIM: u32 = 67;
    pub const EUSERS: u32 = 68;
    pub const EDQUOT: u32 = 69;
    pub const ESTALE: u32 = 70;
    pub const EREMOTE: u32 = 71;
    pub const EBADRPC: u32 = 72;
    pub const ERPCMISMATCH: u32 = 73;
    pub const EPROGUNAVAIL: u32 = 74;
    pub const EPROGMISMATCH: u32 = 75;
    pub const EPROCUNAVAIL: u32 = 76;
    pub const ENOLCK: u32 = 77;
    pub const ENOSYS: u32 = 78;
    pub const EFTYPE: u32 = 79;
    pub const EAUTH: u32 = 80;
    pub const ENEEDAUTH: u32 = 81;
    pub const EPWROFF: u32 = 82;
    pub const EDEVERR: u32 = 83;
    pub const EOVERFLOW: u32 = 84;
    pub const EBADEXEC: u32 = 85;
    pub const EBADARCH: u32 = 86;
    pub const ESHLIBVERS: u32 = 87;
    pub const EBADMACHO: u32 = 88;
    pub const ECANCELED: u32 = 89;
    pub const EIDRM: u32 = 90;
    pub const ENOMSG: u32 = 91;
    pub const EILSEQ: u32 = 92;
    pub const ENOATTR: u32 = 93;
    pub const EBADMSG: u32 = 94;
    pub const EMULTIHOP: u32 = 95;
    pub const ENODATA: u32 = 96;
    pub const ENOLINK: u32 = 97;
    pub const ENOSR: u32 = 98;
    pub const ENOSTR: u32 = 99;
    pub const EPROTO: u32 = 100;
    pub const ETIME: u32 = 101;
    pub const EOPNOTSUPP: u32 = 102;
    pub const ENOPOLICY: u32 = 103;
    pub const ENOTRECOVERABLE: u32 = 104;
    pub const EOWNERDEAD: u32 = 105;
    pub const EQFULL: u32 = 106;
    pub const ELAST: u32 = 106;
    pub const FP_NAN: u32 = 1;
    pub const FP_INFINITE: u32 = 2;
    pub const FP_ZERO: u32 = 3;
    pub const FP_NORMAL: u32 = 4;
    pub const FP_SUBNORMAL: u32 = 5;
    pub const FP_SUPERNORMAL: u32 = 6;
    pub const FP_FAST_FMA: u32 = 1;
    pub const FP_FAST_FMAF: u32 = 1;
    pub const FP_FAST_FMAL: u32 = 1;
    pub const FP_ILOGB0: i32 = -2147483648;
    pub const FP_ILOGBNAN: i32 = -2147483648;
    pub const MATH_ERRNO: u32 = 1;
    pub const MATH_ERREXCEPT: u32 = 2;
    pub const M_E: f64 = 2.718281828459045;
    pub const M_LOG2E: f64 = 1.4426950408889634;
    pub const M_LOG10E: f64 = 0.4342944819032518;
    pub const M_LN2: f64 = 0.6931471805599453;
    pub const M_LN10: f64 = 2.302585092994046;
    pub const M_PI: f64 = 3.141592653589793;
    pub const M_PI_2: f64 = 1.5707963267948966;
    pub const M_PI_4: f64 = 0.7853981633974483;
    pub const M_1_PI: f64 = 0.3183098861837907;
    pub const M_2_PI: f64 = 0.6366197723675814;
    pub const M_2_SQRTPI: f64 = 1.1283791670955126;
    pub const M_SQRT2: f64 = 1.4142135623730951;
    pub const M_SQRT1_2: f64 = 0.7071067811865476;
    pub const FP_SNAN: u32 = 1;
    pub const FP_QNAN: u32 = 1;
    pub const DOMAIN: u32 = 1;
    pub const SING: u32 = 2;
    pub const OVERFLOW: u32 = 3;
    pub const UNDERFLOW: u32 = 4;
    pub const TLOSS: u32 = 5;
    pub const PLOSS: u32 = 6;
    pub const _PTHREAD_MUTEX_SIG_init: u32 = 850045863;
    pub const _PTHREAD_ERRORCHECK_MUTEX_SIG_init: u32 = 850045857;
    pub const _PTHREAD_RECURSIVE_MUTEX_SIG_init: u32 = 850045858;
    pub const _PTHREAD_FIRSTFIT_MUTEX_SIG_init: u32 = 850045859;
    pub const _PTHREAD_COND_SIG_init: u32 = 1018212795;
    pub const _PTHREAD_ONCE_SIG_init: u32 = 816954554;
    pub const _PTHREAD_RWLOCK_SIG_init: u32 = 766030772;
    pub const SCHED_OTHER: u32 = 1;
    pub const SCHED_FIFO: u32 = 4;
    pub const SCHED_RR: u32 = 2;
    pub const __SCHED_PARAM_SIZE__: u32 = 4;
    pub const QOS_MIN_RELATIVE_PRIORITY: i32 = -15;
    pub const PTHREAD_CREATE_JOINABLE: u32 = 1;
    pub const PTHREAD_CREATE_DETACHED: u32 = 2;
    pub const PTHREAD_INHERIT_SCHED: u32 = 1;
    pub const PTHREAD_EXPLICIT_SCHED: u32 = 2;
    pub const PTHREAD_CANCEL_ENABLE: u32 = 1;
    pub const PTHREAD_CANCEL_DISABLE: u32 = 0;
    pub const PTHREAD_CANCEL_DEFERRED: u32 = 2;
    pub const PTHREAD_CANCEL_ASYNCHRONOUS: u32 = 0;
    pub const PTHREAD_SCOPE_SYSTEM: u32 = 1;
    pub const PTHREAD_SCOPE_PROCESS: u32 = 2;
    pub const PTHREAD_PROCESS_SHARED: u32 = 1;
    pub const PTHREAD_PROCESS_PRIVATE: u32 = 2;
    pub const PTHREAD_PRIO_NONE: u32 = 0;
    pub const PTHREAD_PRIO_INHERIT: u32 = 1;
    pub const PTHREAD_PRIO_PROTECT: u32 = 2;
    pub const PTHREAD_MUTEX_NORMAL: u32 = 0;
    pub const PTHREAD_MUTEX_ERRORCHECK: u32 = 1;
    pub const PTHREAD_MUTEX_RECURSIVE: u32 = 2;
    pub const PTHREAD_MUTEX_DEFAULT: u32 = 0;
    pub const PTHREAD_MUTEX_POLICY_FAIRSHARE_NP: u32 = 1;
    pub const PTHREAD_MUTEX_POLICY_FIRSTFIT_NP: u32 = 3;
    pub const SWIFT_HAS_MSVC_ARM_ATOMICS: u32 = 1;
    pub const __PRI_8_LENGTH_MODIFIER__: &[u8; 3] = b"hh\0";
    pub const __PRI_64_LENGTH_MODIFIER__: &[u8; 3] = b"ll\0";
    pub const __SCN_64_LENGTH_MODIFIER__: &[u8; 3] = b"ll\0";
    pub const __PRI_MAX_LENGTH_MODIFIER__: &[u8; 2] = b"j\0";
    pub const __SCN_MAX_LENGTH_MODIFIER__: &[u8; 2] = b"j\0";
    pub const PRId8: &[u8; 4] = b"hhd\0";
    pub const PRIi8: &[u8; 4] = b"hhi\0";
    pub const PRIo8: &[u8; 4] = b"hho\0";
    pub const PRIu8: &[u8; 4] = b"hhu\0";
    pub const PRIx8: &[u8; 4] = b"hhx\0";
    pub const PRIX8: &[u8; 4] = b"hhX\0";
    pub const PRId16: &[u8; 3] = b"hd\0";
    pub const PRIi16: &[u8; 3] = b"hi\0";
    pub const PRIo16: &[u8; 3] = b"ho\0";
    pub const PRIu16: &[u8; 3] = b"hu\0";
    pub const PRIx16: &[u8; 3] = b"hx\0";
    pub const PRIX16: &[u8; 3] = b"hX\0";
    pub const PRId32: &[u8; 2] = b"d\0";
    pub const PRIi32: &[u8; 2] = b"i\0";
    pub const PRIo32: &[u8; 2] = b"o\0";
    pub const PRIu32: &[u8; 2] = b"u\0";
    pub const PRIx32: &[u8; 2] = b"x\0";
    pub const PRIX32: &[u8; 2] = b"X\0";
    pub const PRId64: &[u8; 4] = b"lld\0";
    pub const PRIi64: &[u8; 4] = b"lli\0";
    pub const PRIo64: &[u8; 4] = b"llo\0";
    pub const PRIu64: &[u8; 4] = b"llu\0";
    pub const PRIx64: &[u8; 4] = b"llx\0";
    pub const PRIX64: &[u8; 4] = b"llX\0";
    pub const PRIdLEAST8: &[u8; 4] = b"hhd\0";
    pub const PRIiLEAST8: &[u8; 4] = b"hhi\0";
    pub const PRIoLEAST8: &[u8; 4] = b"hho\0";
    pub const PRIuLEAST8: &[u8; 4] = b"hhu\0";
    pub const PRIxLEAST8: &[u8; 4] = b"hhx\0";
    pub const PRIXLEAST8: &[u8; 4] = b"hhX\0";
    pub const PRIdLEAST16: &[u8; 3] = b"hd\0";
    pub const PRIiLEAST16: &[u8; 3] = b"hi\0";
    pub const PRIoLEAST16: &[u8; 3] = b"ho\0";
    pub const PRIuLEAST16: &[u8; 3] = b"hu\0";
    pub const PRIxLEAST16: &[u8; 3] = b"hx\0";
    pub const PRIXLEAST16: &[u8; 3] = b"hX\0";
    pub const PRIdLEAST32: &[u8; 2] = b"d\0";
    pub const PRIiLEAST32: &[u8; 2] = b"i\0";
    pub const PRIoLEAST32: &[u8; 2] = b"o\0";
    pub const PRIuLEAST32: &[u8; 2] = b"u\0";
    pub const PRIxLEAST32: &[u8; 2] = b"x\0";
    pub const PRIXLEAST32: &[u8; 2] = b"X\0";
    pub const PRIdLEAST64: &[u8; 4] = b"lld\0";
    pub const PRIiLEAST64: &[u8; 4] = b"lli\0";
    pub const PRIoLEAST64: &[u8; 4] = b"llo\0";
    pub const PRIuLEAST64: &[u8; 4] = b"llu\0";
    pub const PRIxLEAST64: &[u8; 4] = b"llx\0";
    pub const PRIXLEAST64: &[u8; 4] = b"llX\0";
    pub const PRIdFAST8: &[u8; 4] = b"hhd\0";
    pub const PRIiFAST8: &[u8; 4] = b"hhi\0";
    pub const PRIoFAST8: &[u8; 4] = b"hho\0";
    pub const PRIuFAST8: &[u8; 4] = b"hhu\0";
    pub const PRIxFAST8: &[u8; 4] = b"hhx\0";
    pub const PRIXFAST8: &[u8; 4] = b"hhX\0";
    pub const PRIdFAST16: &[u8; 3] = b"hd\0";
    pub const PRIiFAST16: &[u8; 3] = b"hi\0";
    pub const PRIoFAST16: &[u8; 3] = b"ho\0";
    pub const PRIuFAST16: &[u8; 3] = b"hu\0";
    pub const PRIxFAST16: &[u8; 3] = b"hx\0";
    pub const PRIXFAST16: &[u8; 3] = b"hX\0";
    pub const PRIdFAST32: &[u8; 2] = b"d\0";
    pub const PRIiFAST32: &[u8; 2] = b"i\0";
    pub const PRIoFAST32: &[u8; 2] = b"o\0";
    pub const PRIuFAST32: &[u8; 2] = b"u\0";
    pub const PRIxFAST32: &[u8; 2] = b"x\0";
    pub const PRIXFAST32: &[u8; 2] = b"X\0";
    pub const PRIdFAST64: &[u8; 4] = b"lld\0";
    pub const PRIiFAST64: &[u8; 4] = b"lli\0";
    pub const PRIoFAST64: &[u8; 4] = b"llo\0";
    pub const PRIuFAST64: &[u8; 4] = b"llu\0";
    pub const PRIxFAST64: &[u8; 4] = b"llx\0";
    pub const PRIXFAST64: &[u8; 4] = b"llX\0";
    pub const PRIdPTR: &[u8; 3] = b"ld\0";
    pub const PRIiPTR: &[u8; 3] = b"li\0";
    pub const PRIoPTR: &[u8; 3] = b"lo\0";
    pub const PRIuPTR: &[u8; 3] = b"lu\0";
    pub const PRIxPTR: &[u8; 3] = b"lx\0";
    pub const PRIXPTR: &[u8; 3] = b"lX\0";
    pub const PRIdMAX: &[u8; 3] = b"jd\0";
    pub const PRIiMAX: &[u8; 3] = b"ji\0";
    pub const PRIoMAX: &[u8; 3] = b"jo\0";
    pub const PRIuMAX: &[u8; 3] = b"ju\0";
    pub const PRIxMAX: &[u8; 3] = b"jx\0";
    pub const PRIXMAX: &[u8; 3] = b"jX\0";
    pub const SCNd8: &[u8; 4] = b"hhd\0";
    pub const SCNi8: &[u8; 4] = b"hhi\0";
    pub const SCNo8: &[u8; 4] = b"hho\0";
    pub const SCNu8: &[u8; 4] = b"hhu\0";
    pub const SCNx8: &[u8; 4] = b"hhx\0";
    pub const SCNd16: &[u8; 3] = b"hd\0";
    pub const SCNi16: &[u8; 3] = b"hi\0";
    pub const SCNo16: &[u8; 3] = b"ho\0";
    pub const SCNu16: &[u8; 3] = b"hu\0";
    pub const SCNx16: &[u8; 3] = b"hx\0";
    pub const SCNd32: &[u8; 2] = b"d\0";
    pub const SCNi32: &[u8; 2] = b"i\0";
    pub const SCNo32: &[u8; 2] = b"o\0";
    pub const SCNu32: &[u8; 2] = b"u\0";
    pub const SCNx32: &[u8; 2] = b"x\0";
    pub const SCNd64: &[u8; 4] = b"lld\0";
    pub const SCNi64: &[u8; 4] = b"lli\0";
    pub const SCNo64: &[u8; 4] = b"llo\0";
    pub const SCNu64: &[u8; 4] = b"llu\0";
    pub const SCNx64: &[u8; 4] = b"llx\0";
    pub const SCNdLEAST8: &[u8; 4] = b"hhd\0";
    pub const SCNiLEAST8: &[u8; 4] = b"hhi\0";
    pub const SCNoLEAST8: &[u8; 4] = b"hho\0";
    pub const SCNuLEAST8: &[u8; 4] = b"hhu\0";
    pub const SCNxLEAST8: &[u8; 4] = b"hhx\0";
    pub const SCNdLEAST16: &[u8; 3] = b"hd\0";
    pub const SCNiLEAST16: &[u8; 3] = b"hi\0";
    pub const SCNoLEAST16: &[u8; 3] = b"ho\0";
    pub const SCNuLEAST16: &[u8; 3] = b"hu\0";
    pub const SCNxLEAST16: &[u8; 3] = b"hx\0";
    pub const SCNdLEAST32: &[u8; 2] = b"d\0";
    pub const SCNiLEAST32: &[u8; 2] = b"i\0";
    pub const SCNoLEAST32: &[u8; 2] = b"o\0";
    pub const SCNuLEAST32: &[u8; 2] = b"u\0";
    pub const SCNxLEAST32: &[u8; 2] = b"x\0";
    pub const SCNdLEAST64: &[u8; 4] = b"lld\0";
    pub const SCNiLEAST64: &[u8; 4] = b"lli\0";
    pub const SCNoLEAST64: &[u8; 4] = b"llo\0";
    pub const SCNuLEAST64: &[u8; 4] = b"llu\0";
    pub const SCNxLEAST64: &[u8; 4] = b"llx\0";
    pub const SCNdFAST8: &[u8; 4] = b"hhd\0";
    pub const SCNiFAST8: &[u8; 4] = b"hhi\0";
    pub const SCNoFAST8: &[u8; 4] = b"hho\0";
    pub const SCNuFAST8: &[u8; 4] = b"hhu\0";
    pub const SCNxFAST8: &[u8; 4] = b"hhx\0";
    pub const SCNdFAST16: &[u8; 3] = b"hd\0";
    pub const SCNiFAST16: &[u8; 3] = b"hi\0";
    pub const SCNoFAST16: &[u8; 3] = b"ho\0";
    pub const SCNuFAST16: &[u8; 3] = b"hu\0";
    pub const SCNxFAST16: &[u8; 3] = b"hx\0";
    pub const SCNdFAST32: &[u8; 2] = b"d\0";
    pub const SCNiFAST32: &[u8; 2] = b"i\0";
    pub const SCNoFAST32: &[u8; 2] = b"o\0";
    pub const SCNuFAST32: &[u8; 2] = b"u\0";
    pub const SCNxFAST32: &[u8; 2] = b"x\0";
    pub const SCNdFAST64: &[u8; 4] = b"lld\0";
    pub const SCNiFAST64: &[u8; 4] = b"lli\0";
    pub const SCNoFAST64: &[u8; 4] = b"llo\0";
    pub const SCNuFAST64: &[u8; 4] = b"llu\0";
    pub const SCNxFAST64: &[u8; 4] = b"llx\0";
    pub const SCNdPTR: &[u8; 3] = b"ld\0";
    pub const SCNiPTR: &[u8; 3] = b"li\0";
    pub const SCNoPTR: &[u8; 3] = b"lo\0";
    pub const SCNuPTR: &[u8; 3] = b"lu\0";
    pub const SCNxPTR: &[u8; 3] = b"lx\0";
    pub const SCNdMAX: &[u8; 3] = b"jd\0";
    pub const SCNiMAX: &[u8; 3] = b"ji\0";
    pub const SCNoMAX: &[u8; 3] = b"jo\0";
    pub const SCNuMAX: &[u8; 3] = b"ju\0";
    pub const SCNxMAX: &[u8; 3] = b"jx\0";
    pub const MAC_OS_X_VERSION_MAX_ALLOWED: u32 = 140000;
    pub const __AVAILABILITY_MACROS_USES_AVAILABILITY: u32 = 1;
    pub const __DARWIN_FD_SETSIZE: u32 = 1024;
    pub const __DARWIN_NBBY: u32 = 8;
    pub const NBBY: u32 = 8;
    pub const FD_SETSIZE: u32 = 1024;
    pub const OBJC_API_VERSION: u32 = 2;
    pub const OBJC_NO_GC: u32 = 1;
    pub const NS_ENFORCE_NSOBJECT_DESIGNATED_INITIALIZER: u32 = 1;
    pub const OBJC_OLD_DISPATCH_PROTOTYPES: u32 = 0;
    pub const __bool_true_false_are_defined: u32 = 1;
    pub const OBJC_BOOL_IS_BOOL: u32 = 1;
    pub const __GNUC_VA_LIST: u32 = 1;
    pub const _LIBCPP_TYPEINFO_COMPARISON_IMPLEMENTATION: u32 = 3;
    pub const _CACHED_RUNES: u32 = 256;
    pub const _CRMASK: i32 = -256;
    pub const _RUNE_MAGIC_A: &[u8; 9] = b"RuneMagA\0";
    pub const _CTYPE_A: u32 = 256;
    pub const _CTYPE_C: u32 = 512;
    pub const _CTYPE_D: u32 = 1024;
    pub const _CTYPE_G: u32 = 2048;
    pub const _CTYPE_L: u32 = 4096;
    pub const _CTYPE_P: u32 = 8192;
    pub const _CTYPE_S: u32 = 16384;
    pub const _CTYPE_U: u32 = 32768;
    pub const _CTYPE_X: u32 = 65536;
    pub const _CTYPE_B: u32 = 131072;
    pub const _CTYPE_R: u32 = 262144;
    pub const _CTYPE_I: u32 = 524288;
    pub const _CTYPE_T: u32 = 1048576;
    pub const _CTYPE_Q: u32 = 2097152;
    pub const _CTYPE_SW0: u32 = 536870912;
    pub const _CTYPE_SW1: u32 = 1073741824;
    pub const _CTYPE_SW2: u32 = 2147483648;
    pub const _CTYPE_SW3: u32 = 3221225472;
    pub const _CTYPE_SWM: u32 = 3758096384;
    pub const _CTYPE_SWS: u32 = 30;
    pub const RENAME_SECLUDE: u32 = 1;
    pub const RENAME_SWAP: u32 = 2;
    pub const RENAME_EXCL: u32 = 4;
    pub const RENAME_RESERVED1: u32 = 8;
    pub const RENAME_NOFOLLOW_ANY: u32 = 16;
    pub const SEEK_SET: u32 = 0;
    pub const SEEK_CUR: u32 = 1;
    pub const SEEK_END: u32 = 2;
    pub const SEEK_HOLE: u32 = 3;
    pub const SEEK_DATA: u32 = 4;
    pub const __SLBF: u32 = 1;
    pub const __SNBF: u32 = 2;
    pub const __SRD: u32 = 4;
    pub const __SWR: u32 = 8;
    pub const __SRW: u32 = 16;
    pub const __SEOF: u32 = 32;
    pub const __SERR: u32 = 64;
    pub const __SMBF: u32 = 128;
    pub const __SAPP: u32 = 256;
    pub const __SSTR: u32 = 512;
    pub const __SOPT: u32 = 1024;
    pub const __SNPT: u32 = 2048;
    pub const __SOFF: u32 = 4096;
    pub const __SMOD: u32 = 8192;
    pub const __SALC: u32 = 16384;
    pub const __SIGN: u32 = 32768;
    pub const _IOFBF: u32 = 0;
    pub const _IOLBF: u32 = 1;
    pub const _IONBF: u32 = 2;
    pub const BUFSIZ: u32 = 1024;
    pub const EOF: i32 = -1;
    pub const FOPEN_MAX: u32 = 20;
    pub const FILENAME_MAX: u32 = 1024;
    pub const P_tmpdir: &[u8; 10] = b"/var/tmp/\0";
    pub const L_tmpnam: u32 = 1024;
    pub const TMP_MAX: u32 = 308915776;
    pub const L_ctermid: u32 = 1024;
    pub const LC_ALL: u32 = 0;
    pub const LC_COLLATE: u32 = 1;
    pub const LC_CTYPE: u32 = 2;
    pub const LC_MONETARY: u32 = 3;
    pub const LC_NUMERIC: u32 = 4;
    pub const LC_TIME: u32 = 5;
    pub const LC_MESSAGES: u32 = 6;
    pub const _LC_LAST: u32 = 7;
    pub const LC_COLLATE_MASK: u32 = 1;
    pub const LC_CTYPE_MASK: u32 = 2;
    pub const LC_MESSAGES_MASK: u32 = 4;
    pub const LC_MONETARY_MASK: u32 = 8;
    pub const LC_NUMERIC_MASK: u32 = 16;
    pub const LC_TIME_MASK: u32 = 32;
    pub const _LC_NUM_MASK: u32 = 6;
    pub const _LC_LAST_MASK: u32 = 32;
    pub const _LIBCPP_HAS_CATOPEN: u32 = 1;
    pub const NL_SETD: u32 = 1;
    pub const NL_CAT_LOCALE: u32 = 1;
    pub const _LIBCPP_GET_C_LOCALE: u32 = 0;
    pub const SWIFT_ABI_DEFAULT_LEAST_VALID_POINTER: u32 = 4096;
    pub const SWIFT_ABI_DEFAULT_FUNCTION_SPARE_BITS_MASK: u32 = 0;
    pub const SWIFT_ABI_DEFAULT_SWIFT_SPARE_BITS_MASK: u32 = 0;
    pub const SWIFT_ABI_DEFAULT_OBJC_RESERVED_BITS_MASK: u32 = 0;
    pub const SWIFT_ABI_DEFAULT_OBJC_NUM_RESERVED_LOW_BITS: u32 = 0;
    pub const SWIFT_ABI_DEFAULT_OBJC_WEAK_REFERENCE_MARKER_MASK: u32 = 0;
    pub const SWIFT_ABI_DEFAULT_OBJC_WEAK_REFERENCE_MARKER_VALUE: u32 = 0;
    pub const SWIFT_ABI_DEFAULT_BRIDGEOBJECT_TAG_32: u32 = 0;
    pub const SWIFT_ABI_DEFAULT_BRIDGEOBJECT_TAG_64: i64 = -9223372036854775808;
    pub const SWIFT_ABI_DEFAULT_64BIT_SPARE_BITS_MASK: i64 = -72057594037927929;
    pub const SWIFT_ABI_DEFAULT_REFERENCE_POISON_DEBUG_VALUE_32: u32 = 1088;
    pub const SWIFT_ABI_DEFAULT_REFERENCE_POISON_DEBUG_VALUE_64: u32 = 1088;
    pub const SWIFT_ABI_I386_SWIFT_SPARE_BITS_MASK: u32 = 3;
    pub const SWIFT_ABI_I386_OBJC_WEAK_REFERENCE_MARKER_MASK: u32 = 1;
    pub const SWIFT_ABI_I386_OBJC_WEAK_REFERENCE_MARKER_VALUE: u32 = 1;
    pub const SWIFT_ABI_I386_IS_OBJC_BIT: u32 = 2;
    pub const SWIFT_ABI_ARM_SWIFT_SPARE_BITS_MASK: u32 = 3;
    pub const SWIFT_ABI_ARM_OBJC_WEAK_REFERENCE_MARKER_MASK: u32 = 1;
    pub const SWIFT_ABI_ARM_OBJC_WEAK_REFERENCE_MARKER_VALUE: u32 = 1;
    pub const SWIFT_ABI_ARM_IS_OBJC_BIT: u32 = 2;
    pub const SWIFT_ABI_DARWIN_X86_64_LEAST_VALID_POINTER: u64 = 4294967296;
    pub const SWIFT_ABI_X86_64_SWIFT_SPARE_BITS_MASK: i64 = -72057594037927929;
    pub const SWIFT_ABI_X86_64_OBJC_RESERVED_BITS_MASK: u32 = 1;
    pub const SWIFT_ABI_X86_64_OBJC_NUM_RESERVED_LOW_BITS: u32 = 1;
    pub const SWIFT_ABI_X86_64_SIMULATOR_OBJC_RESERVED_BITS_MASK: i64 = -9223372036854775808;
    pub const SWIFT_ABI_X86_64_SIMULATOR_OBJC_NUM_RESERVED_LOW_BITS: u32 = 0;
    pub const SWIFT_ABI_X86_64_IS_OBJC_BIT: u64 = 4611686018427387904;
    pub const SWIFT_ABI_X86_64_OBJC_WEAK_REFERENCE_MARKER_MASK: u32 = 3;
    pub const SWIFT_ABI_X86_64_OBJC_WEAK_REFERENCE_MARKER_VALUE: u32 = 2;
    pub const SWIFT_ABI_X86_64_SIMULATOR_OBJC_WEAK_REFERENCE_MARKER_MASK: i64 =
        -9223372036854775807;
    pub const SWIFT_ABI_X86_64_SIMULATOR_OBJC_WEAK_REFERENCE_MARKER_VALUE: u32 = 1;
    pub const SWIFT_ABI_DARWIN_ARM64_LEAST_VALID_POINTER: u64 = 4294967296;
    pub const SWIFT_ABI_ANDROID_ARM64_SWIFT_SPARE_BITS_MASK: u64 = 67553994410557447;
    pub const SWIFT_ABI_ANDROID_ARM64_OBJC_RESERVED_BITS_MASK: u32 = 0;
    pub const SWIFT_ABI_ARM64_SWIFT_SPARE_BITS_MASK: i64 = -1152921504606846969;
    pub const SWIFT_ABI_ARM64_OBJC_RESERVED_BITS_MASK: i64 = -9223372036854775808;
    pub const SWIFT_ABI_ARM64_OBJC_NUM_RESERVED_LOW_BITS: u32 = 0;
    pub const SWIFT_ABI_ARM64_IS_OBJC_BIT: u64 = 4611686018427387904;
    pub const SWIFT_ABI_ARM64_OBJC_WEAK_REFERENCE_MARKER_MASK: i64 = -9223372036854775807;
    pub const SWIFT_ABI_ARM64_OBJC_WEAK_REFERENCE_MARKER_VALUE: u32 = 1;
    pub const SWIFT_ABI_POWERPC_SWIFT_SPARE_BITS_MASK: u32 = 3;
    pub const SWIFT_ABI_POWERPC64_SWIFT_SPARE_BITS_MASK: i64 = -72057594037927929;
    pub const SWIFT_ABI_S390X_SWIFT_SPARE_BITS_MASK: i64 = -72057594037927929;
    pub const SWIFT_ABI_S390X_OBJC_RESERVED_BITS_MASK: i64 = -9223372036854775808;
    pub const SWIFT_ABI_S390X_OBJC_NUM_RESERVED_LOW_BITS: u32 = 0;
    pub const SWIFT_ABI_S390X_IS_OBJC_BIT: u64 = 4611686018427387904;
    pub const SWIFT_ABI_S390X_OBJC_WEAK_REFERENCE_MARKER_MASK: i64 = -9223372036854775807;
    pub const SWIFT_ABI_S390X_OBJC_WEAK_REFERENCE_MARKER_VALUE: u32 = 1;
    pub const SWIFT_ABI_WASM32_LEAST_VALID_POINTER: u32 = 4096;
    pub const SWIFT_TARGET_OS_SIMULATOR: u32 = 0;
    pub const SWIFT_TARGET_OS_DARWIN: u32 = 1;
    pub const SWIFT_ABI_HEAP_OBJECT_HEADER_SIZE_64: u32 = 16;
    pub const SWIFT_ABI_HEAP_OBJECT_HEADER_SIZE_32: u32 = 8;
    pub const OBJC_GETCLASSHOOK_DEFINED: u32 = 1;
    pub const OBJC_ADDLOADIMAGEFUNC_DEFINED: u32 = 1;
    pub const OBJC_SETHOOK_LAZYCLASSNAMER_DEFINED: u32 = 1;
    pub const OBJC_REALIZECLASSFROMSWIFT_DEFINED: u32 = 1;
    pub const _C_ID: u8 = 64u8;
    pub const _C_CLASS: u8 = 35u8;
    pub const _C_SEL: u8 = 58u8;
    pub const _C_CHR: u8 = 99u8;
    pub const _C_UCHR: u8 = 67u8;
    pub const _C_SHT: u8 = 115u8;
    pub const _C_USHT: u8 = 83u8;
    pub const _C_INT: u8 = 105u8;
    pub const _C_UINT: u8 = 73u8;
    pub const _C_LNG: u8 = 108u8;
    pub const _C_ULNG: u8 = 76u8;
    pub const _C_LNG_LNG: u8 = 113u8;
    pub const _C_ULNG_LNG: u8 = 81u8;
    pub const _C_INT128: u8 = 116u8;
    pub const _C_UINT128: u8 = 84u8;
    pub const _C_FLT: u8 = 102u8;
    pub const _C_DBL: u8 = 100u8;
    pub const _C_LNG_DBL: u8 = 68u8;
    pub const _C_BFLD: u8 = 98u8;
    pub const _C_BOOL: u8 = 66u8;
    pub const _C_VOID: u8 = 118u8;
    pub const _C_UNDEF: u8 = 63u8;
    pub const _C_PTR: u8 = 94u8;
    pub const _C_CHARPTR: u8 = 42u8;
    pub const _C_ATOM: u8 = 37u8;
    pub const _C_ARY_B: u8 = 91u8;
    pub const _C_ARY_E: u8 = 93u8;
    pub const _C_UNION_B: u8 = 40u8;
    pub const _C_UNION_E: u8 = 41u8;
    pub const _C_STRUCT_B: u8 = 123u8;
    pub const _C_STRUCT_E: u8 = 125u8;
    pub const _C_VECTOR: u8 = 33u8;
    pub const _C_COMPLEX: u8 = 106u8;
    pub const _C_ATOMIC: u8 = 65u8;
    pub const _C_CONST: u8 = 114u8;
    pub const _C_IN: u8 = 110u8;
    pub const _C_INOUT: u8 = 78u8;
    pub const _C_OUT: u8 = 111u8;
    pub const _C_BYCOPY: u8 = 79u8;
    pub const _C_BYREF: u8 = 82u8;
    pub const _C_ONEWAY: u8 = 86u8;
    pub const _C_GNUREGISTER: u8 = 43u8;
    pub const SWIFT_DTOA_BINARY16_SUPPORT: u32 = 1;
    pub const FLOAT_IS_BINARY32: u32 = 1;
    pub const SWIFT_DTOA_BINARY32_SUPPORT: u32 = 1;
    pub const DOUBLE_IS_BINARY64: u32 = 1;
    pub const LONG_DOUBLE_IS_BINARY64: u32 = 1;
    pub const SWIFT_DTOA_BINARY64_SUPPORT: u32 = 1;
    pub const SWIFT_HAS_VOUCHERS: u32 = 1;
    pub const API_TO_BE_DEPRECATED: u32 = 100000;
    pub const API_TO_BE_DEPRECATED_MACOS: u32 = 100000;
    pub const API_TO_BE_DEPRECATED_IOS: u32 = 100000;
    pub const API_TO_BE_DEPRECATED_TVOS: u32 = 100000;
    pub const API_TO_BE_DEPRECATED_WATCHOS: u32 = 100000;
    pub const API_TO_BE_DEPRECATED_DRIVERKIT: u32 = 100000;
    pub const API_TO_BE_DEPRECATED_VISIONOS: u32 = 100000;
    pub const OS_OBJECT_HAVE_OBJC_SUPPORT: u32 = 0;
    pub const OS_OBJECT_USE_OBJC: u32 = 0;
    pub const OS_OBJECT_SWIFT3: u32 = 0;
    pub const OS_OBJECT_USE_OBJC_RETAIN_RELEASE: u32 = 0;
    pub mod std {
        #[allow(unused_imports)]
        use self::super::super::root;
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__122__libcpp_verbose_abortEPKcz"]
            pub fn __libcpp_verbose_abort(__format: *const ::std::os::raw::c_char, ...);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15__useEPKcz"]
            pub fn __use(arg1: *const ::std::os::raw::c_char, ...);
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_always_lock_free {
            pub _address: u8,
        }
        pub type integral_constant_value_type = u8;
        pub type integral_constant_type = u8;

        #[repr(C)]
        pub struct _Tp {
            pub _address: u8,
        }
        
        extern "C" {
            pub static value: _Tp;
        }
        pub type true_type = u8;
        pub type false_type = u8;
        pub type _BoolConstant = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_same {
            pub _address: u8,
        }
        pub type _IsSame = u8;
        pub type _IsNotSame = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_enum {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct underlying_type {
            pub _address: u8,
        }
        pub type underlying_type_t = u8;
        pub const __legacy_memory_order___mo_relaxed: root::std::__legacy_memory_order = 0;
        pub const __legacy_memory_order___mo_consume: root::std::__legacy_memory_order = 1;
        pub const __legacy_memory_order___mo_acquire: root::std::__legacy_memory_order = 2;
        pub const __legacy_memory_order___mo_release: root::std::__legacy_memory_order = 3;
        pub const __legacy_memory_order___mo_acq_rel: root::std::__legacy_memory_order = 4;
        pub const __legacy_memory_order___mo_seq_cst: root::std::__legacy_memory_order = 5;
        pub type __legacy_memory_order = ::std::os::raw::c_uint;
        pub type __memory_order_underlying_t = u32;
        pub const memory_order_memory_order_relaxed: root::std::memory_order = 0;
        pub const memory_order_memory_order_consume: root::std::memory_order = 1;
        pub const memory_order_memory_order_acquire: root::std::memory_order = 2;
        pub const memory_order_memory_order_release: root::std::memory_order = 3;
        pub const memory_order_memory_order_acq_rel: root::std::memory_order = 4;
        pub const memory_order_memory_order_seq_cst: root::std::memory_order = 5;
        pub type memory_order = ::std::os::raw::c_uint;
        pub type _If = u8;
        pub type conditional_type = u8;
        pub type conditional_t = u8;
        pub type __conditional_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_assignable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_trivially_copyable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct remove_const {
            pub _address: u8,
        }
        pub type remove_const_type = u8;
        pub type __remove_const_t = u8;
        pub type remove_const_t = u8;
        pub type __enable_if_t = u8;
        pub type enable_if_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct remove_volatile {
            pub _address: u8,
        }
        pub type remove_volatile_type = u8;
        pub type __remove_volatile_t = u8;
        pub type remove_volatile_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct remove_cv {
            pub _address: u8,
        }
        pub type remove_cv_type = u8;
        pub type __remove_cv_t = u8;
        pub type remove_cv_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_integral {
            pub _address: u8,
        }
        pub const __libcpp_is_integral_value: root::std::__libcpp_is_integral__bindgen_ty_1 = 0;
        pub type __libcpp_is_integral__bindgen_ty_1 = i32;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_integral {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __cxx_atomic_base_impl {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __cxx_atomic_impl {
            pub _address: u8,
        }
        pub type __cxx_contention_t = u64;
        pub type __cxx_atomic_contention_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_array {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_const {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_lvalue_reference {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_rvalue_reference {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_reference {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_function {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_void {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct remove_reference {
            pub _address: u8,
        }
        pub type remove_reference_type = u8;
        pub type __libcpp_remove_reference_t = u8;
        pub type remove_reference_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_convertible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_referenceable {
            pub _address: u8,
        }
        pub type __add_pointer_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct add_pointer {
            pub _address: u8,
        }
        pub type add_pointer_type = u8;
        pub type add_pointer_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct remove_extent {
            pub _address: u8,
        }
        pub type remove_extent_type = u8;
        pub type __remove_extent_t = u8;
        pub type remove_extent_t = u8;
        pub type __decay_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct decay {
            pub _address: u8,
        }
        pub type decay_type = u8;
        pub type decay_t = u8;
        pub type __remove_cvref_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_same_uncvref {
            pub _address: u8,
        }
        pub type __void_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __common_type2_imp {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __common_type_impl {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __common_types {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct common_type {
            pub _address: u8,
        }
        pub type common_type_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct add_const {
            pub _address: u8,
        }
        pub type add_const_type = u8;
        pub type add_const_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct add_cv {
            pub _address: u8,
        }
        pub type add_cv_type = u8;
        pub type add_cv_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct add_volatile {
            pub _address: u8,
        }
        pub type add_volatile_type = u8;
        pub type add_volatile_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __copy_cv {
            pub _address: u8,
        }
        pub type __copy_cv_type = u8;
        pub type __copy_cv_t = u8;
        pub type __add_lvalue_reference_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct add_lvalue_reference {
            pub _address: u8,
        }
        pub type add_lvalue_reference_type = u8;
        pub type add_lvalue_reference_t = u8;
        pub type __add_rvalue_reference_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct add_rvalue_reference {
            pub _address: u8,
        }
        pub type add_rvalue_reference_type = u8;
        pub type add_rvalue_reference_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __copy_cvref {
            pub _address: u8,
        }
        pub type __copy_cvref_type = u8;
        pub type __copy_cvref_t = u8;
        pub type __make_const_lvalue_ref = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_floating_point {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_floating_point {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_arithmetic {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_signed {
            pub _address: u8,
        }
        pub const float_round_style_round_indeterminate: root::std::float_round_style = -1;
        pub const float_round_style_round_toward_zero: root::std::float_round_style = 0;
        pub const float_round_style_round_to_nearest: root::std::float_round_style = 1;
        pub const float_round_style_round_toward_infinity: root::std::float_round_style = 2;
        pub const float_round_style_round_toward_neg_infinity: root::std::float_round_style = 3;
        pub type float_round_style = ::std::os::raw::c_int;
        pub const float_denorm_style_denorm_indeterminate: root::std::float_denorm_style = -1;
        pub const float_denorm_style_denorm_absent: root::std::float_denorm_style = 0;
        pub const float_denorm_style_denorm_present: root::std::float_denorm_style = 1;
        pub type float_denorm_style = ::std::os::raw::c_int;
        pub type __libcpp_numeric_limits_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct numeric_limits {
            pub _address: u8,
        }
        pub type numeric_limits___base = u8;
        pub type numeric_limits_type = u8;
        extern "C" {
            pub static is_specialized: bool;
        }
        extern "C" {
            pub static digits: ::std::os::raw::c_int;
        }
        extern "C" {
            pub static digits10: ::std::os::raw::c_int;
        }
        extern "C" {
            pub static max_digits10: ::std::os::raw::c_int;
        }
        extern "C" {
            pub static is_signed: bool;
        }
        extern "C" {
            pub static is_integer: bool;
        }
        extern "C" {
            pub static is_exact: bool;
        }
        extern "C" {
            pub static radix: ::std::os::raw::c_int;
        }
        extern "C" {
            pub static min_exponent: ::std::os::raw::c_int;
        }
        extern "C" {
            pub static min_exponent10: ::std::os::raw::c_int;
        }
        extern "C" {
            pub static max_exponent: ::std::os::raw::c_int;
        }
        extern "C" {
            pub static max_exponent10: ::std::os::raw::c_int;
        }
        extern "C" {
            pub static has_infinity: bool;
        }
        extern "C" {
            pub static has_quiet_NaN: bool;
        }
        extern "C" {
            pub static has_signaling_NaN: bool;
        }
        extern "C" {
            pub static has_denorm: root::std::float_denorm_style;
        }
        extern "C" {
            pub static has_denorm_loss: bool;
        }
        extern "C" {
            pub static is_iec559: bool;
        }
        extern "C" {
            pub static is_bounded: bool;
        }
        extern "C" {
            pub static is_modulo: bool;
        }
        extern "C" {
            pub static traps: bool;
        }
        extern "C" {
            pub static tinyness_before: bool;
        }
        extern "C" {
            pub static round_style: root::std::float_round_style;
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __nat {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __nat"][::std::mem::size_of::<__nat>() - 1usize];
            ["Alignment of __nat"][::std::mem::align_of::<__nat>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __type_list {
            pub _address: u8,
        }
        pub type __type_list__Head = u8;
        pub type __type_list__Tail = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __align_type {
            pub _address: u8,
        }
        pub type __align_type_type = u8;
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __struct_double {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of __struct_double"][::std::mem::size_of::<__struct_double>() - 8usize];
            ["Alignment of __struct_double"][::std::mem::align_of::<__struct_double>() - 8usize];
        };
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __struct_double4 {
            pub _bindgen_opaque_blob: [u64; 4usize],
        }
        const _: () = {
            ["Size of __struct_double4"][::std::mem::size_of::<__struct_double4>() - 32usize];
            ["Alignment of __struct_double4"][::std::mem::align_of::<__struct_double4>() - 8usize];
        };
        pub type __all_types = u8;
        extern "C" {
            #[link_name = "\u{1}__min"]
            pub static __select_align___min: ::std::os::raw::c_ulong;
        }
        extern "C" {
            #[link_name = "\u{1}__max"]
            pub static __select_align___max: ::std::os::raw::c_ulong;
        }
        extern "C" {
            #[link_name = "\u{1}value"]
            pub static __select_align_value: ::std::os::raw::c_ulong;
        }
        pub type aligned_storage__Aligner = u8;
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Copy, Clone)]
        pub union aligned_storage_type {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of aligned_storage_type"]
                [::std::mem::size_of::<aligned_storage_type>() - 8usize];
            ["Alignment of aligned_storage_type"]
                [::std::mem::align_of::<aligned_storage_type>() - 8usize];
        };
        pub type aligned_storage_t = u8;
        pub type aligned_union_type = u8;
        pub type aligned_union_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct alignment_of {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_volatile {
            pub _address: u8,
        }
        pub type __apply_cv_impl___apply = u8;
        pub type __apply_cv_t = u8;
        pub type __remove_const_ref_t = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __extract_key_fail_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __extract_key_fail_tag"]
                [::std::mem::size_of::<__extract_key_fail_tag>() - 1usize];
            ["Alignment of __extract_key_fail_tag"]
                [::std::mem::align_of::<__extract_key_fail_tag>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __extract_key_self_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __extract_key_self_tag"]
                [::std::mem::size_of::<__extract_key_self_tag>() - 1usize];
            ["Alignment of __extract_key_self_tag"]
                [::std::mem::align_of::<__extract_key_self_tag>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __extract_key_first_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __extract_key_first_tag"]
                [::std::mem::size_of::<__extract_key_first_tag>() - 1usize];
            ["Alignment of __extract_key_first_tag"]
                [::std::mem::align_of::<__extract_key_first_tag>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __can_extract_key {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __can_extract_map_key {
            pub _address: u8,
        }
        pub type __expand_to_true = u8;
        pub type _And = u8;
        pub type _Or = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct remove_all_extents {
            pub _address: u8,
        }
        pub type remove_all_extents_type = u8;
        pub type __remove_all_extents_t = u8;
        pub type remove_all_extents_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct has_virtual_destructor {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_base_of {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_core_convertible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_member_pointer {
            pub _address: u8,
        }
        pub const __libcpp_is_member_pointer___is_member:
            root::std::__libcpp_is_member_pointer__bindgen_ty_1 = 0;
        pub const __libcpp_is_member_pointer___is_func:
            root::std::__libcpp_is_member_pointer__bindgen_ty_1 = 0;
        pub const __libcpp_is_member_pointer___is_obj:
            root::std::__libcpp_is_member_pointer__bindgen_ty_1 = 0;
        pub type __libcpp_is_member_pointer__bindgen_ty_1 = i32;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_member_function_pointer {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_member_object_pointer {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_reference_wrapper_impl {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_reference_wrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __any {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __any"][::std::mem::size_of::<__any>() - 1usize];
            ["Alignment of __any"][::std::mem::align_of::<__any>() - 1usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15__anyC1Ez"]
            pub fn __any___any(this: *mut root::std::__any, ...);
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __member_pointer_traits {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __member_pointer_class_type {
            pub _address: u8,
        }
        pub type __enable_if_bullet1 = u8;
        pub type __enable_if_bullet2 = u8;
        pub type __enable_if_bullet3 = u8;
        pub type __enable_if_bullet4 = u8;
        pub type __enable_if_bullet5 = u8;
        pub type __enable_if_bullet6 = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __invokable_r {
            pub _address: u8,
        }
        pub type __invokable_r__Result = u8;
        pub type __invokable_r_type = u8;
        pub type __invokable = u8;
        pub type __nothrow_invokable_r = u8;
        pub type __nothrow_invokable = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __invoke_of {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_abstract {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_bounded_array {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_callable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_standard_layout {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_trivial {
            pub _address: u8,
        }
        pub type _IsCharLikeType = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_class {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_nullptr_t_impl {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_nullptr_t {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_null_pointer {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_fundamental {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_compound {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_copy_assignable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_copy_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_default_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_destructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_empty {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_final {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_final {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_implicitly_default_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_literal_type {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_member_pointer {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_move_assignable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_move_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_nothrow_assignable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_nothrow_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct _Lazy {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_nothrow_copy_assignable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_nothrow_copy_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_nothrow_default_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_pointer {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_scalar {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_nothrow_destructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_nothrow_move_assignable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_nothrow_move_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_union {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_object {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_pod {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_polymorphic {
            pub _address: u8,
        }
        pub type __swap_result_t = u8;
        pub mod __detail {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            pub type __swappable_with___swap1 = u8;
            pub type __swappable_with___swap2 = u8;
            pub const __detail___block_size: root::std::__detail::_bindgen_ty_1 = 64;
            pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_swappable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_nothrow_swappable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_trivially_assignable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_trivially_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_trivially_copy_assignable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_trivially_copy_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_trivially_default_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_trivially_destructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_trivially_move_assignable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_trivially_move_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_unbounded_array {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_unsigned {
            pub _address: u8,
        }
        pub type __make_signed_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct make_signed {
            pub _address: u8,
        }
        pub type make_signed_type = u8;
        pub type make_signed_t = u8;
        pub type __make_unsigned_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct make_unsigned {
            pub _address: u8,
        }
        pub type make_unsigned_type = u8;
        pub type make_unsigned_t = u8;
        pub type __copy_unsigned_t = u8;
        pub type __maybe_const = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct _Not {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct rank {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct remove_pointer {
            pub _address: u8,
        }
        pub type remove_pointer_type = u8;
        pub type __remove_pointer_t = u8;
        pub type remove_pointer_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct result_of {
            pub _address: u8,
        }
        pub type result_of_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __type_identity {
            pub _address: u8,
        }
        pub type __type_identity_type = u8;
        pub type __type_identity_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __unwrap_reference {
            pub _address: u8,
        }
        pub type __unwrap_reference_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __unwrap_ref_decay {
            pub _address: u8,
        }
        extern "C" {
            #[link_name = "\u{1}value"]
            pub static __static_gcd_value: ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}value"]
            pub static __static_lcm_value: ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}value"]
            pub static __static_abs_value: ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}value"]
            pub static __static_sign_value: ::std::os::raw::c_long;
        }
        pub const __ll_mul_nan: ::std::os::raw::c_long = -9223372036854775808;
        pub const __ll_mul_min: ::std::os::raw::c_long = -9223372036854775807;
        pub const __ll_mul_max: ::std::os::raw::c_long = 9223372036854775807;
        extern "C" {
            #[link_name = "\u{1}__a_x"]
            pub static __ll_mul___a_x: ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}__a_y"]
            pub static __ll_mul___a_y: ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}value"]
            pub static __ll_mul_value: ::std::os::raw::c_long;
        }
        pub const __ll_div_nan: ::std::os::raw::c_long = -9223372036854775808;
        pub const __ll_div_min: ::std::os::raw::c_long = -9223372036854775807;
        pub const __ll_div_max: ::std::os::raw::c_long = 9223372036854775807;
        extern "C" {
            #[link_name = "\u{1}value"]
            pub static __ll_div_value: ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}__na"]
            pub static ratio___na: ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}__da"]
            pub static ratio___da: ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}__s"]
            pub static ratio___s: ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}__gcd"]
            pub static ratio___gcd: ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}num"]
            pub static ratio_num: ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}den"]
            pub static ratio_den: ::std::os::raw::c_long;
        }
        extern "C" {
            pub static num: ::std::os::raw::c_long;
        }
        extern "C" {
            pub static den: ::std::os::raw::c_long;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_ratio {
            pub _address: u8,
        }
        pub type atto = u8;
        pub type femto = u8;
        pub type pico = u8;
        pub type nano = u8;
        pub type micro = u8;
        pub type milli = u8;
        pub type centi = u8;
        pub type deci = u8;
        pub type deca = u8;
        pub type hecto = u8;
        pub type kilo = u8;
        pub type mega = u8;
        pub type giga = u8;
        pub type tera = u8;
        pub type peta = u8;
        pub type exa = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __ratio_multiply {
            pub _address: u8,
        }
        pub type __ratio_multiply_type = u8;
        pub type ratio_multiply = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __ratio_divide {
            pub _address: u8,
        }
        pub type __ratio_divide_type = u8;
        pub type ratio_divide = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __ratio_add {
            pub _address: u8,
        }
        pub type __ratio_add_type = u8;
        pub type ratio_add = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __ratio_subtract {
            pub _address: u8,
        }
        pub type __ratio_subtract_type = u8;
        pub type ratio_subtract = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ratio_equal {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ratio_not_equal {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ratio_less {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ratio_less_equal {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ratio_greater {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ratio_greater_equal {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __ratio_gcd {
            pub _address: u8,
        }
        pub type __ratio_gcd_type = u8;
        pub mod chrono {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __is_duration {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct treat_as_floating_point {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct duration_values {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct duration {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct duration___no_overflow {
                pub _address: u8,
            }
            pub type duration___no_overflow_type = u8;
            pub type duration_rep = u8;
            pub type duration_period = u8;
            pub type nanoseconds = u64;
            pub type microseconds = u64;
            pub type milliseconds = u64;
            pub type seconds = u64;
            pub type minutes = u64;
            pub type hours = u64;
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __duration_eq {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __duration_lt {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct time_point {
                pub _address: u8,
            }
            pub type time_point_clock = u8;
            pub type time_point_duration = u8;
            pub type time_point_rep = u8;
            pub type time_point_period = u8;
            #[repr(C)]
            #[repr(align(1))]
            #[derive(Debug, Copy, Clone)]
            pub struct steady_clock {
                pub _bindgen_opaque_blob: u8,
            }
            pub type steady_clock_duration = u64;
            pub type steady_clock_rep = u64;
            pub type steady_clock_period = u8;
            pub type steady_clock_time_point = u64;
            const _: () = {
                ["Size of steady_clock"][::std::mem::size_of::<steady_clock>() - 1usize];
                ["Alignment of steady_clock"][::std::mem::align_of::<steady_clock>() - 1usize];
            };
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__16chrono12steady_clock3nowEv"]
                pub fn steady_clock_now() -> root::std::chrono::steady_clock_time_point;
            }
            impl steady_clock {
                #[inline]
                pub unsafe fn now() -> root::std::chrono::steady_clock_time_point {
                    steady_clock_now()
                }
            }
            pub const steady_clock_is_steady: bool = true;
            #[repr(C)]
            #[repr(align(1))]
            #[derive(Debug, Copy, Clone)]
            pub struct system_clock {
                pub _bindgen_opaque_blob: u8,
            }
            pub type system_clock_duration = u64;
            pub type system_clock_rep = u64;
            pub type system_clock_period = u8;
            pub type system_clock_time_point = u64;
            const _: () = {
                ["Size of system_clock"][::std::mem::size_of::<system_clock>() - 1usize];
                ["Alignment of system_clock"][::std::mem::align_of::<system_clock>() - 1usize];
            };
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__16chrono12system_clock3nowEv"]
                pub fn system_clock_now() -> root::std::chrono::system_clock_time_point;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__16chrono12system_clock9to_time_tERKNS0_10time_pointIS1_NS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE"]
                pub fn system_clock_to_time_t(
                    __t: *const root::std::chrono::system_clock_time_point,
                ) -> ::std::os::raw::c_long;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__16chrono12system_clock11from_time_tEl"]
                pub fn system_clock_from_time_t(
                    __t: ::std::os::raw::c_long,
                ) -> root::std::chrono::system_clock_time_point;
            }
            impl system_clock {
                #[inline]
                pub unsafe fn now() -> root::std::chrono::system_clock_time_point {
                    system_clock_now()
                }
                #[inline]
                pub unsafe fn to_time_t(
                    __t: *const root::std::chrono::system_clock_time_point,
                ) -> ::std::os::raw::c_long {
                    system_clock_to_time_t(__t)
                }
                #[inline]
                pub unsafe fn from_time_t(
                    __t: ::std::os::raw::c_long,
                ) -> root::std::chrono::system_clock_time_point {
                    system_clock_from_time_t(__t)
                }
            }
            pub const system_clock_is_steady: bool = false;
            pub type high_resolution_clock = u8;
        }
        pub const __libcpp_polling_count: ::std::os::raw::c_int = 64;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __spinning_backoff_policy {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __spinning_backoff_policy"]
                [::std::mem::size_of::<__spinning_backoff_policy>() - 1usize];
            ["Alignment of __spinning_backoff_policy"]
                [::std::mem::align_of::<__spinning_backoff_policy>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __numeric_type {
            pub _address: u8,
        }
        pub type __numeric_type_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __promote {
            pub _address: u8,
        }
        pub type __libcpp_mutex_t = [u64; 8usize];
        pub type __libcpp_recursive_mutex_t = [u64; 8usize];
        pub type __libcpp_condvar_t = [u64; 6usize];
        pub type __libcpp_exec_once_flag = [u64; 2usize];
        pub type __libcpp_thread_id = u64;
        pub type __libcpp_thread_t = u64;
        pub type __libcpp_tls_key = u64;
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__123__cxx_atomic_notify_oneEPVKv"]
            pub fn __cxx_atomic_notify_one(arg1: *const ::std::os::raw::c_void);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__123__cxx_atomic_notify_allEPVKv"]
            pub fn __cxx_atomic_notify_all(arg1: *const ::std::os::raw::c_void);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__123__libcpp_atomic_monitorEPVKv"]
            pub fn __libcpp_atomic_monitor(
                arg1: *const ::std::os::raw::c_void,
            ) -> root::std::__cxx_contention_t;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__120__libcpp_atomic_waitEPVKvx"]
            pub fn __libcpp_atomic_wait(
                arg1: *const ::std::os::raw::c_void,
                arg2: root::std::__cxx_contention_t,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__123__cxx_atomic_notify_oneEPVKNS_17__cxx_atomic_implIxNS_22__cxx_atomic_base_implIxEEEE"]
            pub fn __cxx_atomic_notify_one1(arg1: *const root::std::__cxx_atomic_contention_t);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__123__cxx_atomic_notify_allEPVKNS_17__cxx_atomic_implIxNS_22__cxx_atomic_base_implIxEEEE"]
            pub fn __cxx_atomic_notify_all1(arg1: *const root::std::__cxx_atomic_contention_t);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__123__libcpp_atomic_monitorEPVKNS_17__cxx_atomic_implIxNS_22__cxx_atomic_base_implIxEEEE"]
            pub fn __libcpp_atomic_monitor1(
                arg1: *const root::std::__cxx_atomic_contention_t,
            ) -> root::std::__cxx_contention_t;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__120__libcpp_atomic_waitEPVKNS_17__cxx_atomic_implIxNS_22__cxx_atomic_base_implIxEEEEx"]
            pub fn __libcpp_atomic_wait1(
                arg1: *const root::std::__cxx_atomic_contention_t,
                arg2: root::std::__cxx_contention_t,
            );
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_atomic_wait_backoff_impl {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __cxx_atomic_wait_test_fn_impl {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct atomic {
            pub _address: u8,
        }
        pub type atomic___base = u8;
        pub type atomic_value_type = u8;
        pub type atomic_difference_type = u8;
        pub type atomic_bool = u8;
        pub type atomic_char = u8;
        pub type atomic_schar = u8;
        pub type atomic_uchar = u8;
        pub type atomic_short = u8;
        pub type atomic_ushort = u8;
        pub type atomic_int = u8;
        pub type atomic_uint = u8;
        pub type atomic_long = u8;
        pub type atomic_ulong = u8;
        pub type atomic_llong = u8;
        pub type atomic_ullong = u8;
        pub type atomic_char16_t = u8;
        pub type atomic_char32_t = u8;
        pub type atomic_wchar_t = u8;
        pub type atomic_int_least8_t = u8;
        pub type atomic_uint_least8_t = u8;
        pub type atomic_int_least16_t = u8;
        pub type atomic_uint_least16_t = u8;
        pub type atomic_int_least32_t = u8;
        pub type atomic_uint_least32_t = u8;
        pub type atomic_int_least64_t = u8;
        pub type atomic_uint_least64_t = u8;
        pub type atomic_int_fast8_t = u8;
        pub type atomic_uint_fast8_t = u8;
        pub type atomic_int_fast16_t = u8;
        pub type atomic_uint_fast16_t = u8;
        pub type atomic_int_fast32_t = u8;
        pub type atomic_uint_fast32_t = u8;
        pub type atomic_int_fast64_t = u8;
        pub type atomic_uint_fast64_t = u8;
        pub type atomic_int8_t = u8;
        pub type atomic_uint8_t = u8;
        pub type atomic_int16_t = u8;
        pub type atomic_uint16_t = u8;
        pub type atomic_int32_t = u8;
        pub type atomic_uint32_t = u8;
        pub type atomic_int64_t = u8;
        pub type atomic_uint64_t = u8;
        pub type atomic_intptr_t = u8;
        pub type atomic_uintptr_t = u8;
        pub type atomic_size_t = u8;
        pub type atomic_ptrdiff_t = u8;
        pub type atomic_intmax_t = u8;
        pub type atomic_uintmax_t = u8;
        pub type __libcpp_signed_lock_free = u64;
        pub type __libcpp_unsigned_lock_free = u64;
        pub type atomic_signed_lock_free = u8;
        pub type atomic_unsigned_lock_free = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct atomic_flag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of atomic_flag"][::std::mem::size_of::<atomic_flag>() - 1usize];
            ["Alignment of atomic_flag"][::std::mem::align_of::<atomic_flag>() - 1usize];
        };
        pub type __move_if_noexcept_result_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __exception_guard_exceptions {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __exception_guard_noexceptions {
            pub _address: u8,
        }
        pub type __exception_guard = u8;
        pub type __integer_sequence___convert = u8;
        pub type __integer_sequence___to_tuple_indices = u8;
        pub type __make_indices_imp = u8;
        pub type integer_sequence_value_type = u8;
        pub type index_sequence = u8;
        pub type __make_integer_sequence = u8;
        pub type make_integer_sequence = u8;
        pub type make_index_sequence = u8;
        pub type index_sequence_for = u8;
        pub type __make_tuple_indices_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __tuple_types {
            pub _address: u8,
        }
        pub type tuple_element_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct tuple_size {
            pub _address: u8,
        }
        pub type __enable_if_tuple_size_imp = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __make_tuple_types_flat {
            pub _address: u8,
        }
        pub type __make_tuple_types__RawTp = u8;
        pub type __make_tuple_types__Maker = u8;
        pub type __make_tuple_types_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __tuple_like_ext {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __tuple_sfinae_base {
            pub _bindgen_opaque_blob: u8,
        }
        pub type __tuple_sfinae_base___constructible = u8;
        pub type __tuple_sfinae_base___convertible = u8;
        pub type __tuple_sfinae_base___assignable = u8;
        const _: () = {
            ["Size of __tuple_sfinae_base"][::std::mem::size_of::<__tuple_sfinae_base>() - 1usize];
            ["Alignment of __tuple_sfinae_base"]
                [::std::mem::align_of::<__tuple_sfinae_base>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __check_tuple_constructor_fail {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __check_tuple_constructor_fail"]
                [::std::mem::size_of::<__check_tuple_constructor_fail>() - 1usize];
            ["Alignment of __check_tuple_constructor_fail"]
                [::std::mem::align_of::<__check_tuple_constructor_fail>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct piecewise_construct_t {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of piecewise_construct_t"]
                [::std::mem::size_of::<piecewise_construct_t>() - 1usize];
            ["Alignment of piecewise_construct_t"]
                [::std::mem::align_of::<piecewise_construct_t>() - 1usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__1L19piecewise_constructE"]
            pub static piecewise_construct: root::std::piecewise_construct_t;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __non_trivially_copyable_base {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct pair {
            pub _address: u8,
        }
        pub type pair_first_type = u8;
        pub type pair_second_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct pair__CheckArgs {
            pub _address: u8,
        }
        pub type pair__CheckArgsDep = u8;
        pub mod pmr {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct polymorphic_allocator {
                pub _address: u8,
            }
        }
        pub type string = [u64; 3usize];
        pub type wstring = [u64; 3usize];
        pub type u16string = [u64; 3usize];
        pub type u32string = [u64; 3usize];
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_filebuf {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_ifstream {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_ofstream {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_fstream {
            pub _address: u8,
        }
        pub type filebuf = u8;
        pub type ifstream = u8;
        pub type ofstream = u8;
        pub type fstream = u8;
        pub type wfilebuf = u8;
        pub type wifstream = u8;
        pub type wofstream = u8;
        pub type wfstream = u8;
        pub type ios = [u64; 19usize];
        pub type wios = [u64; 19usize];
        pub type streamoff = u64;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_istream {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_iostream {
            pub _address: u8,
        }
        pub type istream = u8;
        pub type iostream = u8;
        pub type wistream = u8;
        pub type wiostream = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_ostream {
            pub _address: u8,
        }
        pub type ostream = u8;
        pub type wostream = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_stringbuf {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_istringstream {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_ostringstream {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_stringstream {
            pub _address: u8,
        }
        pub type stringbuf = u8;
        pub type istringstream = u8;
        pub type ostringstream = u8;
        pub type stringstream = u8;
        pub type wstringbuf = u8;
        pub type wistringstream = u8;
        pub type wostringstream = u8;
        pub type wstringstream = u8;
        pub type streambuf = [u64; 8usize];
        pub type wstreambuf = [u64; 8usize];
        pub type streampos = u8;
        pub type wstreampos = u8;
        pub type u16streampos = u8;
        pub type u32streampos = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __save_flags {
            pub _address: u8,
        }
        pub type __save_flags___stream_type = u8;
        pub type __save_flags_fmtflags = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_trivial_equality_predicate {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __equal_to {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __equal_to"][::std::mem::size_of::<__equal_to>() - 1usize];
            ["Alignment of __equal_to"][::std::mem::align_of::<__equal_to>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __less {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_signed_integer {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_unsigned_integer {
            pub _address: u8,
        }
        pub type _IsValidExpansion = u8;
        pub type __test_for_primary_template = u8;
        pub type __is_primary_template = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct input_iterator_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of input_iterator_tag"][::std::mem::size_of::<input_iterator_tag>() - 1usize];
            ["Alignment of input_iterator_tag"]
                [::std::mem::align_of::<input_iterator_tag>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct output_iterator_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of output_iterator_tag"][::std::mem::size_of::<output_iterator_tag>() - 1usize];
            ["Alignment of output_iterator_tag"]
                [::std::mem::align_of::<output_iterator_tag>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct forward_iterator_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of forward_iterator_tag"]
                [::std::mem::size_of::<forward_iterator_tag>() - 1usize];
            ["Alignment of forward_iterator_tag"]
                [::std::mem::align_of::<forward_iterator_tag>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct bidirectional_iterator_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of bidirectional_iterator_tag"]
                [::std::mem::size_of::<bidirectional_iterator_tag>() - 1usize];
            ["Alignment of bidirectional_iterator_tag"]
                [::std::mem::align_of::<bidirectional_iterator_tag>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct random_access_iterator_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of random_access_iterator_tag"]
                [::std::mem::size_of::<random_access_iterator_tag>() - 1usize];
            ["Alignment of random_access_iterator_tag"]
                [::std::mem::align_of::<random_access_iterator_tag>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __iter_traits_cache {
            pub _address: u8,
        }
        pub type __iter_traits_cache_type = u8;
        pub type _ITER_TRAITS = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __iter_concept_concept_test {
            pub _bindgen_opaque_blob: u8,
        }
        pub type __iter_concept_concept_test__Apply = u8;
        const _: () = {
            ["Size of __iter_concept_concept_test"]
                [::std::mem::size_of::<__iter_concept_concept_test>() - 1usize];
            ["Alignment of __iter_concept_concept_test"]
                [::std::mem::align_of::<__iter_concept_concept_test>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __iter_concept_category_test {
            pub _bindgen_opaque_blob: u8,
        }
        pub type __iter_concept_category_test__Apply = u8;
        const _: () = {
            ["Size of __iter_concept_category_test"]
                [::std::mem::size_of::<__iter_concept_category_test>() - 1usize];
            ["Alignment of __iter_concept_category_test"]
                [::std::mem::align_of::<__iter_concept_category_test>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __iter_concept_random_fallback {
            pub _bindgen_opaque_blob: u8,
        }
        pub type __iter_concept_random_fallback__Apply = u8;
        const _: () = {
            ["Size of __iter_concept_random_fallback"]
                [::std::mem::size_of::<__iter_concept_random_fallback>() - 1usize];
            ["Alignment of __iter_concept_random_fallback"]
                [::std::mem::align_of::<__iter_concept_random_fallback>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __test_iter_concept {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __iter_concept_cache {
            pub _address: u8,
        }
        pub type __iter_concept_cache_type = u8;
        pub type _ITER_CONCEPT = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_iterator_typedefs {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_iterator_category {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_iterator_concept {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct iterator_traits {
            pub _address: u8,
        }
        pub type iterator_traits___primary_template = u8;
        pub type __has_input_iterator_category = u8;
        pub type __has_forward_iterator_category = u8;
        pub type __has_bidirectional_iterator_category = u8;
        pub type __has_random_access_iterator_category = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_contiguous_iterator {
            pub _address: u8,
        }
        pub type __has_exactly_input_iterator_category = u8;
        pub type __has_exactly_forward_iterator_category = u8;
        pub type __has_exactly_bidirectional_iterator_category = u8;
        pub type __iter_value_type = u8;
        pub type __iter_key_type = u8;
        pub type __iter_mapped_type = u8;
        pub type __iter_to_alloc_type = u8;
        pub type __iterator_category_type = u8;
        pub type __iterator_pointer_type = u8;
        pub type __iter_diff_t = u8;
        pub type __iter_reference = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_element_type {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_difference_type {
            pub _address: u8,
        }
        pub type __pointer_traits_difference_type_type = u64;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_rebind {
            pub _address: u8,
        }
        pub type __pointer_traits_rebind_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct pointer_traits {
            pub _address: u8,
        }
        pub type pointer_traits_pointer = u8;
        pub type pointer_traits_element_type = u8;
        pub type pointer_traits_difference_type = u8;
        pub type pointer_traits_rebind = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct pointer_traits___nat {
            pub _address: u8,
        }
        pub type __rebind_pointer_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct _HasToAddress {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct _HasArrow {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct _IsFancyPointer {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __to_address_helper {
            pub _address: u8,
        }
        pub type __sfinae_underlying_type_type = u8;
        pub type __sfinae_underlying_type___promoted_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct _IterOps {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct _ClassicAlgPolicy {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of _ClassicAlgPolicy"][::std::mem::size_of::<_ClassicAlgPolicy>() - 1usize];
            ["Alignment of _ClassicAlgPolicy"]
                [::std::mem::align_of::<_ClassicAlgPolicy>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_identity {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __identity {
            pub _bindgen_opaque_blob: u8,
        }
        pub type __identity_is_transparent = u8;
        const _: () = {
            ["Size of __identity"][::std::mem::size_of::<__identity>() - 1usize];
            ["Alignment of __identity"][::std::mem::align_of::<__identity>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct binary_function {
            pub _address: u8,
        }
        pub type binary_function_first_argument_type = u8;
        pub type binary_function_second_argument_type = u8;
        pub type binary_function_result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __binary_function_keep_layout_base {
            pub _address: u8,
        }
        pub type __binary_function_keep_layout_base_first_argument_type = u8;
        pub type __binary_function_keep_layout_base_second_argument_type = u8;
        pub type __binary_function_keep_layout_base_result_type = u8;
        pub type __binary_function = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct binary_negate {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct unary_function {
            pub _address: u8,
        }
        pub type unary_function_argument_type = u8;
        pub type unary_function_result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __unary_function_keep_layout_base {
            pub _address: u8,
        }
        pub type __unary_function_keep_layout_base_argument_type = u8;
        pub type __unary_function_keep_layout_base_result_type = u8;
        pub type __unary_function = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_result_type {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __derives_from_unary_function {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __derives_from_unary_function___two {
            pub _address: u8,
        }
        pub type __derives_from_unary_function_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __derives_from_binary_function {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __derives_from_binary_function___two {
            pub _address: u8,
        }
        pub type __derives_from_binary_function_type = u8;
        pub type __weak_result_type_imp_result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __weak_result_type {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __invoke_return {
            pub _address: u8,
        }
        pub type __invoke_return_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_allocator_type {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct uses_allocator {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct allocator_arg_t {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of allocator_arg_t"][::std::mem::size_of::<allocator_arg_t>() - 1usize];
            ["Alignment of allocator_arg_t"][::std::mem::align_of::<allocator_arg_t>() - 1usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__1L13allocator_argE"]
            pub static allocator_arg: root::std::allocator_arg_t;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __uses_alloc_ctor_imp {
            pub _address: u8,
        }
        pub type __uses_alloc_ctor_imp__RawAlloc = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __uses_alloc_ctor {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __all_default_constructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __tuple_impl {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct tuple {
            pub _address: u8,
        }
        pub type tuple__BaseT = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct tuple__IsThisTuple {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct tuple__EnableUTypesCtor {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct tuple__EnableCtorFromUTypesTuple {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct tuple__CtorPredicateFromPair {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct tuple__EnableCtorFromPair {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct tuple__NothrowConstructibleFromPair {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct tuple__BothImplicitlyConvertible {
            pub _address: u8,
        }
        pub mod __find_detail {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__113__find_detailL11__not_foundE"]
                pub static __not_found: ::std::os::raw::c_ulong;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__113__find_detailL11__ambiguousE"]
                pub static __ambiguous: ::std::os::raw::c_ulong;
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __find_exactly_one_checked {
                pub _address: u8,
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __find_exactly_one_t {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __ignore_t {
            pub _address: u8,
        }
        pub mod _bindgen_mod_id_40010 {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__112_GLOBAL__N_16ignoreE"]
                pub static ignore: u8;
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __tuple_cat_type {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __tuple_cat_return {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __tuple_cat_return_ref_imp {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __tuple_cat_return_ref {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __tuple_cat {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_bind_expression {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_placeholder {
            pub _address: u8,
        }
        pub mod placeholders {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__112placeholders2_1E"]
                pub static _1: u8;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__112placeholders2_2E"]
                pub static _2: u8;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__112placeholders2_3E"]
                pub static _3: u8;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__112placeholders2_4E"]
                pub static _4: u8;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__112placeholders2_5E"]
                pub static _5: u8;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__112placeholders2_6E"]
                pub static _6: u8;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__112placeholders2_7E"]
                pub static _7: u8;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__112placeholders2_8E"]
                pub static _8: u8;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__112placeholders2_9E"]
                pub static _9: u8;
            }
            extern "C" {
                #[link_name = "\u{1}__ZNSt3__112placeholders3_10E"]
                pub static _10: u8;
            }
        }
        pub type __mu_return_invokable_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __mu_return {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_valid_bind_return {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __bind {
            pub _address: u8,
        }
        pub type __bind__Fd = u8;
        pub type __bind__Td = u8;
        pub type __bind___indices = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __bind_r {
            pub _address: u8,
        }
        pub type __bind_r_base = u8;
        pub type __bind_r__Fd = u8;
        pub type __bind_r__Td = u8;
        pub type __bind_r_result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct binder1st {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct binder2nd {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct _PairT {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of _PairT"][::std::mem::size_of::<_PairT>() - 16usize];
            ["Alignment of _PairT"][::std::mem::align_of::<_PairT>() - 8usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct hash {
            pub _address: u8,
        }
        pub type __check_hash_requirements = u8;
        pub type __has_enabled_hash = u8;
        pub type __enable_hash_helper = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_trivial_plus_operation {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct plus {
            pub _address: u8,
        }
        pub type plus___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct minus {
            pub _address: u8,
        }
        pub type minus___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct multiplies {
            pub _address: u8,
        }
        pub type multiplies___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct divides {
            pub _address: u8,
        }
        pub type divides___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct modulus {
            pub _address: u8,
        }
        pub type modulus___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct negate {
            pub _address: u8,
        }
        pub type negate___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct bit_and {
            pub _address: u8,
        }
        pub type bit_and___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct bit_not {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct bit_or {
            pub _address: u8,
        }
        pub type bit_or___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct bit_xor {
            pub _address: u8,
        }
        pub type bit_xor___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct equal_to {
            pub _address: u8,
        }
        pub type equal_to___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct not_equal_to {
            pub _address: u8,
        }
        pub type not_equal_to___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct less {
            pub _address: u8,
        }
        pub type less___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct less_equal {
            pub _address: u8,
        }
        pub type less_equal___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct greater_equal {
            pub _address: u8,
        }
        pub type greater_equal___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct greater {
            pub _address: u8,
        }
        pub type greater___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct logical_and {
            pub _address: u8,
        }
        pub type logical_and___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct logical_not {
            pub _address: u8,
        }
        pub type logical_not___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct logical_or {
            pub _address: u8,
        }
        pub type logical_or___result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct reference_wrapper {
            pub _address: u8,
        }
        pub type reference_wrapper_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_pointer {
            pub _address: u8,
        }
        pub type __pointer_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_const_pointer {
            pub _address: u8,
        }
        pub type __const_pointer_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_void_pointer {
            pub _address: u8,
        }
        pub type __void_pointer_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_const_void_pointer {
            pub _address: u8,
        }
        pub type __const_void_pointer_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_size_type {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_alloc_traits_difference_type {
            pub _address: u8,
        }
        pub type __alloc_traits_difference_type_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_propagate_on_container_copy_assignment {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_propagate_on_container_move_assignment {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_propagate_on_container_swap {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_is_always_equal {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_rebind_other {
            pub _address: u8,
        }
        pub type __allocator_traits_rebind_type = u8;
        pub type __allocator_traits_rebind_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_allocate_hint {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_construct_impl {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_construct {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_destroy {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_max_size {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_select_on_container_copy_construction {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct allocator_traits {
            pub _address: u8,
        }
        pub type allocator_traits_allocator_type = u8;
        pub type allocator_traits_value_type = u8;
        pub type allocator_traits_pointer = u8;
        pub type allocator_traits_const_pointer = u8;
        pub type allocator_traits_void_pointer = u8;
        pub type allocator_traits_const_void_pointer = u8;
        pub type allocator_traits_difference_type = u8;
        pub type allocator_traits_size_type = u8;
        pub type allocator_traits_propagate_on_container_copy_assignment = u8;
        pub type allocator_traits_propagate_on_container_move_assignment = u8;
        pub type allocator_traits_propagate_on_container_swap = u8;
        pub type allocator_traits_is_always_equal = u8;
        pub type allocator_traits_rebind_alloc = u8;
        pub type allocator_traits_rebind_traits = u8;
        pub type __rebind_alloc = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_default_allocator {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_cpp17_move_insertable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_cpp17_copy_insertable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __allocation_guard {
            pub _address: u8,
        }
        pub type __allocation_guard__Pointer = u8;
        pub type __allocation_guard__Size = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __allocation_result {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct allocator {
            pub _address: u8,
        }
        pub type allocator_size_type = u64;
        pub type allocator_difference_type = u64;
        pub type allocator_value_type = u8;
        pub type allocator_propagate_on_container_move_assignment = u8;
        pub type allocator_is_always_equal = u8;
        pub type allocator_pointer = u8;
        pub type allocator_const_pointer = u8;
        pub type allocator_reference = u8;
        pub type allocator_const_reference = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct allocator_rebind {
            pub _address: u8,
        }
        pub type allocator_rebind_other = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __allocator_destructor {
            pub _address: u8,
        }
        pub type __allocator_destructor___alloc_traits = u8;
        pub type __allocator_destructor_pointer = u8;
        pub type __allocator_destructor_size_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct auto_ptr_ref {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct auto_ptr {
            pub _address: u8,
        }
        pub type auto_ptr_element_type = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __default_init_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __default_init_tag"][::std::mem::size_of::<__default_init_tag>() - 1usize];
            ["Alignment of __default_init_tag"]
                [::std::mem::align_of::<__default_init_tag>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __value_init_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __value_init_tag"][::std::mem::size_of::<__value_init_tag>() - 1usize];
            ["Alignment of __value_init_tag"][::std::mem::align_of::<__value_init_tag>() - 1usize];
        };
        pub type __compressed_pair_elem__ParamT = u8;
        pub type __compressed_pair_elem_reference = u8;
        pub type __compressed_pair_elem_const_reference = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __compressed_pair {
            pub _address: u8,
        }
        pub type __compressed_pair__Base1 = u8;
        pub type __compressed_pair__Base2 = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_datasizeof {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_datasizeof__FirstPaddingByte {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_always_bitcastable {
            pub _address: u8,
        }
        pub type __is_always_bitcastable__UnqualFrom = u8;
        pub type __is_always_bitcastable__UnqualTo = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_equality_comparable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_trivially_equality_comparable_impl {
            pub _address: u8,
        }
        pub type __libcpp_is_trivially_equality_comparable = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_is_trivially_lexicographically_comparable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_less_than_comparable {
            pub _address: u8,
        }
        pub type __element_count = ::std::os::raw::c_ulong;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __can_lower_copy_assignment_to_memmove {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __can_lower_move_assignment_to_memmove {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __overload {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __can_rewrap {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __segmented_iterator_traits {
            pub _address: u8,
        }
        pub type __is_segmented_iterator = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __debug_less {
            pub _address: u8,
        }
        pub type __comp_ref_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __copy_loop {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __copy_loop__CopySegment {
            pub _address: u8,
        }
        pub type __copy_loop__CopySegment__Traits = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __copy_trivial {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __copy_trivial"][::std::mem::size_of::<__copy_trivial>() - 1usize];
            ["Alignment of __copy_trivial"][::std::mem::align_of::<__copy_trivial>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __move_loop {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __move_loop__MoveSegment {
            pub _address: u8,
        }
        pub type __move_loop__MoveSegment__Traits = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __move_trivial {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __move_trivial"][::std::mem::size_of::<__move_trivial>() - 1usize];
            ["Alignment of __move_trivial"][::std::mem::align_of::<__move_trivial>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct iterator {
            pub _address: u8,
        }
        pub type iterator_value_type = u8;
        pub type iterator_difference_type = u8;
        pub type iterator_pointer = u8;
        pub type iterator_reference = u8;
        pub type iterator_iterator_category = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct reverse_iterator {
            pub _address: u8,
        }
        pub type reverse_iterator_iterator_type = u8;
        pub type reverse_iterator_iterator_category = u8;
        pub type reverse_iterator_pointer = u8;
        pub type reverse_iterator_value_type = u8;
        pub type reverse_iterator_difference_type = u8;
        pub type reverse_iterator_reference = u8;
        pub type __unconstrained_reverse_iterator = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __unwrap_reverse_iter_impl {
            pub _address: u8,
        }
        pub type __unwrap_reverse_iter_impl__UnwrappedIter = u8;
        pub type __unwrap_reverse_iter_impl__ReverseWrapper = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __unreachable_sentinel {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __unreachable_sentinel"]
                [::std::mem::size_of::<__unreachable_sentinel>() - 1usize];
            ["Alignment of __unreachable_sentinel"]
                [::std::mem::align_of::<__unreachable_sentinel>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct _AllocatorDestroyRangeReverse {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __allocator_has_trivial_copy_construct {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __allocator_has_trivial_move_construct {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct default_delete {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __unique_ptr_deleter_sfinae {
            pub _address: u8,
        }
        pub type __unique_ptr_deleter_sfinae___lval_ref_type = u8;
        pub type __unique_ptr_deleter_sfinae___good_rval_ref_type = u8;
        pub type __unique_ptr_deleter_sfinae___enable_rval_overload = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct unique_ptr {
            pub _address: u8,
        }
        pub type unique_ptr_element_type = u8;
        pub type unique_ptr_deleter_type = u8;
        pub type unique_ptr_pointer = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct unique_ptr___nat {
            pub _address: u8,
        }
        pub type unique_ptr__DeleterSFINAE = u8;
        pub type unique_ptr__LValRefType = u8;
        pub type unique_ptr__GoodRValRefType = u8;
        pub type unique_ptr__BadRValRefType = u8;
        pub type unique_ptr__EnableIfDeleterDefaultConstructible = u8;
        pub type unique_ptr__EnableIfDeleterConstructible = u8;
        pub type unique_ptr__EnableIfMoveConvertible = u8;
        pub type unique_ptr__EnableIfDeleterConvertible = u8;
        pub type unique_ptr__EnableIfDeleterAssignable = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __unique_if {
            pub _address: u8,
        }
        pub type __unique_if___unique_single = u8;
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_refstring {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of __libcpp_refstring"][::std::mem::size_of::<__libcpp_refstring>() - 8usize];
            ["Alignment of __libcpp_refstring"]
                [::std::mem::align_of::<__libcpp_refstring>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__121__throw_runtime_errorEPKc"]
            pub fn __throw_runtime_error(arg1: *const ::std::os::raw::c_char);
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct bad_weak_ptr {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of bad_weak_ptr"][::std::mem::size_of::<bad_weak_ptr>() - 8usize];
            ["Alignment of bad_weak_ptr"][::std::mem::align_of::<bad_weak_ptr>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112bad_weak_ptrD1Ev"]
            pub fn bad_weak_ptr_bad_weak_ptr_destructor(this: *mut root::std::bad_weak_ptr);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt3__112bad_weak_ptr4whatEv"]
            pub fn bad_weak_ptr_what(
                this: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __shared_count {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of __shared_count"][::std::mem::size_of::<__shared_count>() - 16usize];
            ["Alignment of __shared_count"][::std::mem::align_of::<__shared_count>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__114__shared_countD1Ev"]
            pub fn __shared_count___shared_count_destructor(this: *mut root::std::__shared_count);
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __shared_weak_count {
            pub _bindgen_opaque_blob: [u64; 3usize],
        }
        const _: () = {
            ["Size of __shared_weak_count"][::std::mem::size_of::<__shared_weak_count>() - 24usize];
            ["Alignment of __shared_weak_count"]
                [::std::mem::align_of::<__shared_weak_count>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__119__shared_weak_count14__release_weakEv"]
            pub fn __shared_weak_count___release_weak(this: *mut root::std::__shared_weak_count);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__119__shared_weak_count4lockEv"]
            pub fn __shared_weak_count_lock(
                this: *mut root::std::__shared_weak_count,
            ) -> *mut root::std::__shared_weak_count;
        }
        impl __shared_weak_count {
            #[inline]
            pub unsafe fn __release_weak(&mut self) {
                __shared_weak_count___release_weak(self)
            }
            #[inline]
            pub unsafe fn lock(&mut self) -> *mut root::std::__shared_weak_count {
                __shared_weak_count_lock(self)
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__119__shared_weak_countD1Ev"]
            pub fn __shared_weak_count___shared_weak_count_destructor(
                this: *mut root::std::__shared_weak_count,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt3__119__shared_weak_count13__get_deleterERKSt9type_info"]
            pub fn __shared_weak_count___get_deleter(
                this: *mut ::std::os::raw::c_void,
                arg1: *const root::std::type_info,
            ) -> *const ::std::os::raw::c_void;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __shared_ptr_pointer {}
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __for_overwrite_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __for_overwrite_tag"][::std::mem::size_of::<__for_overwrite_tag>() - 1usize];
            ["Alignment of __for_overwrite_tag"]
                [::std::mem::align_of::<__for_overwrite_tag>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __shared_ptr_emplace {}
        pub type __shared_ptr_emplace__CompressedPair = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __shared_ptr_emplace__Storage {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __shared_ptr_dummy_rebind_allocator_type {
            _unused: [u8; 0],
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __compatible_with {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __raw_pointer_compatible_with {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_deletable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_array_deletable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __well_formed_deleter {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __shared_ptr_deleter_ctor_reqs {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct shared_ptr {
            pub _address: u8,
        }
        pub type shared_ptr_element_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct shared_ptr___shared_ptr_default_delete {
            pub _address: u8,
        }
        pub type shared_ptr___shared_ptr_default_allocator_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct weak_ptr {
            pub _address: u8,
        }
        pub type weak_ptr_element_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct owner_less {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct enable_shared_from_this {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __sp_mut {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of __sp_mut"][::std::mem::size_of::<__sp_mut>() - 8usize];
            ["Alignment of __sp_mut"][::std::mem::align_of::<__sp_mut>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18__sp_mut4lockEv"]
            pub fn __sp_mut_lock(this: *mut root::std::__sp_mut);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18__sp_mut6unlockEv"]
            pub fn __sp_mut_unlock(this: *mut root::std::__sp_mut);
        }
        impl __sp_mut {
            #[inline]
            pub unsafe fn lock(&mut self) {
                __sp_mut_lock(self)
            }
            #[inline]
            pub unsafe fn unlock(&mut self) {
                __sp_mut_unlock(self)
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112__get_sp_mutEPKv"]
            pub fn __get_sp_mut(arg1: *const ::std::os::raw::c_void) -> *mut root::std::__sp_mut;
        }
        pub type array___self = u8;
        pub type array_value_type = u8;
        pub type array_reference = u8;
        pub type array_const_reference = u8;
        pub type array_iterator = u8;
        pub type array_const_iterator = u8;
        pub type array_pointer = u8;
        pub type array_const_pointer = u8;
        pub type array_size_type = u64;
        pub type array_difference_type = u64;
        pub type array_reverse_iterator = u8;
        pub type array_const_reverse_iterator = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __copy_backward_loop {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __copy_backward_trivial {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __copy_backward_trivial"]
                [::std::mem::size_of::<__copy_backward_trivial>() - 1usize];
            ["Alignment of __copy_backward_trivial"]
                [::std::mem::align_of::<__copy_backward_trivial>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __move_backward_loop {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __move_backward_trivial {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __move_backward_trivial"]
                [::std::mem::size_of::<__move_backward_trivial>() - 1usize];
            ["Alignment of __move_backward_trivial"]
                [::std::mem::align_of::<__move_backward_trivial>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __destruct_n {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of __destruct_n"][::std::mem::size_of::<__destruct_n>() - 8usize];
            ["Alignment of __destruct_n"][::std::mem::align_of::<__destruct_n>() - 8usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __return_temporary_buffer {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __return_temporary_buffer"]
                [::std::mem::size_of::<__return_temporary_buffer>() - 1usize];
            ["Alignment of __return_temporary_buffer"]
                [::std::mem::align_of::<__return_temporary_buffer>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __invert {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct _ConstTimeDistance {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct _MinmaxElementLessFunc {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_simple_comparator {
            pub _address: u8,
        }
        pub type __use_branchless_sort = u8;
        pub type __is_any_of = u8;
        pub type __sort_is_specialized_in_library = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct _ProjectedPred {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct move_iterator {
            pub _address: u8,
        }
        pub type move_iterator_iterator_type = u8;
        pub type move_iterator_iterator_category = u8;
        pub type move_iterator_value_type = u8;
        pub type move_iterator_difference_type = u8;
        pub type move_iterator_pointer = u8;
        pub type move_iterator___reference = u8;
        pub type move_iterator_reference = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __stable_sort_switch {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __wrap_iter {
            pub _address: u8,
        }
        pub type __wrap_iter_iterator_type = u8;
        pub type __wrap_iter_value_type = u8;
        pub type __wrap_iter_difference_type = u8;
        pub type __wrap_iter_pointer = u8;
        pub type __wrap_iter_reference = u8;
        pub type __wrap_iter_iterator_category = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_random_is_valid_inttype {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_random_is_valid_urng {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __independent_bits_engine {
            pub _address: u8,
        }
        pub type __independent_bits_engine_result_type = u8;
        pub type __independent_bits_engine__Engine_result_type = u8;
        pub type __independent_bits_engine__Working_result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct uniform_int_distribution {
            pub _address: u8,
        }
        pub type uniform_int_distribution_result_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct uniform_int_distribution_param_type {
            pub _address: u8,
        }
        pub type uniform_int_distribution_param_type_distribution_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __set_intersection_result {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __set_symmetric_difference_result {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __set_union_result {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __libcpp_debug_randomizer {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        pub type __libcpp_debug_randomizer_result_type = u32;
        const _: () = {
            ["Size of __libcpp_debug_randomizer"]
                [::std::mem::size_of::<__libcpp_debug_randomizer>() - 16usize];
            ["Alignment of __libcpp_debug_randomizer"]
                [::std::mem::align_of::<__libcpp_debug_randomizer>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__125__libcpp_debug_randomizer4_MinE"]
            pub static __libcpp_debug_randomizer__Min:
                root::std::__libcpp_debug_randomizer_result_type;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__125__libcpp_debug_randomizer4_MaxE"]
            pub static __libcpp_debug_randomizer__Max:
                root::std::__libcpp_debug_randomizer_result_type;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18__rs_getEv"]
            pub fn __rs_get() -> root::std::__rs_default;
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __rs_default {
            pub _bindgen_opaque_blob: u8,
        }
        pub type __rs_default_result_type = u32;
        const _: () = {
            ["Size of __rs_default"][::std::mem::size_of::<__rs_default>() - 1usize];
            ["Alignment of __rs_default"][::std::mem::align_of::<__rs_default>() - 1usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112__rs_defaultC1ERKS0_"]
            pub fn __rs_default___rs_default(
                this: *mut root::std::__rs_default,
                arg1: *const root::std::__rs_default,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112__rs_defaultD1Ev"]
            pub fn __rs_default___rs_default_destructor(this: *mut root::std::__rs_default);
        }
        impl __rs_default {
            #[inline]
            pub unsafe fn new(arg1: *const root::std::__rs_default) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                __rs_default___rs_default(__bindgen_tmp.as_mut_ptr(), arg1);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                __rs_default___rs_default_destructor(self)
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112__rs_default4__c_E"]
            pub static mut __rs_default___c_: ::std::os::raw::c_uint;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112__rs_default4_MinE"]
            pub static __rs_default__Min: root::std::__rs_default_result_type;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112__rs_default4_MaxE"]
            pub static __rs_default__Max: root::std::__rs_default_result_type;
        }
        pub mod __unique_copy_tags {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            #[repr(C)]
            #[repr(align(1))]
            #[derive(Debug, Copy, Clone)]
            pub struct __reread_from_input_tag {
                pub _bindgen_opaque_blob: u8,
            }
            const _: () = {
                ["Size of __reread_from_input_tag"]
                    [::std::mem::size_of::<__reread_from_input_tag>() - 1usize];
                ["Alignment of __reread_from_input_tag"]
                    [::std::mem::align_of::<__reread_from_input_tag>() - 1usize];
            };
            #[repr(C)]
            #[repr(align(1))]
            #[derive(Debug, Copy, Clone)]
            pub struct __reread_from_output_tag {
                pub _bindgen_opaque_blob: u8,
            }
            const _: () = {
                ["Size of __reread_from_output_tag"]
                    [::std::mem::size_of::<__reread_from_output_tag>() - 1usize];
                ["Alignment of __reread_from_output_tag"]
                    [::std::mem::align_of::<__reread_from_output_tag>() - 1usize];
            };
            #[repr(C)]
            #[repr(align(1))]
            #[derive(Debug, Copy, Clone)]
            pub struct __read_from_tmp_value_tag {
                pub _bindgen_opaque_blob: u8,
            }
            const _: () = {
                ["Size of __read_from_tmp_value_tag"]
                    [::std::mem::size_of::<__read_from_tmp_value_tag>() - 1usize];
                ["Alignment of __read_from_tmp_value_tag"]
                    [::std::mem::align_of::<__read_from_tmp_value_tag>() - 1usize];
            };
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct back_insert_iterator {
            pub _address: u8,
        }
        pub type back_insert_iterator_iterator_category = u8;
        pub type back_insert_iterator_value_type = u8;
        pub type back_insert_iterator_difference_type = u8;
        pub type back_insert_iterator_pointer = u8;
        pub type back_insert_iterator_reference = u8;
        pub type back_insert_iterator_container_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __bounded_iter {
            pub _address: u8,
        }
        pub type __bounded_iter_value_type = u8;
        pub type __bounded_iter_difference_type = u8;
        pub type __bounded_iter_pointer = u8;
        pub type __bounded_iter_reference = u8;
        pub type __bounded_iter_iterator_category = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct front_insert_iterator {
            pub _address: u8,
        }
        pub type front_insert_iterator_iterator_category = u8;
        pub type front_insert_iterator_value_type = u8;
        pub type front_insert_iterator_difference_type = u8;
        pub type front_insert_iterator_pointer = u8;
        pub type front_insert_iterator_reference = u8;
        pub type front_insert_iterator_container_type = u8;
        pub type __insert_iterator_iter_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct insert_iterator {
            pub _address: u8,
        }
        pub type insert_iterator_iterator_category = u8;
        pub type insert_iterator_value_type = u8;
        pub type insert_iterator_difference_type = u8;
        pub type insert_iterator_pointer = u8;
        pub type insert_iterator_reference = u8;
        pub type insert_iterator_container_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct istream_iterator {
            pub _address: u8,
        }
        pub type istream_iterator_iterator_category = u8;
        pub type istream_iterator_value_type = u8;
        pub type istream_iterator_difference_type = u8;
        pub type istream_iterator_pointer = u8;
        pub type istream_iterator_reference = u8;
        pub type istream_iterator_char_type = u8;
        pub type istream_iterator_traits_type = u8;
        pub type istream_iterator_istream_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct istreambuf_iterator {
            pub _address: u8,
        }
        pub type istreambuf_iterator_iterator_category = u8;
        pub type istreambuf_iterator_value_type = u8;
        pub type istreambuf_iterator_difference_type = u8;
        pub type istreambuf_iterator_pointer = u8;
        pub type istreambuf_iterator_reference = u8;
        pub type istreambuf_iterator_char_type = u8;
        pub type istreambuf_iterator_traits_type = u8;
        pub type istreambuf_iterator_int_type = u8;
        pub type istreambuf_iterator_streambuf_type = u8;
        pub type istreambuf_iterator_istream_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct istreambuf_iterator___proxy {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ostream_iterator {
            pub _address: u8,
        }
        pub type ostream_iterator_iterator_category = u8;
        pub type ostream_iterator_value_type = u8;
        pub type ostream_iterator_difference_type = u8;
        pub type ostream_iterator_pointer = u8;
        pub type ostream_iterator_reference = u8;
        pub type ostream_iterator_char_type = u8;
        pub type ostream_iterator_traits_type = u8;
        pub type ostream_iterator_ostream_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ostreambuf_iterator {
            pub _address: u8,
        }
        pub type ostreambuf_iterator_iterator_category = u8;
        pub type ostreambuf_iterator_value_type = u8;
        pub type ostreambuf_iterator_difference_type = u8;
        pub type ostreambuf_iterator_pointer = u8;
        pub type ostreambuf_iterator_reference = u8;
        pub type ostreambuf_iterator_char_type = u8;
        pub type ostreambuf_iterator_traits_type = u8;
        pub type ostreambuf_iterator_streambuf_type = u8;
        pub type ostreambuf_iterator_ostream_type = u8;
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15alignEmmRPvRm"]
            pub fn align(
                __align: ::std::os::raw::c_ulong,
                __sz: ::std::os::raw::c_ulong,
                __ptr: *mut *mut ::std::os::raw::c_void,
                __space: *mut ::std::os::raw::c_ulong,
            ) -> *mut ::std::os::raw::c_void;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct raw_storage_iterator {
            pub _address: u8,
        }
        pub type raw_storage_iterator_iterator_category = u8;
        pub type raw_storage_iterator_value_type = u8;
        pub type raw_storage_iterator_difference_type = u8;
        pub type raw_storage_iterator_pointer = u8;
        pub type raw_storage_iterator_reference = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_transparent {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_hash_value_type_imp {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_hash_value_type {
            pub _address: u8,
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112__next_primeEm"]
            pub fn __next_prime(__n: ::std::os::raw::c_ulong) -> ::std::os::raw::c_ulong;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_node_base {
            pub _address: u8,
        }
        pub type __hash_node_base___node_type = u8;
        pub type __hash_node_base___first_node = u8;
        pub type __hash_node_base___node_base_pointer = u8;
        pub type __hash_node_base___node_pointer = u8;
        pub type __hash_node_base___next_pointer = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_node {
            pub _address: u8,
        }
        pub type __hash_node___node_value_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_key_value_types {
            pub _address: u8,
        }
        pub type __hash_key_value_types_key_type = u8;
        pub type __hash_key_value_types___node_value_type = u8;
        pub type __hash_key_value_types___container_value_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_node_types {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_node_types_from_iterator {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __make_hash_node_types {
            pub _address: u8,
        }
        pub type __make_hash_node_types__NodeTp = u8;
        pub type __make_hash_node_types__NodePtr = u8;
        pub type __make_hash_node_types_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_iterator {
            pub _address: u8,
        }
        pub type __hash_iterator__NodeTypes = u8;
        pub type __hash_iterator___node_pointer = u8;
        pub type __hash_iterator___next_pointer = u8;
        pub type __hash_iterator_iterator_category = u8;
        pub type __hash_iterator_value_type = u8;
        pub type __hash_iterator_difference_type = u8;
        pub type __hash_iterator_reference = u8;
        pub type __hash_iterator_pointer = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_const_iterator {
            pub _address: u8,
        }
        pub type __hash_const_iterator__NodeTypes = u8;
        pub type __hash_const_iterator___node_pointer = u8;
        pub type __hash_const_iterator___next_pointer = u8;
        pub type __hash_const_iterator___non_const_iterator = u8;
        pub type __hash_const_iterator_iterator_category = u8;
        pub type __hash_const_iterator_value_type = u8;
        pub type __hash_const_iterator_difference_type = u8;
        pub type __hash_const_iterator_reference = u8;
        pub type __hash_const_iterator_pointer = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_local_iterator {
            pub _address: u8,
        }
        pub type __hash_local_iterator__NodeTypes = u8;
        pub type __hash_local_iterator___node_pointer = u8;
        pub type __hash_local_iterator___next_pointer = u8;
        pub type __hash_local_iterator_iterator_category = u8;
        pub type __hash_local_iterator_value_type = u8;
        pub type __hash_local_iterator_difference_type = u8;
        pub type __hash_local_iterator_reference = u8;
        pub type __hash_local_iterator_pointer = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_const_local_iterator {
            pub _address: u8,
        }
        pub type __hash_const_local_iterator__NodeTypes = u8;
        pub type __hash_const_local_iterator___node_pointer = u8;
        pub type __hash_const_local_iterator___next_pointer = u8;
        pub type __hash_const_local_iterator___pointer_traits = u8;
        pub type __hash_const_local_iterator___node = u8;
        pub type __hash_const_local_iterator___non_const_node = u8;
        pub type __hash_const_local_iterator___non_const_node_pointer = u8;
        pub type __hash_const_local_iterator___non_const_iterator = u8;
        pub type __hash_const_local_iterator_iterator_category = u8;
        pub type __hash_const_local_iterator_value_type = u8;
        pub type __hash_const_local_iterator_difference_type = u8;
        pub type __hash_const_local_iterator_reference = u8;
        pub type __hash_const_local_iterator_pointer = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __bucket_list_deallocator {
            pub _address: u8,
        }
        pub type __bucket_list_deallocator_allocator_type = u8;
        pub type __bucket_list_deallocator___alloc_traits = u8;
        pub type __bucket_list_deallocator_size_type = u8;
        pub type __bucket_list_deallocator_pointer = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_node_destructor {
            pub _address: u8,
        }
        pub type __hash_node_destructor_allocator_type = u8;
        pub type __hash_node_destructor___alloc_traits = u8;
        pub type __hash_node_destructor_pointer = u8;
        pub type __hash_node_destructor__NodeTypes = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __enforce_unordered_container_requirements {
            pub _address: u8,
        }
        pub type __enforce_unordered_container_requirements_type = u32;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_table {
            pub _address: u8,
        }
        pub type __hash_table_value_type = u8;
        pub type __hash_table_hasher = u8;
        pub type __hash_table_key_equal = u8;
        pub type __hash_table_allocator_type = u8;
        pub type __hash_table___alloc_traits = u8;
        pub type __hash_table__NodeTypes = u8;
        pub type __hash_table___node_value_type = u8;
        pub type __hash_table___container_value_type = u8;
        pub type __hash_table_key_type = u8;
        pub type __hash_table_reference = u8;
        pub type __hash_table_const_reference = u8;
        pub type __hash_table_pointer = u8;
        pub type __hash_table_const_pointer = u8;
        pub type __hash_table_size_type = u8;
        pub type __hash_table_difference_type = u8;
        pub type __hash_table___node = u8;
        pub type __hash_table___node_allocator = u8;
        pub type __hash_table___node_traits = u8;
        pub type __hash_table___void_pointer = u8;
        pub type __hash_table___node_pointer = u8;
        pub type __hash_table___node_const_pointer = u8;
        pub type __hash_table___first_node = u8;
        pub type __hash_table___node_base_pointer = u8;
        pub type __hash_table___next_pointer = u8;
        pub type __hash_table___node_base_allocator = u8;
        pub type __hash_table___node_base_traits = u8;
        pub type __hash_table___pointer_allocator = u8;
        pub type __hash_table___bucket_list_deleter = u8;
        pub type __hash_table___bucket_list = u8;
        pub type __hash_table___pointer_alloc_traits = u8;
        pub type __hash_table___node_pointer_pointer = u8;
        pub type __hash_table_iterator = u8;
        pub type __hash_table_const_iterator = u8;
        pub type __hash_table_local_iterator = u8;
        pub type __hash_table_const_local_iterator = u8;
        pub type __hash_table__Dp = u8;
        pub type __hash_table___node_holder = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __is_allocator {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_map_node_destructor {
            pub _address: u8,
        }
        pub type __hash_map_node_destructor_allocator_type = u8;
        pub type __hash_map_node_destructor___alloc_traits = u8;
        pub type __hash_map_node_destructor_pointer = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_value_type {
            pub _address: u8,
        }
        pub type __hash_value_type_key_type = u8;
        pub type __hash_value_type_mapped_type = u8;
        pub type __hash_value_type_value_type = u8;
        pub type __hash_value_type___nc_ref_pair_type = u8;
        pub type __hash_value_type___nc_rref_pair_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_map_iterator {
            pub _address: u8,
        }
        pub type __hash_map_iterator__NodeTypes = u8;
        pub type __hash_map_iterator_iterator_category = u8;
        pub type __hash_map_iterator_value_type = u8;
        pub type __hash_map_iterator_difference_type = u8;
        pub type __hash_map_iterator_reference = u8;
        pub type __hash_map_iterator_pointer = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __hash_map_const_iterator {
            pub _address: u8,
        }
        pub type __hash_map_const_iterator__NodeTypes = u8;
        pub type __hash_map_const_iterator_iterator_category = u8;
        pub type __hash_map_const_iterator_value_type = u8;
        pub type __hash_map_const_iterator_difference_type = u8;
        pub type __hash_map_const_iterator_reference = u8;
        pub type __hash_map_const_iterator_pointer = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct unordered_map {
            pub _address: u8,
        }
        pub type unordered_map_key_type = u8;
        pub type unordered_map_mapped_type = u8;
        pub type unordered_map_hasher = u8;
        pub type unordered_map_key_equal = u8;
        pub type unordered_map_allocator_type = u8;
        pub type unordered_map_value_type = u8;
        pub type unordered_map_reference = u8;
        pub type unordered_map_const_reference = u8;
        pub type unordered_map___value_type = u8;
        pub type unordered_map___hasher = u8;
        pub type unordered_map___key_equal = u8;
        pub type unordered_map___allocator_type = u8;
        pub type unordered_map___table = u8;
        pub type unordered_map__NodeTypes = u8;
        pub type unordered_map___node_pointer = u8;
        pub type unordered_map___node_const_pointer = u8;
        pub type unordered_map___node_traits = u8;
        pub type unordered_map___node_allocator = u8;
        pub type unordered_map___node = u8;
        pub type unordered_map__Dp = u8;
        pub type unordered_map___node_holder = u8;
        pub type unordered_map___alloc_traits = u8;
        pub type unordered_map_pointer = u8;
        pub type unordered_map_const_pointer = u8;
        pub type unordered_map_size_type = u8;
        pub type unordered_map_difference_type = u8;
        pub type unordered_map_iterator = u8;
        pub type unordered_map_const_iterator = u8;
        pub type unordered_map_local_iterator = u8;
        pub type unordered_map_const_local_iterator = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct unordered_multimap {
            pub _address: u8,
        }
        pub type unordered_multimap_key_type = u8;
        pub type unordered_multimap_mapped_type = u8;
        pub type unordered_multimap_hasher = u8;
        pub type unordered_multimap_key_equal = u8;
        pub type unordered_multimap_allocator_type = u8;
        pub type unordered_multimap_value_type = u8;
        pub type unordered_multimap_reference = u8;
        pub type unordered_multimap_const_reference = u8;
        pub type unordered_multimap___value_type = u8;
        pub type unordered_multimap___hasher = u8;
        pub type unordered_multimap___key_equal = u8;
        pub type unordered_multimap___allocator_type = u8;
        pub type unordered_multimap___table = u8;
        pub type unordered_multimap__NodeTypes = u8;
        pub type unordered_multimap___node_traits = u8;
        pub type unordered_multimap___node_allocator = u8;
        pub type unordered_multimap___node = u8;
        pub type unordered_multimap__Dp = u8;
        pub type unordered_multimap___node_holder = u8;
        pub type unordered_multimap___alloc_traits = u8;
        pub type unordered_multimap_pointer = u8;
        pub type unordered_multimap_const_pointer = u8;
        pub type unordered_multimap_size_type = u8;
        pub type unordered_multimap_difference_type = u8;
        pub type unordered_multimap_iterator = u8;
        pub type unordered_multimap_const_iterator = u8;
        pub type unordered_multimap_local_iterator = u8;
        pub type unordered_multimap_const_local_iterator = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __has_storage_type {
            pub _address: u8,
        }
        pub type __bit_reference___storage_type = u8;
        pub type __bit_reference___storage_pointer = u8;
        pub type __bit_reference___container = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __bit_const_reference {
            pub _address: u8,
        }
        pub type __bit_const_reference___storage_type = u8;
        pub type __bit_const_reference___storage_pointer = u8;
        pub type __bit_const_reference___container = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __bit_array {
            pub _address: u8,
        }
        pub type __bit_array_difference_type = u8;
        pub type __bit_array___storage_type = u8;
        pub type __bit_array___storage_pointer = u8;
        pub type __bit_array_iterator = u8;
        pub type __bit_iterator_difference_type = u8;
        pub type __bit_iterator_value_type = u8;
        pub type __bit_iterator_pointer = u8;
        pub type __bit_iterator_reference = u8;
        pub type __bit_iterator_iterator_category = u8;
        pub type __bit_iterator___storage_type = u8;
        pub type __bit_iterator___storage_pointer = u8;
        pub type string_view = [u64; 2usize];
        pub type u16string_view = [u64; 2usize];
        pub type u32string_view = [u64; 2usize];
        pub type wstring_view = [u64; 2usize];
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct char_traits {
            pub _address: u8,
        }
        pub type char_traits_char_type = u8;
        pub type char_traits_int_type = u32;
        pub type char_traits_off_type = u64;
        pub type char_traits_pos_type = u8;
        pub type char_traits_state_type = [u64; 16usize];
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_string_view {
            pub _address: u8,
        }
        pub type basic_string_view_traits_type = u8;
        pub type basic_string_view_value_type = u8;
        pub type basic_string_view_pointer = u8;
        pub type basic_string_view_const_pointer = u8;
        pub type basic_string_view_reference = u8;
        pub type basic_string_view_const_reference = u8;
        pub type basic_string_view_const_iterator = u8;
        pub type basic_string_view_iterator = u8;
        pub type basic_string_view_const_reverse_iterator = u8;
        pub type basic_string_view_reverse_iterator = u8;
        pub type basic_string_view_size_type = u64;
        pub type basic_string_view_difference_type = u64;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __string_view_hash {
            pub _address: u8,
        }
        pub const errc_address_family_not_supported: root::std::errc = 47;
        pub const errc_address_in_use: root::std::errc = 48;
        pub const errc_address_not_available: root::std::errc = 49;
        pub const errc_already_connected: root::std::errc = 56;
        pub const errc_argument_list_too_long: root::std::errc = 7;
        pub const errc_argument_out_of_domain: root::std::errc = 33;
        pub const errc_bad_address: root::std::errc = 14;
        pub const errc_bad_file_descriptor: root::std::errc = 9;
        pub const errc_bad_message: root::std::errc = 94;
        pub const errc_broken_pipe: root::std::errc = 32;
        pub const errc_connection_aborted: root::std::errc = 53;
        pub const errc_connection_already_in_progress: root::std::errc = 37;
        pub const errc_connection_refused: root::std::errc = 61;
        pub const errc_connection_reset: root::std::errc = 54;
        pub const errc_cross_device_link: root::std::errc = 18;
        pub const errc_destination_address_required: root::std::errc = 39;
        pub const errc_device_or_resource_busy: root::std::errc = 16;
        pub const errc_directory_not_empty: root::std::errc = 66;
        pub const errc_executable_format_error: root::std::errc = 8;
        pub const errc_file_exists: root::std::errc = 17;
        pub const errc_file_too_large: root::std::errc = 27;
        pub const errc_filename_too_long: root::std::errc = 63;
        pub const errc_function_not_supported: root::std::errc = 78;
        pub const errc_host_unreachable: root::std::errc = 65;
        pub const errc_identifier_removed: root::std::errc = 90;
        pub const errc_illegal_byte_sequence: root::std::errc = 92;
        pub const errc_inappropriate_io_control_operation: root::std::errc = 25;
        pub const errc_interrupted: root::std::errc = 4;
        pub const errc_invalid_argument: root::std::errc = 22;
        pub const errc_invalid_seek: root::std::errc = 29;
        pub const errc_io_error: root::std::errc = 5;
        pub const errc_is_a_directory: root::std::errc = 21;
        pub const errc_message_size: root::std::errc = 40;
        pub const errc_network_down: root::std::errc = 50;
        pub const errc_network_reset: root::std::errc = 52;
        pub const errc_network_unreachable: root::std::errc = 51;
        pub const errc_no_buffer_space: root::std::errc = 55;
        pub const errc_no_child_process: root::std::errc = 10;
        pub const errc_no_link: root::std::errc = 97;
        pub const errc_no_lock_available: root::std::errc = 77;
        pub const errc_no_message_available: root::std::errc = 96;
        pub const errc_no_message: root::std::errc = 91;
        pub const errc_no_protocol_option: root::std::errc = 42;
        pub const errc_no_space_on_device: root::std::errc = 28;
        pub const errc_no_stream_resources: root::std::errc = 98;
        pub const errc_no_such_device_or_address: root::std::errc = 6;
        pub const errc_no_such_device: root::std::errc = 19;
        pub const errc_no_such_file_or_directory: root::std::errc = 2;
        pub const errc_no_such_process: root::std::errc = 3;
        pub const errc_not_a_directory: root::std::errc = 20;
        pub const errc_not_a_socket: root::std::errc = 38;
        pub const errc_not_a_stream: root::std::errc = 99;
        pub const errc_not_connected: root::std::errc = 57;
        pub const errc_not_enough_memory: root::std::errc = 12;
        pub const errc_not_supported: root::std::errc = 45;
        pub const errc_operation_canceled: root::std::errc = 89;
        pub const errc_operation_in_progress: root::std::errc = 36;
        pub const errc_operation_not_permitted: root::std::errc = 1;
        pub const errc_operation_not_supported: root::std::errc = 102;
        pub const errc_operation_would_block: root::std::errc = 35;
        pub const errc_owner_dead: root::std::errc = 105;
        pub const errc_permission_denied: root::std::errc = 13;
        pub const errc_protocol_error: root::std::errc = 100;
        pub const errc_protocol_not_supported: root::std::errc = 43;
        pub const errc_read_only_file_system: root::std::errc = 30;
        pub const errc_resource_deadlock_would_occur: root::std::errc = 11;
        pub const errc_resource_unavailable_try_again: root::std::errc = 35;
        pub const errc_result_out_of_range: root::std::errc = 34;
        pub const errc_state_not_recoverable: root::std::errc = 104;
        pub const errc_stream_timeout: root::std::errc = 101;
        pub const errc_text_file_busy: root::std::errc = 26;
        pub const errc_timed_out: root::std::errc = 60;
        pub const errc_too_many_files_open_in_system: root::std::errc = 23;
        pub const errc_too_many_files_open: root::std::errc = 24;
        pub const errc_too_many_links: root::std::errc = 31;
        pub const errc_too_many_symbolic_link_levels: root::std::errc = 62;
        pub const errc_value_too_large: root::std::errc = 84;
        pub const errc_wrong_protocol_type: root::std::errc = 41;
        pub type errc = ::std::os::raw::c_int;
        pub type __make_32_64_or_128_bit_t = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct fpos {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __noexcept_move_assign_container {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __string_is_trivial_iterator {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __can_be_converted_to_string_view {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __uninitialized_size_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __uninitialized_size_tag"]
                [::std::mem::size_of::<__uninitialized_size_tag>() - 1usize];
            ["Alignment of __uninitialized_size_tag"]
                [::std::mem::align_of::<__uninitialized_size_tag>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __init_with_sentinel_tag {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __init_with_sentinel_tag"]
                [::std::mem::size_of::<__init_with_sentinel_tag>() - 1usize];
            ["Alignment of __init_with_sentinel_tag"]
                [::std::mem::align_of::<__init_with_sentinel_tag>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_string {
            pub _address: u8,
        }
        pub type basic_string___self = u8;
        pub type basic_string___self_view = u8;
        pub type basic_string_traits_type = u8;
        pub type basic_string_value_type = u8;
        pub type basic_string_allocator_type = u8;
        pub type basic_string___alloc_traits = u8;
        pub type basic_string_size_type = u8;
        pub type basic_string_difference_type = u8;
        pub type basic_string_reference = u8;
        pub type basic_string_const_reference = u8;
        pub type basic_string_pointer = u8;
        pub type basic_string_const_pointer = u8;
        pub type basic_string_iterator = u8;
        pub type basic_string_const_iterator = u8;
        pub type basic_string_reverse_iterator = u8;
        pub type basic_string_const_reverse_iterator = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_string___long {
            pub _address: u8,
        }
        pub const basic_string___min_cap: root::std::basic_string__bindgen_ty_1 = 0;
        pub type basic_string__bindgen_ty_1 = i32;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_string___short {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Copy, Clone)]
        pub struct basic_string___ulx {
            pub _bindgen_opaque_blob: [u8; 0usize],
        }
        pub const basic_string___n_words: root::std::basic_string__bindgen_ty_2 = 0;
        pub type basic_string__bindgen_ty_2 = i32;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_string___raw {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_string___rep {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Copy, Clone)]
        pub struct basic_string___rep__bindgen_ty_1 {
            pub _bindgen_opaque_blob: [u8; 0usize],
        }
        pub const basic_string___alignment: root::std::basic_string__bindgen_ty_3 = 0;
        pub type basic_string__bindgen_ty_3 = i32;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __traits_eq {
            pub _address: u8,
        }
        pub type __traits_eq_char_type = u8;
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__14stoiERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEPmi"]
            pub fn stoi(
                __str: *const root::std::string,
                __idx: *mut ::std::os::raw::c_ulong,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__14stolERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEPmi"]
            pub fn stol(
                __str: *const root::std::string,
                __idx: *mut ::std::os::raw::c_ulong,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15stoulERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEPmi"]
            pub fn stoul(
                __str: *const root::std::string,
                __idx: *mut ::std::os::raw::c_ulong,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_ulong;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15stollERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEPmi"]
            pub fn stoll(
                __str: *const root::std::string,
                __idx: *mut ::std::os::raw::c_ulong,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_longlong;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16stoullERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEPmi"]
            pub fn stoull(
                __str: *const root::std::string,
                __idx: *mut ::std::os::raw::c_ulong,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_ulonglong;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__14stofERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEPm"]
            pub fn stof(
                __str: *const root::std::string,
                __idx: *mut ::std::os::raw::c_ulong,
            ) -> f32;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__14stodERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEPm"]
            pub fn stod(
                __str: *const root::std::string,
                __idx: *mut ::std::os::raw::c_ulong,
            ) -> f64;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15stoldERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEPm"]
            pub fn stold(
                __str: *const root::std::string,
                __idx: *mut ::std::os::raw::c_ulong,
            ) -> f64;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__19to_stringEi"]
            pub fn to_string(__val: ::std::os::raw::c_int) -> root::std::string;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__19to_stringEj"]
            pub fn to_string1(__val: ::std::os::raw::c_uint) -> root::std::string;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__19to_stringEl"]
            pub fn to_string2(__val: ::std::os::raw::c_long) -> root::std::string;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__19to_stringEm"]
            pub fn to_string3(__val: ::std::os::raw::c_ulong) -> root::std::string;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__19to_stringEx"]
            pub fn to_string4(__val: ::std::os::raw::c_longlong) -> root::std::string;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__19to_stringEy"]
            pub fn to_string5(__val: ::std::os::raw::c_ulonglong) -> root::std::string;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__19to_stringEf"]
            pub fn to_string6(__val: f32) -> root::std::string;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__19to_stringEd"]
            pub fn to_string7(__val: f64) -> root::std::string;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__19to_stringEe"]
            pub fn to_string8(__val: f64) -> root::std::string;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__14stoiERKNS_12basic_stringIwNS_11char_traitsIwEENS_9allocatorIwEEEEPmi"]
            pub fn stoi1(
                __str: *const root::std::wstring,
                __idx: *mut ::std::os::raw::c_ulong,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__14stolERKNS_12basic_stringIwNS_11char_traitsIwEENS_9allocatorIwEEEEPmi"]
            pub fn stol1(
                __str: *const root::std::wstring,
                __idx: *mut ::std::os::raw::c_ulong,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15stoulERKNS_12basic_stringIwNS_11char_traitsIwEENS_9allocatorIwEEEEPmi"]
            pub fn stoul1(
                __str: *const root::std::wstring,
                __idx: *mut ::std::os::raw::c_ulong,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_ulong;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15stollERKNS_12basic_stringIwNS_11char_traitsIwEENS_9allocatorIwEEEEPmi"]
            pub fn stoll1(
                __str: *const root::std::wstring,
                __idx: *mut ::std::os::raw::c_ulong,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_longlong;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16stoullERKNS_12basic_stringIwNS_11char_traitsIwEENS_9allocatorIwEEEEPmi"]
            pub fn stoull1(
                __str: *const root::std::wstring,
                __idx: *mut ::std::os::raw::c_ulong,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_ulonglong;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__14stofERKNS_12basic_stringIwNS_11char_traitsIwEENS_9allocatorIwEEEEPm"]
            pub fn stof1(
                __str: *const root::std::wstring,
                __idx: *mut ::std::os::raw::c_ulong,
            ) -> f32;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__14stodERKNS_12basic_stringIwNS_11char_traitsIwEENS_9allocatorIwEEEEPm"]
            pub fn stod1(
                __str: *const root::std::wstring,
                __idx: *mut ::std::os::raw::c_ulong,
            ) -> f64;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15stoldERKNS_12basic_stringIwNS_11char_traitsIwEENS_9allocatorIwEEEEPm"]
            pub fn stold1(
                __str: *const root::std::wstring,
                __idx: *mut ::std::os::raw::c_ulong,
            ) -> f64;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110to_wstringEi"]
            pub fn to_wstring(__val: ::std::os::raw::c_int) -> root::std::wstring;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110to_wstringEj"]
            pub fn to_wstring1(__val: ::std::os::raw::c_uint) -> root::std::wstring;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110to_wstringEl"]
            pub fn to_wstring2(__val: ::std::os::raw::c_long) -> root::std::wstring;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110to_wstringEm"]
            pub fn to_wstring3(__val: ::std::os::raw::c_ulong) -> root::std::wstring;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110to_wstringEx"]
            pub fn to_wstring4(__val: ::std::os::raw::c_longlong) -> root::std::wstring;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110to_wstringEy"]
            pub fn to_wstring5(__val: ::std::os::raw::c_ulonglong) -> root::std::wstring;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110to_wstringEf"]
            pub fn to_wstring6(__val: f32) -> root::std::wstring;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110to_wstringEd"]
            pub fn to_wstring7(__val: f64) -> root::std::wstring;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110to_wstringEe"]
            pub fn to_wstring8(__val: f64) -> root::std::wstring;
        }
        extern "C" {
            pub static npos: root::std::basic_string_size_type;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __string_hash {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct mutex {
            pub _bindgen_opaque_blob: [u64; 8usize],
        }
        pub type mutex_native_handle_type = u64;
        const _: () = {
            ["Size of mutex"][::std::mem::size_of::<mutex>() - 64usize];
            ["Alignment of mutex"][::std::mem::align_of::<mutex>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15mutex4lockEv"]
            pub fn mutex_lock(this: *mut root::std::mutex);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15mutex8try_lockEv"]
            pub fn mutex_try_lock(this: *mut root::std::mutex) -> bool;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15mutex6unlockEv"]
            pub fn mutex_unlock(this: *mut root::std::mutex);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__15mutexD1Ev"]
            pub fn mutex_mutex_destructor(this: *mut root::std::mutex);
        }
        impl mutex {
            #[inline]
            pub unsafe fn lock(&mut self) {
                mutex_lock(self)
            }
            #[inline]
            pub unsafe fn try_lock(&mut self) -> bool {
                mutex_try_lock(self)
            }
            #[inline]
            pub unsafe fn unlock(&mut self) {
                mutex_unlock(self)
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                mutex_mutex_destructor(self)
            }
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct defer_lock_t {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of defer_lock_t"][::std::mem::size_of::<defer_lock_t>() - 1usize];
            ["Alignment of defer_lock_t"][::std::mem::align_of::<defer_lock_t>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct try_to_lock_t {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of try_to_lock_t"][::std::mem::size_of::<try_to_lock_t>() - 1usize];
            ["Alignment of try_to_lock_t"][::std::mem::align_of::<try_to_lock_t>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct adopt_lock_t {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of adopt_lock_t"][::std::mem::size_of::<adopt_lock_t>() - 1usize];
            ["Alignment of adopt_lock_t"][::std::mem::align_of::<adopt_lock_t>() - 1usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__1L10defer_lockE"]
            pub static defer_lock: root::std::defer_lock_t;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__1L11try_to_lockE"]
            pub static try_to_lock: root::std::try_to_lock_t;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__1L10adopt_lockE"]
            pub static adopt_lock: root::std::adopt_lock_t;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct error_category {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of error_category"][::std::mem::size_of::<error_category>() - 8usize];
            ["Alignment of error_category"][::std::mem::align_of::<error_category>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__114error_categoryD1Ev"]
            pub fn error_category_error_category_destructor(this: *mut root::std::error_category);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt3__114error_category23default_error_conditionEi"]
            pub fn error_category_default_error_condition(
                this: *mut ::std::os::raw::c_void,
                __ev: ::std::os::raw::c_int,
            ) -> root::std::error_condition;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt3__114error_category10equivalentEiRKNS_15error_conditionE"]
            pub fn error_category_equivalent(
                this: *mut ::std::os::raw::c_void,
                __code: ::std::os::raw::c_int,
                __condition: *const root::std::error_condition,
            ) -> bool;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt3__114error_category10equivalentERKNS_10error_codeEi"]
            pub fn error_category_equivalent1(
                this: *mut ::std::os::raw::c_void,
                __code: *const root::std::error_code,
                __condition: ::std::os::raw::c_int,
            ) -> bool;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __do_message {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of __do_message"][::std::mem::size_of::<__do_message>() - 8usize];
            ["Alignment of __do_message"][::std::mem::align_of::<__do_message>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__116generic_categoryEv"]
            pub fn generic_category() -> *const root::std::error_category;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__115system_categoryEv"]
            pub fn system_category() -> *const root::std::error_category;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_error_condition_enum {
            pub _address: u8,
        }
        pub mod __adl_only {
            #[allow(unused_imports)]
            use self::super::super::super::root;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct error_condition {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of error_condition"][::std::mem::size_of::<error_condition>() - 16usize];
            ["Alignment of error_condition"][::std::mem::align_of::<error_condition>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNKSt3__115error_condition7messageEv"]
            pub fn error_condition_message(
                this: *const root::std::error_condition,
            ) -> root::std::string;
        }
        impl error_condition {
            #[inline]
            pub unsafe fn message(&self) -> root::std::string {
                error_condition_message(self)
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct is_error_code_enum {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct error_code {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of error_code"][::std::mem::size_of::<error_code>() - 16usize];
            ["Alignment of error_code"][::std::mem::align_of::<error_code>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNKSt3__110error_code7messageEv"]
            pub fn error_code_message(this: *const root::std::error_code) -> root::std::string;
        }
        impl error_code {
            #[inline]
            pub unsafe fn message(&self) -> root::std::string {
                error_code_message(self)
            }
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct system_error {
            pub _bindgen_opaque_blob: [u64; 4usize],
        }
        const _: () = {
            ["Size of system_error"][::std::mem::size_of::<system_error>() - 32usize];
            ["Alignment of system_error"][::std::mem::align_of::<system_error>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112system_errorC1ENS_10error_codeERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE"]
            pub fn system_error_system_error(
                this: *mut root::std::system_error,
                __ec: root::std::error_code,
                __what_arg: *const root::std::string,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112system_errorC1ENS_10error_codeEPKc"]
            pub fn system_error_system_error1(
                this: *mut root::std::system_error,
                __ec: root::std::error_code,
                __what_arg: *const ::std::os::raw::c_char,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112system_errorC1ENS_10error_codeE"]
            pub fn system_error_system_error2(
                this: *mut root::std::system_error,
                __ec: root::std::error_code,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112system_errorC1EiRKNS_14error_categoryERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE"]
            pub fn system_error_system_error3(
                this: *mut root::std::system_error,
                __ev: ::std::os::raw::c_int,
                __ecat: *const root::std::error_category,
                __what_arg: *const root::std::string,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112system_errorC1EiRKNS_14error_categoryEPKc"]
            pub fn system_error_system_error4(
                this: *mut root::std::system_error,
                __ev: ::std::os::raw::c_int,
                __ecat: *const root::std::error_category,
                __what_arg: *const ::std::os::raw::c_char,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112system_errorC1EiRKNS_14error_categoryE"]
            pub fn system_error_system_error5(
                this: *mut root::std::system_error,
                __ev: ::std::os::raw::c_int,
                __ecat: *const root::std::error_category,
            );
        }
        impl system_error {
            #[inline]
            pub unsafe fn new(
                __ec: root::std::error_code,
                __what_arg: *const root::std::string,
            ) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                system_error_system_error(__bindgen_tmp.as_mut_ptr(), __ec, __what_arg);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new1(
                __ec: root::std::error_code,
                __what_arg: *const ::std::os::raw::c_char,
            ) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                system_error_system_error1(__bindgen_tmp.as_mut_ptr(), __ec, __what_arg);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new2(__ec: root::std::error_code) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                system_error_system_error2(__bindgen_tmp.as_mut_ptr(), __ec);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new3(
                __ev: ::std::os::raw::c_int,
                __ecat: *const root::std::error_category,
                __what_arg: *const root::std::string,
            ) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                system_error_system_error3(__bindgen_tmp.as_mut_ptr(), __ev, __ecat, __what_arg);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new4(
                __ev: ::std::os::raw::c_int,
                __ecat: *const root::std::error_category,
                __what_arg: *const ::std::os::raw::c_char,
            ) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                system_error_system_error4(__bindgen_tmp.as_mut_ptr(), __ev, __ecat, __what_arg);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new5(
                __ev: ::std::os::raw::c_int,
                __ecat: *const root::std::error_category,
            ) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                system_error_system_error5(__bindgen_tmp.as_mut_ptr(), __ev, __ecat);
                __bindgen_tmp.assume_init()
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112system_errorD1Ev"]
            pub fn system_error_system_error_destructor(this: *mut root::std::system_error);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__120__throw_system_errorEiPKc"]
            pub fn __throw_system_error(
                __ev: ::std::os::raw::c_int,
                __what_arg: *const ::std::os::raw::c_char,
            );
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct unique_lock {
            pub _address: u8,
        }
        pub type unique_lock_mutex_type = u8;
        pub const cv_status_no_timeout: root::std::cv_status = 0;
        pub const cv_status_timeout: root::std::cv_status = 1;
        pub type cv_status = ::std::os::raw::c_int;
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct condition_variable {
            pub _bindgen_opaque_blob: [u64; 6usize],
        }
        pub type condition_variable_native_handle_type = u64;
        const _: () = {
            ["Size of condition_variable"][::std::mem::size_of::<condition_variable>() - 48usize];
            ["Alignment of condition_variable"]
                [::std::mem::align_of::<condition_variable>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__118condition_variable10notify_oneEv"]
            pub fn condition_variable_notify_one(this: *mut root::std::condition_variable);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__118condition_variable10notify_allEv"]
            pub fn condition_variable_notify_all(this: *mut root::std::condition_variable);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__118condition_variable4waitERNS_11unique_lockINS_5mutexEEE"]
            pub fn condition_variable_wait(
                this: *mut root::std::condition_variable,
                __lk: *mut [u64; 2usize],
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__118condition_variableD1Ev"]
            pub fn condition_variable_condition_variable_destructor(
                this: *mut root::std::condition_variable,
            );
        }
        impl condition_variable {
            #[inline]
            pub unsafe fn notify_one(&mut self) {
                condition_variable_notify_one(self)
            }
            #[inline]
            pub unsafe fn notify_all(&mut self) {
                condition_variable_notify_all(self)
            }
            #[inline]
            pub unsafe fn wait(&mut self, __lk: *mut [u64; 2usize]) {
                condition_variable_wait(self, __lk)
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                condition_variable_condition_variable_destructor(self)
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct lock_guard {
            pub _address: u8,
        }
        pub type lock_guard_mutex_type = u8;
        pub mod this_thread {
            #[allow(unused_imports)]
            use self::super::super::super::root;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __thread_id {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of __thread_id"][::std::mem::size_of::<__thread_id>() - 8usize];
            ["Alignment of __thread_id"][::std::mem::align_of::<__thread_id>() - 8usize];
        };
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct recursive_mutex {
            pub _bindgen_opaque_blob: [u64; 8usize],
        }
        pub type recursive_mutex_native_handle_type = u64;
        const _: () = {
            ["Size of recursive_mutex"][::std::mem::size_of::<recursive_mutex>() - 64usize];
            ["Alignment of recursive_mutex"][::std::mem::align_of::<recursive_mutex>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__115recursive_mutex4lockEv"]
            pub fn recursive_mutex_lock(this: *mut root::std::recursive_mutex);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__115recursive_mutex8try_lockEv"]
            pub fn recursive_mutex_try_lock(this: *mut root::std::recursive_mutex) -> bool;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__115recursive_mutex6unlockEv"]
            pub fn recursive_mutex_unlock(this: *mut root::std::recursive_mutex);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__115recursive_mutexC1Ev"]
            pub fn recursive_mutex_recursive_mutex(this: *mut root::std::recursive_mutex);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__115recursive_mutexD1Ev"]
            pub fn recursive_mutex_recursive_mutex_destructor(
                this: *mut root::std::recursive_mutex,
            );
        }
        impl recursive_mutex {
            #[inline]
            pub unsafe fn lock(&mut self) {
                recursive_mutex_lock(self)
            }
            #[inline]
            pub unsafe fn try_lock(&mut self) -> bool {
                recursive_mutex_try_lock(self)
            }
            #[inline]
            pub unsafe fn unlock(&mut self) {
                recursive_mutex_unlock(self)
            }
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                recursive_mutex_recursive_mutex(__bindgen_tmp.as_mut_ptr());
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                recursive_mutex_recursive_mutex_destructor(self)
            }
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct timed_mutex {
            pub _bindgen_opaque_blob: [u64; 15usize],
        }
        const _: () = {
            ["Size of timed_mutex"][::std::mem::size_of::<timed_mutex>() - 120usize];
            ["Alignment of timed_mutex"][::std::mem::align_of::<timed_mutex>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__111timed_mutex4lockEv"]
            pub fn timed_mutex_lock(this: *mut root::std::timed_mutex);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__111timed_mutex8try_lockEv"]
            pub fn timed_mutex_try_lock(this: *mut root::std::timed_mutex) -> bool;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__111timed_mutex6unlockEv"]
            pub fn timed_mutex_unlock(this: *mut root::std::timed_mutex);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__111timed_mutexC1Ev"]
            pub fn timed_mutex_timed_mutex(this: *mut root::std::timed_mutex);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__111timed_mutexD1Ev"]
            pub fn timed_mutex_timed_mutex_destructor(this: *mut root::std::timed_mutex);
        }
        impl timed_mutex {
            #[inline]
            pub unsafe fn lock(&mut self) {
                timed_mutex_lock(self)
            }
            #[inline]
            pub unsafe fn try_lock(&mut self) -> bool {
                timed_mutex_try_lock(self)
            }
            #[inline]
            pub unsafe fn unlock(&mut self) {
                timed_mutex_unlock(self)
            }
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                timed_mutex_timed_mutex(__bindgen_tmp.as_mut_ptr());
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                timed_mutex_timed_mutex_destructor(self)
            }
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct recursive_timed_mutex {
            pub _bindgen_opaque_blob: [u64; 16usize],
        }
        const _: () = {
            ["Size of recursive_timed_mutex"]
                [::std::mem::size_of::<recursive_timed_mutex>() - 128usize];
            ["Alignment of recursive_timed_mutex"]
                [::std::mem::align_of::<recursive_timed_mutex>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__121recursive_timed_mutex4lockEv"]
            pub fn recursive_timed_mutex_lock(this: *mut root::std::recursive_timed_mutex);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__121recursive_timed_mutex8try_lockEv"]
            pub fn recursive_timed_mutex_try_lock(
                this: *mut root::std::recursive_timed_mutex,
            ) -> bool;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__121recursive_timed_mutex6unlockEv"]
            pub fn recursive_timed_mutex_unlock(this: *mut root::std::recursive_timed_mutex);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__121recursive_timed_mutexC1Ev"]
            pub fn recursive_timed_mutex_recursive_timed_mutex(
                this: *mut root::std::recursive_timed_mutex,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__121recursive_timed_mutexD1Ev"]
            pub fn recursive_timed_mutex_recursive_timed_mutex_destructor(
                this: *mut root::std::recursive_timed_mutex,
            );
        }
        impl recursive_timed_mutex {
            #[inline]
            pub unsafe fn lock(&mut self) {
                recursive_timed_mutex_lock(self)
            }
            #[inline]
            pub unsafe fn try_lock(&mut self) -> bool {
                recursive_timed_mutex_try_lock(self)
            }
            #[inline]
            pub unsafe fn unlock(&mut self) {
                recursive_timed_mutex_unlock(self)
            }
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                recursive_timed_mutex_recursive_timed_mutex(__bindgen_tmp.as_mut_ptr());
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                recursive_timed_mutex_recursive_timed_mutex_destructor(self)
            }
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct once_flag {
            pub _bindgen_opaque_blob: u64,
        }
        pub type once_flag__State_type = u64;
        const _: () = {
            ["Size of once_flag"][::std::mem::size_of::<once_flag>() - 8usize];
            ["Alignment of once_flag"][::std::mem::align_of::<once_flag>() - 8usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __call_once_param {
            pub _address: u8,
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__111__call_onceERVmPvPFvS2_E"]
            pub fn __call_once(
                arg1: *mut root::std::once_flag__State_type,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::option::Option<
                    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
                >,
            );
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct locale {
            pub _bindgen_opaque_blob: u64,
        }
        pub type locale_category = u32;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct locale___imp {
            _unused: [u8; 0],
        }
        const _: () = {
            ["Size of locale"][::std::mem::size_of::<locale>() - 8usize];
            ["Alignment of locale"][::std::mem::align_of::<locale>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNKSt3__16locale4nameEv"]
            pub fn locale_name(this: *const root::std::locale) -> root::std::string;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16locale6globalERKS0_"]
            pub fn locale_global(arg1: *const root::std::locale) -> root::std::locale;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16locale7classicEv"]
            pub fn locale_classic() -> *const root::std::locale;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16localeC1Ev"]
            pub fn locale_locale(this: *mut root::std::locale);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16localeC1ERKS0_"]
            pub fn locale_locale1(this: *mut root::std::locale, arg1: *const root::std::locale);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16localeC1EPKc"]
            pub fn locale_locale2(
                this: *mut root::std::locale,
                arg1: *const ::std::os::raw::c_char,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16localeC1ERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE"]
            pub fn locale_locale3(this: *mut root::std::locale, arg1: *const root::std::string);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16localeC1ERKS0_PKci"]
            pub fn locale_locale4(
                this: *mut root::std::locale,
                arg1: *const root::std::locale,
                arg2: *const ::std::os::raw::c_char,
                arg3: root::std::locale_category,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16localeC1ERKS0_RKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEi"]
            pub fn locale_locale5(
                this: *mut root::std::locale,
                arg1: *const root::std::locale,
                arg2: *const root::std::string,
                arg3: root::std::locale_category,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16localeC1ERKS0_S2_i"]
            pub fn locale_locale6(
                this: *mut root::std::locale,
                arg1: *const root::std::locale,
                arg2: *const root::std::locale,
                arg3: root::std::locale_category,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16localeD1Ev"]
            pub fn locale_locale_destructor(this: *mut root::std::locale);
        }
        impl locale {
            #[inline]
            pub unsafe fn name(&self) -> root::std::string {
                locale_name(self)
            }
            #[inline]
            pub unsafe fn global(arg1: *const root::std::locale) -> root::std::locale {
                locale_global(arg1)
            }
            #[inline]
            pub unsafe fn classic() -> *const root::std::locale {
                locale_classic()
            }
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                locale_locale(__bindgen_tmp.as_mut_ptr());
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new1(arg1: *const root::std::locale) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                locale_locale1(__bindgen_tmp.as_mut_ptr(), arg1);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new2(arg1: *const ::std::os::raw::c_char) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                locale_locale2(__bindgen_tmp.as_mut_ptr(), arg1);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new3(arg1: *const root::std::string) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                locale_locale3(__bindgen_tmp.as_mut_ptr(), arg1);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new4(
                arg1: *const root::std::locale,
                arg2: *const ::std::os::raw::c_char,
                arg3: root::std::locale_category,
            ) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                locale_locale4(__bindgen_tmp.as_mut_ptr(), arg1, arg2, arg3);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new5(
                arg1: *const root::std::locale,
                arg2: *const root::std::string,
                arg3: root::std::locale_category,
            ) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                locale_locale5(__bindgen_tmp.as_mut_ptr(), arg1, arg2, arg3);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new6(
                arg1: *const root::std::locale,
                arg2: *const root::std::locale,
                arg3: root::std::locale_category,
            ) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                locale_locale6(__bindgen_tmp.as_mut_ptr(), arg1, arg2, arg3);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                locale_locale_destructor(self)
            }
        }
        pub const locale_none: root::std::locale_category = 0;
        pub const locale_collate: root::std::locale_category = 1;
        pub const locale_ctype: root::std::locale_category = 2;
        pub const locale_monetary: root::std::locale_category = 8;
        pub const locale_numeric: root::std::locale_category = 16;
        pub const locale_time: root::std::locale_category = 32;
        pub const locale_messages: root::std::locale_category = 4;
        pub const locale_all: root::std::locale_category = 63;
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct locale_facet {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of locale_facet"][::std::mem::size_of::<locale_facet>() - 16usize];
            ["Alignment of locale_facet"][::std::mem::align_of::<locale_facet>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16locale5facetD1Ev"]
            pub fn locale_facet_facet_destructor(this: *mut root::std::locale_facet);
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct locale_id {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of locale_id"][::std::mem::size_of::<locale_id>() - 16usize];
            ["Alignment of locale_id"][::std::mem::align_of::<locale_id>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16locale2id5__getEv"]
            pub fn locale_id___get(this: *mut root::std::locale_id) -> ::std::os::raw::c_long;
        }
        impl locale_id {
            #[inline]
            pub unsafe fn __get(&mut self) -> ::std::os::raw::c_long {
                locale_id___get(self)
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__16locale2id9__next_idE"]
            pub static mut locale_id___next_id: ::std::os::raw::c_int;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct collate {}
        pub type collate_char_type = u8;
        pub type collate_string_type = u8;
        extern "C" {
            pub static mut id: root::std::locale_id;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct collate_byname {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct ctype_base {
            pub _bindgen_opaque_blob: u8,
        }
        pub type ctype_base_mask = u32;
        const _: () = {
            ["Size of ctype_base"][::std::mem::size_of::<ctype_base>() - 1usize];
            ["Alignment of ctype_base"][::std::mem::align_of::<ctype_base>() - 1usize];
        };
        pub const ctype_base_space: root::std::ctype_base_mask = 16384;
        pub const ctype_base_print: root::std::ctype_base_mask = 262144;
        pub const ctype_base_cntrl: root::std::ctype_base_mask = 512;
        pub const ctype_base_upper: root::std::ctype_base_mask = 32768;
        pub const ctype_base_lower: root::std::ctype_base_mask = 4096;
        pub const ctype_base_alpha: root::std::ctype_base_mask = 256;
        pub const ctype_base_digit: root::std::ctype_base_mask = 1024;
        pub const ctype_base_punct: root::std::ctype_base_mask = 8192;
        pub const ctype_base_xdigit: root::std::ctype_base_mask = 65536;
        pub const ctype_base_blank: root::std::ctype_base_mask = 131072;
        pub const ctype_base___regex_word: root::std::ctype_base_mask = 128;
        pub const ctype_base_alnum: root::std::ctype_base_mask = 1280;
        pub const ctype_base_graph: root::std::ctype_base_mask = 9472;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ctype {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ctype_byname {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct codecvt_base {
            pub _bindgen_opaque_blob: u8,
        }
        pub const codecvt_base_result_ok: root::std::codecvt_base_result = 0;
        pub const codecvt_base_result_partial: root::std::codecvt_base_result = 1;
        pub const codecvt_base_result_error: root::std::codecvt_base_result = 2;
        pub const codecvt_base_result_noconv: root::std::codecvt_base_result = 3;
        pub type codecvt_base_result = ::std::os::raw::c_uint;
        const _: () = {
            ["Size of codecvt_base"][::std::mem::size_of::<codecvt_base>() - 1usize];
            ["Alignment of codecvt_base"][::std::mem::align_of::<codecvt_base>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct codecvt {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct codecvt_byname {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct numpunct {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct numpunct_byname {
            pub _address: u8,
        }
        pub type streamsize = u64;
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct ios_base {
            pub _bindgen_opaque_blob: [u64; 17usize],
        }
        pub type ios_base_fmtflags = u32;
        pub type ios_base_iostate = u32;
        pub type ios_base_openmode = u32;
        pub const ios_base_seekdir_beg: root::std::ios_base_seekdir = 0;
        pub const ios_base_seekdir_cur: root::std::ios_base_seekdir = 1;
        pub const ios_base_seekdir_end: root::std::ios_base_seekdir = 2;
        pub type ios_base_seekdir = ::std::os::raw::c_uint;
        pub type ios_base_io_state = u32;
        pub type ios_base_open_mode = u32;
        pub type ios_base_seek_dir = u32;
        pub type ios_base_streamoff = u64;
        pub type ios_base_streampos = u8;
        pub const ios_base_event_erase_event: root::std::ios_base_event = 0;
        pub const ios_base_event_imbue_event: root::std::ios_base_event = 1;
        pub const ios_base_event_copyfmt_event: root::std::ios_base_event = 2;
        pub type ios_base_event = ::std::os::raw::c_uint;
        pub type ios_base_event_callback = u64;
        const _: () = {
            ["Size of ios_base"][::std::mem::size_of::<ios_base>() - 136usize];
            ["Alignment of ios_base"][::std::mem::align_of::<ios_base>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base5imbueERKNS_6localeE"]
            pub fn ios_base_imbue(
                this: *mut root::std::ios_base,
                __loc: *const root::std::locale,
            ) -> root::std::locale;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt3__18ios_base6getlocEv"]
            pub fn ios_base_getloc(this: *const root::std::ios_base) -> root::std::locale;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base6xallocEv"]
            pub fn ios_base_xalloc() -> ::std::os::raw::c_int;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base5iwordEi"]
            pub fn ios_base_iword(
                this: *mut root::std::ios_base,
                __index: ::std::os::raw::c_int,
            ) -> *mut ::std::os::raw::c_long;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base5pwordEi"]
            pub fn ios_base_pword(
                this: *mut root::std::ios_base,
                __index: ::std::os::raw::c_int,
            ) -> *mut *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base17register_callbackEPFvNS0_5eventERS0_iEi"]
            pub fn ios_base_register_callback(
                this: *mut root::std::ios_base,
                __fn: root::std::ios_base_event_callback,
                __index: ::std::os::raw::c_int,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base15sync_with_stdioEb"]
            pub fn ios_base_sync_with_stdio(__sync: bool) -> bool;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base5clearEj"]
            pub fn ios_base_clear(
                this: *mut root::std::ios_base,
                __state: root::std::ios_base_iostate,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base33__set_badbit_and_consider_rethrowEv"]
            pub fn ios_base___set_badbit_and_consider_rethrow(this: *mut root::std::ios_base);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base34__set_failbit_and_consider_rethrowEv"]
            pub fn ios_base___set_failbit_and_consider_rethrow(this: *mut root::std::ios_base);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base4initEPv"]
            pub fn ios_base_init(this: *mut root::std::ios_base, __sb: *mut ::std::os::raw::c_void);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base16__call_callbacksENS0_5eventE"]
            pub fn ios_base___call_callbacks(
                this: *mut root::std::ios_base,
                arg1: root::std::ios_base_event,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base7copyfmtERKS0_"]
            pub fn ios_base_copyfmt(
                this: *mut root::std::ios_base,
                arg1: *const root::std::ios_base,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base4moveERS0_"]
            pub fn ios_base_move(this: *mut root::std::ios_base, arg1: *mut root::std::ios_base);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base4swapERS0_"]
            pub fn ios_base_swap(this: *mut root::std::ios_base, arg1: *mut root::std::ios_base);
        }
        impl ios_base {
            #[inline]
            pub unsafe fn imbue(&mut self, __loc: *const root::std::locale) -> root::std::locale {
                ios_base_imbue(self, __loc)
            }
            #[inline]
            pub unsafe fn getloc(&self) -> root::std::locale {
                ios_base_getloc(self)
            }
            #[inline]
            pub unsafe fn xalloc() -> ::std::os::raw::c_int {
                ios_base_xalloc()
            }
            #[inline]
            pub unsafe fn iword(
                &mut self,
                __index: ::std::os::raw::c_int,
            ) -> *mut ::std::os::raw::c_long {
                ios_base_iword(self, __index)
            }
            #[inline]
            pub unsafe fn pword(
                &mut self,
                __index: ::std::os::raw::c_int,
            ) -> *mut *mut ::std::os::raw::c_void {
                ios_base_pword(self, __index)
            }
            #[inline]
            pub unsafe fn register_callback(
                &mut self,
                __fn: root::std::ios_base_event_callback,
                __index: ::std::os::raw::c_int,
            ) {
                ios_base_register_callback(self, __fn, __index)
            }
            #[inline]
            pub unsafe fn sync_with_stdio(__sync: bool) -> bool {
                ios_base_sync_with_stdio(__sync)
            }
            #[inline]
            pub unsafe fn clear(&mut self, __state: root::std::ios_base_iostate) {
                ios_base_clear(self, __state)
            }
            #[inline]
            pub unsafe fn __set_badbit_and_consider_rethrow(&mut self) {
                ios_base___set_badbit_and_consider_rethrow(self)
            }
            #[inline]
            pub unsafe fn __set_failbit_and_consider_rethrow(&mut self) {
                ios_base___set_failbit_and_consider_rethrow(self)
            }
            #[inline]
            pub unsafe fn init(&mut self, __sb: *mut ::std::os::raw::c_void) {
                ios_base_init(self, __sb)
            }
            #[inline]
            pub unsafe fn __call_callbacks(&mut self, arg1: root::std::ios_base_event) {
                ios_base___call_callbacks(self, arg1)
            }
            #[inline]
            pub unsafe fn copyfmt(&mut self, arg1: *const root::std::ios_base) {
                ios_base_copyfmt(self, arg1)
            }
            #[inline]
            pub unsafe fn move_(&mut self, arg1: *mut root::std::ios_base) {
                ios_base_move(self, arg1)
            }
            #[inline]
            pub unsafe fn swap(&mut self, arg1: *mut root::std::ios_base) {
                ios_base_swap(self, arg1)
            }
        }
        pub const ios_base_boolalpha: root::std::ios_base_fmtflags = 1;
        pub const ios_base_dec: root::std::ios_base_fmtflags = 2;
        pub const ios_base_fixed: root::std::ios_base_fmtflags = 4;
        pub const ios_base_hex: root::std::ios_base_fmtflags = 8;
        pub const ios_base_internal: root::std::ios_base_fmtflags = 16;
        pub const ios_base_left: root::std::ios_base_fmtflags = 32;
        pub const ios_base_oct: root::std::ios_base_fmtflags = 64;
        pub const ios_base_right: root::std::ios_base_fmtflags = 128;
        pub const ios_base_scientific: root::std::ios_base_fmtflags = 256;
        pub const ios_base_showbase: root::std::ios_base_fmtflags = 512;
        pub const ios_base_showpoint: root::std::ios_base_fmtflags = 1024;
        pub const ios_base_showpos: root::std::ios_base_fmtflags = 2048;
        pub const ios_base_skipws: root::std::ios_base_fmtflags = 4096;
        pub const ios_base_unitbuf: root::std::ios_base_fmtflags = 8192;
        pub const ios_base_uppercase: root::std::ios_base_fmtflags = 16384;
        pub const ios_base_adjustfield: root::std::ios_base_fmtflags = 176;
        pub const ios_base_basefield: root::std::ios_base_fmtflags = 74;
        pub const ios_base_floatfield: root::std::ios_base_fmtflags = 260;
        pub const ios_base_badbit: root::std::ios_base_iostate = 1;
        pub const ios_base_eofbit: root::std::ios_base_iostate = 2;
        pub const ios_base_failbit: root::std::ios_base_iostate = 4;
        pub const ios_base_goodbit: root::std::ios_base_iostate = 0;
        pub const ios_base_app: root::std::ios_base_openmode = 1;
        pub const ios_base_ate: root::std::ios_base_openmode = 2;
        pub const ios_base_binary: root::std::ios_base_openmode = 4;
        pub const ios_base_in: root::std::ios_base_openmode = 8;
        pub const ios_base_out: root::std::ios_base_openmode = 16;
        pub const ios_base_trunc: root::std::ios_base_openmode = 32;
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_baseD1Ev"]
            pub fn ios_base_ios_base_destructor(this: *mut root::std::ios_base);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base9__xindex_E"]
            pub static mut ios_base___xindex_: u8;
        }
        pub const io_errc_stream: root::std::io_errc = 1;
        pub type io_errc = ::std::os::raw::c_int;
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__117iostream_categoryEv"]
            pub fn iostream_category() -> *const root::std::error_category;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct ios_base_failure {
            pub _bindgen_opaque_blob: [u64; 4usize],
        }
        const _: () = {
            ["Size of ios_base_failure"][::std::mem::size_of::<ios_base_failure>() - 32usize];
            ["Alignment of ios_base_failure"][::std::mem::align_of::<ios_base_failure>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base7failureC1ERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEERKNS_10error_codeE"]
            pub fn ios_base_failure_failure(
                this: *mut root::std::ios_base_failure,
                __msg: *const root::std::string,
                __ec: *const root::std::error_code,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base7failureC1EPKcRKNS_10error_codeE"]
            pub fn ios_base_failure_failure1(
                this: *mut root::std::ios_base_failure,
                __msg: *const ::std::os::raw::c_char,
                __ec: *const root::std::error_code,
            );
        }
        impl ios_base_failure {
            #[inline]
            pub unsafe fn new(
                __msg: *const root::std::string,
                __ec: *const root::std::error_code,
            ) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                ios_base_failure_failure(__bindgen_tmp.as_mut_ptr(), __msg, __ec);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new1(
                __msg: *const ::std::os::raw::c_char,
                __ec: *const root::std::error_code,
            ) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                ios_base_failure_failure1(__bindgen_tmp.as_mut_ptr(), __msg, __ec);
                __bindgen_tmp.assume_init()
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base7failureD1Ev"]
            pub fn ios_base_failure_failure_destructor(this: *mut root::std::ios_base_failure);
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct ios_base_Init {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of ios_base_Init"][::std::mem::size_of::<ios_base_Init>() - 1usize];
            ["Alignment of ios_base_Init"][::std::mem::align_of::<ios_base_Init>() - 1usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base4InitC1Ev"]
            pub fn ios_base_Init_Init(this: *mut root::std::ios_base_Init);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__18ios_base4InitD1Ev"]
            pub fn ios_base_Init_Init_destructor(this: *mut root::std::ios_base_Init);
        }
        impl ios_base_Init {
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                ios_base_Init_Init(__bindgen_tmp.as_mut_ptr());
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                ios_base_Init_Init_destructor(self)
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_ios {}
        pub type basic_ios_char_type = u8;
        pub type basic_ios_traits_type = u8;
        pub type basic_ios_int_type = u8;
        pub type basic_ios_pos_type = u8;
        pub type basic_ios_off_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct basic_streambuf {}
        pub type basic_streambuf_char_type = u8;
        pub type basic_streambuf_traits_type = u8;
        pub type basic_streambuf_int_type = u8;
        pub type basic_streambuf_pos_type = u8;
        pub type basic_streambuf_off_type = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __num_get_base {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __num_get_base"][::std::mem::size_of::<__num_get_base>() - 1usize];
            ["Alignment of __num_get_base"][::std::mem::align_of::<__num_get_base>() - 1usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__114__num_get_base10__get_baseERNS_8ios_baseE"]
            pub fn __num_get_base___get_base(
                arg1: *mut root::std::ios_base,
            ) -> ::std::os::raw::c_int;
        }
        impl __num_get_base {
            #[inline]
            pub unsafe fn __get_base(arg1: *mut root::std::ios_base) -> ::std::os::raw::c_int {
                __num_get_base___get_base(arg1)
            }
        }
        pub const __num_get_base___num_get_buf_sz: ::std::os::raw::c_int = 40;
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__114__num_get_base5__srcE"]
            pub static __num_get_base___src: [::std::os::raw::c_char; 33usize];
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__116__check_groupingERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEPjS8_Rj"]
            pub fn __check_grouping(
                __grouping: *const root::std::string,
                __g: *mut ::std::os::raw::c_uint,
                __g_end: *mut ::std::os::raw::c_uint,
                __err: *mut root::std::ios_base_iostate,
            );
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __num_get {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct num_get {}
        pub type num_get_char_type = u8;
        pub type num_get_iter_type = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __num_put_base {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __num_put_base"][::std::mem::size_of::<__num_put_base>() - 1usize];
            ["Alignment of __num_put_base"][::std::mem::align_of::<__num_put_base>() - 1usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__114__num_put_base12__format_intEPcPKcbj"]
            pub fn __num_put_base___format_int(
                __fmt: *mut ::std::os::raw::c_char,
                __len: *const ::std::os::raw::c_char,
                __signd: bool,
                __flags: root::std::ios_base_fmtflags,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__114__num_put_base14__format_floatEPcPKcj"]
            pub fn __num_put_base___format_float(
                __fmt: *mut ::std::os::raw::c_char,
                __len: *const ::std::os::raw::c_char,
                __flags: root::std::ios_base_fmtflags,
            ) -> bool;
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__114__num_put_base18__identify_paddingEPcS1_RKNS_8ios_baseE"]
            pub fn __num_put_base___identify_padding(
                __nb: *mut ::std::os::raw::c_char,
                __ne: *mut ::std::os::raw::c_char,
                __iob: *const root::std::ios_base,
            ) -> *mut ::std::os::raw::c_char;
        }
        impl __num_put_base {
            #[inline]
            pub unsafe fn __format_int(
                __fmt: *mut ::std::os::raw::c_char,
                __len: *const ::std::os::raw::c_char,
                __signd: bool,
                __flags: root::std::ios_base_fmtflags,
            ) {
                __num_put_base___format_int(__fmt, __len, __signd, __flags)
            }
            #[inline]
            pub unsafe fn __format_float(
                __fmt: *mut ::std::os::raw::c_char,
                __len: *const ::std::os::raw::c_char,
                __flags: root::std::ios_base_fmtflags,
            ) -> bool {
                __num_put_base___format_float(__fmt, __len, __flags)
            }
            #[inline]
            pub unsafe fn __identify_padding(
                __nb: *mut ::std::os::raw::c_char,
                __ne: *mut ::std::os::raw::c_char,
                __iob: *const root::std::ios_base,
            ) -> *mut ::std::os::raw::c_char {
                __num_put_base___identify_padding(__nb, __ne, __iob)
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __num_put {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct num_put {}
        pub type num_put_char_type = u8;
        pub type num_put_iter_type = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct time_base {
            pub _bindgen_opaque_blob: u8,
        }
        pub const time_base_dateorder_no_order: root::std::time_base_dateorder = 0;
        pub const time_base_dateorder_dmy: root::std::time_base_dateorder = 1;
        pub const time_base_dateorder_mdy: root::std::time_base_dateorder = 2;
        pub const time_base_dateorder_ymd: root::std::time_base_dateorder = 3;
        pub const time_base_dateorder_ydm: root::std::time_base_dateorder = 4;
        pub type time_base_dateorder = ::std::os::raw::c_uint;
        const _: () = {
            ["Size of time_base"][::std::mem::size_of::<time_base>() - 1usize];
            ["Alignment of time_base"][::std::mem::align_of::<time_base>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __time_get_c_storage {}
        pub type __time_get_c_storage_string_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct time_get {}
        pub type time_get_char_type = u8;
        pub type time_get_iter_type = u8;
        pub type time_get_dateorder = u32;
        pub type time_get_string_type = u8;
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __time_get {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of __time_get"][::std::mem::size_of::<__time_get>() - 8usize];
            ["Alignment of __time_get"][::std::mem::align_of::<__time_get>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110__time_getC1EPKc"]
            pub fn __time_get___time_get(
                this: *mut root::std::__time_get,
                __nm: *const ::std::os::raw::c_char,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110__time_getC1ERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE"]
            pub fn __time_get___time_get1(
                this: *mut root::std::__time_get,
                __nm: *const root::std::string,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110__time_getD1Ev"]
            pub fn __time_get___time_get_destructor(this: *mut root::std::__time_get);
        }
        impl __time_get {
            #[inline]
            pub unsafe fn new(__nm: *const ::std::os::raw::c_char) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                __time_get___time_get(__bindgen_tmp.as_mut_ptr(), __nm);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new1(__nm: *const root::std::string) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                __time_get___time_get1(__bindgen_tmp.as_mut_ptr(), __nm);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                __time_get___time_get_destructor(self)
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __time_get_storage {
            pub _address: u8,
        }
        pub type __time_get_storage_string_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct time_get_byname {
            pub _address: u8,
        }
        pub type time_get_byname_dateorder = u32;
        pub type time_get_byname_iter_type = u8;
        pub type time_get_byname_char_type = u8;
        pub type time_get_byname_string_type = u8;
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __time_put {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of __time_put"][::std::mem::size_of::<__time_put>() - 8usize];
            ["Alignment of __time_put"][::std::mem::align_of::<__time_put>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNKSt3__110__time_put8__do_putEPcRS1_PK2tmcc"]
            pub fn __time_put___do_put(
                this: *const root::std::__time_put,
                __nb: *mut ::std::os::raw::c_char,
                __ne: *mut *mut ::std::os::raw::c_char,
                __tm: *mut root::tm,
                __fmt: ::std::os::raw::c_char,
                __mod: ::std::os::raw::c_char,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt3__110__time_put8__do_putEPwRS1_PK2tmcc"]
            pub fn __time_put___do_put1(
                this: *const root::std::__time_put,
                __wb: *mut u32,
                __we: *mut *mut u32,
                __tm: *mut root::tm,
                __fmt: ::std::os::raw::c_char,
                __mod: ::std::os::raw::c_char,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110__time_putC1EPKc"]
            pub fn __time_put___time_put(
                this: *mut root::std::__time_put,
                __nm: *const ::std::os::raw::c_char,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110__time_putC1ERKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE"]
            pub fn __time_put___time_put1(
                this: *mut root::std::__time_put,
                __nm: *const root::std::string,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__110__time_putD1Ev"]
            pub fn __time_put___time_put_destructor(this: *mut root::std::__time_put);
        }
        impl __time_put {
            #[inline]
            pub unsafe fn __do_put(
                &self,
                __nb: *mut ::std::os::raw::c_char,
                __ne: *mut *mut ::std::os::raw::c_char,
                __tm: *mut root::tm,
                __fmt: ::std::os::raw::c_char,
                __mod: ::std::os::raw::c_char,
            ) {
                __time_put___do_put(self, __nb, __ne, __tm, __fmt, __mod)
            }
            #[inline]
            pub unsafe fn __do_put1(
                &self,
                __wb: *mut u32,
                __we: *mut *mut u32,
                __tm: *mut root::tm,
                __fmt: ::std::os::raw::c_char,
                __mod: ::std::os::raw::c_char,
            ) {
                __time_put___do_put1(self, __wb, __we, __tm, __fmt, __mod)
            }
            #[inline]
            pub unsafe fn new(__nm: *const ::std::os::raw::c_char) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                __time_put___time_put(__bindgen_tmp.as_mut_ptr(), __nm);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new1(__nm: *const root::std::string) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                __time_put___time_put1(__bindgen_tmp.as_mut_ptr(), __nm);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                __time_put___time_put_destructor(self)
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct time_put {}
        pub type time_put_char_type = u8;
        pub type time_put_iter_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct time_put_byname {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct money_base {
            pub _bindgen_opaque_blob: u8,
        }
        pub const money_base_part_none: root::std::money_base_part = 0;
        pub const money_base_part_space: root::std::money_base_part = 1;
        pub const money_base_part_symbol: root::std::money_base_part = 2;
        pub const money_base_part_sign: root::std::money_base_part = 3;
        pub const money_base_part_value: root::std::money_base_part = 4;
        pub type money_base_part = ::std::os::raw::c_uint;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct money_base_pattern {
            pub _bindgen_opaque_blob: [u8; 4usize],
        }
        const _: () = {
            ["Size of money_base_pattern"][::std::mem::size_of::<money_base_pattern>() - 4usize];
            ["Alignment of money_base_pattern"]
                [::std::mem::align_of::<money_base_pattern>() - 1usize];
        };
        const _: () = {
            ["Size of money_base"][::std::mem::size_of::<money_base>() - 1usize];
            ["Alignment of money_base"][::std::mem::align_of::<money_base>() - 1usize];
        };
        pub type moneypunct_char_type = u8;
        pub type moneypunct_string_type = u8;
        extern "C" {
            pub static intl: bool;
        }
        pub type moneypunct_byname_pattern = [u8; 4usize];
        pub type moneypunct_byname_char_type = u8;
        pub type moneypunct_byname_string_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __money_get {
            pub _address: u8,
        }
        pub type __money_get_char_type = u8;
        pub type __money_get_string_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct money_get {}
        pub type money_get_char_type = u8;
        pub type money_get_iter_type = u8;
        pub type money_get_string_type = u8;
        extern "C" {
            #[link_name = "\u{1}__ZNSt3__112__do_nothingEPv"]
            pub fn __do_nothing(arg1: *mut ::std::os::raw::c_void);
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __money_put {
            pub _address: u8,
        }
        pub type __money_put_char_type = u8;
        pub type __money_put_string_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct money_put {}
        pub type money_put_char_type = u8;
        pub type money_put_iter_type = u8;
        pub type money_put_string_type = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct messages_base {
            pub _bindgen_opaque_blob: u8,
        }
        pub type messages_base_catalog = u64;
        const _: () = {
            ["Size of messages_base"][::std::mem::size_of::<messages_base>() - 1usize];
            ["Alignment of messages_base"][::std::mem::align_of::<messages_base>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct messages {}
        pub type messages_char_type = u8;
        pub type messages_string_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct messages_byname {
            pub _address: u8,
        }
        pub type messages_byname_catalog = u64;
        pub type messages_byname_string_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct wstring_convert {
            pub _address: u8,
        }
        pub type wstring_convert_byte_string = u8;
        pub type wstring_convert_wide_string = u8;
        pub type wstring_convert_state_type = u8;
        pub type wstring_convert_int_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct wbuffer_convert {}
        pub type wbuffer_convert_char_type = u8;
        pub type wbuffer_convert_traits_type = u8;
        pub type wbuffer_convert_int_type = u8;
        pub type wbuffer_convert_pos_type = u8;
        pub type wbuffer_convert_off_type = u8;
        pub type wbuffer_convert_state_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __temp_value {
            pub _address: u8,
        }
        pub type __temp_value__Traits = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Copy, Clone)]
        pub struct __temp_value__bindgen_ty_1 {
            pub _bindgen_opaque_blob: [u8; 0usize],
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __split_buffer {
            pub _address: u8,
        }
        pub type __split_buffer_value_type = u8;
        pub type __split_buffer_allocator_type = u8;
        pub type __split_buffer___alloc_rr = u8;
        pub type __split_buffer___alloc_traits = u8;
        pub type __split_buffer_reference = u8;
        pub type __split_buffer_const_reference = u8;
        pub type __split_buffer_size_type = u8;
        pub type __split_buffer_difference_type = u8;
        pub type __split_buffer_pointer = u8;
        pub type __split_buffer_const_pointer = u8;
        pub type __split_buffer_iterator = u8;
        pub type __split_buffer_const_iterator = u8;
        pub type __split_buffer___alloc_ref = u8;
        pub type __split_buffer___alloc_const_ref = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __split_buffer__ConstructTransaction {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct vector {
            pub _address: u8,
        }
        pub type vector___default_allocator_type = u8;
        pub type vector___self = u8;
        pub type vector_value_type = u8;
        pub type vector_allocator_type = u8;
        pub type vector___alloc_traits = u8;
        pub type vector_reference = u8;
        pub type vector_const_reference = u8;
        pub type vector_size_type = u8;
        pub type vector_difference_type = u8;
        pub type vector_pointer = u8;
        pub type vector_const_pointer = u8;
        pub type vector_iterator = u8;
        pub type vector_const_iterator = u8;
        pub type vector_reverse_iterator = u8;
        pub type vector_const_reverse_iterator = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct vector___destroy_vector {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct vector__ConstructTransaction {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __builtin_new_allocator {
            pub _bindgen_opaque_blob: u8,
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct __builtin_new_allocator___builtin_new_deleter {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        pub type __builtin_new_allocator___builtin_new_deleter_pointer_type = u64;
        const _: () = {
            ["Size of __builtin_new_allocator___builtin_new_deleter"]
                [::std::mem::size_of::<__builtin_new_allocator___builtin_new_deleter>() - 16usize];
            ["Alignment of __builtin_new_allocator___builtin_new_deleter"]
                [::std::mem::align_of::<__builtin_new_allocator___builtin_new_deleter>() - 8usize];
        };
        pub type __builtin_new_allocator___holder_t = [u64; 3usize];
        const _: () = {
            ["Size of __builtin_new_allocator"]
                [::std::mem::size_of::<__builtin_new_allocator>() - 1usize];
            ["Alignment of __builtin_new_allocator"]
                [::std::mem::align_of::<__builtin_new_allocator>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct bad_function_call {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of bad_function_call"][::std::mem::size_of::<bad_function_call>() - 8usize];
            ["Alignment of bad_function_call"]
                [::std::mem::align_of::<bad_function_call>() - 8usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct function {
            pub _address: u8,
        }
        pub mod __function {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __maybe_derive_from_unary_function {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __maybe_derive_from_binary_function {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __alloc_func {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __default_alloc_func {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __base {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __func {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __value_func {
                pub _address: u8,
            }
            #[repr(C)]
            #[repr(align(8))]
            #[derive(Copy, Clone)]
            pub union __policy_storage {
                pub _bindgen_opaque_blob: [u64; 2usize],
            }
            const _: () = {
                ["Size of __policy_storage"][::std::mem::size_of::<__policy_storage>() - 16usize];
                ["Alignment of __policy_storage"]
                    [::std::mem::align_of::<__policy_storage>() - 8usize];
            };
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __use_small_storage {
                pub _address: u8,
            }
            #[repr(C)]
            #[repr(align(8))]
            #[derive(Debug, Copy, Clone)]
            pub struct __policy {
                pub _bindgen_opaque_blob: [u64; 4usize],
            }
            const _: () = {
                ["Size of __policy"][::std::mem::size_of::<__policy>() - 32usize];
                ["Alignment of __policy"][::std::mem::align_of::<__policy>() - 8usize];
            };
            pub type __fast_forward = u8;
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __policy_invoker {
                pub _address: u8,
            }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct __policy_func {
                pub _address: u8,
            }
            extern "C" {
                pub fn _Block_copy(
                    arg1: *const ::std::os::raw::c_void,
                ) -> *mut ::std::os::raw::c_void;
            }
            extern "C" {
                pub fn _Block_release(arg1: *const ::std::os::raw::c_void);
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __mem_fn {
            pub _address: u8,
        }
        pub type __mem_fn_type = u8;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct mem_fun_t {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct mem_fun1_t {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct mem_fun_ref_t {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct mem_fun1_ref_t {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct const_mem_fun_t {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct const_mem_fun1_t {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct const_mem_fun_ref_t {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct const_mem_fun1_ref_t {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct pointer_to_binary_function {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct pointer_to_unary_function {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct unary_negate {
            pub _address: u8,
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct exception {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of exception"][::std::mem::size_of::<exception>() - 8usize];
            ["Alignment of exception"][::std::mem::align_of::<exception>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt9exceptionD1Ev"]
            pub fn exception_exception_destructor(this: *mut root::std::exception);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt9exception4whatEv"]
            pub fn exception_what(
                this: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct bad_exception {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of bad_exception"][::std::mem::size_of::<bad_exception>() - 8usize];
            ["Alignment of bad_exception"][::std::mem::align_of::<bad_exception>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt13bad_exceptionD1Ev"]
            pub fn bad_exception_bad_exception_destructor(this: *mut root::std::bad_exception);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt13bad_exception4whatEv"]
            pub fn bad_exception_what(
                this: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct nothrow_t {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of nothrow_t"][::std::mem::size_of::<nothrow_t>() - 1usize];
            ["Alignment of nothrow_t"][::std::mem::align_of::<nothrow_t>() - 1usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZSt7nothrow"]
            pub static nothrow: root::std::nothrow_t;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct bad_alloc {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of bad_alloc"][::std::mem::size_of::<bad_alloc>() - 8usize];
            ["Alignment of bad_alloc"][::std::mem::align_of::<bad_alloc>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt9bad_allocC1Ev"]
            pub fn bad_alloc_bad_alloc(this: *mut root::std::bad_alloc);
        }
        impl bad_alloc {
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                bad_alloc_bad_alloc(__bindgen_tmp.as_mut_ptr());
                __bindgen_tmp.assume_init()
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt9bad_allocD1Ev"]
            pub fn bad_alloc_bad_alloc_destructor(this: *mut root::std::bad_alloc);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt9bad_alloc4whatEv"]
            pub fn bad_alloc_what(
                this: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct bad_array_new_length {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of bad_array_new_length"]
                [::std::mem::size_of::<bad_array_new_length>() - 8usize];
            ["Alignment of bad_array_new_length"]
                [::std::mem::align_of::<bad_array_new_length>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt20bad_array_new_lengthC1Ev"]
            pub fn bad_array_new_length_bad_array_new_length(
                this: *mut root::std::bad_array_new_length,
            );
        }
        impl bad_array_new_length {
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                bad_array_new_length_bad_array_new_length(__bindgen_tmp.as_mut_ptr());
                __bindgen_tmp.assume_init()
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt20bad_array_new_lengthD1Ev"]
            pub fn bad_array_new_length_bad_array_new_length_destructor(
                this: *mut root::std::bad_array_new_length,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt20bad_array_new_length4whatEv"]
            pub fn bad_array_new_length_what(
                this: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        pub type new_handler = u64;
        extern "C" {
            #[link_name = "\u{1}__ZSt15set_new_handlerPFvvE"]
            pub fn set_new_handler(arg1: root::std::new_handler) -> root::std::new_handler;
        }
        extern "C" {
            #[link_name = "\u{1}__ZSt15get_new_handlerv"]
            pub fn get_new_handler() -> root::std::new_handler;
        }
        extern "C" {
            #[link_name = "\u{1}__ZSt17__throw_bad_allocv"]
            pub fn __throw_bad_alloc();
        }
        pub type align_val_t = ::std::os::raw::c_ulong;
        pub type unexpected_handler = u64;
        extern "C" {
            #[link_name = "\u{1}__ZSt14set_unexpectedPFvvE"]
            pub fn set_unexpected(
                arg1: root::std::unexpected_handler,
            ) -> root::std::unexpected_handler;
        }
        extern "C" {
            #[link_name = "\u{1}__ZSt14get_unexpectedv"]
            pub fn get_unexpected() -> root::std::unexpected_handler;
        }
        extern "C" {
            #[link_name = "\u{1}__ZSt10unexpectedv"]
            pub fn unexpected();
        }
        pub type terminate_handler = u64;
        extern "C" {
            #[link_name = "\u{1}__ZSt13set_terminatePFvvE"]
            pub fn set_terminate(
                arg1: root::std::terminate_handler,
            ) -> root::std::terminate_handler;
        }
        extern "C" {
            #[link_name = "\u{1}__ZSt13get_terminatev"]
            pub fn get_terminate() -> root::std::terminate_handler;
        }
        extern "C" {
            #[link_name = "\u{1}__ZSt18uncaught_exceptionv"]
            pub fn uncaught_exception() -> bool;
        }
        extern "C" {
            #[link_name = "\u{1}__ZSt19uncaught_exceptionsv"]
            pub fn uncaught_exceptions() -> ::std::os::raw::c_int;
        }
        extern "C" {
            #[link_name = "\u{1}__ZSt17current_exceptionv"]
            pub fn current_exception() -> root::std::exception_ptr;
        }
        extern "C" {
            #[link_name = "\u{1}__ZSt17rethrow_exceptionSt13exception_ptr"]
            pub fn rethrow_exception(arg1: root::std::exception_ptr);
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct exception_ptr {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of exception_ptr"][::std::mem::size_of::<exception_ptr>() - 8usize];
            ["Alignment of exception_ptr"][::std::mem::align_of::<exception_ptr>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt13exception_ptrC1ERKS_"]
            pub fn exception_ptr_exception_ptr(
                this: *mut root::std::exception_ptr,
                arg1: *const root::std::exception_ptr,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt13exception_ptrD1Ev"]
            pub fn exception_ptr_exception_ptr_destructor(this: *mut root::std::exception_ptr);
        }
        impl exception_ptr {
            #[inline]
            pub unsafe fn new(arg1: *const root::std::exception_ptr) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                exception_ptr_exception_ptr(__bindgen_tmp.as_mut_ptr(), arg1);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                exception_ptr_exception_ptr_destructor(self)
            }
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct nested_exception {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of nested_exception"][::std::mem::size_of::<nested_exception>() - 16usize];
            ["Alignment of nested_exception"][::std::mem::align_of::<nested_exception>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNKSt16nested_exception14rethrow_nestedEv"]
            pub fn nested_exception_rethrow_nested(this: *const root::std::nested_exception);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt16nested_exceptionC1Ev"]
            pub fn nested_exception_nested_exception(this: *mut root::std::nested_exception);
        }
        impl nested_exception {
            #[inline]
            pub unsafe fn rethrow_nested(&self) {
                nested_exception_rethrow_nested(self)
            }
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                nested_exception_nested_exception(__bindgen_tmp.as_mut_ptr());
                __bindgen_tmp.assume_init()
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt16nested_exceptionD1Ev"]
            pub fn nested_exception_nested_exception_destructor(
                this: *mut root::std::nested_exception,
            );
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __nested {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __can_dynamic_cast {
            pub _address: u8,
        }
        extern "C" {
            #[link_name = "\u{1}__ZSt9terminatev"]
            pub fn terminate();
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct initializer_list {
            pub _address: u8,
        }
        pub type initializer_list_value_type = u8;
        pub type initializer_list_reference = u8;
        pub type initializer_list_const_reference = u8;
        pub type initializer_list_size_type = u64;
        pub type initializer_list_iterator = u8;
        pub type initializer_list_const_iterator = u8;
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __type_info_implementations {
            pub _bindgen_opaque_blob: u8,
        }
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __type_info_implementations___string_impl_base {
            pub _bindgen_opaque_blob: u8,
        }
        pub type __type_info_implementations___string_impl_base___type_name_t = u64;
        const _: () = {
            ["Size of __type_info_implementations___string_impl_base"]
                [::std::mem::size_of::<__type_info_implementations___string_impl_base>() - 1usize];
            ["Alignment of __type_info_implementations___string_impl_base"]
                [::std::mem::align_of::<__type_info_implementations___string_impl_base>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __type_info_implementations___unique_impl {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __type_info_implementations___unique_impl"]
                [::std::mem::size_of::<__type_info_implementations___unique_impl>() - 1usize];
            ["Alignment of __type_info_implementations___unique_impl"]
                [::std::mem::align_of::<__type_info_implementations___unique_impl>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __type_info_implementations___non_unique_impl {
            pub _bindgen_opaque_blob: u8,
        }
        const _: () = {
            ["Size of __type_info_implementations___non_unique_impl"]
                [::std::mem::size_of::<__type_info_implementations___non_unique_impl>() - 1usize];
            ["Alignment of __type_info_implementations___non_unique_impl"]
                [::std::mem::align_of::<__type_info_implementations___non_unique_impl>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(1))]
        #[derive(Debug, Copy, Clone)]
        pub struct __type_info_implementations___non_unique_arm_rtti_bit_impl {
            pub _bindgen_opaque_blob: u8,
        }
        pub type __type_info_implementations___non_unique_arm_rtti_bit_impl___type_name_t = u64;
        pub type __type_info_implementations___non_unique_arm_rtti_bit_impl___non_unique_rtti_bit =
            u8;
        const _: () = {
            ["Size of __type_info_implementations___non_unique_arm_rtti_bit_impl"]
                [::std::mem::size_of::<__type_info_implementations___non_unique_arm_rtti_bit_impl>(
                ) - 1usize];
            ["Alignment of __type_info_implementations___non_unique_arm_rtti_bit_impl"]
                [::std::mem::align_of::<__type_info_implementations___non_unique_arm_rtti_bit_impl>(
                ) - 1usize];
        };
        pub type __type_info_implementations___impl = u8;
        const _: () = {
            ["Size of __type_info_implementations"]
                [::std::mem::size_of::<__type_info_implementations>() - 1usize];
            ["Alignment of __type_info_implementations"]
                [::std::mem::align_of::<__type_info_implementations>() - 1usize];
        };
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct type_info {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        pub type type_info___impl = u8;
        const _: () = {
            ["Size of type_info"][::std::mem::size_of::<type_info>() - 16usize];
            ["Alignment of type_info"][::std::mem::align_of::<type_info>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt9type_infoD1Ev"]
            pub fn type_info_type_info_destructor(this: *mut root::std::type_info);
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct bad_cast {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of bad_cast"][::std::mem::size_of::<bad_cast>() - 8usize];
            ["Alignment of bad_cast"][::std::mem::align_of::<bad_cast>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt8bad_castC1Ev"]
            pub fn bad_cast_bad_cast(this: *mut root::std::bad_cast);
        }
        impl bad_cast {
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                bad_cast_bad_cast(__bindgen_tmp.as_mut_ptr());
                __bindgen_tmp.assume_init()
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt8bad_castD1Ev"]
            pub fn bad_cast_bad_cast_destructor(this: *mut root::std::bad_cast);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt8bad_cast4whatEv"]
            pub fn bad_cast_what(
                this: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct bad_typeid {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of bad_typeid"][::std::mem::size_of::<bad_typeid>() - 8usize];
            ["Alignment of bad_typeid"][::std::mem::align_of::<bad_typeid>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt10bad_typeidC1Ev"]
            pub fn bad_typeid_bad_typeid(this: *mut root::std::bad_typeid);
        }
        impl bad_typeid {
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                bad_typeid_bad_typeid(__bindgen_tmp.as_mut_ptr());
                __bindgen_tmp.assume_init()
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt10bad_typeidD1Ev"]
            pub fn bad_typeid_bad_typeid_destructor(this: *mut root::std::bad_typeid);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt10bad_typeid4whatEv"]
            pub fn bad_typeid_what(
                this: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct logic_error {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of logic_error"][::std::mem::size_of::<logic_error>() - 16usize];
            ["Alignment of logic_error"][::std::mem::align_of::<logic_error>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt11logic_errorC1ERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE"]
            pub fn logic_error_logic_error(
                this: *mut root::std::logic_error,
                arg1: *const root::std::string,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt11logic_errorC1EPKc"]
            pub fn logic_error_logic_error1(
                this: *mut root::std::logic_error,
                arg1: *const ::std::os::raw::c_char,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt11logic_errorC1ERKS_"]
            pub fn logic_error_logic_error2(
                this: *mut root::std::logic_error,
                arg1: *const root::std::logic_error,
            );
        }
        impl logic_error {
            #[inline]
            pub unsafe fn new(arg1: *const root::std::string) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                logic_error_logic_error(__bindgen_tmp.as_mut_ptr(), arg1);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                logic_error_logic_error1(__bindgen_tmp.as_mut_ptr(), arg1);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new2(arg1: *const root::std::logic_error) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                logic_error_logic_error2(__bindgen_tmp.as_mut_ptr(), arg1);
                __bindgen_tmp.assume_init()
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt11logic_errorD1Ev"]
            pub fn logic_error_logic_error_destructor(this: *mut root::std::logic_error);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt11logic_error4whatEv"]
            pub fn logic_error_what(
                this: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct runtime_error {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of runtime_error"][::std::mem::size_of::<runtime_error>() - 16usize];
            ["Alignment of runtime_error"][::std::mem::align_of::<runtime_error>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt13runtime_errorC1ERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE"]
            pub fn runtime_error_runtime_error(
                this: *mut root::std::runtime_error,
                arg1: *const root::std::string,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt13runtime_errorC1EPKc"]
            pub fn runtime_error_runtime_error1(
                this: *mut root::std::runtime_error,
                arg1: *const ::std::os::raw::c_char,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt13runtime_errorC1ERKS_"]
            pub fn runtime_error_runtime_error2(
                this: *mut root::std::runtime_error,
                arg1: *const root::std::runtime_error,
            );
        }
        impl runtime_error {
            #[inline]
            pub unsafe fn new(arg1: *const root::std::string) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                runtime_error_runtime_error(__bindgen_tmp.as_mut_ptr(), arg1);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                runtime_error_runtime_error1(__bindgen_tmp.as_mut_ptr(), arg1);
                __bindgen_tmp.assume_init()
            }
            #[inline]
            pub unsafe fn new2(arg1: *const root::std::runtime_error) -> Self {
                let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
                runtime_error_runtime_error2(__bindgen_tmp.as_mut_ptr(), arg1);
                __bindgen_tmp.assume_init()
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZNSt13runtime_errorD1Ev"]
            pub fn runtime_error_runtime_error_destructor(this: *mut root::std::runtime_error);
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt13runtime_error4whatEv"]
            pub fn runtime_error_what(
                this: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct domain_error {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of domain_error"][::std::mem::size_of::<domain_error>() - 16usize];
            ["Alignment of domain_error"][::std::mem::align_of::<domain_error>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt12domain_errorD1Ev"]
            pub fn domain_error_domain_error_destructor(this: *mut root::std::domain_error);
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct invalid_argument {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of invalid_argument"][::std::mem::size_of::<invalid_argument>() - 16usize];
            ["Alignment of invalid_argument"][::std::mem::align_of::<invalid_argument>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt16invalid_argumentD1Ev"]
            pub fn invalid_argument_invalid_argument_destructor(
                this: *mut root::std::invalid_argument,
            );
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct length_error {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of length_error"][::std::mem::size_of::<length_error>() - 16usize];
            ["Alignment of length_error"][::std::mem::align_of::<length_error>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt12length_errorD1Ev"]
            pub fn length_error_length_error_destructor(this: *mut root::std::length_error);
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct out_of_range {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of out_of_range"][::std::mem::size_of::<out_of_range>() - 16usize];
            ["Alignment of out_of_range"][::std::mem::align_of::<out_of_range>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt12out_of_rangeD1Ev"]
            pub fn out_of_range_out_of_range_destructor(this: *mut root::std::out_of_range);
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct range_error {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of range_error"][::std::mem::size_of::<range_error>() - 16usize];
            ["Alignment of range_error"][::std::mem::align_of::<range_error>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt11range_errorD1Ev"]
            pub fn range_error_range_error_destructor(this: *mut root::std::range_error);
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct overflow_error {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of overflow_error"][::std::mem::size_of::<overflow_error>() - 16usize];
            ["Alignment of overflow_error"][::std::mem::align_of::<overflow_error>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt14overflow_errorD1Ev"]
            pub fn overflow_error_overflow_error_destructor(this: *mut root::std::overflow_error);
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct underflow_error {
            pub _bindgen_opaque_blob: [u64; 2usize],
        }
        const _: () = {
            ["Size of underflow_error"][::std::mem::size_of::<underflow_error>() - 16usize];
            ["Alignment of underflow_error"][::std::mem::align_of::<underflow_error>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt15underflow_errorD1Ev"]
            pub fn underflow_error_underflow_error_destructor(
                this: *mut root::std::underflow_error,
            );
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct bad_variant_access {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of bad_variant_access"][::std::mem::size_of::<bad_variant_access>() - 8usize];
            ["Alignment of bad_variant_access"]
                [::std::mem::align_of::<bad_variant_access>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNKSt18bad_variant_access4whatEv"]
            pub fn bad_variant_access_what(
                this: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        #[repr(C)]
        #[repr(align(8))]
        #[derive(Debug, Copy, Clone)]
        pub struct bad_optional_access {
            pub _bindgen_opaque_blob: u64,
        }
        const _: () = {
            ["Size of bad_optional_access"][::std::mem::size_of::<bad_optional_access>() - 8usize];
            ["Alignment of bad_optional_access"]
                [::std::mem::align_of::<bad_optional_access>() - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNSt19bad_optional_accessD1Ev"]
            pub fn bad_optional_access_bad_optional_access_destructor(
                this: *mut root::std::bad_optional_access,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZNKSt19bad_optional_access4whatEv"]
            pub fn bad_optional_access_what(
                this: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
    }
    pub type max_align_t = f64;
    pub type __int8_t = ::std::os::raw::c_schar;
    pub type __uint8_t = ::std::os::raw::c_uchar;
    pub type __int16_t = ::std::os::raw::c_short;
    pub type __uint16_t = ::std::os::raw::c_ushort;
    pub type __int32_t = ::std::os::raw::c_int;
    pub type __uint32_t = ::std::os::raw::c_uint;
    pub type __int64_t = ::std::os::raw::c_longlong;
    pub type __uint64_t = ::std::os::raw::c_ulonglong;
    pub type __darwin_intptr_t = ::std::os::raw::c_long;
    pub type __darwin_natural_t = ::std::os::raw::c_uint;
    pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union __mbstate_t {
        pub __mbstate8: [::std::os::raw::c_char; 128usize],
        pub _mbstateL: ::std::os::raw::c_longlong,
    }
    const _: () = {
        ["Size of __mbstate_t"][::std::mem::size_of::<__mbstate_t>() - 128usize];
        ["Alignment of __mbstate_t"][::std::mem::align_of::<__mbstate_t>() - 8usize];
        ["Offset of field: __mbstate_t::__mbstate8"]
            [::std::mem::offset_of!(__mbstate_t, __mbstate8) - 0usize];
        ["Offset of field: __mbstate_t::_mbstateL"]
            [::std::mem::offset_of!(__mbstate_t, _mbstateL) - 0usize];
    };
    pub type __darwin_mbstate_t = root::__mbstate_t;
    pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
    pub type __darwin_size_t = ::std::os::raw::c_ulong;
    pub type __darwin_va_list = root::__builtin_va_list;
    pub type __darwin_wchar_t = ::std::os::raw::c_int;
    pub type __darwin_rune_t = root::__darwin_wchar_t;
    pub type __darwin_wint_t = ::std::os::raw::c_int;
    pub type __darwin_clock_t = ::std::os::raw::c_ulong;
    pub type __darwin_socklen_t = root::__uint32_t;
    pub type __darwin_ssize_t = ::std::os::raw::c_long;
    pub type __darwin_time_t = ::std::os::raw::c_long;
    pub type __darwin_blkcnt_t = root::__int64_t;
    pub type __darwin_blksize_t = root::__int32_t;
    pub type __darwin_dev_t = root::__int32_t;
    pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
    pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
    pub type __darwin_gid_t = root::__uint32_t;
    pub type __darwin_id_t = root::__uint32_t;
    pub type __darwin_ino64_t = root::__uint64_t;
    pub type __darwin_ino_t = root::__darwin_ino64_t;
    pub type __darwin_mach_port_name_t = root::__darwin_natural_t;
    pub type __darwin_mach_port_t = root::__darwin_mach_port_name_t;
    pub type __darwin_mode_t = root::__uint16_t;
    pub type __darwin_off_t = root::__int64_t;
    pub type __darwin_pid_t = root::__int32_t;
    pub type __darwin_sigset_t = root::__uint32_t;
    pub type __darwin_suseconds_t = root::__int32_t;
    pub type __darwin_uid_t = root::__uint32_t;
    pub type __darwin_useconds_t = root::__uint32_t;
    pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
    pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_pthread_handler_rec {
        pub __routine:
            ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        pub __arg: *mut ::std::os::raw::c_void,
        pub __next: *mut root::__darwin_pthread_handler_rec,
    }
    const _: () = {
        ["Size of __darwin_pthread_handler_rec"]
            [::std::mem::size_of::<__darwin_pthread_handler_rec>() - 24usize];
        ["Alignment of __darwin_pthread_handler_rec"]
            [::std::mem::align_of::<__darwin_pthread_handler_rec>() - 8usize];
        ["Offset of field: __darwin_pthread_handler_rec::__routine"]
            [::std::mem::offset_of!(__darwin_pthread_handler_rec, __routine) - 0usize];
        ["Offset of field: __darwin_pthread_handler_rec::__arg"]
            [::std::mem::offset_of!(__darwin_pthread_handler_rec, __arg) - 8usize];
        ["Offset of field: __darwin_pthread_handler_rec::__next"]
            [::std::mem::offset_of!(__darwin_pthread_handler_rec, __next) - 16usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _opaque_pthread_attr_t {
        pub __sig: ::std::os::raw::c_long,
        pub __opaque: [::std::os::raw::c_char; 56usize],
    }
    const _: () = {
        ["Size of _opaque_pthread_attr_t"]
            [::std::mem::size_of::<_opaque_pthread_attr_t>() - 64usize];
        ["Alignment of _opaque_pthread_attr_t"]
            [::std::mem::align_of::<_opaque_pthread_attr_t>() - 8usize];
        ["Offset of field: _opaque_pthread_attr_t::__sig"]
            [::std::mem::offset_of!(_opaque_pthread_attr_t, __sig) - 0usize];
        ["Offset of field: _opaque_pthread_attr_t::__opaque"]
            [::std::mem::offset_of!(_opaque_pthread_attr_t, __opaque) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _opaque_pthread_cond_t {
        pub __sig: ::std::os::raw::c_long,
        pub __opaque: [::std::os::raw::c_char; 40usize],
    }
    const _: () = {
        ["Size of _opaque_pthread_cond_t"]
            [::std::mem::size_of::<_opaque_pthread_cond_t>() - 48usize];
        ["Alignment of _opaque_pthread_cond_t"]
            [::std::mem::align_of::<_opaque_pthread_cond_t>() - 8usize];
        ["Offset of field: _opaque_pthread_cond_t::__sig"]
            [::std::mem::offset_of!(_opaque_pthread_cond_t, __sig) - 0usize];
        ["Offset of field: _opaque_pthread_cond_t::__opaque"]
            [::std::mem::offset_of!(_opaque_pthread_cond_t, __opaque) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _opaque_pthread_condattr_t {
        pub __sig: ::std::os::raw::c_long,
        pub __opaque: [::std::os::raw::c_char; 8usize],
    }
    const _: () = {
        ["Size of _opaque_pthread_condattr_t"]
            [::std::mem::size_of::<_opaque_pthread_condattr_t>() - 16usize];
        ["Alignment of _opaque_pthread_condattr_t"]
            [::std::mem::align_of::<_opaque_pthread_condattr_t>() - 8usize];
        ["Offset of field: _opaque_pthread_condattr_t::__sig"]
            [::std::mem::offset_of!(_opaque_pthread_condattr_t, __sig) - 0usize];
        ["Offset of field: _opaque_pthread_condattr_t::__opaque"]
            [::std::mem::offset_of!(_opaque_pthread_condattr_t, __opaque) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _opaque_pthread_mutex_t {
        pub __sig: ::std::os::raw::c_long,
        pub __opaque: [::std::os::raw::c_char; 56usize],
    }
    const _: () = {
        ["Size of _opaque_pthread_mutex_t"]
            [::std::mem::size_of::<_opaque_pthread_mutex_t>() - 64usize];
        ["Alignment of _opaque_pthread_mutex_t"]
            [::std::mem::align_of::<_opaque_pthread_mutex_t>() - 8usize];
        ["Offset of field: _opaque_pthread_mutex_t::__sig"]
            [::std::mem::offset_of!(_opaque_pthread_mutex_t, __sig) - 0usize];
        ["Offset of field: _opaque_pthread_mutex_t::__opaque"]
            [::std::mem::offset_of!(_opaque_pthread_mutex_t, __opaque) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _opaque_pthread_mutexattr_t {
        pub __sig: ::std::os::raw::c_long,
        pub __opaque: [::std::os::raw::c_char; 8usize],
    }
    const _: () = {
        ["Size of _opaque_pthread_mutexattr_t"]
            [::std::mem::size_of::<_opaque_pthread_mutexattr_t>() - 16usize];
        ["Alignment of _opaque_pthread_mutexattr_t"]
            [::std::mem::align_of::<_opaque_pthread_mutexattr_t>() - 8usize];
        ["Offset of field: _opaque_pthread_mutexattr_t::__sig"]
            [::std::mem::offset_of!(_opaque_pthread_mutexattr_t, __sig) - 0usize];
        ["Offset of field: _opaque_pthread_mutexattr_t::__opaque"]
            [::std::mem::offset_of!(_opaque_pthread_mutexattr_t, __opaque) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _opaque_pthread_once_t {
        pub __sig: ::std::os::raw::c_long,
        pub __opaque: [::std::os::raw::c_char; 8usize],
    }
    const _: () = {
        ["Size of _opaque_pthread_once_t"]
            [::std::mem::size_of::<_opaque_pthread_once_t>() - 16usize];
        ["Alignment of _opaque_pthread_once_t"]
            [::std::mem::align_of::<_opaque_pthread_once_t>() - 8usize];
        ["Offset of field: _opaque_pthread_once_t::__sig"]
            [::std::mem::offset_of!(_opaque_pthread_once_t, __sig) - 0usize];
        ["Offset of field: _opaque_pthread_once_t::__opaque"]
            [::std::mem::offset_of!(_opaque_pthread_once_t, __opaque) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _opaque_pthread_rwlock_t {
        pub __sig: ::std::os::raw::c_long,
        pub __opaque: [::std::os::raw::c_char; 192usize],
    }
    const _: () = {
        ["Size of _opaque_pthread_rwlock_t"]
            [::std::mem::size_of::<_opaque_pthread_rwlock_t>() - 200usize];
        ["Alignment of _opaque_pthread_rwlock_t"]
            [::std::mem::align_of::<_opaque_pthread_rwlock_t>() - 8usize];
        ["Offset of field: _opaque_pthread_rwlock_t::__sig"]
            [::std::mem::offset_of!(_opaque_pthread_rwlock_t, __sig) - 0usize];
        ["Offset of field: _opaque_pthread_rwlock_t::__opaque"]
            [::std::mem::offset_of!(_opaque_pthread_rwlock_t, __opaque) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _opaque_pthread_rwlockattr_t {
        pub __sig: ::std::os::raw::c_long,
        pub __opaque: [::std::os::raw::c_char; 16usize],
    }
    const _: () = {
        ["Size of _opaque_pthread_rwlockattr_t"]
            [::std::mem::size_of::<_opaque_pthread_rwlockattr_t>() - 24usize];
        ["Alignment of _opaque_pthread_rwlockattr_t"]
            [::std::mem::align_of::<_opaque_pthread_rwlockattr_t>() - 8usize];
        ["Offset of field: _opaque_pthread_rwlockattr_t::__sig"]
            [::std::mem::offset_of!(_opaque_pthread_rwlockattr_t, __sig) - 0usize];
        ["Offset of field: _opaque_pthread_rwlockattr_t::__opaque"]
            [::std::mem::offset_of!(_opaque_pthread_rwlockattr_t, __opaque) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _opaque_pthread_t {
        pub __sig: ::std::os::raw::c_long,
        pub __cleanup_stack: *mut root::__darwin_pthread_handler_rec,
        pub __opaque: [::std::os::raw::c_char; 8176usize],
    }
    const _: () = {
        ["Size of _opaque_pthread_t"][::std::mem::size_of::<_opaque_pthread_t>() - 8192usize];
        ["Alignment of _opaque_pthread_t"][::std::mem::align_of::<_opaque_pthread_t>() - 8usize];
        ["Offset of field: _opaque_pthread_t::__sig"]
            [::std::mem::offset_of!(_opaque_pthread_t, __sig) - 0usize];
        ["Offset of field: _opaque_pthread_t::__cleanup_stack"]
            [::std::mem::offset_of!(_opaque_pthread_t, __cleanup_stack) - 8usize];
        ["Offset of field: _opaque_pthread_t::__opaque"]
            [::std::mem::offset_of!(_opaque_pthread_t, __opaque) - 16usize];
    };
    pub type __darwin_pthread_attr_t = root::_opaque_pthread_attr_t;
    pub type __darwin_pthread_cond_t = root::_opaque_pthread_cond_t;
    pub type __darwin_pthread_condattr_t = root::_opaque_pthread_condattr_t;
    pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
    pub type __darwin_pthread_mutex_t = root::_opaque_pthread_mutex_t;
    pub type __darwin_pthread_mutexattr_t = root::_opaque_pthread_mutexattr_t;
    pub type __darwin_pthread_once_t = root::_opaque_pthread_once_t;
    pub type __darwin_pthread_rwlock_t = root::_opaque_pthread_rwlock_t;
    pub type __darwin_pthread_rwlockattr_t = root::_opaque_pthread_rwlockattr_t;
    pub type __darwin_pthread_t = *mut root::_opaque_pthread_t;
    pub type __darwin_nl_item = ::std::os::raw::c_int;
    pub type __darwin_wctrans_t = ::std::os::raw::c_int;
    pub type __darwin_wctype_t = root::__uint32_t;
    pub const idtype_t_P_ALL: root::idtype_t = 0;
    pub const idtype_t_P_PID: root::idtype_t = 1;
    pub const idtype_t_P_PGID: root::idtype_t = 2;
    pub type idtype_t = ::std::os::raw::c_uint;
    pub type pid_t = root::__darwin_pid_t;
    pub type id_t = root::__darwin_id_t;
    pub type sig_atomic_t = ::std::os::raw::c_int;
    pub type u_int8_t = ::std::os::raw::c_uchar;
    pub type u_int16_t = ::std::os::raw::c_ushort;
    pub type u_int32_t = ::std::os::raw::c_uint;
    pub type u_int64_t = ::std::os::raw::c_ulonglong;
    pub type register_t = i64;
    pub type user_addr_t = root::u_int64_t;
    pub type user_size_t = root::u_int64_t;
    pub type user_ssize_t = i64;
    pub type user_long_t = i64;
    pub type user_ulong_t = root::u_int64_t;
    pub type user_time_t = i64;
    pub type user_off_t = i64;
    pub type syscall_arg_t = root::u_int64_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_arm_exception_state {
        pub __exception: root::__uint32_t,
        pub __fsr: root::__uint32_t,
        pub __far: root::__uint32_t,
    }
    const _: () = {
        ["Size of __darwin_arm_exception_state"]
            [::std::mem::size_of::<__darwin_arm_exception_state>() - 12usize];
        ["Alignment of __darwin_arm_exception_state"]
            [::std::mem::align_of::<__darwin_arm_exception_state>() - 4usize];
        ["Offset of field: __darwin_arm_exception_state::__exception"]
            [::std::mem::offset_of!(__darwin_arm_exception_state, __exception) - 0usize];
        ["Offset of field: __darwin_arm_exception_state::__fsr"]
            [::std::mem::offset_of!(__darwin_arm_exception_state, __fsr) - 4usize];
        ["Offset of field: __darwin_arm_exception_state::__far"]
            [::std::mem::offset_of!(__darwin_arm_exception_state, __far) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_arm_exception_state64 {
        pub __far: root::__uint64_t,
        pub __esr: root::__uint32_t,
        pub __exception: root::__uint32_t,
    }
    const _: () = {
        ["Size of __darwin_arm_exception_state64"]
            [::std::mem::size_of::<__darwin_arm_exception_state64>() - 16usize];
        ["Alignment of __darwin_arm_exception_state64"]
            [::std::mem::align_of::<__darwin_arm_exception_state64>() - 8usize];
        ["Offset of field: __darwin_arm_exception_state64::__far"]
            [::std::mem::offset_of!(__darwin_arm_exception_state64, __far) - 0usize];
        ["Offset of field: __darwin_arm_exception_state64::__esr"]
            [::std::mem::offset_of!(__darwin_arm_exception_state64, __esr) - 8usize];
        ["Offset of field: __darwin_arm_exception_state64::__exception"]
            [::std::mem::offset_of!(__darwin_arm_exception_state64, __exception) - 12usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_arm_thread_state {
        pub __r: [root::__uint32_t; 13usize],
        pub __sp: root::__uint32_t,
        pub __lr: root::__uint32_t,
        pub __pc: root::__uint32_t,
        pub __cpsr: root::__uint32_t,
    }
    const _: () = {
        ["Size of __darwin_arm_thread_state"]
            [::std::mem::size_of::<__darwin_arm_thread_state>() - 68usize];
        ["Alignment of __darwin_arm_thread_state"]
            [::std::mem::align_of::<__darwin_arm_thread_state>() - 4usize];
        ["Offset of field: __darwin_arm_thread_state::__r"]
            [::std::mem::offset_of!(__darwin_arm_thread_state, __r) - 0usize];
        ["Offset of field: __darwin_arm_thread_state::__sp"]
            [::std::mem::offset_of!(__darwin_arm_thread_state, __sp) - 52usize];
        ["Offset of field: __darwin_arm_thread_state::__lr"]
            [::std::mem::offset_of!(__darwin_arm_thread_state, __lr) - 56usize];
        ["Offset of field: __darwin_arm_thread_state::__pc"]
            [::std::mem::offset_of!(__darwin_arm_thread_state, __pc) - 60usize];
        ["Offset of field: __darwin_arm_thread_state::__cpsr"]
            [::std::mem::offset_of!(__darwin_arm_thread_state, __cpsr) - 64usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_arm_thread_state64 {
        pub __x: [root::__uint64_t; 29usize],
        pub __fp: root::__uint64_t,
        pub __lr: root::__uint64_t,
        pub __sp: root::__uint64_t,
        pub __pc: root::__uint64_t,
        pub __cpsr: root::__uint32_t,
        pub __pad: root::__uint32_t,
    }
    const _: () = {
        ["Size of __darwin_arm_thread_state64"]
            [::std::mem::size_of::<__darwin_arm_thread_state64>() - 272usize];
        ["Alignment of __darwin_arm_thread_state64"]
            [::std::mem::align_of::<__darwin_arm_thread_state64>() - 8usize];
        ["Offset of field: __darwin_arm_thread_state64::__x"]
            [::std::mem::offset_of!(__darwin_arm_thread_state64, __x) - 0usize];
        ["Offset of field: __darwin_arm_thread_state64::__fp"]
            [::std::mem::offset_of!(__darwin_arm_thread_state64, __fp) - 232usize];
        ["Offset of field: __darwin_arm_thread_state64::__lr"]
            [::std::mem::offset_of!(__darwin_arm_thread_state64, __lr) - 240usize];
        ["Offset of field: __darwin_arm_thread_state64::__sp"]
            [::std::mem::offset_of!(__darwin_arm_thread_state64, __sp) - 248usize];
        ["Offset of field: __darwin_arm_thread_state64::__pc"]
            [::std::mem::offset_of!(__darwin_arm_thread_state64, __pc) - 256usize];
        ["Offset of field: __darwin_arm_thread_state64::__cpsr"]
            [::std::mem::offset_of!(__darwin_arm_thread_state64, __cpsr) - 264usize];
        ["Offset of field: __darwin_arm_thread_state64::__pad"]
            [::std::mem::offset_of!(__darwin_arm_thread_state64, __pad) - 268usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_arm_vfp_state {
        pub __r: [root::__uint32_t; 64usize],
        pub __fpscr: root::__uint32_t,
    }
    const _: () = {
        ["Size of __darwin_arm_vfp_state"]
            [::std::mem::size_of::<__darwin_arm_vfp_state>() - 260usize];
        ["Alignment of __darwin_arm_vfp_state"]
            [::std::mem::align_of::<__darwin_arm_vfp_state>() - 4usize];
        ["Offset of field: __darwin_arm_vfp_state::__r"]
            [::std::mem::offset_of!(__darwin_arm_vfp_state, __r) - 0usize];
        ["Offset of field: __darwin_arm_vfp_state::__fpscr"]
            [::std::mem::offset_of!(__darwin_arm_vfp_state, __fpscr) - 256usize];
    };
    #[repr(C)]
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_arm_neon_state64 {
        pub __v: [root::__uint128_t; 32usize],
        pub __fpsr: root::__uint32_t,
        pub __fpcr: root::__uint32_t,
    }
    const _: () = {
        ["Size of __darwin_arm_neon_state64"]
            [::std::mem::size_of::<__darwin_arm_neon_state64>() - 528usize];
        ["Alignment of __darwin_arm_neon_state64"]
            [::std::mem::align_of::<__darwin_arm_neon_state64>() - 16usize];
        ["Offset of field: __darwin_arm_neon_state64::__v"]
            [::std::mem::offset_of!(__darwin_arm_neon_state64, __v) - 0usize];
        ["Offset of field: __darwin_arm_neon_state64::__fpsr"]
            [::std::mem::offset_of!(__darwin_arm_neon_state64, __fpsr) - 512usize];
        ["Offset of field: __darwin_arm_neon_state64::__fpcr"]
            [::std::mem::offset_of!(__darwin_arm_neon_state64, __fpcr) - 516usize];
    };
    #[repr(C)]
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_arm_neon_state {
        pub __v: [root::__uint128_t; 16usize],
        pub __fpsr: root::__uint32_t,
        pub __fpcr: root::__uint32_t,
    }
    const _: () = {
        ["Size of __darwin_arm_neon_state"]
            [::std::mem::size_of::<__darwin_arm_neon_state>() - 272usize];
        ["Alignment of __darwin_arm_neon_state"]
            [::std::mem::align_of::<__darwin_arm_neon_state>() - 16usize];
        ["Offset of field: __darwin_arm_neon_state::__v"]
            [::std::mem::offset_of!(__darwin_arm_neon_state, __v) - 0usize];
        ["Offset of field: __darwin_arm_neon_state::__fpsr"]
            [::std::mem::offset_of!(__darwin_arm_neon_state, __fpsr) - 256usize];
        ["Offset of field: __darwin_arm_neon_state::__fpcr"]
            [::std::mem::offset_of!(__darwin_arm_neon_state, __fpcr) - 260usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __arm_pagein_state {
        pub __pagein_error: ::std::os::raw::c_int,
    }
    const _: () = {
        ["Size of __arm_pagein_state"][::std::mem::size_of::<__arm_pagein_state>() - 4usize];
        ["Alignment of __arm_pagein_state"][::std::mem::align_of::<__arm_pagein_state>() - 4usize];
        ["Offset of field: __arm_pagein_state::__pagein_error"]
            [::std::mem::offset_of!(__arm_pagein_state, __pagein_error) - 0usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __arm_legacy_debug_state {
        pub __bvr: [root::__uint32_t; 16usize],
        pub __bcr: [root::__uint32_t; 16usize],
        pub __wvr: [root::__uint32_t; 16usize],
        pub __wcr: [root::__uint32_t; 16usize],
    }
    const _: () = {
        ["Size of __arm_legacy_debug_state"]
            [::std::mem::size_of::<__arm_legacy_debug_state>() - 256usize];
        ["Alignment of __arm_legacy_debug_state"]
            [::std::mem::align_of::<__arm_legacy_debug_state>() - 4usize];
        ["Offset of field: __arm_legacy_debug_state::__bvr"]
            [::std::mem::offset_of!(__arm_legacy_debug_state, __bvr) - 0usize];
        ["Offset of field: __arm_legacy_debug_state::__bcr"]
            [::std::mem::offset_of!(__arm_legacy_debug_state, __bcr) - 64usize];
        ["Offset of field: __arm_legacy_debug_state::__wvr"]
            [::std::mem::offset_of!(__arm_legacy_debug_state, __wvr) - 128usize];
        ["Offset of field: __arm_legacy_debug_state::__wcr"]
            [::std::mem::offset_of!(__arm_legacy_debug_state, __wcr) - 192usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_arm_debug_state32 {
        pub __bvr: [root::__uint32_t; 16usize],
        pub __bcr: [root::__uint32_t; 16usize],
        pub __wvr: [root::__uint32_t; 16usize],
        pub __wcr: [root::__uint32_t; 16usize],
        pub __mdscr_el1: root::__uint64_t,
    }
    const _: () = {
        ["Size of __darwin_arm_debug_state32"]
            [::std::mem::size_of::<__darwin_arm_debug_state32>() - 264usize];
        ["Alignment of __darwin_arm_debug_state32"]
            [::std::mem::align_of::<__darwin_arm_debug_state32>() - 8usize];
        ["Offset of field: __darwin_arm_debug_state32::__bvr"]
            [::std::mem::offset_of!(__darwin_arm_debug_state32, __bvr) - 0usize];
        ["Offset of field: __darwin_arm_debug_state32::__bcr"]
            [::std::mem::offset_of!(__darwin_arm_debug_state32, __bcr) - 64usize];
        ["Offset of field: __darwin_arm_debug_state32::__wvr"]
            [::std::mem::offset_of!(__darwin_arm_debug_state32, __wvr) - 128usize];
        ["Offset of field: __darwin_arm_debug_state32::__wcr"]
            [::std::mem::offset_of!(__darwin_arm_debug_state32, __wcr) - 192usize];
        ["Offset of field: __darwin_arm_debug_state32::__mdscr_el1"]
            [::std::mem::offset_of!(__darwin_arm_debug_state32, __mdscr_el1) - 256usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_arm_debug_state64 {
        pub __bvr: [root::__uint64_t; 16usize],
        pub __bcr: [root::__uint64_t; 16usize],
        pub __wvr: [root::__uint64_t; 16usize],
        pub __wcr: [root::__uint64_t; 16usize],
        pub __mdscr_el1: root::__uint64_t,
    }
    const _: () = {
        ["Size of __darwin_arm_debug_state64"]
            [::std::mem::size_of::<__darwin_arm_debug_state64>() - 520usize];
        ["Alignment of __darwin_arm_debug_state64"]
            [::std::mem::align_of::<__darwin_arm_debug_state64>() - 8usize];
        ["Offset of field: __darwin_arm_debug_state64::__bvr"]
            [::std::mem::offset_of!(__darwin_arm_debug_state64, __bvr) - 0usize];
        ["Offset of field: __darwin_arm_debug_state64::__bcr"]
            [::std::mem::offset_of!(__darwin_arm_debug_state64, __bcr) - 128usize];
        ["Offset of field: __darwin_arm_debug_state64::__wvr"]
            [::std::mem::offset_of!(__darwin_arm_debug_state64, __wvr) - 256usize];
        ["Offset of field: __darwin_arm_debug_state64::__wcr"]
            [::std::mem::offset_of!(__darwin_arm_debug_state64, __wcr) - 384usize];
        ["Offset of field: __darwin_arm_debug_state64::__mdscr_el1"]
            [::std::mem::offset_of!(__darwin_arm_debug_state64, __mdscr_el1) - 512usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_arm_cpmu_state64 {
        pub __ctrs: [root::__uint64_t; 16usize],
    }
    const _: () = {
        ["Size of __darwin_arm_cpmu_state64"]
            [::std::mem::size_of::<__darwin_arm_cpmu_state64>() - 128usize];
        ["Alignment of __darwin_arm_cpmu_state64"]
            [::std::mem::align_of::<__darwin_arm_cpmu_state64>() - 8usize];
        ["Offset of field: __darwin_arm_cpmu_state64::__ctrs"]
            [::std::mem::offset_of!(__darwin_arm_cpmu_state64, __ctrs) - 0usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_mcontext32 {
        pub __es: root::__darwin_arm_exception_state,
        pub __ss: root::__darwin_arm_thread_state,
        pub __fs: root::__darwin_arm_vfp_state,
    }
    const _: () = {
        ["Size of __darwin_mcontext32"][::std::mem::size_of::<__darwin_mcontext32>() - 340usize];
        ["Alignment of __darwin_mcontext32"]
            [::std::mem::align_of::<__darwin_mcontext32>() - 4usize];
        ["Offset of field: __darwin_mcontext32::__es"]
            [::std::mem::offset_of!(__darwin_mcontext32, __es) - 0usize];
        ["Offset of field: __darwin_mcontext32::__ss"]
            [::std::mem::offset_of!(__darwin_mcontext32, __ss) - 12usize];
        ["Offset of field: __darwin_mcontext32::__fs"]
            [::std::mem::offset_of!(__darwin_mcontext32, __fs) - 80usize];
    };
    #[repr(C)]
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_mcontext64 {
        pub __es: root::__darwin_arm_exception_state64,
        pub __ss: root::__darwin_arm_thread_state64,
        pub __ns: root::__darwin_arm_neon_state64,
    }
    const _: () = {
        ["Size of __darwin_mcontext64"][::std::mem::size_of::<__darwin_mcontext64>() - 816usize];
        ["Alignment of __darwin_mcontext64"]
            [::std::mem::align_of::<__darwin_mcontext64>() - 16usize];
        ["Offset of field: __darwin_mcontext64::__es"]
            [::std::mem::offset_of!(__darwin_mcontext64, __es) - 0usize];
        ["Offset of field: __darwin_mcontext64::__ss"]
            [::std::mem::offset_of!(__darwin_mcontext64, __ss) - 16usize];
        ["Offset of field: __darwin_mcontext64::__ns"]
            [::std::mem::offset_of!(__darwin_mcontext64, __ns) - 288usize];
    };
    pub type mcontext_t = *mut root::__darwin_mcontext64;
    pub type pthread_attr_t = root::__darwin_pthread_attr_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_sigaltstack {
        pub ss_sp: *mut ::std::os::raw::c_void,
        pub ss_size: root::__darwin_size_t,
        pub ss_flags: ::std::os::raw::c_int,
    }
    const _: () = {
        ["Size of __darwin_sigaltstack"][::std::mem::size_of::<__darwin_sigaltstack>() - 24usize];
        ["Alignment of __darwin_sigaltstack"]
            [::std::mem::align_of::<__darwin_sigaltstack>() - 8usize];
        ["Offset of field: __darwin_sigaltstack::ss_sp"]
            [::std::mem::offset_of!(__darwin_sigaltstack, ss_sp) - 0usize];
        ["Offset of field: __darwin_sigaltstack::ss_size"]
            [::std::mem::offset_of!(__darwin_sigaltstack, ss_size) - 8usize];
        ["Offset of field: __darwin_sigaltstack::ss_flags"]
            [::std::mem::offset_of!(__darwin_sigaltstack, ss_flags) - 16usize];
    };
    pub type stack_t = root::__darwin_sigaltstack;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __darwin_ucontext {
        pub uc_onstack: ::std::os::raw::c_int,
        pub uc_sigmask: root::__darwin_sigset_t,
        pub uc_stack: root::__darwin_sigaltstack,
        pub uc_link: *mut root::__darwin_ucontext,
        pub uc_mcsize: root::__darwin_size_t,
        pub uc_mcontext: *mut root::__darwin_mcontext64,
    }
    const _: () = {
        ["Size of __darwin_ucontext"][::std::mem::size_of::<__darwin_ucontext>() - 56usize];
        ["Alignment of __darwin_ucontext"][::std::mem::align_of::<__darwin_ucontext>() - 8usize];
        ["Offset of field: __darwin_ucontext::uc_onstack"]
            [::std::mem::offset_of!(__darwin_ucontext, uc_onstack) - 0usize];
        ["Offset of field: __darwin_ucontext::uc_sigmask"]
            [::std::mem::offset_of!(__darwin_ucontext, uc_sigmask) - 4usize];
        ["Offset of field: __darwin_ucontext::uc_stack"]
            [::std::mem::offset_of!(__darwin_ucontext, uc_stack) - 8usize];
        ["Offset of field: __darwin_ucontext::uc_link"]
            [::std::mem::offset_of!(__darwin_ucontext, uc_link) - 32usize];
        ["Offset of field: __darwin_ucontext::uc_mcsize"]
            [::std::mem::offset_of!(__darwin_ucontext, uc_mcsize) - 40usize];
        ["Offset of field: __darwin_ucontext::uc_mcontext"]
            [::std::mem::offset_of!(__darwin_ucontext, uc_mcontext) - 48usize];
    };
    pub type ucontext_t = root::__darwin_ucontext;
    pub type sigset_t = root::__darwin_sigset_t;
    pub type uid_t = root::__darwin_uid_t;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union sigval {
        pub sival_int: ::std::os::raw::c_int,
        pub sival_ptr: *mut ::std::os::raw::c_void,
    }
    const _: () = {
        ["Size of sigval"][::std::mem::size_of::<sigval>() - 8usize];
        ["Alignment of sigval"][::std::mem::align_of::<sigval>() - 8usize];
        ["Offset of field: sigval::sival_int"][::std::mem::offset_of!(sigval, sival_int) - 0usize];
        ["Offset of field: sigval::sival_ptr"][::std::mem::offset_of!(sigval, sival_ptr) - 0usize];
    };
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sigevent {
        pub sigev_notify: ::std::os::raw::c_int,
        pub sigev_signo: ::std::os::raw::c_int,
        pub sigev_value: root::sigval,
        pub sigev_notify_function: ::std::option::Option<unsafe extern "C" fn(arg1: root::sigval)>,
        pub sigev_notify_attributes: *mut root::pthread_attr_t,
    }
    const _: () = {
        ["Size of sigevent"][::std::mem::size_of::<sigevent>() - 32usize];
        ["Alignment of sigevent"][::std::mem::align_of::<sigevent>() - 8usize];
        ["Offset of field: sigevent::sigev_notify"]
            [::std::mem::offset_of!(sigevent, sigev_notify) - 0usize];
        ["Offset of field: sigevent::sigev_signo"]
            [::std::mem::offset_of!(sigevent, sigev_signo) - 4usize];
        ["Offset of field: sigevent::sigev_value"]
            [::std::mem::offset_of!(sigevent, sigev_value) - 8usize];
        ["Offset of field: sigevent::sigev_notify_function"]
            [::std::mem::offset_of!(sigevent, sigev_notify_function) - 16usize];
        ["Offset of field: sigevent::sigev_notify_attributes"]
            [::std::mem::offset_of!(sigevent, sigev_notify_attributes) - 24usize];
    };
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __siginfo {
        pub si_signo: ::std::os::raw::c_int,
        pub si_errno: ::std::os::raw::c_int,
        pub si_code: ::std::os::raw::c_int,
        pub si_pid: root::pid_t,
        pub si_uid: root::uid_t,
        pub si_status: ::std::os::raw::c_int,
        pub si_addr: *mut ::std::os::raw::c_void,
        pub si_value: root::sigval,
        pub si_band: ::std::os::raw::c_long,
        pub __pad: [::std::os::raw::c_ulong; 7usize],
    }
    const _: () = {
        ["Size of __siginfo"][::std::mem::size_of::<__siginfo>() - 104usize];
        ["Alignment of __siginfo"][::std::mem::align_of::<__siginfo>() - 8usize];
        ["Offset of field: __siginfo::si_signo"]
            [::std::mem::offset_of!(__siginfo, si_signo) - 0usize];
        ["Offset of field: __siginfo::si_errno"]
            [::std::mem::offset_of!(__siginfo, si_errno) - 4usize];
        ["Offset of field: __siginfo::si_code"]
            [::std::mem::offset_of!(__siginfo, si_code) - 8usize];
        ["Offset of field: __siginfo::si_pid"][::std::mem::offset_of!(__siginfo, si_pid) - 12usize];
        ["Offset of field: __siginfo::si_uid"][::std::mem::offset_of!(__siginfo, si_uid) - 16usize];
        ["Offset of field: __siginfo::si_status"]
            [::std::mem::offset_of!(__siginfo, si_status) - 20usize];
        ["Offset of field: __siginfo::si_addr"]
            [::std::mem::offset_of!(__siginfo, si_addr) - 24usize];
        ["Offset of field: __siginfo::si_value"]
            [::std::mem::offset_of!(__siginfo, si_value) - 32usize];
        ["Offset of field: __siginfo::si_band"]
            [::std::mem::offset_of!(__siginfo, si_band) - 40usize];
        ["Offset of field: __siginfo::__pad"][::std::mem::offset_of!(__siginfo, __pad) - 48usize];
    };
    pub type siginfo_t = root::__siginfo;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union __sigaction_u {
        pub __sa_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        pub __sa_sigaction: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: ::std::os::raw::c_int,
                arg2: *mut root::__siginfo,
                arg3: *mut ::std::os::raw::c_void,
            ),
        >,
    }
    const _: () = {
        ["Size of __sigaction_u"][::std::mem::size_of::<__sigaction_u>() - 8usize];
        ["Alignment of __sigaction_u"][::std::mem::align_of::<__sigaction_u>() - 8usize];
        ["Offset of field: __sigaction_u::__sa_handler"]
            [::std::mem::offset_of!(__sigaction_u, __sa_handler) - 0usize];
        ["Offset of field: __sigaction_u::__sa_sigaction"]
            [::std::mem::offset_of!(__sigaction_u, __sa_sigaction) - 0usize];
    };
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __sigaction {
        pub __sigaction_u: root::__sigaction_u,
        pub sa_tramp: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: ::std::os::raw::c_int,
                arg3: ::std::os::raw::c_int,
                arg4: *mut root::siginfo_t,
                arg5: *mut ::std::os::raw::c_void,
            ),
        >,
        pub sa_mask: root::sigset_t,
        pub sa_flags: ::std::os::raw::c_int,
    }
    const _: () = {
        ["Size of __sigaction"][::std::mem::size_of::<__sigaction>() - 24usize];
        ["Alignment of __sigaction"][::std::mem::align_of::<__sigaction>() - 8usize];
        ["Offset of field: __sigaction::__sigaction_u"]
            [::std::mem::offset_of!(__sigaction, __sigaction_u) - 0usize];
        ["Offset of field: __sigaction::sa_tramp"]
            [::std::mem::offset_of!(__sigaction, sa_tramp) - 8usize];
        ["Offset of field: __sigaction::sa_mask"]
            [::std::mem::offset_of!(__sigaction, sa_mask) - 16usize];
        ["Offset of field: __sigaction::sa_flags"]
            [::std::mem::offset_of!(__sigaction, sa_flags) - 20usize];
    };
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sigaction {
        pub __sigaction_u: root::__sigaction_u,
        pub sa_mask: root::sigset_t,
        pub sa_flags: ::std::os::raw::c_int,
    }
    const _: () = {
        ["Size of sigaction"][::std::mem::size_of::<sigaction>() - 16usize];
        ["Alignment of sigaction"][::std::mem::align_of::<sigaction>() - 8usize];
        ["Offset of field: sigaction::__sigaction_u"]
            [::std::mem::offset_of!(sigaction, __sigaction_u) - 0usize];
        ["Offset of field: sigaction::sa_mask"]
            [::std::mem::offset_of!(sigaction, sa_mask) - 8usize];
        ["Offset of field: sigaction::sa_flags"]
            [::std::mem::offset_of!(sigaction, sa_flags) - 12usize];
    };
    pub type sig_t = ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sigvec {
        pub sv_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        pub sv_mask: ::std::os::raw::c_int,
        pub sv_flags: ::std::os::raw::c_int,
    }
    const _: () = {
        ["Size of sigvec"][::std::mem::size_of::<sigvec>() - 16usize];
        ["Alignment of sigvec"][::std::mem::align_of::<sigvec>() - 8usize];
        ["Offset of field: sigvec::sv_handler"]
            [::std::mem::offset_of!(sigvec, sv_handler) - 0usize];
        ["Offset of field: sigvec::sv_mask"][::std::mem::offset_of!(sigvec, sv_mask) - 8usize];
        ["Offset of field: sigvec::sv_flags"][::std::mem::offset_of!(sigvec, sv_flags) - 12usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sigstack {
        pub ss_sp: *mut ::std::os::raw::c_char,
        pub ss_onstack: ::std::os::raw::c_int,
    }
    const _: () = {
        ["Size of sigstack"][::std::mem::size_of::<sigstack>() - 16usize];
        ["Alignment of sigstack"][::std::mem::align_of::<sigstack>() - 8usize];
        ["Offset of field: sigstack::ss_sp"][::std::mem::offset_of!(sigstack, ss_sp) - 0usize];
        ["Offset of field: sigstack::ss_onstack"]
            [::std::mem::offset_of!(sigstack, ss_onstack) - 8usize];
    };
    extern "C" {
        pub fn signal(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        ) -> ::std::option::Option<
            unsafe extern "C" fn(
                arg1: ::std::os::raw::c_int,
                arg2: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
            ),
        >;
    }
    pub type int_least8_t = i8;
    pub type int_least16_t = i16;
    pub type int_least32_t = i32;
    pub type int_least64_t = i64;
    pub type uint_least8_t = u8;
    pub type uint_least16_t = u16;
    pub type uint_least32_t = u32;
    pub type uint_least64_t = u64;
    pub type int_fast8_t = i8;
    pub type int_fast16_t = i16;
    pub type int_fast32_t = i32;
    pub type int_fast64_t = i64;
    pub type uint_fast8_t = u8;
    pub type uint_fast16_t = u16;
    pub type uint_fast32_t = u32;
    pub type uint_fast64_t = u64;
    pub type intmax_t = ::std::os::raw::c_long;
    pub type uintmax_t = ::std::os::raw::c_ulong;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct timeval {
        pub tv_sec: root::__darwin_time_t,
        pub tv_usec: root::__darwin_suseconds_t,
    }
    const _: () = {
        ["Size of timeval"][::std::mem::size_of::<timeval>() - 16usize];
        ["Alignment of timeval"][::std::mem::align_of::<timeval>() - 8usize];
        ["Offset of field: timeval::tv_sec"][::std::mem::offset_of!(timeval, tv_sec) - 0usize];
        ["Offset of field: timeval::tv_usec"][::std::mem::offset_of!(timeval, tv_usec) - 8usize];
    };
    pub type rlim_t = root::__uint64_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct rusage {
        pub ru_utime: root::timeval,
        pub ru_stime: root::timeval,
        pub ru_maxrss: ::std::os::raw::c_long,
        pub ru_ixrss: ::std::os::raw::c_long,
        pub ru_idrss: ::std::os::raw::c_long,
        pub ru_isrss: ::std::os::raw::c_long,
        pub ru_minflt: ::std::os::raw::c_long,
        pub ru_majflt: ::std::os::raw::c_long,
        pub ru_nswap: ::std::os::raw::c_long,
        pub ru_inblock: ::std::os::raw::c_long,
        pub ru_oublock: ::std::os::raw::c_long,
        pub ru_msgsnd: ::std::os::raw::c_long,
        pub ru_msgrcv: ::std::os::raw::c_long,
        pub ru_nsignals: ::std::os::raw::c_long,
        pub ru_nvcsw: ::std::os::raw::c_long,
        pub ru_nivcsw: ::std::os::raw::c_long,
    }
    const _: () = {
        ["Size of rusage"][::std::mem::size_of::<rusage>() - 144usize];
        ["Alignment of rusage"][::std::mem::align_of::<rusage>() - 8usize];
        ["Offset of field: rusage::ru_utime"][::std::mem::offset_of!(rusage, ru_utime) - 0usize];
        ["Offset of field: rusage::ru_stime"][::std::mem::offset_of!(rusage, ru_stime) - 16usize];
        ["Offset of field: rusage::ru_maxrss"][::std::mem::offset_of!(rusage, ru_maxrss) - 32usize];
        ["Offset of field: rusage::ru_ixrss"][::std::mem::offset_of!(rusage, ru_ixrss) - 40usize];
        ["Offset of field: rusage::ru_idrss"][::std::mem::offset_of!(rusage, ru_idrss) - 48usize];
        ["Offset of field: rusage::ru_isrss"][::std::mem::offset_of!(rusage, ru_isrss) - 56usize];
        ["Offset of field: rusage::ru_minflt"][::std::mem::offset_of!(rusage, ru_minflt) - 64usize];
        ["Offset of field: rusage::ru_majflt"][::std::mem::offset_of!(rusage, ru_majflt) - 72usize];
        ["Offset of field: rusage::ru_nswap"][::std::mem::offset_of!(rusage, ru_nswap) - 80usize];
        ["Offset of field: rusage::ru_inblock"]
            [::std::mem::offset_of!(rusage, ru_inblock) - 88usize];
        ["Offset of field: rusage::ru_oublock"]
            [::std::mem::offset_of!(rusage, ru_oublock) - 96usize];
        ["Offset of field: rusage::ru_msgsnd"]
            [::std::mem::offset_of!(rusage, ru_msgsnd) - 104usize];
        ["Offset of field: rusage::ru_msgrcv"]
            [::std::mem::offset_of!(rusage, ru_msgrcv) - 112usize];
        ["Offset of field: rusage::ru_nsignals"]
            [::std::mem::offset_of!(rusage, ru_nsignals) - 120usize];
        ["Offset of field: rusage::ru_nvcsw"][::std::mem::offset_of!(rusage, ru_nvcsw) - 128usize];
        ["Offset of field: rusage::ru_nivcsw"]
            [::std::mem::offset_of!(rusage, ru_nivcsw) - 136usize];
    };
    pub type rusage_info_t = *mut ::std::os::raw::c_void;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct rusage_info_v0 {
        pub ri_uuid: [u8; 16usize],
        pub ri_user_time: u64,
        pub ri_system_time: u64,
        pub ri_pkg_idle_wkups: u64,
        pub ri_interrupt_wkups: u64,
        pub ri_pageins: u64,
        pub ri_wired_size: u64,
        pub ri_resident_size: u64,
        pub ri_phys_footprint: u64,
        pub ri_proc_start_abstime: u64,
        pub ri_proc_exit_abstime: u64,
    }
    const _: () = {
        ["Size of rusage_info_v0"][::std::mem::size_of::<rusage_info_v0>() - 96usize];
        ["Alignment of rusage_info_v0"][::std::mem::align_of::<rusage_info_v0>() - 8usize];
        ["Offset of field: rusage_info_v0::ri_uuid"]
            [::std::mem::offset_of!(rusage_info_v0, ri_uuid) - 0usize];
        ["Offset of field: rusage_info_v0::ri_user_time"]
            [::std::mem::offset_of!(rusage_info_v0, ri_user_time) - 16usize];
        ["Offset of field: rusage_info_v0::ri_system_time"]
            [::std::mem::offset_of!(rusage_info_v0, ri_system_time) - 24usize];
        ["Offset of field: rusage_info_v0::ri_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v0, ri_pkg_idle_wkups) - 32usize];
        ["Offset of field: rusage_info_v0::ri_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v0, ri_interrupt_wkups) - 40usize];
        ["Offset of field: rusage_info_v0::ri_pageins"]
            [::std::mem::offset_of!(rusage_info_v0, ri_pageins) - 48usize];
        ["Offset of field: rusage_info_v0::ri_wired_size"]
            [::std::mem::offset_of!(rusage_info_v0, ri_wired_size) - 56usize];
        ["Offset of field: rusage_info_v0::ri_resident_size"]
            [::std::mem::offset_of!(rusage_info_v0, ri_resident_size) - 64usize];
        ["Offset of field: rusage_info_v0::ri_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v0, ri_phys_footprint) - 72usize];
        ["Offset of field: rusage_info_v0::ri_proc_start_abstime"]
            [::std::mem::offset_of!(rusage_info_v0, ri_proc_start_abstime) - 80usize];
        ["Offset of field: rusage_info_v0::ri_proc_exit_abstime"]
            [::std::mem::offset_of!(rusage_info_v0, ri_proc_exit_abstime) - 88usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct rusage_info_v1 {
        pub ri_uuid: [u8; 16usize],
        pub ri_user_time: u64,
        pub ri_system_time: u64,
        pub ri_pkg_idle_wkups: u64,
        pub ri_interrupt_wkups: u64,
        pub ri_pageins: u64,
        pub ri_wired_size: u64,
        pub ri_resident_size: u64,
        pub ri_phys_footprint: u64,
        pub ri_proc_start_abstime: u64,
        pub ri_proc_exit_abstime: u64,
        pub ri_child_user_time: u64,
        pub ri_child_system_time: u64,
        pub ri_child_pkg_idle_wkups: u64,
        pub ri_child_interrupt_wkups: u64,
        pub ri_child_pageins: u64,
        pub ri_child_elapsed_abstime: u64,
    }
    const _: () = {
        ["Size of rusage_info_v1"][::std::mem::size_of::<rusage_info_v1>() - 144usize];
        ["Alignment of rusage_info_v1"][::std::mem::align_of::<rusage_info_v1>() - 8usize];
        ["Offset of field: rusage_info_v1::ri_uuid"]
            [::std::mem::offset_of!(rusage_info_v1, ri_uuid) - 0usize];
        ["Offset of field: rusage_info_v1::ri_user_time"]
            [::std::mem::offset_of!(rusage_info_v1, ri_user_time) - 16usize];
        ["Offset of field: rusage_info_v1::ri_system_time"]
            [::std::mem::offset_of!(rusage_info_v1, ri_system_time) - 24usize];
        ["Offset of field: rusage_info_v1::ri_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v1, ri_pkg_idle_wkups) - 32usize];
        ["Offset of field: rusage_info_v1::ri_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v1, ri_interrupt_wkups) - 40usize];
        ["Offset of field: rusage_info_v1::ri_pageins"]
            [::std::mem::offset_of!(rusage_info_v1, ri_pageins) - 48usize];
        ["Offset of field: rusage_info_v1::ri_wired_size"]
            [::std::mem::offset_of!(rusage_info_v1, ri_wired_size) - 56usize];
        ["Offset of field: rusage_info_v1::ri_resident_size"]
            [::std::mem::offset_of!(rusage_info_v1, ri_resident_size) - 64usize];
        ["Offset of field: rusage_info_v1::ri_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v1, ri_phys_footprint) - 72usize];
        ["Offset of field: rusage_info_v1::ri_proc_start_abstime"]
            [::std::mem::offset_of!(rusage_info_v1, ri_proc_start_abstime) - 80usize];
        ["Offset of field: rusage_info_v1::ri_proc_exit_abstime"]
            [::std::mem::offset_of!(rusage_info_v1, ri_proc_exit_abstime) - 88usize];
        ["Offset of field: rusage_info_v1::ri_child_user_time"]
            [::std::mem::offset_of!(rusage_info_v1, ri_child_user_time) - 96usize];
        ["Offset of field: rusage_info_v1::ri_child_system_time"]
            [::std::mem::offset_of!(rusage_info_v1, ri_child_system_time) - 104usize];
        ["Offset of field: rusage_info_v1::ri_child_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v1, ri_child_pkg_idle_wkups) - 112usize];
        ["Offset of field: rusage_info_v1::ri_child_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v1, ri_child_interrupt_wkups) - 120usize];
        ["Offset of field: rusage_info_v1::ri_child_pageins"]
            [::std::mem::offset_of!(rusage_info_v1, ri_child_pageins) - 128usize];
        ["Offset of field: rusage_info_v1::ri_child_elapsed_abstime"]
            [::std::mem::offset_of!(rusage_info_v1, ri_child_elapsed_abstime) - 136usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct rusage_info_v2 {
        pub ri_uuid: [u8; 16usize],
        pub ri_user_time: u64,
        pub ri_system_time: u64,
        pub ri_pkg_idle_wkups: u64,
        pub ri_interrupt_wkups: u64,
        pub ri_pageins: u64,
        pub ri_wired_size: u64,
        pub ri_resident_size: u64,
        pub ri_phys_footprint: u64,
        pub ri_proc_start_abstime: u64,
        pub ri_proc_exit_abstime: u64,
        pub ri_child_user_time: u64,
        pub ri_child_system_time: u64,
        pub ri_child_pkg_idle_wkups: u64,
        pub ri_child_interrupt_wkups: u64,
        pub ri_child_pageins: u64,
        pub ri_child_elapsed_abstime: u64,
        pub ri_diskio_bytesread: u64,
        pub ri_diskio_byteswritten: u64,
    }
    const _: () = {
        ["Size of rusage_info_v2"][::std::mem::size_of::<rusage_info_v2>() - 160usize];
        ["Alignment of rusage_info_v2"][::std::mem::align_of::<rusage_info_v2>() - 8usize];
        ["Offset of field: rusage_info_v2::ri_uuid"]
            [::std::mem::offset_of!(rusage_info_v2, ri_uuid) - 0usize];
        ["Offset of field: rusage_info_v2::ri_user_time"]
            [::std::mem::offset_of!(rusage_info_v2, ri_user_time) - 16usize];
        ["Offset of field: rusage_info_v2::ri_system_time"]
            [::std::mem::offset_of!(rusage_info_v2, ri_system_time) - 24usize];
        ["Offset of field: rusage_info_v2::ri_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v2, ri_pkg_idle_wkups) - 32usize];
        ["Offset of field: rusage_info_v2::ri_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v2, ri_interrupt_wkups) - 40usize];
        ["Offset of field: rusage_info_v2::ri_pageins"]
            [::std::mem::offset_of!(rusage_info_v2, ri_pageins) - 48usize];
        ["Offset of field: rusage_info_v2::ri_wired_size"]
            [::std::mem::offset_of!(rusage_info_v2, ri_wired_size) - 56usize];
        ["Offset of field: rusage_info_v2::ri_resident_size"]
            [::std::mem::offset_of!(rusage_info_v2, ri_resident_size) - 64usize];
        ["Offset of field: rusage_info_v2::ri_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v2, ri_phys_footprint) - 72usize];
        ["Offset of field: rusage_info_v2::ri_proc_start_abstime"]
            [::std::mem::offset_of!(rusage_info_v2, ri_proc_start_abstime) - 80usize];
        ["Offset of field: rusage_info_v2::ri_proc_exit_abstime"]
            [::std::mem::offset_of!(rusage_info_v2, ri_proc_exit_abstime) - 88usize];
        ["Offset of field: rusage_info_v2::ri_child_user_time"]
            [::std::mem::offset_of!(rusage_info_v2, ri_child_user_time) - 96usize];
        ["Offset of field: rusage_info_v2::ri_child_system_time"]
            [::std::mem::offset_of!(rusage_info_v2, ri_child_system_time) - 104usize];
        ["Offset of field: rusage_info_v2::ri_child_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v2, ri_child_pkg_idle_wkups) - 112usize];
        ["Offset of field: rusage_info_v2::ri_child_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v2, ri_child_interrupt_wkups) - 120usize];
        ["Offset of field: rusage_info_v2::ri_child_pageins"]
            [::std::mem::offset_of!(rusage_info_v2, ri_child_pageins) - 128usize];
        ["Offset of field: rusage_info_v2::ri_child_elapsed_abstime"]
            [::std::mem::offset_of!(rusage_info_v2, ri_child_elapsed_abstime) - 136usize];
        ["Offset of field: rusage_info_v2::ri_diskio_bytesread"]
            [::std::mem::offset_of!(rusage_info_v2, ri_diskio_bytesread) - 144usize];
        ["Offset of field: rusage_info_v2::ri_diskio_byteswritten"]
            [::std::mem::offset_of!(rusage_info_v2, ri_diskio_byteswritten) - 152usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct rusage_info_v3 {
        pub ri_uuid: [u8; 16usize],
        pub ri_user_time: u64,
        pub ri_system_time: u64,
        pub ri_pkg_idle_wkups: u64,
        pub ri_interrupt_wkups: u64,
        pub ri_pageins: u64,
        pub ri_wired_size: u64,
        pub ri_resident_size: u64,
        pub ri_phys_footprint: u64,
        pub ri_proc_start_abstime: u64,
        pub ri_proc_exit_abstime: u64,
        pub ri_child_user_time: u64,
        pub ri_child_system_time: u64,
        pub ri_child_pkg_idle_wkups: u64,
        pub ri_child_interrupt_wkups: u64,
        pub ri_child_pageins: u64,
        pub ri_child_elapsed_abstime: u64,
        pub ri_diskio_bytesread: u64,
        pub ri_diskio_byteswritten: u64,
        pub ri_cpu_time_qos_default: u64,
        pub ri_cpu_time_qos_maintenance: u64,
        pub ri_cpu_time_qos_background: u64,
        pub ri_cpu_time_qos_utility: u64,
        pub ri_cpu_time_qos_legacy: u64,
        pub ri_cpu_time_qos_user_initiated: u64,
        pub ri_cpu_time_qos_user_interactive: u64,
        pub ri_billed_system_time: u64,
        pub ri_serviced_system_time: u64,
    }
    const _: () = {
        ["Size of rusage_info_v3"][::std::mem::size_of::<rusage_info_v3>() - 232usize];
        ["Alignment of rusage_info_v3"][::std::mem::align_of::<rusage_info_v3>() - 8usize];
        ["Offset of field: rusage_info_v3::ri_uuid"]
            [::std::mem::offset_of!(rusage_info_v3, ri_uuid) - 0usize];
        ["Offset of field: rusage_info_v3::ri_user_time"]
            [::std::mem::offset_of!(rusage_info_v3, ri_user_time) - 16usize];
        ["Offset of field: rusage_info_v3::ri_system_time"]
            [::std::mem::offset_of!(rusage_info_v3, ri_system_time) - 24usize];
        ["Offset of field: rusage_info_v3::ri_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v3, ri_pkg_idle_wkups) - 32usize];
        ["Offset of field: rusage_info_v3::ri_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v3, ri_interrupt_wkups) - 40usize];
        ["Offset of field: rusage_info_v3::ri_pageins"]
            [::std::mem::offset_of!(rusage_info_v3, ri_pageins) - 48usize];
        ["Offset of field: rusage_info_v3::ri_wired_size"]
            [::std::mem::offset_of!(rusage_info_v3, ri_wired_size) - 56usize];
        ["Offset of field: rusage_info_v3::ri_resident_size"]
            [::std::mem::offset_of!(rusage_info_v3, ri_resident_size) - 64usize];
        ["Offset of field: rusage_info_v3::ri_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v3, ri_phys_footprint) - 72usize];
        ["Offset of field: rusage_info_v3::ri_proc_start_abstime"]
            [::std::mem::offset_of!(rusage_info_v3, ri_proc_start_abstime) - 80usize];
        ["Offset of field: rusage_info_v3::ri_proc_exit_abstime"]
            [::std::mem::offset_of!(rusage_info_v3, ri_proc_exit_abstime) - 88usize];
        ["Offset of field: rusage_info_v3::ri_child_user_time"]
            [::std::mem::offset_of!(rusage_info_v3, ri_child_user_time) - 96usize];
        ["Offset of field: rusage_info_v3::ri_child_system_time"]
            [::std::mem::offset_of!(rusage_info_v3, ri_child_system_time) - 104usize];
        ["Offset of field: rusage_info_v3::ri_child_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v3, ri_child_pkg_idle_wkups) - 112usize];
        ["Offset of field: rusage_info_v3::ri_child_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v3, ri_child_interrupt_wkups) - 120usize];
        ["Offset of field: rusage_info_v3::ri_child_pageins"]
            [::std::mem::offset_of!(rusage_info_v3, ri_child_pageins) - 128usize];
        ["Offset of field: rusage_info_v3::ri_child_elapsed_abstime"]
            [::std::mem::offset_of!(rusage_info_v3, ri_child_elapsed_abstime) - 136usize];
        ["Offset of field: rusage_info_v3::ri_diskio_bytesread"]
            [::std::mem::offset_of!(rusage_info_v3, ri_diskio_bytesread) - 144usize];
        ["Offset of field: rusage_info_v3::ri_diskio_byteswritten"]
            [::std::mem::offset_of!(rusage_info_v3, ri_diskio_byteswritten) - 152usize];
        ["Offset of field: rusage_info_v3::ri_cpu_time_qos_default"]
            [::std::mem::offset_of!(rusage_info_v3, ri_cpu_time_qos_default) - 160usize];
        ["Offset of field: rusage_info_v3::ri_cpu_time_qos_maintenance"]
            [::std::mem::offset_of!(rusage_info_v3, ri_cpu_time_qos_maintenance) - 168usize];
        ["Offset of field: rusage_info_v3::ri_cpu_time_qos_background"]
            [::std::mem::offset_of!(rusage_info_v3, ri_cpu_time_qos_background) - 176usize];
        ["Offset of field: rusage_info_v3::ri_cpu_time_qos_utility"]
            [::std::mem::offset_of!(rusage_info_v3, ri_cpu_time_qos_utility) - 184usize];
        ["Offset of field: rusage_info_v3::ri_cpu_time_qos_legacy"]
            [::std::mem::offset_of!(rusage_info_v3, ri_cpu_time_qos_legacy) - 192usize];
        ["Offset of field: rusage_info_v3::ri_cpu_time_qos_user_initiated"]
            [::std::mem::offset_of!(rusage_info_v3, ri_cpu_time_qos_user_initiated) - 200usize];
        ["Offset of field: rusage_info_v3::ri_cpu_time_qos_user_interactive"]
            [::std::mem::offset_of!(rusage_info_v3, ri_cpu_time_qos_user_interactive) - 208usize];
        ["Offset of field: rusage_info_v3::ri_billed_system_time"]
            [::std::mem::offset_of!(rusage_info_v3, ri_billed_system_time) - 216usize];
        ["Offset of field: rusage_info_v3::ri_serviced_system_time"]
            [::std::mem::offset_of!(rusage_info_v3, ri_serviced_system_time) - 224usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct rusage_info_v4 {
        pub ri_uuid: [u8; 16usize],
        pub ri_user_time: u64,
        pub ri_system_time: u64,
        pub ri_pkg_idle_wkups: u64,
        pub ri_interrupt_wkups: u64,
        pub ri_pageins: u64,
        pub ri_wired_size: u64,
        pub ri_resident_size: u64,
        pub ri_phys_footprint: u64,
        pub ri_proc_start_abstime: u64,
        pub ri_proc_exit_abstime: u64,
        pub ri_child_user_time: u64,
        pub ri_child_system_time: u64,
        pub ri_child_pkg_idle_wkups: u64,
        pub ri_child_interrupt_wkups: u64,
        pub ri_child_pageins: u64,
        pub ri_child_elapsed_abstime: u64,
        pub ri_diskio_bytesread: u64,
        pub ri_diskio_byteswritten: u64,
        pub ri_cpu_time_qos_default: u64,
        pub ri_cpu_time_qos_maintenance: u64,
        pub ri_cpu_time_qos_background: u64,
        pub ri_cpu_time_qos_utility: u64,
        pub ri_cpu_time_qos_legacy: u64,
        pub ri_cpu_time_qos_user_initiated: u64,
        pub ri_cpu_time_qos_user_interactive: u64,
        pub ri_billed_system_time: u64,
        pub ri_serviced_system_time: u64,
        pub ri_logical_writes: u64,
        pub ri_lifetime_max_phys_footprint: u64,
        pub ri_instructions: u64,
        pub ri_cycles: u64,
        pub ri_billed_energy: u64,
        pub ri_serviced_energy: u64,
        pub ri_interval_max_phys_footprint: u64,
        pub ri_runnable_time: u64,
    }
    const _: () = {
        ["Size of rusage_info_v4"][::std::mem::size_of::<rusage_info_v4>() - 296usize];
        ["Alignment of rusage_info_v4"][::std::mem::align_of::<rusage_info_v4>() - 8usize];
        ["Offset of field: rusage_info_v4::ri_uuid"]
            [::std::mem::offset_of!(rusage_info_v4, ri_uuid) - 0usize];
        ["Offset of field: rusage_info_v4::ri_user_time"]
            [::std::mem::offset_of!(rusage_info_v4, ri_user_time) - 16usize];
        ["Offset of field: rusage_info_v4::ri_system_time"]
            [::std::mem::offset_of!(rusage_info_v4, ri_system_time) - 24usize];
        ["Offset of field: rusage_info_v4::ri_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v4, ri_pkg_idle_wkups) - 32usize];
        ["Offset of field: rusage_info_v4::ri_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v4, ri_interrupt_wkups) - 40usize];
        ["Offset of field: rusage_info_v4::ri_pageins"]
            [::std::mem::offset_of!(rusage_info_v4, ri_pageins) - 48usize];
        ["Offset of field: rusage_info_v4::ri_wired_size"]
            [::std::mem::offset_of!(rusage_info_v4, ri_wired_size) - 56usize];
        ["Offset of field: rusage_info_v4::ri_resident_size"]
            [::std::mem::offset_of!(rusage_info_v4, ri_resident_size) - 64usize];
        ["Offset of field: rusage_info_v4::ri_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v4, ri_phys_footprint) - 72usize];
        ["Offset of field: rusage_info_v4::ri_proc_start_abstime"]
            [::std::mem::offset_of!(rusage_info_v4, ri_proc_start_abstime) - 80usize];
        ["Offset of field: rusage_info_v4::ri_proc_exit_abstime"]
            [::std::mem::offset_of!(rusage_info_v4, ri_proc_exit_abstime) - 88usize];
        ["Offset of field: rusage_info_v4::ri_child_user_time"]
            [::std::mem::offset_of!(rusage_info_v4, ri_child_user_time) - 96usize];
        ["Offset of field: rusage_info_v4::ri_child_system_time"]
            [::std::mem::offset_of!(rusage_info_v4, ri_child_system_time) - 104usize];
        ["Offset of field: rusage_info_v4::ri_child_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v4, ri_child_pkg_idle_wkups) - 112usize];
        ["Offset of field: rusage_info_v4::ri_child_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v4, ri_child_interrupt_wkups) - 120usize];
        ["Offset of field: rusage_info_v4::ri_child_pageins"]
            [::std::mem::offset_of!(rusage_info_v4, ri_child_pageins) - 128usize];
        ["Offset of field: rusage_info_v4::ri_child_elapsed_abstime"]
            [::std::mem::offset_of!(rusage_info_v4, ri_child_elapsed_abstime) - 136usize];
        ["Offset of field: rusage_info_v4::ri_diskio_bytesread"]
            [::std::mem::offset_of!(rusage_info_v4, ri_diskio_bytesread) - 144usize];
        ["Offset of field: rusage_info_v4::ri_diskio_byteswritten"]
            [::std::mem::offset_of!(rusage_info_v4, ri_diskio_byteswritten) - 152usize];
        ["Offset of field: rusage_info_v4::ri_cpu_time_qos_default"]
            [::std::mem::offset_of!(rusage_info_v4, ri_cpu_time_qos_default) - 160usize];
        ["Offset of field: rusage_info_v4::ri_cpu_time_qos_maintenance"]
            [::std::mem::offset_of!(rusage_info_v4, ri_cpu_time_qos_maintenance) - 168usize];
        ["Offset of field: rusage_info_v4::ri_cpu_time_qos_background"]
            [::std::mem::offset_of!(rusage_info_v4, ri_cpu_time_qos_background) - 176usize];
        ["Offset of field: rusage_info_v4::ri_cpu_time_qos_utility"]
            [::std::mem::offset_of!(rusage_info_v4, ri_cpu_time_qos_utility) - 184usize];
        ["Offset of field: rusage_info_v4::ri_cpu_time_qos_legacy"]
            [::std::mem::offset_of!(rusage_info_v4, ri_cpu_time_qos_legacy) - 192usize];
        ["Offset of field: rusage_info_v4::ri_cpu_time_qos_user_initiated"]
            [::std::mem::offset_of!(rusage_info_v4, ri_cpu_time_qos_user_initiated) - 200usize];
        ["Offset of field: rusage_info_v4::ri_cpu_time_qos_user_interactive"]
            [::std::mem::offset_of!(rusage_info_v4, ri_cpu_time_qos_user_interactive) - 208usize];
        ["Offset of field: rusage_info_v4::ri_billed_system_time"]
            [::std::mem::offset_of!(rusage_info_v4, ri_billed_system_time) - 216usize];
        ["Offset of field: rusage_info_v4::ri_serviced_system_time"]
            [::std::mem::offset_of!(rusage_info_v4, ri_serviced_system_time) - 224usize];
        ["Offset of field: rusage_info_v4::ri_logical_writes"]
            [::std::mem::offset_of!(rusage_info_v4, ri_logical_writes) - 232usize];
        ["Offset of field: rusage_info_v4::ri_lifetime_max_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v4, ri_lifetime_max_phys_footprint) - 240usize];
        ["Offset of field: rusage_info_v4::ri_instructions"]
            [::std::mem::offset_of!(rusage_info_v4, ri_instructions) - 248usize];
        ["Offset of field: rusage_info_v4::ri_cycles"]
            [::std::mem::offset_of!(rusage_info_v4, ri_cycles) - 256usize];
        ["Offset of field: rusage_info_v4::ri_billed_energy"]
            [::std::mem::offset_of!(rusage_info_v4, ri_billed_energy) - 264usize];
        ["Offset of field: rusage_info_v4::ri_serviced_energy"]
            [::std::mem::offset_of!(rusage_info_v4, ri_serviced_energy) - 272usize];
        ["Offset of field: rusage_info_v4::ri_interval_max_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v4, ri_interval_max_phys_footprint) - 280usize];
        ["Offset of field: rusage_info_v4::ri_runnable_time"]
            [::std::mem::offset_of!(rusage_info_v4, ri_runnable_time) - 288usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct rusage_info_v5 {
        pub ri_uuid: [u8; 16usize],
        pub ri_user_time: u64,
        pub ri_system_time: u64,
        pub ri_pkg_idle_wkups: u64,
        pub ri_interrupt_wkups: u64,
        pub ri_pageins: u64,
        pub ri_wired_size: u64,
        pub ri_resident_size: u64,
        pub ri_phys_footprint: u64,
        pub ri_proc_start_abstime: u64,
        pub ri_proc_exit_abstime: u64,
        pub ri_child_user_time: u64,
        pub ri_child_system_time: u64,
        pub ri_child_pkg_idle_wkups: u64,
        pub ri_child_interrupt_wkups: u64,
        pub ri_child_pageins: u64,
        pub ri_child_elapsed_abstime: u64,
        pub ri_diskio_bytesread: u64,
        pub ri_diskio_byteswritten: u64,
        pub ri_cpu_time_qos_default: u64,
        pub ri_cpu_time_qos_maintenance: u64,
        pub ri_cpu_time_qos_background: u64,
        pub ri_cpu_time_qos_utility: u64,
        pub ri_cpu_time_qos_legacy: u64,
        pub ri_cpu_time_qos_user_initiated: u64,
        pub ri_cpu_time_qos_user_interactive: u64,
        pub ri_billed_system_time: u64,
        pub ri_serviced_system_time: u64,
        pub ri_logical_writes: u64,
        pub ri_lifetime_max_phys_footprint: u64,
        pub ri_instructions: u64,
        pub ri_cycles: u64,
        pub ri_billed_energy: u64,
        pub ri_serviced_energy: u64,
        pub ri_interval_max_phys_footprint: u64,
        pub ri_runnable_time: u64,
        pub ri_flags: u64,
    }
    const _: () = {
        ["Size of rusage_info_v5"][::std::mem::size_of::<rusage_info_v5>() - 304usize];
        ["Alignment of rusage_info_v5"][::std::mem::align_of::<rusage_info_v5>() - 8usize];
        ["Offset of field: rusage_info_v5::ri_uuid"]
            [::std::mem::offset_of!(rusage_info_v5, ri_uuid) - 0usize];
        ["Offset of field: rusage_info_v5::ri_user_time"]
            [::std::mem::offset_of!(rusage_info_v5, ri_user_time) - 16usize];
        ["Offset of field: rusage_info_v5::ri_system_time"]
            [::std::mem::offset_of!(rusage_info_v5, ri_system_time) - 24usize];
        ["Offset of field: rusage_info_v5::ri_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v5, ri_pkg_idle_wkups) - 32usize];
        ["Offset of field: rusage_info_v5::ri_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v5, ri_interrupt_wkups) - 40usize];
        ["Offset of field: rusage_info_v5::ri_pageins"]
            [::std::mem::offset_of!(rusage_info_v5, ri_pageins) - 48usize];
        ["Offset of field: rusage_info_v5::ri_wired_size"]
            [::std::mem::offset_of!(rusage_info_v5, ri_wired_size) - 56usize];
        ["Offset of field: rusage_info_v5::ri_resident_size"]
            [::std::mem::offset_of!(rusage_info_v5, ri_resident_size) - 64usize];
        ["Offset of field: rusage_info_v5::ri_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v5, ri_phys_footprint) - 72usize];
        ["Offset of field: rusage_info_v5::ri_proc_start_abstime"]
            [::std::mem::offset_of!(rusage_info_v5, ri_proc_start_abstime) - 80usize];
        ["Offset of field: rusage_info_v5::ri_proc_exit_abstime"]
            [::std::mem::offset_of!(rusage_info_v5, ri_proc_exit_abstime) - 88usize];
        ["Offset of field: rusage_info_v5::ri_child_user_time"]
            [::std::mem::offset_of!(rusage_info_v5, ri_child_user_time) - 96usize];
        ["Offset of field: rusage_info_v5::ri_child_system_time"]
            [::std::mem::offset_of!(rusage_info_v5, ri_child_system_time) - 104usize];
        ["Offset of field: rusage_info_v5::ri_child_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v5, ri_child_pkg_idle_wkups) - 112usize];
        ["Offset of field: rusage_info_v5::ri_child_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v5, ri_child_interrupt_wkups) - 120usize];
        ["Offset of field: rusage_info_v5::ri_child_pageins"]
            [::std::mem::offset_of!(rusage_info_v5, ri_child_pageins) - 128usize];
        ["Offset of field: rusage_info_v5::ri_child_elapsed_abstime"]
            [::std::mem::offset_of!(rusage_info_v5, ri_child_elapsed_abstime) - 136usize];
        ["Offset of field: rusage_info_v5::ri_diskio_bytesread"]
            [::std::mem::offset_of!(rusage_info_v5, ri_diskio_bytesread) - 144usize];
        ["Offset of field: rusage_info_v5::ri_diskio_byteswritten"]
            [::std::mem::offset_of!(rusage_info_v5, ri_diskio_byteswritten) - 152usize];
        ["Offset of field: rusage_info_v5::ri_cpu_time_qos_default"]
            [::std::mem::offset_of!(rusage_info_v5, ri_cpu_time_qos_default) - 160usize];
        ["Offset of field: rusage_info_v5::ri_cpu_time_qos_maintenance"]
            [::std::mem::offset_of!(rusage_info_v5, ri_cpu_time_qos_maintenance) - 168usize];
        ["Offset of field: rusage_info_v5::ri_cpu_time_qos_background"]
            [::std::mem::offset_of!(rusage_info_v5, ri_cpu_time_qos_background) - 176usize];
        ["Offset of field: rusage_info_v5::ri_cpu_time_qos_utility"]
            [::std::mem::offset_of!(rusage_info_v5, ri_cpu_time_qos_utility) - 184usize];
        ["Offset of field: rusage_info_v5::ri_cpu_time_qos_legacy"]
            [::std::mem::offset_of!(rusage_info_v5, ri_cpu_time_qos_legacy) - 192usize];
        ["Offset of field: rusage_info_v5::ri_cpu_time_qos_user_initiated"]
            [::std::mem::offset_of!(rusage_info_v5, ri_cpu_time_qos_user_initiated) - 200usize];
        ["Offset of field: rusage_info_v5::ri_cpu_time_qos_user_interactive"]
            [::std::mem::offset_of!(rusage_info_v5, ri_cpu_time_qos_user_interactive) - 208usize];
        ["Offset of field: rusage_info_v5::ri_billed_system_time"]
            [::std::mem::offset_of!(rusage_info_v5, ri_billed_system_time) - 216usize];
        ["Offset of field: rusage_info_v5::ri_serviced_system_time"]
            [::std::mem::offset_of!(rusage_info_v5, ri_serviced_system_time) - 224usize];
        ["Offset of field: rusage_info_v5::ri_logical_writes"]
            [::std::mem::offset_of!(rusage_info_v5, ri_logical_writes) - 232usize];
        ["Offset of field: rusage_info_v5::ri_lifetime_max_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v5, ri_lifetime_max_phys_footprint) - 240usize];
        ["Offset of field: rusage_info_v5::ri_instructions"]
            [::std::mem::offset_of!(rusage_info_v5, ri_instructions) - 248usize];
        ["Offset of field: rusage_info_v5::ri_cycles"]
            [::std::mem::offset_of!(rusage_info_v5, ri_cycles) - 256usize];
        ["Offset of field: rusage_info_v5::ri_billed_energy"]
            [::std::mem::offset_of!(rusage_info_v5, ri_billed_energy) - 264usize];
        ["Offset of field: rusage_info_v5::ri_serviced_energy"]
            [::std::mem::offset_of!(rusage_info_v5, ri_serviced_energy) - 272usize];
        ["Offset of field: rusage_info_v5::ri_interval_max_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v5, ri_interval_max_phys_footprint) - 280usize];
        ["Offset of field: rusage_info_v5::ri_runnable_time"]
            [::std::mem::offset_of!(rusage_info_v5, ri_runnable_time) - 288usize];
        ["Offset of field: rusage_info_v5::ri_flags"]
            [::std::mem::offset_of!(rusage_info_v5, ri_flags) - 296usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct rusage_info_v6 {
        pub ri_uuid: [u8; 16usize],
        pub ri_user_time: u64,
        pub ri_system_time: u64,
        pub ri_pkg_idle_wkups: u64,
        pub ri_interrupt_wkups: u64,
        pub ri_pageins: u64,
        pub ri_wired_size: u64,
        pub ri_resident_size: u64,
        pub ri_phys_footprint: u64,
        pub ri_proc_start_abstime: u64,
        pub ri_proc_exit_abstime: u64,
        pub ri_child_user_time: u64,
        pub ri_child_system_time: u64,
        pub ri_child_pkg_idle_wkups: u64,
        pub ri_child_interrupt_wkups: u64,
        pub ri_child_pageins: u64,
        pub ri_child_elapsed_abstime: u64,
        pub ri_diskio_bytesread: u64,
        pub ri_diskio_byteswritten: u64,
        pub ri_cpu_time_qos_default: u64,
        pub ri_cpu_time_qos_maintenance: u64,
        pub ri_cpu_time_qos_background: u64,
        pub ri_cpu_time_qos_utility: u64,
        pub ri_cpu_time_qos_legacy: u64,
        pub ri_cpu_time_qos_user_initiated: u64,
        pub ri_cpu_time_qos_user_interactive: u64,
        pub ri_billed_system_time: u64,
        pub ri_serviced_system_time: u64,
        pub ri_logical_writes: u64,
        pub ri_lifetime_max_phys_footprint: u64,
        pub ri_instructions: u64,
        pub ri_cycles: u64,
        pub ri_billed_energy: u64,
        pub ri_serviced_energy: u64,
        pub ri_interval_max_phys_footprint: u64,
        pub ri_runnable_time: u64,
        pub ri_flags: u64,
        pub ri_user_ptime: u64,
        pub ri_system_ptime: u64,
        pub ri_pinstructions: u64,
        pub ri_pcycles: u64,
        pub ri_energy_nj: u64,
        pub ri_penergy_nj: u64,
        pub ri_secure_time_in_system: u64,
        pub ri_secure_ptime_in_system: u64,
        pub ri_reserved: [u64; 12usize],
    }
    const _: () = {
        ["Size of rusage_info_v6"][::std::mem::size_of::<rusage_info_v6>() - 464usize];
        ["Alignment of rusage_info_v6"][::std::mem::align_of::<rusage_info_v6>() - 8usize];
        ["Offset of field: rusage_info_v6::ri_uuid"]
            [::std::mem::offset_of!(rusage_info_v6, ri_uuid) - 0usize];
        ["Offset of field: rusage_info_v6::ri_user_time"]
            [::std::mem::offset_of!(rusage_info_v6, ri_user_time) - 16usize];
        ["Offset of field: rusage_info_v6::ri_system_time"]
            [::std::mem::offset_of!(rusage_info_v6, ri_system_time) - 24usize];
        ["Offset of field: rusage_info_v6::ri_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v6, ri_pkg_idle_wkups) - 32usize];
        ["Offset of field: rusage_info_v6::ri_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v6, ri_interrupt_wkups) - 40usize];
        ["Offset of field: rusage_info_v6::ri_pageins"]
            [::std::mem::offset_of!(rusage_info_v6, ri_pageins) - 48usize];
        ["Offset of field: rusage_info_v6::ri_wired_size"]
            [::std::mem::offset_of!(rusage_info_v6, ri_wired_size) - 56usize];
        ["Offset of field: rusage_info_v6::ri_resident_size"]
            [::std::mem::offset_of!(rusage_info_v6, ri_resident_size) - 64usize];
        ["Offset of field: rusage_info_v6::ri_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v6, ri_phys_footprint) - 72usize];
        ["Offset of field: rusage_info_v6::ri_proc_start_abstime"]
            [::std::mem::offset_of!(rusage_info_v6, ri_proc_start_abstime) - 80usize];
        ["Offset of field: rusage_info_v6::ri_proc_exit_abstime"]
            [::std::mem::offset_of!(rusage_info_v6, ri_proc_exit_abstime) - 88usize];
        ["Offset of field: rusage_info_v6::ri_child_user_time"]
            [::std::mem::offset_of!(rusage_info_v6, ri_child_user_time) - 96usize];
        ["Offset of field: rusage_info_v6::ri_child_system_time"]
            [::std::mem::offset_of!(rusage_info_v6, ri_child_system_time) - 104usize];
        ["Offset of field: rusage_info_v6::ri_child_pkg_idle_wkups"]
            [::std::mem::offset_of!(rusage_info_v6, ri_child_pkg_idle_wkups) - 112usize];
        ["Offset of field: rusage_info_v6::ri_child_interrupt_wkups"]
            [::std::mem::offset_of!(rusage_info_v6, ri_child_interrupt_wkups) - 120usize];
        ["Offset of field: rusage_info_v6::ri_child_pageins"]
            [::std::mem::offset_of!(rusage_info_v6, ri_child_pageins) - 128usize];
        ["Offset of field: rusage_info_v6::ri_child_elapsed_abstime"]
            [::std::mem::offset_of!(rusage_info_v6, ri_child_elapsed_abstime) - 136usize];
        ["Offset of field: rusage_info_v6::ri_diskio_bytesread"]
            [::std::mem::offset_of!(rusage_info_v6, ri_diskio_bytesread) - 144usize];
        ["Offset of field: rusage_info_v6::ri_diskio_byteswritten"]
            [::std::mem::offset_of!(rusage_info_v6, ri_diskio_byteswritten) - 152usize];
        ["Offset of field: rusage_info_v6::ri_cpu_time_qos_default"]
            [::std::mem::offset_of!(rusage_info_v6, ri_cpu_time_qos_default) - 160usize];
        ["Offset of field: rusage_info_v6::ri_cpu_time_qos_maintenance"]
            [::std::mem::offset_of!(rusage_info_v6, ri_cpu_time_qos_maintenance) - 168usize];
        ["Offset of field: rusage_info_v6::ri_cpu_time_qos_background"]
            [::std::mem::offset_of!(rusage_info_v6, ri_cpu_time_qos_background) - 176usize];
        ["Offset of field: rusage_info_v6::ri_cpu_time_qos_utility"]
            [::std::mem::offset_of!(rusage_info_v6, ri_cpu_time_qos_utility) - 184usize];
        ["Offset of field: rusage_info_v6::ri_cpu_time_qos_legacy"]
            [::std::mem::offset_of!(rusage_info_v6, ri_cpu_time_qos_legacy) - 192usize];
        ["Offset of field: rusage_info_v6::ri_cpu_time_qos_user_initiated"]
            [::std::mem::offset_of!(rusage_info_v6, ri_cpu_time_qos_user_initiated) - 200usize];
        ["Offset of field: rusage_info_v6::ri_cpu_time_qos_user_interactive"]
            [::std::mem::offset_of!(rusage_info_v6, ri_cpu_time_qos_user_interactive) - 208usize];
        ["Offset of field: rusage_info_v6::ri_billed_system_time"]
            [::std::mem::offset_of!(rusage_info_v6, ri_billed_system_time) - 216usize];
        ["Offset of field: rusage_info_v6::ri_serviced_system_time"]
            [::std::mem::offset_of!(rusage_info_v6, ri_serviced_system_time) - 224usize];
        ["Offset of field: rusage_info_v6::ri_logical_writes"]
            [::std::mem::offset_of!(rusage_info_v6, ri_logical_writes) - 232usize];
        ["Offset of field: rusage_info_v6::ri_lifetime_max_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v6, ri_lifetime_max_phys_footprint) - 240usize];
        ["Offset of field: rusage_info_v6::ri_instructions"]
            [::std::mem::offset_of!(rusage_info_v6, ri_instructions) - 248usize];
        ["Offset of field: rusage_info_v6::ri_cycles"]
            [::std::mem::offset_of!(rusage_info_v6, ri_cycles) - 256usize];
        ["Offset of field: rusage_info_v6::ri_billed_energy"]
            [::std::mem::offset_of!(rusage_info_v6, ri_billed_energy) - 264usize];
        ["Offset of field: rusage_info_v6::ri_serviced_energy"]
            [::std::mem::offset_of!(rusage_info_v6, ri_serviced_energy) - 272usize];
        ["Offset of field: rusage_info_v6::ri_interval_max_phys_footprint"]
            [::std::mem::offset_of!(rusage_info_v6, ri_interval_max_phys_footprint) - 280usize];
        ["Offset of field: rusage_info_v6::ri_runnable_time"]
            [::std::mem::offset_of!(rusage_info_v6, ri_runnable_time) - 288usize];
        ["Offset of field: rusage_info_v6::ri_flags"]
            [::std::mem::offset_of!(rusage_info_v6, ri_flags) - 296usize];
        ["Offset of field: rusage_info_v6::ri_user_ptime"]
            [::std::mem::offset_of!(rusage_info_v6, ri_user_ptime) - 304usize];
        ["Offset of field: rusage_info_v6::ri_system_ptime"]
            [::std::mem::offset_of!(rusage_info_v6, ri_system_ptime) - 312usize];
        ["Offset of field: rusage_info_v6::ri_pinstructions"]
            [::std::mem::offset_of!(rusage_info_v6, ri_pinstructions) - 320usize];
        ["Offset of field: rusage_info_v6::ri_pcycles"]
            [::std::mem::offset_of!(rusage_info_v6, ri_pcycles) - 328usize];
        ["Offset of field: rusage_info_v6::ri_energy_nj"]
            [::std::mem::offset_of!(rusage_info_v6, ri_energy_nj) - 336usize];
        ["Offset of field: rusage_info_v6::ri_penergy_nj"]
            [::std::mem::offset_of!(rusage_info_v6, ri_penergy_nj) - 344usize];
        ["Offset of field: rusage_info_v6::ri_secure_time_in_system"]
            [::std::mem::offset_of!(rusage_info_v6, ri_secure_time_in_system) - 352usize];
        ["Offset of field: rusage_info_v6::ri_secure_ptime_in_system"]
            [::std::mem::offset_of!(rusage_info_v6, ri_secure_ptime_in_system) - 360usize];
        ["Offset of field: rusage_info_v6::ri_reserved"]
            [::std::mem::offset_of!(rusage_info_v6, ri_reserved) - 368usize];
    };
    pub type rusage_info_current = root::rusage_info_v6;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct rlimit {
        pub rlim_cur: root::rlim_t,
        pub rlim_max: root::rlim_t,
    }
    const _: () = {
        ["Size of rlimit"][::std::mem::size_of::<rlimit>() - 16usize];
        ["Alignment of rlimit"][::std::mem::align_of::<rlimit>() - 8usize];
        ["Offset of field: rlimit::rlim_cur"][::std::mem::offset_of!(rlimit, rlim_cur) - 0usize];
        ["Offset of field: rlimit::rlim_max"][::std::mem::offset_of!(rlimit, rlim_max) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct proc_rlimit_control_wakeupmon {
        pub wm_flags: u32,
        pub wm_rate: i32,
    }
    const _: () = {
        ["Size of proc_rlimit_control_wakeupmon"]
            [::std::mem::size_of::<proc_rlimit_control_wakeupmon>() - 8usize];
        ["Alignment of proc_rlimit_control_wakeupmon"]
            [::std::mem::align_of::<proc_rlimit_control_wakeupmon>() - 4usize];
        ["Offset of field: proc_rlimit_control_wakeupmon::wm_flags"]
            [::std::mem::offset_of!(proc_rlimit_control_wakeupmon, wm_flags) - 0usize];
        ["Offset of field: proc_rlimit_control_wakeupmon::wm_rate"]
            [::std::mem::offset_of!(proc_rlimit_control_wakeupmon, wm_rate) - 4usize];
    };
    extern "C" {
        pub fn getpriority(arg1: ::std::os::raw::c_int, arg2: root::id_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn getiopolicy_np(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn getrlimit(
            arg1: ::std::os::raw::c_int,
            arg2: *mut root::rlimit,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn getrusage(
            arg1: ::std::os::raw::c_int,
            arg2: *mut root::rusage,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn setpriority(
            arg1: ::std::os::raw::c_int,
            arg2: root::id_t,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn setiopolicy_np(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn setrlimit(
            arg1: ::std::os::raw::c_int,
            arg2: *const root::rlimit,
        ) -> ::std::os::raw::c_int;
    }
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct _OSUnalignedU16 {
        pub __val: u16,
    }
    const _: () = {
        ["Size of _OSUnalignedU16"][::std::mem::size_of::<_OSUnalignedU16>() - 2usize];
        ["Alignment of _OSUnalignedU16"][::std::mem::align_of::<_OSUnalignedU16>() - 1usize];
        ["Offset of field: _OSUnalignedU16::__val"]
            [::std::mem::offset_of!(_OSUnalignedU16, __val) - 0usize];
    };
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct _OSUnalignedU32 {
        pub __val: u32,
    }
    const _: () = {
        ["Size of _OSUnalignedU32"][::std::mem::size_of::<_OSUnalignedU32>() - 4usize];
        ["Alignment of _OSUnalignedU32"][::std::mem::align_of::<_OSUnalignedU32>() - 1usize];
        ["Offset of field: _OSUnalignedU32::__val"]
            [::std::mem::offset_of!(_OSUnalignedU32, __val) - 0usize];
    };
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct _OSUnalignedU64 {
        pub __val: u64,
    }
    const _: () = {
        ["Size of _OSUnalignedU64"][::std::mem::size_of::<_OSUnalignedU64>() - 8usize];
        ["Alignment of _OSUnalignedU64"][::std::mem::align_of::<_OSUnalignedU64>() - 1usize];
        ["Offset of field: _OSUnalignedU64::__val"]
            [::std::mem::offset_of!(_OSUnalignedU64, __val) - 0usize];
    };
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union wait {
        pub w_status: ::std::os::raw::c_int,
        pub w_T: root::wait__bindgen_ty_1,
        pub w_S: root::wait__bindgen_ty_2,
    }
    #[repr(C)]
    #[repr(align(4))]
    #[derive(Debug, Copy, Clone)]
    pub struct wait__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: root::__BindgenBitfieldUnit<[u8; 4usize]>,
    }
    const _: () = {
        ["Size of wait__bindgen_ty_1"][::std::mem::size_of::<wait__bindgen_ty_1>() - 4usize];
        ["Alignment of wait__bindgen_ty_1"][::std::mem::align_of::<wait__bindgen_ty_1>() - 4usize];
    };
    impl wait__bindgen_ty_1 {
        #[inline]
        pub fn w_Termsig(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u32) }
        }
        #[inline]
        pub fn set_w_Termsig(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(0usize, 7u8, val as u64)
            }
        }
        #[inline]
        pub fn w_Coredump(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
        }
        #[inline]
        pub fn set_w_Coredump(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn w_Retcode(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
        }
        #[inline]
        pub fn set_w_Retcode(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(8usize, 8u8, val as u64)
            }
        }
        #[inline]
        pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
        }
        #[inline]
        pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(
            w_Termsig: ::std::os::raw::c_uint,
            w_Coredump: ::std::os::raw::c_uint,
            w_Retcode: ::std::os::raw::c_uint,
            w_Filler: ::std::os::raw::c_uint,
        ) -> root::__BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit: root::__BindgenBitfieldUnit<[u8; 4usize]> =
                Default::default();
            __bindgen_bitfield_unit.set(0usize, 7u8, {
                let w_Termsig: u32 = unsafe { ::std::mem::transmute(w_Termsig) };
                w_Termsig as u64
            });
            __bindgen_bitfield_unit.set(7usize, 1u8, {
                let w_Coredump: u32 = unsafe { ::std::mem::transmute(w_Coredump) };
                w_Coredump as u64
            });
            __bindgen_bitfield_unit.set(8usize, 8u8, {
                let w_Retcode: u32 = unsafe { ::std::mem::transmute(w_Retcode) };
                w_Retcode as u64
            });
            __bindgen_bitfield_unit.set(16usize, 16u8, {
                let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
                w_Filler as u64
            });
            __bindgen_bitfield_unit
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    #[derive(Debug, Copy, Clone)]
    pub struct wait__bindgen_ty_2 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: root::__BindgenBitfieldUnit<[u8; 4usize]>,
    }
    const _: () = {
        ["Size of wait__bindgen_ty_2"][::std::mem::size_of::<wait__bindgen_ty_2>() - 4usize];
        ["Alignment of wait__bindgen_ty_2"][::std::mem::align_of::<wait__bindgen_ty_2>() - 4usize];
    };
    impl wait__bindgen_ty_2 {
        #[inline]
        pub fn w_Stopval(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
        }
        #[inline]
        pub fn set_w_Stopval(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(0usize, 8u8, val as u64)
            }
        }
        #[inline]
        pub fn w_Stopsig(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
        }
        #[inline]
        pub fn set_w_Stopsig(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(8usize, 8u8, val as u64)
            }
        }
        #[inline]
        pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
        }
        #[inline]
        pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(
            w_Stopval: ::std::os::raw::c_uint,
            w_Stopsig: ::std::os::raw::c_uint,
            w_Filler: ::std::os::raw::c_uint,
        ) -> root::__BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit: root::__BindgenBitfieldUnit<[u8; 4usize]> =
                Default::default();
            __bindgen_bitfield_unit.set(0usize, 8u8, {
                let w_Stopval: u32 = unsafe { ::std::mem::transmute(w_Stopval) };
                w_Stopval as u64
            });
            __bindgen_bitfield_unit.set(8usize, 8u8, {
                let w_Stopsig: u32 = unsafe { ::std::mem::transmute(w_Stopsig) };
                w_Stopsig as u64
            });
            __bindgen_bitfield_unit.set(16usize, 16u8, {
                let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
                w_Filler as u64
            });
            __bindgen_bitfield_unit
        }
    }
    const _: () = {
        ["Size of wait"][::std::mem::size_of::<wait>() - 4usize];
        ["Alignment of wait"][::std::mem::align_of::<wait>() - 4usize];
        ["Offset of field: wait::w_status"][::std::mem::offset_of!(wait, w_status) - 0usize];
        ["Offset of field: wait::w_T"][::std::mem::offset_of!(wait, w_T) - 0usize];
        ["Offset of field: wait::w_S"][::std::mem::offset_of!(wait, w_S) - 0usize];
    };
    extern "C" {
        pub fn wait(arg1: *mut ::std::os::raw::c_int) -> root::pid_t;
    }
    extern "C" {
        pub fn waitpid(
            arg1: root::pid_t,
            arg2: *mut ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ) -> root::pid_t;
    }
    extern "C" {
        pub fn waitid(
            arg1: root::idtype_t,
            arg2: root::id_t,
            arg3: *mut root::siginfo_t,
            arg4: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wait3(
            arg1: *mut ::std::os::raw::c_int,
            arg2: ::std::os::raw::c_int,
            arg3: *mut root::rusage,
        ) -> root::pid_t;
    }
    extern "C" {
        pub fn wait4(
            arg1: root::pid_t,
            arg2: *mut ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: *mut root::rusage,
        ) -> root::pid_t;
    }
    extern "C" {
        pub fn alloca(arg1: usize) -> *mut ::std::os::raw::c_void;
    }
    pub type ct_rune_t = root::__darwin_ct_rune_t;
    pub type rune_t = root::__darwin_rune_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct div_t {
        pub quot: ::std::os::raw::c_int,
        pub rem: ::std::os::raw::c_int,
    }
    const _: () = {
        ["Size of div_t"][::std::mem::size_of::<div_t>() - 8usize];
        ["Alignment of div_t"][::std::mem::align_of::<div_t>() - 4usize];
        ["Offset of field: div_t::quot"][::std::mem::offset_of!(div_t, quot) - 0usize];
        ["Offset of field: div_t::rem"][::std::mem::offset_of!(div_t, rem) - 4usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ldiv_t {
        pub quot: ::std::os::raw::c_long,
        pub rem: ::std::os::raw::c_long,
    }
    const _: () = {
        ["Size of ldiv_t"][::std::mem::size_of::<ldiv_t>() - 16usize];
        ["Alignment of ldiv_t"][::std::mem::align_of::<ldiv_t>() - 8usize];
        ["Offset of field: ldiv_t::quot"][::std::mem::offset_of!(ldiv_t, quot) - 0usize];
        ["Offset of field: ldiv_t::rem"][::std::mem::offset_of!(ldiv_t, rem) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct lldiv_t {
        pub quot: ::std::os::raw::c_longlong,
        pub rem: ::std::os::raw::c_longlong,
    }
    const _: () = {
        ["Size of lldiv_t"][::std::mem::size_of::<lldiv_t>() - 16usize];
        ["Alignment of lldiv_t"][::std::mem::align_of::<lldiv_t>() - 8usize];
        ["Offset of field: lldiv_t::quot"][::std::mem::offset_of!(lldiv_t, quot) - 0usize];
        ["Offset of field: lldiv_t::rem"][::std::mem::offset_of!(lldiv_t, rem) - 8usize];
    };
    extern "C" {
        pub static mut __mb_cur_max: ::std::os::raw::c_int;
    }
    pub type malloc_type_id_t = ::std::os::raw::c_ulonglong;
    extern "C" {
        pub fn malloc_type_malloc(
            size: usize,
            type_id: root::malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn malloc_type_calloc(
            count: usize,
            size: usize,
            type_id: root::malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn malloc_type_free(ptr: *mut ::std::os::raw::c_void, type_id: root::malloc_type_id_t);
    }
    extern "C" {
        pub fn malloc_type_realloc(
            ptr: *mut ::std::os::raw::c_void,
            size: usize,
            type_id: root::malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn malloc_type_valloc(
            size: usize,
            type_id: root::malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn malloc_type_aligned_alloc(
            alignment: usize,
            size: usize,
            type_id: root::malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn malloc_type_posix_memalign(
            memptr: *mut *mut ::std::os::raw::c_void,
            alignment: usize,
            size: usize,
            type_id: root::malloc_type_id_t,
        ) -> ::std::os::raw::c_int;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _malloc_zone_t {
        _unused: [u8; 0],
    }
    pub type malloc_zone_t = root::_malloc_zone_t;
    extern "C" {
        pub fn malloc_type_zone_malloc(
            zone: *mut root::malloc_zone_t,
            size: usize,
            type_id: root::malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn malloc_type_zone_calloc(
            zone: *mut root::malloc_zone_t,
            count: usize,
            size: usize,
            type_id: root::malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn malloc_type_zone_free(
            zone: *mut root::malloc_zone_t,
            ptr: *mut ::std::os::raw::c_void,
            type_id: root::malloc_type_id_t,
        );
    }
    extern "C" {
        pub fn malloc_type_zone_realloc(
            zone: *mut root::malloc_zone_t,
            ptr: *mut ::std::os::raw::c_void,
            size: usize,
            type_id: root::malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn malloc_type_zone_valloc(
            zone: *mut root::malloc_zone_t,
            size: usize,
            type_id: root::malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn malloc_type_zone_memalign(
            zone: *mut root::malloc_zone_t,
            alignment: usize,
            size: usize,
            type_id: root::malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn calloc(__count: usize, __size: usize) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn free(arg1: *mut ::std::os::raw::c_void);
    }
    extern "C" {
        pub fn realloc(
            __ptr: *mut ::std::os::raw::c_void,
            __size: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn reallocf(
            __ptr: *mut ::std::os::raw::c_void,
            __size: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn valloc(arg1: usize) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn aligned_alloc(__alignment: usize, __size: usize) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn posix_memalign(
            __memptr: *mut *mut ::std::os::raw::c_void,
            __alignment: usize,
            __size: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn abort() -> !;
    }
    extern "C" {
        pub fn abs(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn atexit(arg1: ::std::option::Option<unsafe extern "C" fn()>)
            -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn atof(arg1: *const ::std::os::raw::c_char) -> f64;
    }
    extern "C" {
        pub fn atoi(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn atol(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn atoll(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn bsearch(
            __key: *const ::std::os::raw::c_void,
            __base: *const ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            __compar: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *const ::std::os::raw::c_void,
                    arg2: *const ::std::os::raw::c_void,
                ) -> ::std::os::raw::c_int,
            >,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn div(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int) -> root::div_t;
    }
    extern "C" {
        pub fn exit(arg1: ::std::os::raw::c_int) -> !;
    }
    extern "C" {
        pub fn getenv(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn labs(arg1: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn ldiv(arg1: ::std::os::raw::c_long, arg2: ::std::os::raw::c_long) -> root::ldiv_t;
    }
    extern "C" {
        pub fn llabs(arg1: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn lldiv(
            arg1: ::std::os::raw::c_longlong,
            arg2: ::std::os::raw::c_longlong,
        ) -> root::lldiv_t;
    }
    extern "C" {
        pub fn mblen(__s: *const ::std::os::raw::c_char, __n: usize) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mbstowcs(arg1: *mut u32, arg2: *const ::std::os::raw::c_char, arg3: usize) -> usize;
    }
    extern "C" {
        pub fn mbtowc(
            arg1: *mut u32,
            arg2: *const ::std::os::raw::c_char,
            arg3: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn qsort(
            __base: *mut ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            __compar: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *const ::std::os::raw::c_void,
                    arg2: *const ::std::os::raw::c_void,
                ) -> ::std::os::raw::c_int,
            >,
        );
    }
    extern "C" {
        pub fn rand() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn srand(arg1: ::std::os::raw::c_uint);
    }
    extern "C" {
        pub fn strtod(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
        ) -> f64;
    }
    extern "C" {
        pub fn strtof(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
        ) -> f32;
    }
    extern "C" {
        pub fn strtol(
            __str: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn strtold(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
        ) -> f64;
    }
    extern "C" {
        pub fn strtoll(
            __str: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn strtoul(
            __str: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn strtoull(
            __str: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_ulonglong;
    }
    extern "C" {
        pub fn system(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcstombs(arg1: *mut ::std::os::raw::c_char, arg2: *const u32, arg3: usize) -> usize;
    }
    extern "C" {
        pub fn wctomb(arg1: *mut ::std::os::raw::c_char, arg2: u32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn _Exit(arg1: ::std::os::raw::c_int) -> !;
    }
    extern "C" {
        pub fn a64l(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn drand48() -> f64;
    }
    extern "C" {
        pub fn ecvt(
            arg1: f64,
            arg2: ::std::os::raw::c_int,
            arg3: *mut ::std::os::raw::c_int,
            arg4: *mut ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn erand48(arg1: *mut ::std::os::raw::c_ushort) -> f64;
    }
    extern "C" {
        pub fn fcvt(
            arg1: f64,
            arg2: ::std::os::raw::c_int,
            arg3: *mut ::std::os::raw::c_int,
            arg4: *mut ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn gcvt(
            arg1: f64,
            arg2: ::std::os::raw::c_int,
            arg3: *mut ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn getsubopt(
            arg1: *mut *mut ::std::os::raw::c_char,
            arg2: *const *mut ::std::os::raw::c_char,
            arg3: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn grantpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn initstate(
            arg1: ::std::os::raw::c_uint,
            arg2: *mut ::std::os::raw::c_char,
            arg3: usize,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn jrand48(arg1: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn l64a(arg1: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn lcong48(arg1: *mut ::std::os::raw::c_ushort);
    }
    extern "C" {
        pub fn lrand48() -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn mktemp(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn mkstemp(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mrand48() -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn nrand48(arg1: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn posix_openpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ptsname(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn ptsname_r(
            fildes: ::std::os::raw::c_int,
            buffer: *mut ::std::os::raw::c_char,
            buflen: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn putenv(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn random() -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn rand_r(arg1: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}_realpath$DARWIN_EXTSN"]
        pub fn realpath(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn seed48(arg1: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
    }
    extern "C" {
        pub fn setenv(
            __name: *const ::std::os::raw::c_char,
            __value: *const ::std::os::raw::c_char,
            __overwrite: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn setkey(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn setstate(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn srand48(arg1: ::std::os::raw::c_long);
    }
    extern "C" {
        pub fn srandom(arg1: ::std::os::raw::c_uint);
    }
    extern "C" {
        pub fn unlockpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn unsetenv(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    pub type dev_t = root::__darwin_dev_t;
    pub type mode_t = root::__darwin_mode_t;
    extern "C" {
        pub fn arc4random() -> u32;
    }
    extern "C" {
        pub fn arc4random_addrandom(
            arg1: *mut ::std::os::raw::c_uchar,
            arg2: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn arc4random_buf(__buf: *mut ::std::os::raw::c_void, __nbytes: usize);
    }
    extern "C" {
        pub fn arc4random_stir();
    }
    extern "C" {
        pub fn arc4random_uniform(__upper_bound: u32) -> u32;
    }
    extern "C" {
        pub fn atexit_b(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn bsearch_b(
            __key: *const ::std::os::raw::c_void,
            __base: *const ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            __compar: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn cgetcap(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn cgetclose() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn cgetent(
            arg1: *mut *mut ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
            arg3: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn cgetfirst(
            arg1: *mut *mut ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn cgetmatch(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn cgetnext(
            arg1: *mut *mut ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn cgetnum(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: *mut ::std::os::raw::c_long,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn cgetset(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn cgetstr(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn cgetustr(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn daemon(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn devname(arg1: root::dev_t, arg2: root::mode_t) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn devname_r(
            arg1: root::dev_t,
            arg2: root::mode_t,
            buf: *mut ::std::os::raw::c_char,
            len: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn getbsize(
            arg1: *mut ::std::os::raw::c_int,
            arg2: *mut ::std::os::raw::c_long,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn getloadavg(arg1: *mut f64, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn getprogname() -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn setprogname(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn heapsort(
            __base: *mut ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            __compar: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *const ::std::os::raw::c_void,
                    arg2: *const ::std::os::raw::c_void,
                ) -> ::std::os::raw::c_int,
            >,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn heapsort_b(
            __base: *mut ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            __compar: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mergesort(
            __base: *mut ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            __compar: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *const ::std::os::raw::c_void,
                    arg2: *const ::std::os::raw::c_void,
                ) -> ::std::os::raw::c_int,
            >,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mergesort_b(
            __base: *mut ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            __compar: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn psort(
            __base: *mut ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            __compar: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *const ::std::os::raw::c_void,
                    arg2: *const ::std::os::raw::c_void,
                ) -> ::std::os::raw::c_int,
            >,
        );
    }
    extern "C" {
        pub fn psort_b(
            __base: *mut ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            __compar: *mut ::std::os::raw::c_void,
        );
    }
    extern "C" {
        pub fn psort_r(
            __base: *mut ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            arg1: *mut ::std::os::raw::c_void,
            __compar: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::std::os::raw::c_void,
                    arg2: *const ::std::os::raw::c_void,
                    arg3: *const ::std::os::raw::c_void,
                ) -> ::std::os::raw::c_int,
            >,
        );
    }
    extern "C" {
        pub fn qsort_b(
            __base: *mut ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            __compar: *mut ::std::os::raw::c_void,
        );
    }
    extern "C" {
        pub fn qsort_r(
            __base: *mut ::std::os::raw::c_void,
            __nel: usize,
            __width: usize,
            arg1: *mut ::std::os::raw::c_void,
            __compar: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::std::os::raw::c_void,
                    arg2: *const ::std::os::raw::c_void,
                    arg3: *const ::std::os::raw::c_void,
                ) -> ::std::os::raw::c_int,
            >,
        );
    }
    extern "C" {
        pub fn radixsort(
            __base: *mut *const ::std::os::raw::c_uchar,
            __nel: ::std::os::raw::c_int,
            __table: *const ::std::os::raw::c_uchar,
            __endbyte: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn rpmatch(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn sradixsort(
            __base: *mut *const ::std::os::raw::c_uchar,
            __nel: ::std::os::raw::c_int,
            __table: *const ::std::os::raw::c_uchar,
            __endbyte: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn sranddev();
    }
    extern "C" {
        pub fn srandomdev();
    }
    extern "C" {
        pub fn strtonum(
            __numstr: *const ::std::os::raw::c_char,
            __minval: ::std::os::raw::c_longlong,
            __maxval: ::std::os::raw::c_longlong,
            __errstrp: *mut *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn strtoq(
            __str: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn strtouq(
            __str: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_ulonglong;
    }
    extern "C" {
        pub static mut suboptarg: *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn __assert_rtn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
            arg4: *const ::std::os::raw::c_char,
        ) -> !;
    }
    extern "C" {
        pub fn memchr(
            __s: *const ::std::os::raw::c_void,
            __c: ::std::os::raw::c_int,
            __n: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn memcmp(
            __s1: *const ::std::os::raw::c_void,
            __s2: *const ::std::os::raw::c_void,
            __n: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn memcpy(
            __dst: *mut ::std::os::raw::c_void,
            __src: *const ::std::os::raw::c_void,
            __n: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn memmove(
            __dst: *mut ::std::os::raw::c_void,
            __src: *const ::std::os::raw::c_void,
            __len: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn memset(
            __b: *mut ::std::os::raw::c_void,
            __c: ::std::os::raw::c_int,
            __len: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn strcat(
            __s1: *mut ::std::os::raw::c_char,
            __s2: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strchr(
            __s: *const ::std::os::raw::c_char,
            __c: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strcmp(
            __s1: *const ::std::os::raw::c_char,
            __s2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strcoll(
            __s1: *const ::std::os::raw::c_char,
            __s2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strcpy(
            __dst: *mut ::std::os::raw::c_char,
            __src: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strcspn(
            __s: *const ::std::os::raw::c_char,
            __charset: *const ::std::os::raw::c_char,
        ) -> usize;
    }
    extern "C" {
        pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strlen(__s: *const ::std::os::raw::c_char) -> usize;
    }
    extern "C" {
        pub fn strncat(
            __s1: *mut ::std::os::raw::c_char,
            __s2: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strncmp(
            __s1: *const ::std::os::raw::c_char,
            __s2: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strncpy(
            __dst: *mut ::std::os::raw::c_char,
            __src: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strpbrk(
            __s: *const ::std::os::raw::c_char,
            __charset: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strrchr(
            __s: *const ::std::os::raw::c_char,
            __c: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strspn(
            __s: *const ::std::os::raw::c_char,
            __charset: *const ::std::os::raw::c_char,
        ) -> usize;
    }
    extern "C" {
        pub fn strstr(
            __big: *const ::std::os::raw::c_char,
            __little: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strtok(
            __str: *mut ::std::os::raw::c_char,
            __sep: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strxfrm(
            __s1: *mut ::std::os::raw::c_char,
            __s2: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn strtok_r(
            __str: *mut ::std::os::raw::c_char,
            __sep: *const ::std::os::raw::c_char,
            __lasts: *mut *mut ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strerror_r(
            __errnum: ::std::os::raw::c_int,
            __strerrbuf: *mut ::std::os::raw::c_char,
            __buflen: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strdup(__s1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn memccpy(
            __dst: *mut ::std::os::raw::c_void,
            __src: *const ::std::os::raw::c_void,
            __c: ::std::os::raw::c_int,
            __n: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn stpcpy(
            __dst: *mut ::std::os::raw::c_char,
            __src: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn stpncpy(
            __dst: *mut ::std::os::raw::c_char,
            __src: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strndup(
            __s1: *const ::std::os::raw::c_char,
            __n: usize,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strnlen(__s1: *const ::std::os::raw::c_char, __n: usize) -> usize;
    }
    extern "C" {
        pub fn strsignal(__sig: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn memmem(
            __big: *const ::std::os::raw::c_void,
            __big_len: usize,
            __little: *const ::std::os::raw::c_void,
            __little_len: usize,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn memset_pattern4(
            __b: *mut ::std::os::raw::c_void,
            __pattern4: *const ::std::os::raw::c_void,
            __len: usize,
        );
    }
    extern "C" {
        pub fn memset_pattern8(
            __b: *mut ::std::os::raw::c_void,
            __pattern8: *const ::std::os::raw::c_void,
            __len: usize,
        );
    }
    extern "C" {
        pub fn memset_pattern16(
            __b: *mut ::std::os::raw::c_void,
            __pattern16: *const ::std::os::raw::c_void,
            __len: usize,
        );
    }
    extern "C" {
        pub fn strcasestr(
            __big: *const ::std::os::raw::c_char,
            __little: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strnstr(
            __big: *const ::std::os::raw::c_char,
            __little: *const ::std::os::raw::c_char,
            __len: usize,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strlcat(
            __dst: *mut ::std::os::raw::c_char,
            __source: *const ::std::os::raw::c_char,
            __size: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn strlcpy(
            __dst: *mut ::std::os::raw::c_char,
            __source: *const ::std::os::raw::c_char,
            __size: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn strmode(__mode: ::std::os::raw::c_int, __bp: *mut ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn strsep(
            __stringp: *mut *mut ::std::os::raw::c_char,
            __delim: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn swab(
            arg1: *const ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: isize,
        );
    }
    extern "C" {
        pub fn timingsafe_bcmp(
            __b1: *const ::std::os::raw::c_void,
            __b2: *const ::std::os::raw::c_void,
            __len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strsignal_r(
            __sig: ::std::os::raw::c_int,
            __strsignalbuf: *mut ::std::os::raw::c_char,
            __buflen: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn bcmp(
            arg1: *const ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
            arg3: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn bcopy(
            arg1: *const ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: usize,
        );
    }
    extern "C" {
        pub fn bzero(arg1: *mut ::std::os::raw::c_void, arg2: usize);
    }
    extern "C" {
        pub fn index(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn rindex(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn ffs(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strcasecmp(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strncasecmp(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ffsl(arg1: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ffsll(arg1: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fls(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn flsl(arg1: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn flsll(arg1: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
    }
    pub type clock_t = root::__darwin_clock_t;
    pub type time_t = root::__darwin_time_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct timespec {
        pub tv_sec: root::__darwin_time_t,
        pub tv_nsec: ::std::os::raw::c_long,
    }
    const _: () = {
        ["Size of timespec"][::std::mem::size_of::<timespec>() - 16usize];
        ["Alignment of timespec"][::std::mem::align_of::<timespec>() - 8usize];
        ["Offset of field: timespec::tv_sec"][::std::mem::offset_of!(timespec, tv_sec) - 0usize];
        ["Offset of field: timespec::tv_nsec"][::std::mem::offset_of!(timespec, tv_nsec) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct tm {
        pub tm_sec: ::std::os::raw::c_int,
        pub tm_min: ::std::os::raw::c_int,
        pub tm_hour: ::std::os::raw::c_int,
        pub tm_mday: ::std::os::raw::c_int,
        pub tm_mon: ::std::os::raw::c_int,
        pub tm_year: ::std::os::raw::c_int,
        pub tm_wday: ::std::os::raw::c_int,
        pub tm_yday: ::std::os::raw::c_int,
        pub tm_isdst: ::std::os::raw::c_int,
        pub tm_gmtoff: ::std::os::raw::c_long,
        pub tm_zone: *mut ::std::os::raw::c_char,
    }
    const _: () = {
        ["Size of tm"][::std::mem::size_of::<tm>() - 56usize];
        ["Alignment of tm"][::std::mem::align_of::<tm>() - 8usize];
        ["Offset of field: tm::tm_sec"][::std::mem::offset_of!(tm, tm_sec) - 0usize];
        ["Offset of field: tm::tm_min"][::std::mem::offset_of!(tm, tm_min) - 4usize];
        ["Offset of field: tm::tm_hour"][::std::mem::offset_of!(tm, tm_hour) - 8usize];
        ["Offset of field: tm::tm_mday"][::std::mem::offset_of!(tm, tm_mday) - 12usize];
        ["Offset of field: tm::tm_mon"][::std::mem::offset_of!(tm, tm_mon) - 16usize];
        ["Offset of field: tm::tm_year"][::std::mem::offset_of!(tm, tm_year) - 20usize];
        ["Offset of field: tm::tm_wday"][::std::mem::offset_of!(tm, tm_wday) - 24usize];
        ["Offset of field: tm::tm_yday"][::std::mem::offset_of!(tm, tm_yday) - 28usize];
        ["Offset of field: tm::tm_isdst"][::std::mem::offset_of!(tm, tm_isdst) - 32usize];
        ["Offset of field: tm::tm_gmtoff"][::std::mem::offset_of!(tm, tm_gmtoff) - 40usize];
        ["Offset of field: tm::tm_zone"][::std::mem::offset_of!(tm, tm_zone) - 48usize];
    };
    extern "C" {
        pub static mut tzname: [*mut ::std::os::raw::c_char; 0usize];
    }
    extern "C" {
        pub static mut getdate_err: ::std::os::raw::c_int;
    }
    extern "C" {
        pub static mut timezone: ::std::os::raw::c_long;
    }
    extern "C" {
        pub static mut daylight: ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn asctime(arg1: *const root::tm) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn clock() -> root::clock_t;
    }
    extern "C" {
        pub fn ctime(arg1: *const root::time_t) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn difftime(arg1: root::time_t, arg2: root::time_t) -> f64;
    }
    extern "C" {
        pub fn getdate(arg1: *const ::std::os::raw::c_char) -> *mut root::tm;
    }
    extern "C" {
        pub fn gmtime(arg1: *const root::time_t) -> *mut root::tm;
    }
    extern "C" {
        pub fn localtime(arg1: *const root::time_t) -> *mut root::tm;
    }
    extern "C" {
        pub fn mktime(arg1: *mut root::tm) -> root::time_t;
    }
    extern "C" {
        pub fn strftime(
            arg1: *mut ::std::os::raw::c_char,
            arg2: usize,
            arg3: *const ::std::os::raw::c_char,
            arg4: *const root::tm,
        ) -> usize;
    }
    extern "C" {
        pub fn strptime(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: *mut root::tm,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn time(arg1: *mut root::time_t) -> root::time_t;
    }
    extern "C" {
        pub fn tzset();
    }
    extern "C" {
        pub fn asctime_r(
            arg1: *const root::tm,
            arg2: *mut ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn ctime_r(
            arg1: *const root::time_t,
            arg2: *mut ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn gmtime_r(arg1: *const root::time_t, arg2: *mut root::tm) -> *mut root::tm;
    }
    extern "C" {
        pub fn localtime_r(arg1: *const root::time_t, arg2: *mut root::tm) -> *mut root::tm;
    }
    extern "C" {
        pub fn posix2time(arg1: root::time_t) -> root::time_t;
    }
    extern "C" {
        pub fn tzsetwall();
    }
    extern "C" {
        pub fn time2posix(arg1: root::time_t) -> root::time_t;
    }
    extern "C" {
        pub fn timelocal(arg1: *mut root::tm) -> root::time_t;
    }
    extern "C" {
        pub fn timegm(arg1: *mut root::tm) -> root::time_t;
    }
    extern "C" {
        pub fn nanosleep(
            __rqtp: *const root::timespec,
            __rmtp: *mut root::timespec,
        ) -> ::std::os::raw::c_int;
    }
    pub const clockid_t__CLOCK_REALTIME: root::clockid_t = 0;
    pub const clockid_t__CLOCK_MONOTONIC: root::clockid_t = 6;
    pub const clockid_t__CLOCK_MONOTONIC_RAW: root::clockid_t = 4;
    pub const clockid_t__CLOCK_MONOTONIC_RAW_APPROX: root::clockid_t = 5;
    pub const clockid_t__CLOCK_UPTIME_RAW: root::clockid_t = 8;
    pub const clockid_t__CLOCK_UPTIME_RAW_APPROX: root::clockid_t = 9;
    pub const clockid_t__CLOCK_PROCESS_CPUTIME_ID: root::clockid_t = 12;
    pub const clockid_t__CLOCK_THREAD_CPUTIME_ID: root::clockid_t = 16;
    pub type clockid_t = ::std::os::raw::c_uint;
    extern "C" {
        pub fn clock_getres(
            __clock_id: root::clockid_t,
            __res: *mut root::timespec,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn clock_gettime(
            __clock_id: root::clockid_t,
            __tp: *mut root::timespec,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn clock_gettime_nsec_np(__clock_id: root::clockid_t) -> root::__uint64_t;
    }
    extern "C" {
        pub fn clock_settime(
            __clock_id: root::clockid_t,
            __tp: *const root::timespec,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn timespec_get(
            ts: *mut root::timespec,
            base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn __error() -> *mut ::std::os::raw::c_int;
    }
    pub type float_t = f32;
    pub type double_t = f64;
    extern "C" {
        pub fn __math_errhandling() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn __fpclassifyf(arg1: f32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn __fpclassifyd(arg1: f64) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn __fpclassifyl(arg1: f64) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn acosf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn acos(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn acosl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn asinf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn asin(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn asinl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn atanf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn atan(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn atanl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn atan2f(arg1: f32, arg2: f32) -> f32;
    }
    extern "C" {
        pub fn atan2(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn atan2l(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn cosf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn cos(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn cosl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn sinf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn sin(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn sinl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn tanf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn tan(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn tanl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn acoshf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn acosh(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn acoshl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn asinhf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn asinh(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn asinhl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn atanhf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn atanh(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn atanhl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn coshf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn cosh(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn coshl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn sinhf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn sinh(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn sinhl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn tanhf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn tanh(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn tanhl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn expf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn exp(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn expl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn exp2f(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn exp2(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn exp2l(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn expm1f(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn expm1(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn expm1l(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn logf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn log(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn logl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn log10f(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn log10(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn log10l(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn log2f(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn log2(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn log2l(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn log1pf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn log1p(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn log1pl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn logbf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn logb(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn logbl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn modff(arg1: f32, arg2: *mut f32) -> f32;
    }
    extern "C" {
        pub fn modf(arg1: f64, arg2: *mut f64) -> f64;
    }
    extern "C" {
        pub fn modfl(arg1: f64, arg2: *mut f64) -> f64;
    }
    extern "C" {
        pub fn ldexpf(arg1: f32, arg2: ::std::os::raw::c_int) -> f32;
    }
    extern "C" {
        pub fn ldexp(arg1: f64, arg2: ::std::os::raw::c_int) -> f64;
    }
    extern "C" {
        pub fn ldexpl(arg1: f64, arg2: ::std::os::raw::c_int) -> f64;
    }
    extern "C" {
        pub fn frexpf(arg1: f32, arg2: *mut ::std::os::raw::c_int) -> f32;
    }
    extern "C" {
        pub fn frexp(arg1: f64, arg2: *mut ::std::os::raw::c_int) -> f64;
    }
    extern "C" {
        pub fn frexpl(arg1: f64, arg2: *mut ::std::os::raw::c_int) -> f64;
    }
    extern "C" {
        pub fn ilogbf(arg1: f32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ilogb(arg1: f64) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ilogbl(arg1: f64) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn scalbnf(arg1: f32, arg2: ::std::os::raw::c_int) -> f32;
    }
    extern "C" {
        pub fn scalbn(arg1: f64, arg2: ::std::os::raw::c_int) -> f64;
    }
    extern "C" {
        pub fn scalbnl(arg1: f64, arg2: ::std::os::raw::c_int) -> f64;
    }
    extern "C" {
        pub fn scalblnf(arg1: f32, arg2: ::std::os::raw::c_long) -> f32;
    }
    extern "C" {
        pub fn scalbln(arg1: f64, arg2: ::std::os::raw::c_long) -> f64;
    }
    extern "C" {
        pub fn scalblnl(arg1: f64, arg2: ::std::os::raw::c_long) -> f64;
    }
    extern "C" {
        pub fn fabsf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn fabs(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn fabsl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn cbrtf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn cbrt(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn cbrtl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn hypotf(arg1: f32, arg2: f32) -> f32;
    }
    extern "C" {
        pub fn hypot(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn hypotl(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn powf(arg1: f32, arg2: f32) -> f32;
    }
    extern "C" {
        pub fn pow(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn powl(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn sqrtf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn sqrt(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn sqrtl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn erff(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn erf(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn erfl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn erfcf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn erfc(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn erfcl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn lgammaf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn lgamma(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn lgammal(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn tgammaf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn tgamma(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn tgammal(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn ceilf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn ceil(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn ceill(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn floorf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn floor(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn floorl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn nearbyintf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn nearbyint(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn nearbyintl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn rintf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn rint(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn rintl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn lrintf(arg1: f32) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn lrint(arg1: f64) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn lrintl(arg1: f64) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn roundf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn round(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn roundl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn lroundf(arg1: f32) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn lround(arg1: f64) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn lroundl(arg1: f64) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn llrintf(arg1: f32) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn llrint(arg1: f64) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn llrintl(arg1: f64) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn llroundf(arg1: f32) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn llround(arg1: f64) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn llroundl(arg1: f64) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn truncf(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn trunc(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn truncl(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn fmodf(arg1: f32, arg2: f32) -> f32;
    }
    extern "C" {
        pub fn fmod(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn fmodl(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn remainderf(arg1: f32, arg2: f32) -> f32;
    }
    extern "C" {
        pub fn remainder(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn remainderl(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn remquof(arg1: f32, arg2: f32, arg3: *mut ::std::os::raw::c_int) -> f32;
    }
    extern "C" {
        pub fn remquo(arg1: f64, arg2: f64, arg3: *mut ::std::os::raw::c_int) -> f64;
    }
    extern "C" {
        pub fn remquol(arg1: f64, arg2: f64, arg3: *mut ::std::os::raw::c_int) -> f64;
    }
    extern "C" {
        pub fn copysignf(arg1: f32, arg2: f32) -> f32;
    }
    extern "C" {
        pub fn copysign(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn copysignl(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn nanf(arg1: *const ::std::os::raw::c_char) -> f32;
    }
    extern "C" {
        pub fn nan(arg1: *const ::std::os::raw::c_char) -> f64;
    }
    extern "C" {
        pub fn nanl(arg1: *const ::std::os::raw::c_char) -> f64;
    }
    extern "C" {
        pub fn nextafterf(arg1: f32, arg2: f32) -> f32;
    }
    extern "C" {
        pub fn nextafter(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn nextafterl(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn nexttoward(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn nexttowardf(arg1: f32, arg2: f64) -> f32;
    }
    extern "C" {
        pub fn nexttowardl(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn fdimf(arg1: f32, arg2: f32) -> f32;
    }
    extern "C" {
        pub fn fdim(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn fdiml(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn fmaxf(arg1: f32, arg2: f32) -> f32;
    }
    extern "C" {
        pub fn fmax(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn fmaxl(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn fminf(arg1: f32, arg2: f32) -> f32;
    }
    extern "C" {
        pub fn fmin(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn fminl(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn fmaf(arg1: f32, arg2: f32, arg3: f32) -> f32;
    }
    extern "C" {
        pub fn fma(arg1: f64, arg2: f64, arg3: f64) -> f64;
    }
    extern "C" {
        pub fn fmal(arg1: f64, arg2: f64, arg3: f64) -> f64;
    }
    extern "C" {
        pub fn __exp10f(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn __exp10(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn __cospif(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn __cospi(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn __sinpif(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn __sinpi(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn __tanpif(arg1: f32) -> f32;
    }
    extern "C" {
        pub fn __tanpi(arg1: f64) -> f64;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __float2 {
        pub __sinval: f32,
        pub __cosval: f32,
    }
    const _: () = {
        ["Size of __float2"][::std::mem::size_of::<__float2>() - 8usize];
        ["Alignment of __float2"][::std::mem::align_of::<__float2>() - 4usize];
        ["Offset of field: __float2::__sinval"]
            [::std::mem::offset_of!(__float2, __sinval) - 0usize];
        ["Offset of field: __float2::__cosval"]
            [::std::mem::offset_of!(__float2, __cosval) - 4usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __double2 {
        pub __sinval: f64,
        pub __cosval: f64,
    }
    const _: () = {
        ["Size of __double2"][::std::mem::size_of::<__double2>() - 16usize];
        ["Alignment of __double2"][::std::mem::align_of::<__double2>() - 8usize];
        ["Offset of field: __double2::__sinval"]
            [::std::mem::offset_of!(__double2, __sinval) - 0usize];
        ["Offset of field: __double2::__cosval"]
            [::std::mem::offset_of!(__double2, __cosval) - 8usize];
    };
    extern "C" {
        pub fn __sincosf_stret(arg1: f32) -> root::__float2;
    }
    extern "C" {
        pub fn __sincos_stret(arg1: f64) -> root::__double2;
    }
    extern "C" {
        pub fn __sincospif_stret(arg1: f32) -> root::__float2;
    }
    extern "C" {
        pub fn __sincospi_stret(arg1: f64) -> root::__double2;
    }
    extern "C" {
        pub fn j0(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn j1(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn jn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn y0(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn y1(arg1: f64) -> f64;
    }
    extern "C" {
        pub fn yn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
    }
    extern "C" {
        pub fn scalb(arg1: f64, arg2: f64) -> f64;
    }
    extern "C" {
        pub static mut signgam: ::std::os::raw::c_int;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sched_param {
        pub sched_priority: ::std::os::raw::c_int,
        pub __opaque: [::std::os::raw::c_char; 4usize],
    }
    const _: () = {
        ["Size of sched_param"][::std::mem::size_of::<sched_param>() - 8usize];
        ["Alignment of sched_param"][::std::mem::align_of::<sched_param>() - 4usize];
        ["Offset of field: sched_param::sched_priority"]
            [::std::mem::offset_of!(sched_param, sched_priority) - 0usize];
        ["Offset of field: sched_param::__opaque"]
            [::std::mem::offset_of!(sched_param, __opaque) - 4usize];
    };
    extern "C" {
        pub fn sched_yield() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn sched_get_priority_min(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn sched_get_priority_max(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    pub type pthread_cond_t = root::__darwin_pthread_cond_t;
    pub type pthread_condattr_t = root::__darwin_pthread_condattr_t;
    pub type pthread_key_t = root::__darwin_pthread_key_t;
    pub type pthread_mutex_t = root::__darwin_pthread_mutex_t;
    pub type pthread_mutexattr_t = root::__darwin_pthread_mutexattr_t;
    pub type pthread_once_t = root::__darwin_pthread_once_t;
    pub type pthread_rwlock_t = root::__darwin_pthread_rwlock_t;
    pub type pthread_rwlockattr_t = root::__darwin_pthread_rwlockattr_t;
    pub type pthread_t = root::__darwin_pthread_t;
    pub const qos_class_t_QOS_CLASS_USER_INTERACTIVE: root::qos_class_t = 33;
    pub const qos_class_t_QOS_CLASS_USER_INITIATED: root::qos_class_t = 25;
    pub const qos_class_t_QOS_CLASS_DEFAULT: root::qos_class_t = 21;
    pub const qos_class_t_QOS_CLASS_UTILITY: root::qos_class_t = 17;
    pub const qos_class_t_QOS_CLASS_BACKGROUND: root::qos_class_t = 9;
    pub const qos_class_t_QOS_CLASS_UNSPECIFIED: root::qos_class_t = 0;
    pub type qos_class_t = ::std::os::raw::c_uint;
    extern "C" {
        pub fn qos_class_self() -> root::qos_class_t;
    }
    extern "C" {
        pub fn qos_class_main() -> root::qos_class_t;
    }
    extern "C" {
        pub fn pthread_attr_set_qos_class_np(
            __attr: *mut root::pthread_attr_t,
            __qos_class: root::qos_class_t,
            __relative_priority: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_get_qos_class_np(
            __attr: *mut root::pthread_attr_t,
            __qos_class: *mut root::qos_class_t,
            __relative_priority: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_set_qos_class_self_np(
            __qos_class: root::qos_class_t,
            __relative_priority: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_get_qos_class_np(
            __pthread: root::pthread_t,
            __qos_class: *mut root::qos_class_t,
            __relative_priority: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct pthread_override_s {
        _unused: [u8; 0],
    }
    pub type pthread_override_t = *mut root::pthread_override_s;
    extern "C" {
        pub fn pthread_override_qos_class_start_np(
            __pthread: root::pthread_t,
            __qos_class: root::qos_class_t,
            __relative_priority: ::std::os::raw::c_int,
        ) -> root::pthread_override_t;
    }
    extern "C" {
        pub fn pthread_override_qos_class_end_np(
            __override: root::pthread_override_t,
        ) -> ::std::os::raw::c_int;
    }
    pub type mach_port_t = root::__darwin_mach_port_t;
    extern "C" {
        pub fn pthread_atfork(
            arg1: ::std::option::Option<unsafe extern "C" fn()>,
            arg2: ::std::option::Option<unsafe extern "C" fn()>,
            arg3: ::std::option::Option<unsafe extern "C" fn()>,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_destroy(arg1: *mut root::pthread_attr_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_getdetachstate(
            arg1: *const root::pthread_attr_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_getguardsize(
            arg1: *const root::pthread_attr_t,
            arg2: *mut usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_getinheritsched(
            arg1: *const root::pthread_attr_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_getschedparam(
            arg1: *const root::pthread_attr_t,
            arg2: *mut root::sched_param,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_getschedpolicy(
            arg1: *const root::pthread_attr_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_getscope(
            arg1: *const root::pthread_attr_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_getstack(
            arg1: *const root::pthread_attr_t,
            arg2: *mut *mut ::std::os::raw::c_void,
            arg3: *mut usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_getstackaddr(
            arg1: *const root::pthread_attr_t,
            arg2: *mut *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_getstacksize(
            arg1: *const root::pthread_attr_t,
            arg2: *mut usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_init(arg1: *mut root::pthread_attr_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_setdetachstate(
            arg1: *mut root::pthread_attr_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_setguardsize(
            arg1: *mut root::pthread_attr_t,
            arg2: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_setinheritsched(
            arg1: *mut root::pthread_attr_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_setschedparam(
            arg1: *mut root::pthread_attr_t,
            arg2: *const root::sched_param,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_setschedpolicy(
            arg1: *mut root::pthread_attr_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_setscope(
            arg1: *mut root::pthread_attr_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_setstack(
            arg1: *mut root::pthread_attr_t,
            arg2: *mut ::std::os::raw::c_void,
            arg3: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_setstackaddr(
            arg1: *mut root::pthread_attr_t,
            arg2: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_attr_setstacksize(
            arg1: *mut root::pthread_attr_t,
            arg2: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_cancel(arg1: root::pthread_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_cond_broadcast(arg1: *mut root::pthread_cond_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_cond_destroy(arg1: *mut root::pthread_cond_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_cond_init(
            arg1: *mut root::pthread_cond_t,
            arg2: *const root::pthread_condattr_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_cond_signal(arg1: *mut root::pthread_cond_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_cond_timedwait(
            arg1: *mut root::pthread_cond_t,
            arg2: *mut root::pthread_mutex_t,
            arg3: *const root::timespec,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_cond_wait(
            arg1: *mut root::pthread_cond_t,
            arg2: *mut root::pthread_mutex_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_condattr_destroy(
            arg1: *mut root::pthread_condattr_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_condattr_init(arg1: *mut root::pthread_condattr_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_condattr_getpshared(
            arg1: *const root::pthread_condattr_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_condattr_setpshared(
            arg1: *mut root::pthread_condattr_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_create(
            arg1: *mut root::pthread_t,
            arg2: *const root::pthread_attr_t,
            arg3: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::std::os::raw::c_void,
                ) -> *mut ::std::os::raw::c_void,
            >,
            arg4: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_detach(arg1: root::pthread_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_equal(arg1: root::pthread_t, arg2: root::pthread_t)
            -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_exit(arg1: *mut ::std::os::raw::c_void) -> !;
    }
    extern "C" {
        pub fn pthread_getconcurrency() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_getschedparam(
            arg1: root::pthread_t,
            arg2: *mut ::std::os::raw::c_int,
            arg3: *mut root::sched_param,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_getspecific(arg1: root::pthread_key_t) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn pthread_join(
            arg1: root::pthread_t,
            arg2: *mut *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_key_create(
            arg1: *mut root::pthread_key_t,
            arg2: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_key_delete(arg1: root::pthread_key_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutex_destroy(arg1: *mut root::pthread_mutex_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutex_getprioceiling(
            arg1: *const root::pthread_mutex_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutex_init(
            arg1: *mut root::pthread_mutex_t,
            arg2: *const root::pthread_mutexattr_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutex_lock(arg1: *mut root::pthread_mutex_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutex_setprioceiling(
            arg1: *mut root::pthread_mutex_t,
            arg2: ::std::os::raw::c_int,
            arg3: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutex_trylock(arg1: *mut root::pthread_mutex_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutex_unlock(arg1: *mut root::pthread_mutex_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_destroy(
            arg1: *mut root::pthread_mutexattr_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_getprioceiling(
            arg1: *const root::pthread_mutexattr_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_getprotocol(
            arg1: *const root::pthread_mutexattr_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_getpshared(
            arg1: *const root::pthread_mutexattr_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_gettype(
            arg1: *const root::pthread_mutexattr_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_getpolicy_np(
            arg1: *const root::pthread_mutexattr_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_init(
            arg1: *mut root::pthread_mutexattr_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_setprioceiling(
            arg1: *mut root::pthread_mutexattr_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_setprotocol(
            arg1: *mut root::pthread_mutexattr_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_setpshared(
            arg1: *mut root::pthread_mutexattr_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_settype(
            arg1: *mut root::pthread_mutexattr_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mutexattr_setpolicy_np(
            arg1: *mut root::pthread_mutexattr_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_once(
            arg1: *mut root::pthread_once_t,
            arg2: ::std::option::Option<unsafe extern "C" fn()>,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_rwlock_destroy(arg1: *mut root::pthread_rwlock_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_rwlock_init(
            arg1: *mut root::pthread_rwlock_t,
            arg2: *const root::pthread_rwlockattr_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_rwlock_rdlock(arg1: *mut root::pthread_rwlock_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_rwlock_tryrdlock(arg1: *mut root::pthread_rwlock_t)
            -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_rwlock_trywrlock(arg1: *mut root::pthread_rwlock_t)
            -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_rwlock_wrlock(arg1: *mut root::pthread_rwlock_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_rwlock_unlock(arg1: *mut root::pthread_rwlock_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_rwlockattr_destroy(
            arg1: *mut root::pthread_rwlockattr_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_rwlockattr_getpshared(
            arg1: *const root::pthread_rwlockattr_t,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_rwlockattr_init(
            arg1: *mut root::pthread_rwlockattr_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_rwlockattr_setpshared(
            arg1: *mut root::pthread_rwlockattr_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_self() -> root::pthread_t;
    }
    extern "C" {
        pub fn pthread_setcancelstate(
            arg1: ::std::os::raw::c_int,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_setcanceltype(
            arg1: ::std::os::raw::c_int,
            arg2: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_setconcurrency(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_setschedparam(
            arg1: root::pthread_t,
            arg2: ::std::os::raw::c_int,
            arg3: *const root::sched_param,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_setspecific(
            arg1: root::pthread_key_t,
            arg2: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_testcancel();
    }
    extern "C" {
        pub fn pthread_is_threaded_np() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_threadid_np(
            arg1: root::pthread_t,
            arg2: *mut root::__uint64_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_getname_np(
            arg1: root::pthread_t,
            arg2: *mut ::std::os::raw::c_char,
            arg3: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_setname_np(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_main_np() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_mach_thread_np(arg1: root::pthread_t) -> root::mach_port_t;
    }
    extern "C" {
        pub fn pthread_get_stacksize_np(arg1: root::pthread_t) -> usize;
    }
    extern "C" {
        pub fn pthread_get_stackaddr_np(arg1: root::pthread_t) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn pthread_cond_signal_thread_np(
            arg1: *mut root::pthread_cond_t,
            arg2: root::pthread_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_cond_timedwait_relative_np(
            arg1: *mut root::pthread_cond_t,
            arg2: *mut root::pthread_mutex_t,
            arg3: *const root::timespec,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_create_suspended_np(
            arg1: *mut root::pthread_t,
            arg2: *const root::pthread_attr_t,
            arg3: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::std::os::raw::c_void,
                ) -> *mut ::std::os::raw::c_void,
            >,
            arg4: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_kill(
            arg1: root::pthread_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_from_mach_thread_np(arg1: root::mach_port_t) -> root::pthread_t;
    }
    extern "C" {
        pub fn pthread_sigmask(
            arg1: ::std::os::raw::c_int,
            arg2: *const root::sigset_t,
            arg3: *mut root::sigset_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_yield_np();
    }
    extern "C" {
        pub fn pthread_jit_write_protect_np(enabled: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn pthread_jit_write_protect_supported_np() -> ::std::os::raw::c_int;
    }
    pub type pthread_jit_write_callback_t = ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >;
    extern "C" {
        pub fn pthread_jit_write_with_callback_np(
            callback: root::pthread_jit_write_callback_t,
            ctx: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pthread_jit_write_freeze_callbacks_np();
    }
    extern "C" {
        pub fn pthread_cpu_number_np(cpu_number_out: *mut usize) -> ::std::os::raw::c_int;
    }
    pub type __libcpp_timespec_t = root::timespec;
    pub mod swift {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod impl_ {
            #[allow(unused_imports)]
            use self::super::super::super::root;
        }
        #[doc = " A simple wrapper for std::atomic that provides the most important\n interfaces and fixes the API bug where all of the orderings default\n to sequentially-consistent.\n\n It also sometimes uses a different implementation in cases where\n std::atomic has made unfortunate choices; our uses of this broadly\n don't have the ABI-compatibility issues that std::atomic faces."]
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct atomic {
            pub _address: u8,
        }
        pub mod runtime {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            pub mod backtrace {
                #[allow(unused_imports)]
                use self::super::super::super::super::root;
                #[repr(C)]
                #[derive(Debug, Copy, Clone)]
                pub struct CrashInfo {
                    pub crashing_thread: u64,
                    pub signal: u64,
                    pub fault_address: u64,
                    pub mctx: u64,
                }
                const _: () = {
                    ["Size of CrashInfo"][::std::mem::size_of::<CrashInfo>() - 32usize];
                    ["Alignment of CrashInfo"][::std::mem::align_of::<CrashInfo>() - 8usize];
                    ["Offset of field: CrashInfo::crashing_thread"]
                        [::std::mem::offset_of!(CrashInfo, crashing_thread) - 0usize];
                    ["Offset of field: CrashInfo::signal"]
                        [::std::mem::offset_of!(CrashInfo, signal) - 8usize];
                    ["Offset of field: CrashInfo::fault_address"]
                        [::std::mem::offset_of!(CrashInfo, fault_address) - 16usize];
                    ["Offset of field: CrashInfo::mctx"]
                        [::std::mem::offset_of!(CrashInfo, mctx) - 24usize];
                };
            }
        }
        extern "C" {
            pub fn swift_slowAlloc(bytes: usize, alignMask: usize) -> *mut ::std::os::raw::c_void;
        }
        pub type MallocTypeId = ::std::os::raw::c_ulonglong;
        extern "C" {
            #[link_name = "\u{1}__ZN5swift20swift_slowAllocTypedEmmy"]
            pub fn swift_slowAllocTyped(
                bytes: usize,
                alignMask: usize,
                typeId: root::swift::MallocTypeId,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            pub fn swift_slowDealloc(
                ptr: *mut ::std::os::raw::c_void,
                bytes: usize,
                alignMask: usize,
            );
        }
        #[doc = " Same as \\c std::is_trivially_copyable, which we cannot use directly\n because it is not implemented yet in all C++11 standard libraries.\n\n Unlike \\c llvm::isPodLike, this trait should produce a precise result and\n is not intended to be specialized."]
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct IsTriviallyCopyable {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct IsTriviallyConstructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct IsTriviallyDestructible {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct InProcess {
            _unused: [u8; 0],
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct TargetMetadata {
            pub _address: u8,
        }
        pub type Metadata = root::swift::TargetMetadata;
        extern "C" {
            #[link_name = "\u{1}__ZN5swift11fatalErrorvEjPKcPc"]
            pub fn fatalErrorv(
                flags: u32,
                format: *const ::std::os::raw::c_char,
                args: root::va_list,
            ) -> !;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift10fatalErrorEjPKcz"]
            pub fn fatalError(flags: u32, format: *const ::std::os::raw::c_char, ...) -> !;
        }
        extern "C" {
            #[doc = " swift::warning() emits a warning from the runtime."]
            #[link_name = "\u{1}__ZN5swift8warningvEjPKcPc"]
            pub fn warningv(flags: u32, format: *const ::std::os::raw::c_char, args: root::va_list);
        }
        extern "C" {
            #[doc = " swift::warning() emits a warning from the runtime."]
            #[link_name = "\u{1}__ZN5swift7warningEjPKcz"]
            pub fn warning(flags: u32, format: *const ::std::os::raw::c_char, ...);
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift24swift_dynamicCastFailureEPKNS_14TargetMetadataINS_9InProcessEEES4_PKc"]
            pub fn swift_dynamicCastFailure(
                sourceType: *const root::swift::Metadata,
                targetType: *const root::swift::Metadata,
                message: *const ::std::os::raw::c_char,
            ) -> !;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift24swift_dynamicCastFailureEPKvPKcS1_S3_S3_"]
            pub fn swift_dynamicCastFailure1(
                sourceType: *const ::std::os::raw::c_void,
                sourceName: *const ::std::os::raw::c_char,
                targetType: *const ::std::os::raw::c_void,
                targetName: *const ::std::os::raw::c_char,
                message: *const ::std::os::raw::c_char,
            ) -> !;
        }
        extern "C" {
            pub fn swift_reportError(flags: u32, message: *const ::std::os::raw::c_char);
        }
        extern "C" {
            pub fn swift_reportWarning(flags: u32, message: *const ::std::os::raw::c_char);
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift25swift_abortRetainOverflowEv"]
            pub fn swift_abortRetainOverflow() -> !;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift24swift_abortRetainUnownedEPKv"]
            pub fn swift_abortRetainUnowned(object: *const ::std::os::raw::c_void) -> !;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift32swift_abortUnownedRetainOverflowEv"]
            pub fn swift_abortUnownedRetainOverflow() -> !;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift29swift_abortWeakRetainOverflowEv"]
            pub fn swift_abortWeakRetainOverflow() -> !;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift37swift_abortDynamicReplacementEnablingEv"]
            pub fn swift_abortDynamicReplacementEnabling() -> !;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift38swift_abortDynamicReplacementDisablingEv"]
            pub fn swift_abortDynamicReplacementDisabling() -> !;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift33swift_abortDisabledUnicodeSupportEv"]
            pub fn swift_abortDisabledUnicodeSupport() -> !;
        }
        extern "C" {
            #[doc = " This function dumps one line of a stack trace. It is assumed that \\p framePC\n is the address of the stack frame at index \\p index. If \\p shortOutput is\n true, this functions prints only the name of the symbol and offset, ignores\n \\p index argument and omits the newline."]
            #[link_name = "\u{1}__ZN5swift19dumpStackTraceEntryEjPvb"]
            pub fn dumpStackTraceEntry(
                index: ::std::os::raw::c_uint,
                framePC: *mut ::std::os::raw::c_void,
                shortOutput: bool,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift20withCurrentBacktraceENSt3__18functionIFvPPviEEE"]
            pub fn withCurrentBacktrace(call: u8) -> bool;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN5swift21printCurrentBacktraceEj"]
            pub fn printCurrentBacktrace(framesToSkip: ::std::os::raw::c_uint);
        }
        #[doc = " Debugger breakpoint ABI. This structure is passed to the debugger (and needs\n to be stable) and describes extra information about a fatal error or a\n non-fatal warning, which should be logged as a runtime issue. Please keep\n all integer values pointer-sized."]
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct RuntimeErrorDetails {
            pub version: usize,
            pub errorType: *const ::std::os::raw::c_char,
            pub currentStackDescription: *const ::std::os::raw::c_char,
            pub framesToSkip: usize,
            pub memoryAddress: *const ::std::os::raw::c_void,
            pub numExtraThreads: usize,
            pub threads: *mut root::swift::RuntimeErrorDetails_Thread,
            pub numFixIts: usize,
            pub fixIts: *mut root::swift::RuntimeErrorDetails_FixIt,
            pub numNotes: usize,
            pub notes: *mut root::swift::RuntimeErrorDetails_Note,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct RuntimeErrorDetails_Thread {
            pub description: *const ::std::os::raw::c_char,
            pub threadID: u64,
            pub numFrames: usize,
            pub frames: *mut *mut ::std::os::raw::c_void,
        }
        const _: () = {
            ["Size of RuntimeErrorDetails_Thread"]
                [::std::mem::size_of::<RuntimeErrorDetails_Thread>() - 32usize];
            ["Alignment of RuntimeErrorDetails_Thread"]
                [::std::mem::align_of::<RuntimeErrorDetails_Thread>() - 8usize];
            ["Offset of field: RuntimeErrorDetails_Thread::description"]
                [::std::mem::offset_of!(RuntimeErrorDetails_Thread, description) - 0usize];
            ["Offset of field: RuntimeErrorDetails_Thread::threadID"]
                [::std::mem::offset_of!(RuntimeErrorDetails_Thread, threadID) - 8usize];
            ["Offset of field: RuntimeErrorDetails_Thread::numFrames"]
                [::std::mem::offset_of!(RuntimeErrorDetails_Thread, numFrames) - 16usize];
            ["Offset of field: RuntimeErrorDetails_Thread::frames"]
                [::std::mem::offset_of!(RuntimeErrorDetails_Thread, frames) - 24usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct RuntimeErrorDetails_FixIt {
            pub filename: *const ::std::os::raw::c_char,
            pub startLine: usize,
            pub startColumn: usize,
            pub endLine: usize,
            pub endColumn: usize,
            pub replacementText: *const ::std::os::raw::c_char,
        }
        const _: () = {
            ["Size of RuntimeErrorDetails_FixIt"]
                [::std::mem::size_of::<RuntimeErrorDetails_FixIt>() - 48usize];
            ["Alignment of RuntimeErrorDetails_FixIt"]
                [::std::mem::align_of::<RuntimeErrorDetails_FixIt>() - 8usize];
            ["Offset of field: RuntimeErrorDetails_FixIt::filename"]
                [::std::mem::offset_of!(RuntimeErrorDetails_FixIt, filename) - 0usize];
            ["Offset of field: RuntimeErrorDetails_FixIt::startLine"]
                [::std::mem::offset_of!(RuntimeErrorDetails_FixIt, startLine) - 8usize];
            ["Offset of field: RuntimeErrorDetails_FixIt::startColumn"]
                [::std::mem::offset_of!(RuntimeErrorDetails_FixIt, startColumn) - 16usize];
            ["Offset of field: RuntimeErrorDetails_FixIt::endLine"]
                [::std::mem::offset_of!(RuntimeErrorDetails_FixIt, endLine) - 24usize];
            ["Offset of field: RuntimeErrorDetails_FixIt::endColumn"]
                [::std::mem::offset_of!(RuntimeErrorDetails_FixIt, endColumn) - 32usize];
            ["Offset of field: RuntimeErrorDetails_FixIt::replacementText"]
                [::std::mem::offset_of!(RuntimeErrorDetails_FixIt, replacementText) - 40usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct RuntimeErrorDetails_Note {
            pub description: *const ::std::os::raw::c_char,
            pub numFixIts: usize,
            pub fixIts: *mut root::swift::RuntimeErrorDetails_FixIt,
        }
        const _: () = {
            ["Size of RuntimeErrorDetails_Note"]
                [::std::mem::size_of::<RuntimeErrorDetails_Note>() - 24usize];
            ["Alignment of RuntimeErrorDetails_Note"]
                [::std::mem::align_of::<RuntimeErrorDetails_Note>() - 8usize];
            ["Offset of field: RuntimeErrorDetails_Note::description"]
                [::std::mem::offset_of!(RuntimeErrorDetails_Note, description) - 0usize];
            ["Offset of field: RuntimeErrorDetails_Note::numFixIts"]
                [::std::mem::offset_of!(RuntimeErrorDetails_Note, numFixIts) - 8usize];
            ["Offset of field: RuntimeErrorDetails_Note::fixIts"]
                [::std::mem::offset_of!(RuntimeErrorDetails_Note, fixIts) - 16usize];
        };
        pub const RuntimeErrorDetails_currentVersion: usize = 2;
        const _: () = {
            ["Size of RuntimeErrorDetails"][::std::mem::size_of::<RuntimeErrorDetails>() - 88usize];
            ["Alignment of RuntimeErrorDetails"]
                [::std::mem::align_of::<RuntimeErrorDetails>() - 8usize];
            ["Offset of field: RuntimeErrorDetails::version"]
                [::std::mem::offset_of!(RuntimeErrorDetails, version) - 0usize];
            ["Offset of field: RuntimeErrorDetails::errorType"]
                [::std::mem::offset_of!(RuntimeErrorDetails, errorType) - 8usize];
            ["Offset of field: RuntimeErrorDetails::currentStackDescription"]
                [::std::mem::offset_of!(RuntimeErrorDetails, currentStackDescription) - 16usize];
            ["Offset of field: RuntimeErrorDetails::framesToSkip"]
                [::std::mem::offset_of!(RuntimeErrorDetails, framesToSkip) - 24usize];
            ["Offset of field: RuntimeErrorDetails::memoryAddress"]
                [::std::mem::offset_of!(RuntimeErrorDetails, memoryAddress) - 32usize];
            ["Offset of field: RuntimeErrorDetails::numExtraThreads"]
                [::std::mem::offset_of!(RuntimeErrorDetails, numExtraThreads) - 40usize];
            ["Offset of field: RuntimeErrorDetails::threads"]
                [::std::mem::offset_of!(RuntimeErrorDetails, threads) - 48usize];
            ["Offset of field: RuntimeErrorDetails::numFixIts"]
                [::std::mem::offset_of!(RuntimeErrorDetails, numFixIts) - 56usize];
            ["Offset of field: RuntimeErrorDetails::fixIts"]
                [::std::mem::offset_of!(RuntimeErrorDetails, fixIts) - 64usize];
            ["Offset of field: RuntimeErrorDetails::numNotes"]
                [::std::mem::offset_of!(RuntimeErrorDetails, numNotes) - 72usize];
            ["Offset of field: RuntimeErrorDetails::notes"]
                [::std::mem::offset_of!(RuntimeErrorDetails, notes) - 80usize];
        };
        pub const swift_RuntimeErrorFlagNone: root::swift::_bindgen_ty_1 = 0;
        pub const swift_RuntimeErrorFlagFatal: root::swift::_bindgen_ty_1 = 1;
        pub type _bindgen_ty_1 = usize;
        extern "C" {
            #[doc = " Debugger hook. Calling this stops the debugger with a message and details\n about the issues. Called by overlays."]
            pub fn _swift_reportToDebugger(
                flags: usize,
                message: *const ::std::os::raw::c_char,
                details: *mut root::swift::RuntimeErrorDetails,
            );
        }
        extern "C" {
            pub static mut _swift_reportFatalErrorsToDebugger: bool;
        }
        extern "C" {
            pub fn _swift_shouldReportFatalErrorsToDebugger() -> bool;
        }
        extern "C" {
            pub static mut _swift_debug_metadataAllocationIterationEnabled: bool;
        }
        extern "C" {
            pub static _swift_debug_allocationPoolPointer: *const ::std::os::raw::c_void;
        }
        extern "C" {
            pub static mut _swift_debug_metadataAllocationBacktraceList: u8;
        }
        extern "C" {
            pub static _swift_debug_protocolConformanceStatePointer: *const ::std::os::raw::c_void;
        }
        extern "C" {
            pub static _swift_debug_multiPayloadEnumPointerSpareBitsMask: u64;
        }
        pub const RefCountInlinedness_RefCountNotInline: root::swift::RefCountInlinedness = 0;
        pub const RefCountInlinedness_RefCountIsInline: root::swift::RefCountInlinedness = 1;
        pub type RefCountInlinedness = ::std::os::raw::c_uint;
        pub const PerformDeinit_DontPerformDeinit: root::swift::PerformDeinit = 0;
        pub const PerformDeinit_DoPerformDeinit: root::swift::PerformDeinit = 1;
        pub type PerformDeinit = ::std::os::raw::c_uint;
        extern "C" {
            #[link_name = "\u{1}Inlinedness"]
            pub static RefCountBitsT_Inlinedness: root::swift::RefCountInlinedness;
        }
        pub type RefCountBitsT_BitsType = u8;
        pub type RefCountBitsT_SignedBitsType = u8;
        pub type RefCountBitsT_Offsets = u8;
        pub const RefCountBitsT_Immortal_t_Immortal: root::swift::RefCountBitsT_Immortal_t = 0;
        pub type RefCountBitsT_Immortal_t = i32;
        pub type InlineRefCountBits = u64;
        #[repr(C)]
        #[repr(align(16))]
        #[derive(Debug, Copy, Clone)]
        pub struct SideTableRefCountBits {
            pub _base: u64,
            pub weakBits: u32,
        }
        const _: () = {
            ["Size of SideTableRefCountBits"]
                [::std::mem::size_of::<SideTableRefCountBits>() - 16usize];
            ["Alignment of SideTableRefCountBits"]
                [::std::mem::align_of::<SideTableRefCountBits>() - 16usize];
            ["Offset of field: SideTableRefCountBits::weakBits"]
                [::std::mem::offset_of!(SideTableRefCountBits, weakBits) - 8usize];
        };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct RefCounts {
            pub refCounts: u8,
        }
        pub const RefCounts_Initialized_t_Initialized: root::swift::RefCounts_Initialized_t = 0;
        pub type RefCounts_Initialized_t = i32;
        pub const RefCounts_Immortal_t_Immortal: root::swift::RefCounts_Immortal_t = 0;
        pub type RefCounts_Immortal_t = i32;
        pub type InlineRefCounts = root::swift::RefCounts;
        pub type SideTableRefCounts = root::swift::RefCounts;
        #[repr(C)]
        #[repr(align(16))]
        #[derive(Debug, Copy, Clone)]
        pub struct HeapObjectSideTableEntry {
            pub object: u64,
            pub __bindgen_padding_0: u64,
            pub refCounts: root::swift::SideTableRefCounts,
            pub immutableCOWBuffer: bool,
        }
        // const _: () = {
        //     ["Size of HeapObjectSideTableEntry"]
        //         [::std::mem::size_of::<HeapObjectSideTableEntry>() - 48usize];
        //     ["Alignment of HeapObjectSideTableEntry"]
        //         [::std::mem::align_of::<HeapObjectSideTableEntry>() - 16usize];
        //     ["Offset of field: HeapObjectSideTableEntry::object"]
        //         [::std::mem::offset_of!(HeapObjectSideTableEntry, object) - 0usize];
        //     ["Offset of field: HeapObjectSideTableEntry::refCounts"]
        //         [::std::mem::offset_of!(HeapObjectSideTableEntry, refCounts) - 16usize];
        //     ["Offset of field: HeapObjectSideTableEntry::immutableCOWBuffer"]
        //         [::std::mem::offset_of!(HeapObjectSideTableEntry, immutableCOWBuffer) - 32usize];
        // };
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct TargetHeapMetadata {
            pub _address: u8,
        }
        pub type HeapMetadata = root::swift::TargetHeapMetadata;
        #[doc = " The Swift heap-object header.\n This must match RefCountedStructTy in IRGen."]
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct HeapObject {
            #[doc = " This is always a valid pointer to a metadata object."]
            pub metadata: *const root::swift::HeapMetadata,
            pub refCounts: root::swift::InlineRefCounts,
        }
        const _: () = {
            ["Size of HeapObject"][::std::mem::size_of::<HeapObject>() - 16usize];
            ["Alignment of HeapObject"][::std::mem::align_of::<HeapObject>() - 8usize];
            ["Offset of field: HeapObject::metadata"]
                [::std::mem::offset_of!(HeapObject, metadata) - 0usize];
            ["Offset of field: HeapObject::refCounts"]
                [::std::mem::offset_of!(HeapObject, refCounts) - 8usize];
        };
        extern "C" {
            #[link_name = "\u{1}__ZNK5swift10HeapObject4dumpEv"]
            pub fn HeapObject_dump(this: *const root::swift::HeapObject);
        }
        impl HeapObject {
            #[inline]
            pub unsafe fn dump(&self) {
                HeapObject_dump(self)
            }
        }
        extern "C" {
            pub fn _swift_instantiateInertHeapObject(
                address: *mut ::std::os::raw::c_void,
                metadata: *const root::swift::HeapMetadata,
            );
        }
        extern "C" {
            pub fn swift_retainCount(obj: *mut root::swift::HeapObject) -> root::__swift_size_t;
        }
        extern "C" {
            pub fn swift_unownedRetainCount(
                obj: *mut root::swift::HeapObject,
            ) -> root::__swift_size_t;
        }
        extern "C" {
            pub fn swift_weakRetainCount(obj: *mut root::swift::HeapObject)
                -> root::__swift_size_t;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct OpaqueValue {
            _unused: [u8; 0],
        }
        extern "C" {
            #[doc = " Allocates a new heap object.  The returned memory is\n uninitialized outside of the heap-object header.  The object\n has an initial retain count of 1, and its metadata is set to\n the given value.\n\n At some point \"soon after return\", it will become an\n invariant that metadata->getSize(returnValue) will equal\n requiredSize.\n\n Either aborts or throws a swift exception if the allocation fails.\n\n \\param requiredSize - the required size of the allocation,\n   including the header\n \\param requiredAlignmentMask - the required alignment of the allocation;\n   always one less than a power of 2 that's at least alignof(void*)\n \\return never null\n\n POSSIBILITIES: The argument order is fair game.  It may be useful\n to have a variant which guarantees zero-initialized memory."]
            pub fn swift_allocObject(
                metadata: *const root::swift::HeapMetadata,
                requiredSize: usize,
                requiredAlignmentMask: usize,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Initializes the object header of a stack allocated object.\n\n \\param metadata - the object's metadata which is stored in the header\n \\param object - the pointer to the object's memory on the stack\n \\returns the passed object pointer."]
            pub fn swift_initStackObject(
                metadata: *const root::swift::HeapMetadata,
                object: *mut root::swift::HeapObject,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Initializes the object header of a static object which is statically\n allocated in the data section.\n\n \\param metadata - the object's metadata which is stored in the header\n \\param object - the address of the object in the data section. It is assumed\n        that at offset -1 there is a swift_once token allocated.\n \\returns the passed object pointer."]
            pub fn swift_initStaticObject(
                metadata: *const root::swift::HeapMetadata,
                object: *mut root::swift::HeapObject,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Performs verification that the lifetime of a stack allocated object has\n ended. It aborts if the reference counts of the object indicate that the\n object did escape to some other location."]
            pub fn swift_verifyEndOfLifetime(object: *mut root::swift::HeapObject);
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct BoxPair {
            pub object: *mut root::swift::HeapObject,
            pub buffer: *mut root::swift::OpaqueValue,
        }
        const _: () = {
            ["Size of BoxPair"][::std::mem::size_of::<BoxPair>() - 16usize];
            ["Alignment of BoxPair"][::std::mem::align_of::<BoxPair>() - 8usize];
            ["Offset of field: BoxPair::object"][::std::mem::offset_of!(BoxPair, object) - 0usize];
            ["Offset of field: BoxPair::buffer"][::std::mem::offset_of!(BoxPair, buffer) - 8usize];
        };
        extern "C" {
            #[doc = " Allocates a heap object that can contain a value of the given type.\n Returns a Box structure containing a HeapObject* pointer to the\n allocated object, and a pointer to the value inside the heap object.\n The value pointer points to an uninitialized buffer of size and alignment\n appropriate to store a value of the given type.\n The heap object has an initial retain count of 1, and its metadata is set\n such that destroying the heap object destroys the contained value."]
            #[link_name = "\u{1}_swift_allocBox"]
            pub fn swift_allocBox(type_: *const root::swift::Metadata) -> root::swift::BoxPair;
        }
        extern "C" {
            #[doc = " Performs a uniqueness check on the pointer to a box structure. If the check\n fails allocates a new box and stores the pointer in the buffer.\n\n  if (!isUnique(buffer[0]))\n    buffer[0] = swift_allocBox(type)"]
            #[link_name = "\u{1}_swift_makeBoxUnique"]
            pub fn swift_makeBoxUnique(
                buffer: *mut root::swift::OpaqueValue,
                type_: *const root::swift::Metadata,
                alignMask: usize,
            ) -> root::swift::BoxPair;
        }
        extern "C" {
            #[doc = " Returns the address of a heap object representing all empty box types."]
            pub fn swift_allocEmptyBox() -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Atomically increments the retain count of an object.\n\n \\param object - may be null, in which case this is a no-op\n\n \\return object - we return the object because this enables tail call\n optimization and the argument register to be live through the call on\n architectures whose argument and return register is the same register.\n\n POSSIBILITIES: We may end up wanting a bunch of different variants:\n  - the general version which correctly handles null values, swift\n     objects, and ObjC objects\n    - a variant that assumes that its operand is a swift object\n      - a variant that can safely use non-atomic operations\n      - maybe a variant that can assume a non-null object\n It may also prove worthwhile to have this use a custom CC\n which preserves a larger set of registers."]
            pub fn swift_retain(
                object: *mut root::swift::HeapObject,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            pub fn swift_retain_n(
                object: *mut root::swift::HeapObject,
                n: u32,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            pub fn swift_nonatomic_retain(
                object: *mut root::swift::HeapObject,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            pub fn swift_nonatomic_retain_n(
                object: *mut root::swift::HeapObject,
                n: u32,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Atomically increments the reference count of an object, unless it has\n already been destroyed. Returns nil if the object is dead."]
            pub fn swift_tryRetain(
                object: *mut root::swift::HeapObject,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Returns true if an object is in the process of being deallocated."]
            pub fn swift_isDeallocating(object: *mut root::swift::HeapObject) -> bool;
        }
        extern "C" {
            #[doc = " Atomically decrements the retain count of an object.  If the\n retain count reaches zero, the object is destroyed as follows:\n\n   size_t allocSize = object->metadata->destroy(object);\n   if (allocSize) swift_deallocObject(object, allocSize);\n\n \\param object - may be null, in which case this is a no-op\n\n POSSIBILITIES: We may end up wanting a bunch of different variants:\n  - the general version which correctly handles null values, swift\n     objects, and ObjC objects\n    - a variant that assumes that its operand is a swift object\n      - a variant that can safely use non-atomic operations\n      - maybe a variant that can assume a non-null object\n It's unlikely that a custom CC would be beneficial here."]
            pub fn swift_release(object: *mut root::swift::HeapObject);
        }
        extern "C" {
            pub fn swift_nonatomic_release(object: *mut root::swift::HeapObject);
        }
        extern "C" {
            #[doc = " Atomically decrements the retain count of an object n times. If the retain\n count reaches zero, the object is destroyed"]
            pub fn swift_release_n(object: *mut root::swift::HeapObject, n: u32);
        }
        extern "C" {
            #[doc = " Sets the RC_DEALLOCATING_FLAG flag. This is done non-atomically.\n The strong reference count of \\p object must be 1 and no other thread may\n retain the object during executing this function."]
            pub fn swift_setDeallocating(object: *mut root::swift::HeapObject);
        }
        extern "C" {
            pub fn swift_nonatomic_release_n(object: *mut root::swift::HeapObject, n: u32);
        }
        extern "C" {
            #[doc = " Is this pointer a non-null unique reference to an object?"]
            pub fn swift_isUniquelyReferenced(arg1: *const ::std::os::raw::c_void) -> bool;
        }
        extern "C" {
            #[doc = " Is this non-null pointer a unique reference to an object?"]
            pub fn swift_isUniquelyReferenced_nonNull(arg1: *const ::std::os::raw::c_void) -> bool;
        }
        extern "C" {
            #[doc = " Is this non-null BridgeObject a unique reference to an object?"]
            pub fn swift_isUniquelyReferenced_nonNull_bridgeObject(bits: usize) -> bool;
        }
        extern "C" {
            #[doc = " Is this pointer a non-null unique reference to an object\n that uses Swift reference counting?"]
            pub fn swift_isUniquelyReferencedNonObjC(arg1: *const ::std::os::raw::c_void) -> bool;
        }
        extern "C" {
            #[doc = " Is this non-null pointer a unique reference to an object\n that uses Swift reference counting?"]
            pub fn swift_isUniquelyReferencedNonObjC_nonNull(
                arg1: *const ::std::os::raw::c_void,
            ) -> bool;
        }
        extern "C" {
            #[doc = " Is this non-null BridgeObject a unique reference to an object\n that uses Swift reference counting?"]
            pub fn swift_isUniquelyReferencedNonObjC_nonNull_bridgeObject(bits: usize) -> bool;
        }
        extern "C" {
            #[doc = " Is this native Swift pointer a non-null unique reference to\n an object?"]
            pub fn swift_isUniquelyReferenced_native(arg1: *const root::swift::HeapObject) -> bool;
        }
        extern "C" {
            #[doc = " Is this non-null native Swift pointer a unique reference to\n an object?"]
            pub fn swift_isUniquelyReferenced_nonNull_native(
                arg1: *const root::swift::HeapObject,
            ) -> bool;
        }
        extern "C" {
            #[doc = " Is this native Swift pointer non-null and has a reference count greater than\n one.\n This runtime call will print an error message with file name and location if\n the closure is escaping but it will not abort.\n\n \\p type: 0 - withoutActuallyEscaping verification\n              Was the closure passed to a withoutActuallyEscaping block\n              escaped in the block?\n          1 - @objc closure sentinel verification\n              Was the closure passed to Objective-C escaped?"]
            pub fn swift_isEscapingClosureAtFileLocation(
                object: *const root::swift::HeapObject,
                filename: *const ::std::os::raw::c_uchar,
                filenameLength: i32,
                line: i32,
                column: i32,
                type_: ::std::os::raw::c_uint,
            ) -> bool;
        }
        extern "C" {
            #[doc = " Deallocate the given memory.\n\n It must have been returned by swift_allocObject and the strong reference\n must have the RC_DEALLOCATING_FLAG flag set, but otherwise the object is\n in an unknown state.\n\n \\param object - never null\n \\param allocatedSize - the allocated size of the object from the\n   program's perspective, i.e. the value\n \\param allocatedAlignMask - the alignment requirement that was passed\n   to allocObject\n\n POSSIBILITIES: It may be useful to have a variant which\n requires the object to have been fully zeroed from offsets\n sizeof(SwiftHeapObject) to allocatedSize."]
            pub fn swift_deallocObject(
                object: *mut root::swift::HeapObject,
                allocatedSize: usize,
                allocatedAlignMask: usize,
            );
        }
        extern "C" {
            #[doc = " Deallocate an uninitialized object with a strong reference count of +1.\n\n It must have been returned by swift_allocObject, but otherwise the object is\n in an unknown state.\n\n \\param object - never null\n \\param allocatedSize - the allocated size of the object from the\n   program's perspective, i.e. the value\n \\param allocatedAlignMask - the alignment requirement that was passed\n   to allocObject\n"]
            pub fn swift_deallocUninitializedObject(
                object: *mut root::swift::HeapObject,
                allocatedSize: usize,
                allocatedAlignMask: usize,
            );
        }
        extern "C" {
            #[doc = " Deallocate the given memory.\n\n It must have been returned by swift_allocObject, possibly used as an\n Objective-C class instance, and the strong reference must have the\n RC_DEALLOCATING_FLAG flag set, but otherwise the object is in an unknown\n state.\n\n \\param object - never null\n \\param allocatedSize - the allocated size of the object from the\n   program's perspective, i.e. the value\n \\param allocatedAlignMask - the alignment requirement that was passed\n   to allocObject\n\n POSSIBILITIES: It may be useful to have a variant which\n requires the object to have been fully zeroed from offsets\n sizeof(SwiftHeapObject) to allocatedSize."]
            pub fn swift_deallocClassInstance(
                object: *mut root::swift::HeapObject,
                allocatedSize: usize,
                allocatedAlignMask: usize,
            );
        }
        extern "C" {
            #[doc = " Deallocate the given memory after destroying instance variables.\n\n Destroys instance variables in classes more derived than the given metatype.\n\n It must have been returned by swift_allocObject, possibly used as an\n Objective-C class instance, and the strong reference must be equal to 1.\n\n \\param object - may be null\n \\param type - most derived class whose instance variables do not need to\n   be destroyed\n \\param allocatedSize - the allocated size of the object from the\n   program's perspective, i.e. the value\n \\param allocatedAlignMask - the alignment requirement that was passed\n   to allocObject"]
            pub fn swift_deallocPartialClassInstance(
                object: *mut root::swift::HeapObject,
                type_: *const root::swift::HeapMetadata,
                allocatedSize: usize,
                allocatedAlignMask: usize,
            );
        }
        extern "C" {
            #[doc = " Deallocate the given memory allocated by swift_allocBox; it was returned\n by swift_allocBox but is otherwise in an unknown state. The given Metadata\n pointer must be the same metadata pointer that was passed to swift_allocBox\n when the memory was allocated."]
            pub fn swift_deallocBox(object: *mut root::swift::HeapObject);
        }
        extern "C" {
            #[doc = " Project the value out of a box. `object` must have been allocated\n using `swift_allocBox`, or by the compiler using a statically-emitted\n box metadata object."]
            pub fn swift_projectBox(
                object: *mut root::swift::HeapObject,
            ) -> *mut root::swift::OpaqueValue;
        }
        #[doc = " RAII object that wraps a Swift heap object and releases it upon\n destruction."]
        #[repr(C)]
        #[derive(Debug)]
        pub struct SwiftRAII {
            pub object: *mut root::swift::HeapObject,
        }
        const _: () = {
            ["Size of SwiftRAII"][::std::mem::size_of::<SwiftRAII>() - 8usize];
            ["Alignment of SwiftRAII"][::std::mem::align_of::<SwiftRAII>() - 8usize];
            ["Offset of field: SwiftRAII::object"]
                [::std::mem::offset_of!(SwiftRAII, object) - 0usize];
        };
        #[doc = " An unowned reference in memory.  This is ABI."]
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct UnownedReference {
            pub Value: *mut root::swift::HeapObject,
        }
        const _: () = {
            ["Size of UnownedReference"][::std::mem::size_of::<UnownedReference>() - 8usize];
            ["Alignment of UnownedReference"][::std::mem::align_of::<UnownedReference>() - 8usize];
            ["Offset of field: UnownedReference::Value"]
                [::std::mem::offset_of!(UnownedReference, Value) - 0usize];
        };
        extern "C" {
            #[doc = " Increment the unowned retain count."]
            pub fn swift_unownedRetain(
                value: *mut root::swift::HeapObject,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Decrement the unowned retain count."]
            pub fn swift_unownedRelease(value: *mut root::swift::HeapObject);
        }
        extern "C" {
            #[doc = " Increment the unowned retain count."]
            pub fn swift_nonatomic_unownedRetain(
                value: *mut root::swift::HeapObject,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[doc = " Decrement the unowned retain count."]
            pub fn swift_nonatomic_unownedRelease(value: *mut root::swift::HeapObject);
        }
        extern "C" {
            #[doc = " Increment the unowned retain count by n."]
            pub fn swift_unownedRetain_n(
                value: *mut root::swift::HeapObject,
                n: ::std::os::raw::c_int,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Decrement the unowned retain count by n."]
            pub fn swift_unownedRelease_n(
                value: *mut root::swift::HeapObject,
                n: ::std::os::raw::c_int,
            );
        }
        extern "C" {
            #[doc = " Increment the unowned retain count by n."]
            pub fn swift_nonatomic_unownedRetain_n(
                value: *mut root::swift::HeapObject,
                n: ::std::os::raw::c_int,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Decrement the unowned retain count by n."]
            pub fn swift_nonatomic_unownedRelease_n(
                value: *mut root::swift::HeapObject,
                n: ::std::os::raw::c_int,
            );
        }
        extern "C" {
            #[doc = " Increment the strong retain count of an object, aborting if it has\n been deallocated."]
            pub fn swift_unownedRetainStrong(
                value: *mut root::swift::HeapObject,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Increment the strong retain count of an object, aborting if it has\n been deallocated."]
            pub fn swift_nonatomic_unownedRetainStrong(
                value: *mut root::swift::HeapObject,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Increment the strong retain count of an object which may have been\n deallocated, aborting if it has been deallocated, and decrement its\n unowned reference count."]
            pub fn swift_unownedRetainStrongAndRelease(value: *mut root::swift::HeapObject);
        }
        extern "C" {
            #[doc = " Increment the strong retain count of an object which may have been\n deallocated, aborting if it has been deallocated, and decrement its\n unowned reference count."]
            pub fn swift_nonatomic_unownedRetainStrongAndRelease(
                value: *mut root::swift::HeapObject,
            );
        }
        extern "C" {
            #[doc = " Aborts if the object has been deallocated."]
            pub fn swift_unownedCheck(value: *mut root::swift::HeapObject);
        }
        #[doc = "/\n/****************************** WEAK REFERENCES ******************************/\n/"]
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct WeakReference {
            _unused: [u8; 0],
        }
        extern "C" {
            #[doc = " Initialize a weak reference.\n\n \\param ref - never null\n \\param value - can be null\n \\return ref"]
            pub fn swift_weakInit(
                ref_: *mut root::swift::WeakReference,
                value: *mut root::swift::HeapObject,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = " Assign a new value to a weak reference.\n\n \\param ref - never null\n \\param value - can be null\n \\return ref"]
            pub fn swift_weakAssign(
                ref_: *mut root::swift::WeakReference,
                value: *mut root::swift::HeapObject,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = " Load a value from a weak reference.  If the current value is a\n non-null object that has begun deallocation, returns null;\n otherwise, retains the object before returning.\n\n \\param ref - never null\n \\return can be null"]
            pub fn swift_weakLoadStrong(
                ref_: *mut root::swift::WeakReference,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Load a value from a weak reference as if by swift_weakLoadStrong,\n but leaving the reference in an uninitialized state.\n\n \\param ref - never null\n \\return can be null"]
            pub fn swift_weakTakeStrong(
                ref_: *mut root::swift::WeakReference,
            ) -> *mut root::swift::HeapObject;
        }
        extern "C" {
            #[doc = " Destroy a weak reference.\n\n \\param ref - never null, but can refer to a null object"]
            pub fn swift_weakDestroy(ref_: *mut root::swift::WeakReference);
        }
        extern "C" {
            #[doc = " Copy initialize a weak reference.\n\n \\param dest - never null, but can refer to a null object\n \\param src - never null, but can refer to a null object\n \\return dest"]
            pub fn swift_weakCopyInit(
                dest: *mut root::swift::WeakReference,
                src: *mut root::swift::WeakReference,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = " Take initialize a weak reference.\n\n \\param dest - never null, but can refer to a null object\n \\param src - never null, but can refer to a null object\n \\return dest"]
            pub fn swift_weakTakeInit(
                dest: *mut root::swift::WeakReference,
                src: *mut root::swift::WeakReference,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = " Copy assign a weak reference.\n\n \\param dest - never null, but can refer to a null object\n \\param src - never null, but can refer to a null object\n \\return dest"]
            pub fn swift_weakCopyAssign(
                dest: *mut root::swift::WeakReference,
                src: *mut root::swift::WeakReference,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = " Take assign a weak reference.\n\n \\param dest - never null, but can refer to a null object\n \\param src - never null, but can refer to a null object\n \\return dest"]
            pub fn swift_weakTakeAssign(
                dest: *mut root::swift::WeakReference,
                src: *mut root::swift::WeakReference,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = "/\n/************************* OTHER REFERENCE-COUNTING **************************/\n/"]
            pub fn swift_bridgeObjectRetain(
                value: *mut ::std::os::raw::c_void,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[doc = " Increment the strong retain count of a bridged object by n."]
            pub fn swift_bridgeObjectRetain_n(
                value: *mut ::std::os::raw::c_void,
                n: ::std::os::raw::c_int,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            pub fn swift_nonatomic_bridgeObjectRetain(
                value: *mut ::std::os::raw::c_void,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[doc = " Increment the strong retain count of a bridged object by n."]
            pub fn swift_nonatomic_bridgeObjectRetain_n(
                value: *mut ::std::os::raw::c_void,
                n: ::std::os::raw::c_int,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[doc = " Increment the strong retain count of an object which might not be a native\n Swift object."]
            pub fn swift_unknownObjectRetain(
                value: *mut ::std::os::raw::c_void,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[doc = " Increment the strong retain count of an object which might not be a native\n Swift object by n."]
            pub fn swift_unknownObjectRetain_n(
                value: *mut ::std::os::raw::c_void,
                n: ::std::os::raw::c_int,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[doc = " Increment the strong retain count of an object which might not be a native\n Swift object."]
            pub fn swift_nonatomic_unknownObjectRetain(
                value: *mut ::std::os::raw::c_void,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[doc = " Increment the strong retain count of an object which might not be a native\n Swift object by n."]
            pub fn swift_nonatomic_unknownObjectRetain_n(
                value: *mut ::std::os::raw::c_void,
                n: ::std::os::raw::c_int,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            pub fn swift_bridgeObjectRelease(value: *mut ::std::os::raw::c_void);
        }
        extern "C" {
            #[doc = " Decrement the strong retain count of a bridged object by n."]
            pub fn swift_bridgeObjectRelease_n(
                value: *mut ::std::os::raw::c_void,
                n: ::std::os::raw::c_int,
            );
        }
        extern "C" {
            pub fn swift_nonatomic_bridgeObjectRelease(value: *mut ::std::os::raw::c_void);
        }
        extern "C" {
            #[doc = " Decrement the strong retain count of a bridged object by n."]
            pub fn swift_nonatomic_bridgeObjectRelease_n(
                value: *mut ::std::os::raw::c_void,
                n: ::std::os::raw::c_int,
            );
        }
        extern "C" {
            #[doc = " Decrement the strong retain count of an object which might not be a native\n Swift object."]
            pub fn swift_unknownObjectRelease(value: *mut ::std::os::raw::c_void);
        }
        extern "C" {
            #[doc = " Decrement the strong retain count of an object which might not be a native\n Swift object by n."]
            pub fn swift_unknownObjectRelease_n(
                value: *mut ::std::os::raw::c_void,
                n: ::std::os::raw::c_int,
            );
        }
        extern "C" {
            #[doc = " Decrement the strong retain count of an object which might not be a native\n Swift object."]
            pub fn swift_nonatomic_unknownObjectRelease(value: *mut ::std::os::raw::c_void);
        }
        extern "C" {
            #[doc = " Decrement the strong retain count of an object which might not be a native\n Swift object by n."]
            pub fn swift_nonatomic_unknownObjectRelease_n(
                value: *mut ::std::os::raw::c_void,
                n: ::std::os::raw::c_int,
            );
        }
        extern "C" {
            #[doc = " Initialize a weak reference.\n\n \\param ref - never null\n \\param value - not necessarily a native Swift object; can be null\n \\return ref"]
            pub fn swift_unknownObjectWeakInit(
                ref_: *mut root::swift::WeakReference,
                value: *mut ::std::os::raw::c_void,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = " Assign a new value to a weak reference.\n\n \\param ref - never null\n \\param value - not necessarily a native Swift object; can be null\n \\return ref"]
            pub fn swift_unknownObjectWeakAssign(
                ref_: *mut root::swift::WeakReference,
                value: *mut ::std::os::raw::c_void,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = " Load a value from a weak reference, much like swift_weakLoadStrong\n but without requiring the variable to refer to a native Swift object.\n\n \\param ref - never null\n \\return can be null"]
            pub fn swift_unknownObjectWeakLoadStrong(
                ref_: *mut root::swift::WeakReference,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[doc = " Load a value from a weak reference as if by\n swift_unknownObjectWeakLoadStrong, but leaving the reference in an\n uninitialized state.\n\n \\param ref - never null\n \\return can be null"]
            pub fn swift_unknownObjectWeakTakeStrong(
                ref_: *mut root::swift::WeakReference,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[doc = " Destroy a weak reference variable that might not refer to a native\n Swift object."]
            pub fn swift_unknownObjectWeakDestroy(object: *mut root::swift::WeakReference);
        }
        extern "C" {
            #[doc = " Copy-initialize a weak reference variable from one that might not\n refer to a native Swift object.\n \\return dest"]
            pub fn swift_unknownObjectWeakCopyInit(
                dest: *mut root::swift::WeakReference,
                src: *mut root::swift::WeakReference,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = " Take-initialize a weak reference variable from one that might not\n refer to a native Swift object.\n \\return dest"]
            pub fn swift_unknownObjectWeakTakeInit(
                dest: *mut root::swift::WeakReference,
                src: *mut root::swift::WeakReference,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = " Copy-assign a weak reference variable from another when either\n or both variables might not refer to a native Swift object.\n \\return dest"]
            pub fn swift_unknownObjectWeakCopyAssign(
                dest: *mut root::swift::WeakReference,
                src: *mut root::swift::WeakReference,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = " Take-assign a weak reference variable from another when either\n or both variables might not refer to a native Swift object.\n \\return dest"]
            pub fn swift_unknownObjectWeakTakeAssign(
                dest: *mut root::swift::WeakReference,
                src: *mut root::swift::WeakReference,
            ) -> *mut root::swift::WeakReference;
        }
        extern "C" {
            #[doc = " Initialize an unowned reference to an object with unknown reference\n counting.\n \\return ref"]
            pub fn swift_unknownObjectUnownedInit(
                ref_: *mut root::swift::UnownedReference,
                value: *mut ::std::os::raw::c_void,
            ) -> *mut root::swift::UnownedReference;
        }
        extern "C" {
            #[doc = " Assign to an unowned reference holding an object with unknown reference\n counting.\n \\return ref"]
            pub fn swift_unknownObjectUnownedAssign(
                ref_: *mut root::swift::UnownedReference,
                value: *mut ::std::os::raw::c_void,
            ) -> *mut root::swift::UnownedReference;
        }
        extern "C" {
            #[doc = " Load from an unowned reference to an object with unknown reference\n counting."]
            pub fn swift_unknownObjectUnownedLoadStrong(
                ref_: *mut root::swift::UnownedReference,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[doc = " Take from an unowned reference to an object with unknown reference\n counting."]
            pub fn swift_unknownObjectUnownedTakeStrong(
                ref_: *mut root::swift::UnownedReference,
            ) -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            #[doc = " Destroy an unowned reference to an object with unknown reference counting."]
            pub fn swift_unknownObjectUnownedDestroy(ref_: *mut root::swift::UnownedReference);
        }
        extern "C" {
            #[doc = " Copy-initialize an unowned reference variable from one that might not\n refer to a native Swift object.\n \\return dest"]
            pub fn swift_unknownObjectUnownedCopyInit(
                dest: *mut root::swift::UnownedReference,
                src: *mut root::swift::UnownedReference,
            ) -> *mut root::swift::UnownedReference;
        }
        extern "C" {
            #[doc = " Take-initialize an unowned reference variable from one that might not\n refer to a native Swift object."]
            pub fn swift_unknownObjectUnownedTakeInit(
                dest: *mut root::swift::UnownedReference,
                src: *mut root::swift::UnownedReference,
            ) -> *mut root::swift::UnownedReference;
        }
        extern "C" {
            #[doc = " Copy-assign an unowned reference variable from another when either\n or both variables might not refer to a native Swift object.\n \\return dest"]
            pub fn swift_unknownObjectUnownedCopyAssign(
                dest: *mut root::swift::UnownedReference,
                src: *mut root::swift::UnownedReference,
            ) -> *mut root::swift::UnownedReference;
        }
        extern "C" {
            #[doc = " Take-assign an unowned reference variable from another when either\n or both variables might not refer to a native Swift object.\n \\return dest"]
            pub fn swift_unknownObjectUnownedTakeAssign(
                dest: *mut root::swift::UnownedReference,
                src: *mut root::swift::UnownedReference,
            ) -> *mut root::swift::UnownedReference;
        }
        extern "C" {
            #[doc = " Return `*ref == value` when ref might not refer to a native Swift object.\n Does not halt when *ref is a dead object as long as *ref != value."]
            pub fn swift_unknownObjectUnownedIsEqual(
                ref_: *mut root::swift::UnownedReference,
                value: *mut ::std::os::raw::c_void,
            ) -> bool;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct TypeNamePair {
            pub data: *const ::std::os::raw::c_char,
            pub length: usize,
        }
        const _: () = {
            ["Size of TypeNamePair"][::std::mem::size_of::<TypeNamePair>() - 16usize];
            ["Alignment of TypeNamePair"][::std::mem::align_of::<TypeNamePair>() - 8usize];
            ["Offset of field: TypeNamePair::data"]
                [::std::mem::offset_of!(TypeNamePair, data) - 0usize];
            ["Offset of field: TypeNamePair::length"]
                [::std::mem::offset_of!(TypeNamePair, length) - 8usize];
        };
        extern "C" {
            #[doc = " Return the name of a Swift type represented by a metadata object.\n func _getTypeName(_ type: Any.Type, qualified: Bool)\n   -> (UnsafePointer<UInt8>, Int)"]
            #[link_name = "\u{1}_swift_getTypeName"]
            pub fn swift_getTypeName(
                type_: *const root::swift::Metadata,
                qualified: bool,
            ) -> root::swift::TypeNamePair;
        }
        extern "C" {
            #[doc = " Return the mangled name of a Swift type represented by a metadata object.\n func _getMangledTypeName(_ type: Any.Type)\n   -> (UnsafePointer<UInt8>, Int)"]
            #[link_name = "\u{1}_swift_getFunctionFullNameFromMangledName"]
            pub fn swift_getFunctionFullNameFromMangledName(
                mangledNameStart: *const ::std::os::raw::c_char,
                mangledNameLength: usize,
            ) -> root::swift::TypeNamePair;
        }
        extern "C" {
            #[doc = " Return the human-readable full name of the mangled function name passed in.\n func _getMangledTypeName(_ mangledName: UnsafePointer<UInt8>,\n                          mangledNameLength: UInt)\n   -> (UnsafePointer<UInt8>, Int)"]
            #[link_name = "\u{1}_swift_getMangledTypeName"]
            pub fn swift_getMangledTypeName(
                type_: *const root::swift::Metadata,
            ) -> root::swift::TypeNamePair;
        }
        extern "C" {
            pub fn swift_rootObjCDealloc(self_: *mut root::swift::HeapObject);
        }
        extern "C" {
            #[link_name = "\u{1}_swift_stdlib_NSStringFromUTF8"]
            pub fn swift_stdlib_NSStringFromUTF8(
                cstr: *const ::std::os::raw::c_char,
                len: ::std::os::raw::c_int,
            ) -> root::id;
        }
    }
    extern "C" {
        pub fn imaxabs(j: root::intmax_t) -> root::intmax_t;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct imaxdiv_t {
        pub quot: root::intmax_t,
        pub rem: root::intmax_t,
    }
    const _: () = {
        ["Size of imaxdiv_t"][::std::mem::size_of::<imaxdiv_t>() - 16usize];
        ["Alignment of imaxdiv_t"][::std::mem::align_of::<imaxdiv_t>() - 8usize];
        ["Offset of field: imaxdiv_t::quot"][::std::mem::offset_of!(imaxdiv_t, quot) - 0usize];
        ["Offset of field: imaxdiv_t::rem"][::std::mem::offset_of!(imaxdiv_t, rem) - 8usize];
    };
    extern "C" {
        pub fn imaxdiv(__numer: root::intmax_t, __denom: root::intmax_t) -> root::imaxdiv_t;
    }
    extern "C" {
        pub fn strtoimax(
            __nptr: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> root::intmax_t;
    }
    extern "C" {
        pub fn strtoumax(
            __nptr: *const ::std::os::raw::c_char,
            __endptr: *mut *mut ::std::os::raw::c_char,
            __base: ::std::os::raw::c_int,
        ) -> root::uintmax_t;
    }
    extern "C" {
        pub fn wcstoimax(
            __nptr: *const u32,
            __endptr: *mut *mut u32,
            __base: ::std::os::raw::c_int,
        ) -> root::intmax_t;
    }
    extern "C" {
        pub fn wcstoumax(
            __nptr: *const u32,
            __endptr: *mut *mut u32,
            __base: ::std::os::raw::c_int,
        ) -> root::uintmax_t;
    }
    pub type mbstate_t = root::__darwin_mbstate_t;
    pub type u_char = ::std::os::raw::c_uchar;
    pub type u_short = ::std::os::raw::c_ushort;
    pub type u_int = ::std::os::raw::c_uint;
    pub type u_long = ::std::os::raw::c_ulong;
    pub type ushort = ::std::os::raw::c_ushort;
    pub type uint = ::std::os::raw::c_uint;
    pub type u_quad_t = root::u_int64_t;
    pub type quad_t = i64;
    pub type qaddr_t = *mut root::quad_t;
    pub type caddr_t = *mut ::std::os::raw::c_char;
    pub type daddr_t = i32;
    pub type fixpt_t = root::u_int32_t;
    pub type blkcnt_t = root::__darwin_blkcnt_t;
    pub type blksize_t = root::__darwin_blksize_t;
    pub type gid_t = root::__darwin_gid_t;
    pub type in_addr_t = root::__uint32_t;
    pub type in_port_t = root::__uint16_t;
    pub type ino_t = root::__darwin_ino_t;
    pub type ino64_t = root::__darwin_ino64_t;
    pub type key_t = root::__int32_t;
    pub type nlink_t = root::__uint16_t;
    pub type off_t = root::__darwin_off_t;
    pub type segsz_t = i32;
    pub type swblk_t = i32;
    pub type useconds_t = root::__darwin_useconds_t;
    pub type suseconds_t = root::__darwin_suseconds_t;
    pub type rsize_t = root::__darwin_size_t;
    pub type errno_t = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct fd_set {
        pub fds_bits: [root::__int32_t; 32usize],
    }
    const _: () = {
        ["Size of fd_set"][::std::mem::size_of::<fd_set>() - 128usize];
        ["Alignment of fd_set"][::std::mem::align_of::<fd_set>() - 4usize];
        ["Offset of field: fd_set::fds_bits"][::std::mem::offset_of!(fd_set, fds_bits) - 0usize];
    };
    extern "C" {
        pub fn __darwin_check_fd_set_overflow(
            arg1: ::std::os::raw::c_int,
            arg2: *const ::std::os::raw::c_void,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    pub type fd_mask = root::__int32_t;
    pub type fsblkcnt_t = root::__darwin_fsblkcnt_t;
    pub type fsfilcnt_t = root::__darwin_fsfilcnt_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct objc_class {
        _unused: [u8; 0],
    }
    pub type Class = *mut root::objc_class;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct objc_object {
        pub isa: root::Class,
    }
    const _: () = {
        ["Size of objc_object"][::std::mem::size_of::<objc_object>() - 8usize];
        ["Alignment of objc_object"][::std::mem::align_of::<objc_object>() - 8usize];
        ["Offset of field: objc_object::isa"][::std::mem::offset_of!(objc_object, isa) - 0usize];
    };
    pub type id = *mut root::objc_object;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct objc_selector {
        _unused: [u8; 0],
    }
    pub type SEL = *mut root::objc_selector;
    pub type IMP = ::std::option::Option<unsafe extern "C" fn()>;
    pub type BOOL = bool;
    pub type objc_zone_t = *mut root::_malloc_zone_t;
    extern "C" {
        pub fn sel_getName(sel: root::SEL) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn sel_registerName(str_: *const ::std::os::raw::c_char) -> root::SEL;
    }
    extern "C" {
        pub fn object_getClassName(obj: root::id) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn object_getIndexedIvars(obj: root::id) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn sel_isMapped(sel: root::SEL) -> root::BOOL;
    }
    extern "C" {
        pub fn sel_getUid(str_: *const ::std::os::raw::c_char) -> root::SEL;
    }
    pub type objc_objectptr_t = *const ::std::os::raw::c_void;
    extern "C" {
        pub fn objc_retainedObject(obj: root::objc_objectptr_t) -> root::id;
    }
    extern "C" {
        pub fn objc_unretainedObject(obj: root::objc_objectptr_t) -> root::id;
    }
    extern "C" {
        pub fn objc_unretainedPointer(obj: root::id) -> root::objc_objectptr_t;
    }
    pub type __swift_int64_t = ::std::os::raw::c_longlong;
    pub type __swift_uint64_t = ::std::os::raw::c_ulonglong;
    pub type __swift_int32_t = ::std::os::raw::c_int;
    pub type __swift_uint32_t = ::std::os::raw::c_uint;
    pub type __swift_int16_t = ::std::os::raw::c_short;
    pub type __swift_uint16_t = ::std::os::raw::c_ushort;
    pub type __swift_int8_t = ::std::os::raw::c_schar;
    pub type __swift_uint8_t = ::std::os::raw::c_uchar;
    pub type __swift_intptr_t = root::__swift_int64_t;
    pub type __swift_uintptr_t = root::__swift_uint64_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct InlineRefCountsPlaceholder {
        pub refCounts: root::__swift_uintptr_t,
    }
    const _: () = {
        ["Size of InlineRefCountsPlaceholder"]
            [::std::mem::size_of::<InlineRefCountsPlaceholder>() - 8usize];
        ["Alignment of InlineRefCountsPlaceholder"]
            [::std::mem::align_of::<InlineRefCountsPlaceholder>() - 8usize];
        ["Offset of field: InlineRefCountsPlaceholder::refCounts"]
            [::std::mem::offset_of!(InlineRefCountsPlaceholder, refCounts) - 0usize];
    };
    pub type va_list = root::__builtin_va_list;
    pub type __gnuc_va_list = root::__builtin_va_list;
    pub type wint_t = root::__darwin_wint_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _RuneEntry {
        pub __min: root::__darwin_rune_t,
        pub __max: root::__darwin_rune_t,
        pub __map: root::__darwin_rune_t,
        pub __types: *mut root::__uint32_t,
    }
    const _: () = {
        ["Size of _RuneEntry"][::std::mem::size_of::<_RuneEntry>() - 24usize];
        ["Alignment of _RuneEntry"][::std::mem::align_of::<_RuneEntry>() - 8usize];
        ["Offset of field: _RuneEntry::__min"][::std::mem::offset_of!(_RuneEntry, __min) - 0usize];
        ["Offset of field: _RuneEntry::__max"][::std::mem::offset_of!(_RuneEntry, __max) - 4usize];
        ["Offset of field: _RuneEntry::__map"][::std::mem::offset_of!(_RuneEntry, __map) - 8usize];
        ["Offset of field: _RuneEntry::__types"]
            [::std::mem::offset_of!(_RuneEntry, __types) - 16usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _RuneRange {
        pub __nranges: ::std::os::raw::c_int,
        pub __ranges: *mut root::_RuneEntry,
    }
    const _: () = {
        ["Size of _RuneRange"][::std::mem::size_of::<_RuneRange>() - 16usize];
        ["Alignment of _RuneRange"][::std::mem::align_of::<_RuneRange>() - 8usize];
        ["Offset of field: _RuneRange::__nranges"]
            [::std::mem::offset_of!(_RuneRange, __nranges) - 0usize];
        ["Offset of field: _RuneRange::__ranges"]
            [::std::mem::offset_of!(_RuneRange, __ranges) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _RuneCharClass {
        pub __name: [::std::os::raw::c_char; 14usize],
        pub __mask: root::__uint32_t,
    }
    const _: () = {
        ["Size of _RuneCharClass"][::std::mem::size_of::<_RuneCharClass>() - 20usize];
        ["Alignment of _RuneCharClass"][::std::mem::align_of::<_RuneCharClass>() - 4usize];
        ["Offset of field: _RuneCharClass::__name"]
            [::std::mem::offset_of!(_RuneCharClass, __name) - 0usize];
        ["Offset of field: _RuneCharClass::__mask"]
            [::std::mem::offset_of!(_RuneCharClass, __mask) - 16usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _RuneLocale {
        pub __magic: [::std::os::raw::c_char; 8usize],
        pub __encoding: [::std::os::raw::c_char; 32usize],
        pub __sgetrune: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_char,
                arg2: root::__darwin_size_t,
                arg3: *mut *const ::std::os::raw::c_char,
            ) -> root::__darwin_rune_t,
        >,
        pub __sputrune: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: root::__darwin_rune_t,
                arg2: *mut ::std::os::raw::c_char,
                arg3: root::__darwin_size_t,
                arg4: *mut *mut ::std::os::raw::c_char,
            ) -> ::std::os::raw::c_int,
        >,
        pub __invalid_rune: root::__darwin_rune_t,
        pub __runetype: [root::__uint32_t; 256usize],
        pub __maplower: [root::__darwin_rune_t; 256usize],
        pub __mapupper: [root::__darwin_rune_t; 256usize],
        pub __runetype_ext: root::_RuneRange,
        pub __maplower_ext: root::_RuneRange,
        pub __mapupper_ext: root::_RuneRange,
        pub __variable: *mut ::std::os::raw::c_void,
        pub __variable_len: ::std::os::raw::c_int,
        pub __ncharclasses: ::std::os::raw::c_int,
        pub __charclasses: *mut root::_RuneCharClass,
    }
    const _: () = {
        ["Size of _RuneLocale"][::std::mem::size_of::<_RuneLocale>() - 3208usize];
        ["Alignment of _RuneLocale"][::std::mem::align_of::<_RuneLocale>() - 8usize];
        ["Offset of field: _RuneLocale::__magic"]
            [::std::mem::offset_of!(_RuneLocale, __magic) - 0usize];
        ["Offset of field: _RuneLocale::__encoding"]
            [::std::mem::offset_of!(_RuneLocale, __encoding) - 8usize];
        ["Offset of field: _RuneLocale::__sgetrune"]
            [::std::mem::offset_of!(_RuneLocale, __sgetrune) - 40usize];
        ["Offset of field: _RuneLocale::__sputrune"]
            [::std::mem::offset_of!(_RuneLocale, __sputrune) - 48usize];
        ["Offset of field: _RuneLocale::__invalid_rune"]
            [::std::mem::offset_of!(_RuneLocale, __invalid_rune) - 56usize];
        ["Offset of field: _RuneLocale::__runetype"]
            [::std::mem::offset_of!(_RuneLocale, __runetype) - 60usize];
        ["Offset of field: _RuneLocale::__maplower"]
            [::std::mem::offset_of!(_RuneLocale, __maplower) - 1084usize];
        ["Offset of field: _RuneLocale::__mapupper"]
            [::std::mem::offset_of!(_RuneLocale, __mapupper) - 2108usize];
        ["Offset of field: _RuneLocale::__runetype_ext"]
            [::std::mem::offset_of!(_RuneLocale, __runetype_ext) - 3136usize];
        ["Offset of field: _RuneLocale::__maplower_ext"]
            [::std::mem::offset_of!(_RuneLocale, __maplower_ext) - 3152usize];
        ["Offset of field: _RuneLocale::__mapupper_ext"]
            [::std::mem::offset_of!(_RuneLocale, __mapupper_ext) - 3168usize];
        ["Offset of field: _RuneLocale::__variable"]
            [::std::mem::offset_of!(_RuneLocale, __variable) - 3184usize];
        ["Offset of field: _RuneLocale::__variable_len"]
            [::std::mem::offset_of!(_RuneLocale, __variable_len) - 3192usize];
        ["Offset of field: _RuneLocale::__ncharclasses"]
            [::std::mem::offset_of!(_RuneLocale, __ncharclasses) - 3196usize];
        ["Offset of field: _RuneLocale::__charclasses"]
            [::std::mem::offset_of!(_RuneLocale, __charclasses) - 3200usize];
    };
    extern "C" {
        pub static mut _DefaultRuneLocale: root::_RuneLocale;
    }
    extern "C" {
        pub static mut _CurrentRuneLocale: *mut root::_RuneLocale;
    }
    extern "C" {
        pub fn ___runetype(arg1: root::__darwin_ct_rune_t) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn ___tolower(arg1: root::__darwin_ct_rune_t) -> root::__darwin_ct_rune_t;
    }
    extern "C" {
        pub fn ___toupper(arg1: root::__darwin_ct_rune_t) -> root::__darwin_ct_rune_t;
    }
    extern "C" {
        pub fn __maskrune(
            arg1: root::__darwin_ct_rune_t,
            arg2: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn __toupper(arg1: root::__darwin_ct_rune_t) -> root::__darwin_ct_rune_t;
    }
    extern "C" {
        pub fn __tolower(arg1: root::__darwin_ct_rune_t) -> root::__darwin_ct_rune_t;
    }
    pub type wctrans_t = root::__darwin_wctrans_t;
    pub type wctype_t = root::__darwin_wctype_t;
    extern "C" {
        pub fn wctype(arg1: *const ::std::os::raw::c_char) -> root::wctype_t;
    }
    extern "C" {
        pub fn nextwctype(arg1: root::wint_t, arg2: root::wctype_t) -> root::wint_t;
    }
    extern "C" {
        pub fn towctrans(arg1: root::wint_t, arg2: root::wctrans_t) -> root::wint_t;
    }
    extern "C" {
        pub fn wctrans(arg1: *const ::std::os::raw::c_char) -> root::wctrans_t;
    }
    extern "C" {
        pub fn renameat(
            arg1: ::std::os::raw::c_int,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
            arg4: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn renamex_np(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn renameatx_np(
            arg1: ::std::os::raw::c_int,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
            arg4: *const ::std::os::raw::c_char,
            arg5: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    pub type fpos_t = root::__darwin_off_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __sbuf {
        pub _base: *mut ::std::os::raw::c_uchar,
        pub _size: ::std::os::raw::c_int,
    }
    const _: () = {
        ["Size of __sbuf"][::std::mem::size_of::<__sbuf>() - 16usize];
        ["Alignment of __sbuf"][::std::mem::align_of::<__sbuf>() - 8usize];
        ["Offset of field: __sbuf::_base"][::std::mem::offset_of!(__sbuf, _base) - 0usize];
        ["Offset of field: __sbuf::_size"][::std::mem::offset_of!(__sbuf, _size) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __sFILEX {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __sFILE {
        pub _p: *mut ::std::os::raw::c_uchar,
        pub _r: ::std::os::raw::c_int,
        pub _w: ::std::os::raw::c_int,
        pub _flags: ::std::os::raw::c_short,
        pub _file: ::std::os::raw::c_short,
        pub _bf: root::__sbuf,
        pub _lbfsize: ::std::os::raw::c_int,
        pub _cookie: *mut ::std::os::raw::c_void,
        pub _close: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        >,
        pub _read: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut ::std::os::raw::c_char,
                arg3: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub _seek: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: root::fpos_t,
                arg3: ::std::os::raw::c_int,
            ) -> root::fpos_t,
        >,
        pub _write: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_char,
                arg3: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub _ub: root::__sbuf,
        pub _extra: *mut root::__sFILEX,
        pub _ur: ::std::os::raw::c_int,
        pub _ubuf: [::std::os::raw::c_uchar; 3usize],
        pub _nbuf: [::std::os::raw::c_uchar; 1usize],
        pub _lb: root::__sbuf,
        pub _blksize: ::std::os::raw::c_int,
        pub _offset: root::fpos_t,
    }
    const _: () = {
        ["Size of __sFILE"][::std::mem::size_of::<__sFILE>() - 152usize];
        ["Alignment of __sFILE"][::std::mem::align_of::<__sFILE>() - 8usize];
        ["Offset of field: __sFILE::_p"][::std::mem::offset_of!(__sFILE, _p) - 0usize];
        ["Offset of field: __sFILE::_r"][::std::mem::offset_of!(__sFILE, _r) - 8usize];
        ["Offset of field: __sFILE::_w"][::std::mem::offset_of!(__sFILE, _w) - 12usize];
        ["Offset of field: __sFILE::_flags"][::std::mem::offset_of!(__sFILE, _flags) - 16usize];
        ["Offset of field: __sFILE::_file"][::std::mem::offset_of!(__sFILE, _file) - 18usize];
        ["Offset of field: __sFILE::_bf"][::std::mem::offset_of!(__sFILE, _bf) - 24usize];
        ["Offset of field: __sFILE::_lbfsize"][::std::mem::offset_of!(__sFILE, _lbfsize) - 40usize];
        ["Offset of field: __sFILE::_cookie"][::std::mem::offset_of!(__sFILE, _cookie) - 48usize];
        ["Offset of field: __sFILE::_close"][::std::mem::offset_of!(__sFILE, _close) - 56usize];
        ["Offset of field: __sFILE::_read"][::std::mem::offset_of!(__sFILE, _read) - 64usize];
        ["Offset of field: __sFILE::_seek"][::std::mem::offset_of!(__sFILE, _seek) - 72usize];
        ["Offset of field: __sFILE::_write"][::std::mem::offset_of!(__sFILE, _write) - 80usize];
        ["Offset of field: __sFILE::_ub"][::std::mem::offset_of!(__sFILE, _ub) - 88usize];
        ["Offset of field: __sFILE::_extra"][::std::mem::offset_of!(__sFILE, _extra) - 104usize];
        ["Offset of field: __sFILE::_ur"][::std::mem::offset_of!(__sFILE, _ur) - 112usize];
        ["Offset of field: __sFILE::_ubuf"][::std::mem::offset_of!(__sFILE, _ubuf) - 116usize];
        ["Offset of field: __sFILE::_nbuf"][::std::mem::offset_of!(__sFILE, _nbuf) - 119usize];
        ["Offset of field: __sFILE::_lb"][::std::mem::offset_of!(__sFILE, _lb) - 120usize];
        ["Offset of field: __sFILE::_blksize"]
            [::std::mem::offset_of!(__sFILE, _blksize) - 136usize];
        ["Offset of field: __sFILE::_offset"][::std::mem::offset_of!(__sFILE, _offset) - 144usize];
    };
    pub type FILE = root::__sFILE;
    extern "C" {
        pub static mut __stdinp: *mut root::FILE;
    }
    extern "C" {
        pub static mut __stdoutp: *mut root::FILE;
    }
    extern "C" {
        pub static mut __stderrp: *mut root::FILE;
    }
    extern "C" {
        pub fn clearerr(arg1: *mut root::FILE);
    }
    extern "C" {
        pub fn fclose(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn feof(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ferror(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fflush(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fgetc(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fgetpos(arg1: *mut root::FILE, arg2: *mut root::fpos_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fgets(
            arg1: *mut ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_int,
            arg3: *mut root::FILE,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn fopen(
            __filename: *const ::std::os::raw::c_char,
            __mode: *const ::std::os::raw::c_char,
        ) -> *mut root::FILE;
    }
    extern "C" {
        pub fn fprintf(
            arg1: *mut root::FILE,
            arg2: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fputc(arg1: ::std::os::raw::c_int, arg2: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fputs(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut root::FILE,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fread(
            __ptr: *mut ::std::os::raw::c_void,
            __size: usize,
            __nitems: usize,
            __stream: *mut root::FILE,
        ) -> usize;
    }
    extern "C" {
        pub fn freopen(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: *mut root::FILE,
        ) -> *mut root::FILE;
    }
    extern "C" {
        pub fn fscanf(
            arg1: *mut root::FILE,
            arg2: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fseek(
            arg1: *mut root::FILE,
            arg2: ::std::os::raw::c_long,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fsetpos(arg1: *mut root::FILE, arg2: *const root::fpos_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ftell(arg1: *mut root::FILE) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn fwrite(
            __ptr: *const ::std::os::raw::c_void,
            __size: usize,
            __nitems: usize,
            __stream: *mut root::FILE,
        ) -> usize;
    }
    extern "C" {
        pub fn getc(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn getchar() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gets(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn perror(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn printf(arg1: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn putc(arg1: ::std::os::raw::c_int, arg2: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn putchar(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn puts(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn remove(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn rename(
            __old: *const ::std::os::raw::c_char,
            __new: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn rewind(arg1: *mut root::FILE);
    }
    extern "C" {
        pub fn scanf(arg1: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn setbuf(arg1: *mut root::FILE, arg2: *mut ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn setvbuf(
            arg1: *mut root::FILE,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
            arg4: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn sprintf(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn sscanf(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn tmpfile() -> *mut root::FILE;
    }
    extern "C" {
        pub fn tmpnam(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn ungetc(arg1: ::std::os::raw::c_int, arg2: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vfprintf(
            arg1: *mut root::FILE,
            arg2: *const ::std::os::raw::c_char,
            arg3: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vprintf(
            arg1: *const ::std::os::raw::c_char,
            arg2: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vsprintf(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ctermid(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn fdopen(
            arg1: ::std::os::raw::c_int,
            arg2: *const ::std::os::raw::c_char,
        ) -> *mut root::FILE;
    }
    extern "C" {
        pub fn fileno(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn pclose(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn popen(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
        ) -> *mut root::FILE;
    }
    extern "C" {
        pub fn __srget(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn __svfscanf(
            arg1: *mut root::FILE,
            arg2: *const ::std::os::raw::c_char,
            arg3: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn __swbuf(arg1: ::std::os::raw::c_int, arg2: *mut root::FILE)
            -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn flockfile(arg1: *mut root::FILE);
    }
    extern "C" {
        pub fn ftrylockfile(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn funlockfile(arg1: *mut root::FILE);
    }
    extern "C" {
        pub fn getc_unlocked(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn getchar_unlocked() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn putc_unlocked(
            arg1: ::std::os::raw::c_int,
            arg2: *mut root::FILE,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn putchar_unlocked(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn getw(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn putw(arg1: ::std::os::raw::c_int, arg2: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn tempnam(
            __dir: *const ::std::os::raw::c_char,
            __prefix: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn fseeko(
            __stream: *mut root::FILE,
            __offset: root::off_t,
            __whence: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ftello(__stream: *mut root::FILE) -> root::off_t;
    }
    extern "C" {
        pub fn snprintf(
            __str: *mut ::std::os::raw::c_char,
            __size: usize,
            __format: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vfscanf(
            __stream: *mut root::FILE,
            __format: *const ::std::os::raw::c_char,
            arg1: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vscanf(
            __format: *const ::std::os::raw::c_char,
            arg1: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vsnprintf(
            __str: *mut ::std::os::raw::c_char,
            __size: usize,
            __format: *const ::std::os::raw::c_char,
            arg1: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vsscanf(
            __str: *const ::std::os::raw::c_char,
            __format: *const ::std::os::raw::c_char,
            arg1: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn dprintf(
            arg1: ::std::os::raw::c_int,
            arg2: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vdprintf(
            arg1: ::std::os::raw::c_int,
            arg2: *const ::std::os::raw::c_char,
            arg3: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn getdelim(
            __linep: *mut *mut ::std::os::raw::c_char,
            __linecapp: *mut usize,
            __delimiter: ::std::os::raw::c_int,
            __stream: *mut root::FILE,
        ) -> isize;
    }
    extern "C" {
        pub fn getline(
            __linep: *mut *mut ::std::os::raw::c_char,
            __linecapp: *mut usize,
            __stream: *mut root::FILE,
        ) -> isize;
    }
    extern "C" {
        pub fn fmemopen(
            __buf: *mut ::std::os::raw::c_void,
            __size: usize,
            __mode: *const ::std::os::raw::c_char,
        ) -> *mut root::FILE;
    }
    extern "C" {
        pub fn open_memstream(
            __bufp: *mut *mut ::std::os::raw::c_char,
            __sizep: *mut usize,
        ) -> *mut root::FILE;
    }
    extern "C" {
        pub static sys_nerr: ::std::os::raw::c_int;
    }
    extern "C" {
        pub static sys_errlist: [*const ::std::os::raw::c_char; 0usize];
    }
    extern "C" {
        pub fn asprintf(
            arg1: *mut *mut ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ctermid_r(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn fgetln(arg1: *mut root::FILE, arg2: *mut usize) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn fmtcheck(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn fpurge(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn setbuffer(
            arg1: *mut root::FILE,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn setlinebuf(arg1: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vasprintf(
            arg1: *mut *mut ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn funopen(
            arg1: *const ::std::os::raw::c_void,
            arg2: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::std::os::raw::c_void,
                    arg2: *mut ::std::os::raw::c_char,
                    arg3: ::std::os::raw::c_int,
                ) -> ::std::os::raw::c_int,
            >,
            arg3: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::std::os::raw::c_void,
                    arg2: *const ::std::os::raw::c_char,
                    arg3: ::std::os::raw::c_int,
                ) -> ::std::os::raw::c_int,
            >,
            arg4: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::std::os::raw::c_void,
                    arg2: root::fpos_t,
                    arg3: ::std::os::raw::c_int,
                ) -> root::fpos_t,
            >,
            arg5: ::std::option::Option<
                unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
            >,
        ) -> *mut root::FILE;
    }
    extern "C" {
        pub fn btowc(arg1: ::std::os::raw::c_int) -> root::wint_t;
    }
    extern "C" {
        pub fn fgetwc(arg1: *mut root::FILE) -> root::wint_t;
    }
    extern "C" {
        pub fn fgetws(
            arg1: *mut u32,
            arg2: ::std::os::raw::c_int,
            arg3: *mut root::FILE,
        ) -> *mut u32;
    }
    extern "C" {
        pub fn fputwc(arg1: u32, arg2: *mut root::FILE) -> root::wint_t;
    }
    extern "C" {
        pub fn fputws(arg1: *const u32, arg2: *mut root::FILE) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fwide(arg1: *mut root::FILE, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fwprintf(arg1: *mut root::FILE, arg2: *const u32, ...) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fwscanf(arg1: *mut root::FILE, arg2: *const u32, ...) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn getwc(arg1: *mut root::FILE) -> root::wint_t;
    }
    extern "C" {
        pub fn getwchar() -> root::wint_t;
    }
    extern "C" {
        pub fn mbrlen(
            arg1: *const ::std::os::raw::c_char,
            arg2: usize,
            arg3: *mut root::mbstate_t,
        ) -> usize;
    }
    extern "C" {
        pub fn mbrtowc(
            arg1: *mut u32,
            arg2: *const ::std::os::raw::c_char,
            arg3: usize,
            arg4: *mut root::mbstate_t,
        ) -> usize;
    }
    extern "C" {
        pub fn mbsinit(arg1: *const root::mbstate_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mbsrtowcs(
            arg1: *mut u32,
            arg2: *mut *const ::std::os::raw::c_char,
            arg3: usize,
            arg4: *mut root::mbstate_t,
        ) -> usize;
    }
    extern "C" {
        pub fn putwc(arg1: u32, arg2: *mut root::FILE) -> root::wint_t;
    }
    extern "C" {
        pub fn putwchar(arg1: u32) -> root::wint_t;
    }
    extern "C" {
        pub fn swprintf(
            arg1: *mut u32,
            arg2: usize,
            arg3: *const u32,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn swscanf(arg1: *const u32, arg2: *const u32, ...) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ungetwc(arg1: root::wint_t, arg2: *mut root::FILE) -> root::wint_t;
    }
    extern "C" {
        pub fn vfwprintf(
            arg1: *mut root::FILE,
            arg2: *const u32,
            arg3: root::__darwin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vswprintf(
            arg1: *mut u32,
            arg2: usize,
            arg3: *const u32,
            arg4: root::__darwin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vwprintf(arg1: *const u32, arg2: root::__darwin_va_list) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcrtomb(
            arg1: *mut ::std::os::raw::c_char,
            arg2: u32,
            arg3: *mut root::mbstate_t,
        ) -> usize;
    }
    extern "C" {
        pub fn wcscat(arg1: *mut u32, arg2: *const u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcschr(arg1: *const u32, arg2: u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcscmp(arg1: *const u32, arg2: *const u32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcscoll(arg1: *const u32, arg2: *const u32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcscpy(arg1: *mut u32, arg2: *const u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcscspn(arg1: *const u32, arg2: *const u32) -> usize;
    }
    extern "C" {
        pub fn wcsftime(
            arg1: *mut u32,
            arg2: usize,
            arg3: *const u32,
            arg4: *const root::tm,
        ) -> usize;
    }
    extern "C" {
        pub fn wcslen(arg1: *const u32) -> usize;
    }
    extern "C" {
        pub fn wcsncat(arg1: *mut u32, arg2: *const u32, arg3: usize) -> *mut u32;
    }
    extern "C" {
        pub fn wcsncmp(arg1: *const u32, arg2: *const u32, arg3: usize) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcsncpy(arg1: *mut u32, arg2: *const u32, arg3: usize) -> *mut u32;
    }
    extern "C" {
        pub fn wcspbrk(arg1: *const u32, arg2: *const u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcsrchr(arg1: *const u32, arg2: u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcsrtombs(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *mut *const u32,
            arg3: usize,
            arg4: *mut root::mbstate_t,
        ) -> usize;
    }
    extern "C" {
        pub fn wcsspn(arg1: *const u32, arg2: *const u32) -> usize;
    }
    extern "C" {
        pub fn wcsstr(arg1: *const u32, arg2: *const u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcsxfrm(arg1: *mut u32, arg2: *const u32, arg3: usize) -> usize;
    }
    extern "C" {
        pub fn wctob(arg1: root::wint_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcstod(arg1: *const u32, arg2: *mut *mut u32) -> f64;
    }
    extern "C" {
        pub fn wcstok(arg1: *mut u32, arg2: *const u32, arg3: *mut *mut u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcstol(
            arg1: *const u32,
            arg2: *mut *mut u32,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn wcstoul(
            arg1: *const u32,
            arg2: *mut *mut u32,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn wmemchr(arg1: *const u32, arg2: u32, arg3: usize) -> *mut u32;
    }
    extern "C" {
        pub fn wmemcmp(arg1: *const u32, arg2: *const u32, arg3: usize) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wmemcpy(arg1: *mut u32, arg2: *const u32, arg3: usize) -> *mut u32;
    }
    extern "C" {
        pub fn wmemmove(arg1: *mut u32, arg2: *const u32, arg3: usize) -> *mut u32;
    }
    extern "C" {
        pub fn wmemset(arg1: *mut u32, arg2: u32, arg3: usize) -> *mut u32;
    }
    extern "C" {
        pub fn wprintf(arg1: *const u32, ...) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wscanf(arg1: *const u32, ...) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcswidth(arg1: *const u32, arg2: usize) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcwidth(arg1: u32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vfwscanf(
            arg1: *mut root::FILE,
            arg2: *const u32,
            arg3: root::__darwin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vswscanf(
            arg1: *const u32,
            arg2: *const u32,
            arg3: root::__darwin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vwscanf(arg1: *const u32, arg2: root::__darwin_va_list) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcstof(arg1: *const u32, arg2: *mut *mut u32) -> f32;
    }
    extern "C" {
        pub fn wcstold(arg1: *const u32, arg2: *mut *mut u32) -> f64;
    }
    extern "C" {
        pub fn wcstoll(
            arg1: *const u32,
            arg2: *mut *mut u32,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn wcstoull(
            arg1: *const u32,
            arg2: *mut *mut u32,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_ulonglong;
    }
    extern "C" {
        pub fn mbsnrtowcs(
            arg1: *mut u32,
            arg2: *mut *const ::std::os::raw::c_char,
            arg3: usize,
            arg4: usize,
            arg5: *mut root::mbstate_t,
        ) -> usize;
    }
    extern "C" {
        pub fn wcpcpy(arg1: *mut u32, arg2: *const u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcpncpy(arg1: *mut u32, arg2: *const u32, arg3: usize) -> *mut u32;
    }
    extern "C" {
        pub fn wcsdup(arg1: *const u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcscasecmp(arg1: *const u32, arg2: *const u32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcsncasecmp(arg1: *const u32, arg2: *const u32, n: usize) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcsnlen(arg1: *const u32, arg2: usize) -> usize;
    }
    extern "C" {
        pub fn wcsnrtombs(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *mut *const u32,
            arg3: usize,
            arg4: usize,
            arg5: *mut root::mbstate_t,
        ) -> usize;
    }
    extern "C" {
        pub fn open_wmemstream(__bufp: *mut *mut u32, __sizep: *mut usize) -> *mut root::FILE;
    }
    extern "C" {
        pub fn fgetwln(arg1: *mut root::FILE, arg2: *mut usize) -> *mut u32;
    }
    extern "C" {
        pub fn wcslcat(arg1: *mut u32, arg2: *const u32, arg3: usize) -> usize;
    }
    extern "C" {
        pub fn wcslcpy(arg1: *mut u32, arg2: *const u32, arg3: usize) -> usize;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct lconv {
        pub decimal_point: *mut ::std::os::raw::c_char,
        pub thousands_sep: *mut ::std::os::raw::c_char,
        pub grouping: *mut ::std::os::raw::c_char,
        pub int_curr_symbol: *mut ::std::os::raw::c_char,
        pub currency_symbol: *mut ::std::os::raw::c_char,
        pub mon_decimal_point: *mut ::std::os::raw::c_char,
        pub mon_thousands_sep: *mut ::std::os::raw::c_char,
        pub mon_grouping: *mut ::std::os::raw::c_char,
        pub positive_sign: *mut ::std::os::raw::c_char,
        pub negative_sign: *mut ::std::os::raw::c_char,
        pub int_frac_digits: ::std::os::raw::c_char,
        pub frac_digits: ::std::os::raw::c_char,
        pub p_cs_precedes: ::std::os::raw::c_char,
        pub p_sep_by_space: ::std::os::raw::c_char,
        pub n_cs_precedes: ::std::os::raw::c_char,
        pub n_sep_by_space: ::std::os::raw::c_char,
        pub p_sign_posn: ::std::os::raw::c_char,
        pub n_sign_posn: ::std::os::raw::c_char,
        pub int_p_cs_precedes: ::std::os::raw::c_char,
        pub int_n_cs_precedes: ::std::os::raw::c_char,
        pub int_p_sep_by_space: ::std::os::raw::c_char,
        pub int_n_sep_by_space: ::std::os::raw::c_char,
        pub int_p_sign_posn: ::std::os::raw::c_char,
        pub int_n_sign_posn: ::std::os::raw::c_char,
    }
    const _: () = {
        ["Size of lconv"][::std::mem::size_of::<lconv>() - 96usize];
        ["Alignment of lconv"][::std::mem::align_of::<lconv>() - 8usize];
        ["Offset of field: lconv::decimal_point"]
            [::std::mem::offset_of!(lconv, decimal_point) - 0usize];
        ["Offset of field: lconv::thousands_sep"]
            [::std::mem::offset_of!(lconv, thousands_sep) - 8usize];
        ["Offset of field: lconv::grouping"][::std::mem::offset_of!(lconv, grouping) - 16usize];
        ["Offset of field: lconv::int_curr_symbol"]
            [::std::mem::offset_of!(lconv, int_curr_symbol) - 24usize];
        ["Offset of field: lconv::currency_symbol"]
            [::std::mem::offset_of!(lconv, currency_symbol) - 32usize];
        ["Offset of field: lconv::mon_decimal_point"]
            [::std::mem::offset_of!(lconv, mon_decimal_point) - 40usize];
        ["Offset of field: lconv::mon_thousands_sep"]
            [::std::mem::offset_of!(lconv, mon_thousands_sep) - 48usize];
        ["Offset of field: lconv::mon_grouping"]
            [::std::mem::offset_of!(lconv, mon_grouping) - 56usize];
        ["Offset of field: lconv::positive_sign"]
            [::std::mem::offset_of!(lconv, positive_sign) - 64usize];
        ["Offset of field: lconv::negative_sign"]
            [::std::mem::offset_of!(lconv, negative_sign) - 72usize];
        ["Offset of field: lconv::int_frac_digits"]
            [::std::mem::offset_of!(lconv, int_frac_digits) - 80usize];
        ["Offset of field: lconv::frac_digits"]
            [::std::mem::offset_of!(lconv, frac_digits) - 81usize];
        ["Offset of field: lconv::p_cs_precedes"]
            [::std::mem::offset_of!(lconv, p_cs_precedes) - 82usize];
        ["Offset of field: lconv::p_sep_by_space"]
            [::std::mem::offset_of!(lconv, p_sep_by_space) - 83usize];
        ["Offset of field: lconv::n_cs_precedes"]
            [::std::mem::offset_of!(lconv, n_cs_precedes) - 84usize];
        ["Offset of field: lconv::n_sep_by_space"]
            [::std::mem::offset_of!(lconv, n_sep_by_space) - 85usize];
        ["Offset of field: lconv::p_sign_posn"]
            [::std::mem::offset_of!(lconv, p_sign_posn) - 86usize];
        ["Offset of field: lconv::n_sign_posn"]
            [::std::mem::offset_of!(lconv, n_sign_posn) - 87usize];
        ["Offset of field: lconv::int_p_cs_precedes"]
            [::std::mem::offset_of!(lconv, int_p_cs_precedes) - 88usize];
        ["Offset of field: lconv::int_n_cs_precedes"]
            [::std::mem::offset_of!(lconv, int_n_cs_precedes) - 89usize];
        ["Offset of field: lconv::int_p_sep_by_space"]
            [::std::mem::offset_of!(lconv, int_p_sep_by_space) - 90usize];
        ["Offset of field: lconv::int_n_sep_by_space"]
            [::std::mem::offset_of!(lconv, int_n_sep_by_space) - 91usize];
        ["Offset of field: lconv::int_p_sign_posn"]
            [::std::mem::offset_of!(lconv, int_p_sign_posn) - 92usize];
        ["Offset of field: lconv::int_n_sign_posn"]
            [::std::mem::offset_of!(lconv, int_n_sign_posn) - 93usize];
    };
    extern "C" {
        pub fn localeconv() -> *mut root::lconv;
    }
    extern "C" {
        pub fn setlocale(
            arg1: ::std::os::raw::c_int,
            arg2: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _xlocale {
        _unused: [u8; 0],
    }
    pub type locale_t = *mut root::_xlocale;
    extern "C" {
        pub fn ___mb_cur_max() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ___mb_cur_max_l(arg1: root::locale_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub static _c_locale: root::locale_t;
    }
    extern "C" {
        pub fn duplocale(arg1: root::locale_t) -> root::locale_t;
    }
    extern "C" {
        pub fn freelocale(arg1: root::locale_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn localeconv_l(arg1: root::locale_t) -> *mut root::lconv;
    }
    extern "C" {
        pub fn newlocale(
            arg1: ::std::os::raw::c_int,
            arg2: *const ::std::os::raw::c_char,
            arg3: root::locale_t,
        ) -> root::locale_t;
    }
    extern "C" {
        pub fn querylocale(
            arg1: ::std::os::raw::c_int,
            arg2: root::locale_t,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn uselocale(arg1: root::locale_t) -> root::locale_t;
    }
    extern "C" {
        pub fn ___runetype_l(
            arg1: root::__darwin_ct_rune_t,
            arg2: root::locale_t,
        ) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn ___tolower_l(
            arg1: root::__darwin_ct_rune_t,
            arg2: root::locale_t,
        ) -> root::__darwin_ct_rune_t;
    }
    extern "C" {
        pub fn ___toupper_l(
            arg1: root::__darwin_ct_rune_t,
            arg2: root::locale_t,
        ) -> root::__darwin_ct_rune_t;
    }
    extern "C" {
        pub fn __maskrune_l(
            arg1: root::__darwin_ct_rune_t,
            arg2: ::std::os::raw::c_ulong,
            arg3: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wctype_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: root::locale_t,
        ) -> root::wctype_t;
    }
    extern "C" {
        pub fn strtoimax_l(
            nptr: *const ::std::os::raw::c_char,
            endptr: *mut *mut ::std::os::raw::c_char,
            base: ::std::os::raw::c_int,
            arg1: root::locale_t,
        ) -> root::intmax_t;
    }
    extern "C" {
        pub fn strtoumax_l(
            nptr: *const ::std::os::raw::c_char,
            endptr: *mut *mut ::std::os::raw::c_char,
            base: ::std::os::raw::c_int,
            arg1: root::locale_t,
        ) -> root::uintmax_t;
    }
    extern "C" {
        pub fn wcstoimax_l(
            nptr: *const u32,
            endptr: *mut *mut u32,
            base: ::std::os::raw::c_int,
            arg1: root::locale_t,
        ) -> root::intmax_t;
    }
    extern "C" {
        pub fn wcstoumax_l(
            nptr: *const u32,
            endptr: *mut *mut u32,
            base: ::std::os::raw::c_int,
            arg1: root::locale_t,
        ) -> root::uintmax_t;
    }
    extern "C" {
        pub fn fprintf_l(
            arg1: *mut root::FILE,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fscanf_l(
            arg1: *mut root::FILE,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn printf_l(
            arg1: root::locale_t,
            arg2: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn scanf_l(
            arg1: root::locale_t,
            arg2: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn sprintf_l(
            arg1: *mut ::std::os::raw::c_char,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn sscanf_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vfprintf_l(
            arg1: *mut root::FILE,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            arg4: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vprintf_l(
            arg1: root::locale_t,
            arg2: *const ::std::os::raw::c_char,
            arg3: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vsprintf_l(
            arg1: *mut ::std::os::raw::c_char,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            arg4: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn snprintf_l(
            arg1: *mut ::std::os::raw::c_char,
            arg2: usize,
            arg3: root::locale_t,
            arg4: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vfscanf_l(
            arg1: *mut root::FILE,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            arg4: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vscanf_l(
            arg1: root::locale_t,
            arg2: *const ::std::os::raw::c_char,
            arg3: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vsnprintf_l(
            arg1: *mut ::std::os::raw::c_char,
            arg2: usize,
            arg3: root::locale_t,
            arg4: *const ::std::os::raw::c_char,
            arg5: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vsscanf_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            arg4: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn dprintf_l(
            arg1: ::std::os::raw::c_int,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vdprintf_l(
            arg1: ::std::os::raw::c_int,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            arg4: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn asprintf_l(
            arg1: *mut *mut ::std::os::raw::c_char,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vasprintf_l(
            arg1: *mut *mut ::std::os::raw::c_char,
            arg2: root::locale_t,
            arg3: *const ::std::os::raw::c_char,
            arg4: root::va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn atof_l(arg1: *const ::std::os::raw::c_char, arg2: root::locale_t) -> f64;
    }
    extern "C" {
        pub fn atoi_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn atol_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: root::locale_t,
        ) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn atoll_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: root::locale_t,
        ) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn mblen_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: usize,
            arg3: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mbstowcs_l(
            arg1: *mut u32,
            arg2: *const ::std::os::raw::c_char,
            arg3: usize,
            arg4: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn mbtowc_l(
            arg1: *mut u32,
            arg2: *const ::std::os::raw::c_char,
            arg3: usize,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strtod_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
            arg3: root::locale_t,
        ) -> f64;
    }
    extern "C" {
        pub fn strtof_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
            arg3: root::locale_t,
        ) -> f32;
    }
    extern "C" {
        pub fn strtol_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn strtold_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
            arg3: root::locale_t,
        ) -> f64;
    }
    extern "C" {
        pub fn strtoll_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn strtoq_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn strtoul_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn strtoull_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_ulonglong;
    }
    extern "C" {
        pub fn strtouq_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_ulonglong;
    }
    extern "C" {
        pub fn wcstombs_l(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *const u32,
            arg3: usize,
            arg4: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn wctomb_l(
            arg1: *mut ::std::os::raw::c_char,
            arg2: u32,
            arg3: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strcoll_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strxfrm_l(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: usize,
            arg4: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn strcasecmp_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strcasestr_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: root::locale_t,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn strncasecmp_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: usize,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn strftime_l(
            arg1: *mut ::std::os::raw::c_char,
            arg2: usize,
            arg3: *const ::std::os::raw::c_char,
            arg4: *const root::tm,
            arg5: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn strptime_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: *mut root::tm,
            arg4: root::locale_t,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn btowc_l(arg1: ::std::os::raw::c_int, arg2: root::locale_t) -> root::wint_t;
    }
    extern "C" {
        pub fn fgetwc_l(arg1: *mut root::FILE, arg2: root::locale_t) -> root::wint_t;
    }
    extern "C" {
        pub fn fgetws_l(
            arg1: *mut u32,
            arg2: ::std::os::raw::c_int,
            arg3: *mut root::FILE,
            arg4: root::locale_t,
        ) -> *mut u32;
    }
    extern "C" {
        pub fn fputwc_l(arg1: u32, arg2: *mut root::FILE, arg3: root::locale_t) -> root::wint_t;
    }
    extern "C" {
        pub fn fputws_l(
            arg1: *const u32,
            arg2: *mut root::FILE,
            arg3: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fwprintf_l(
            arg1: *mut root::FILE,
            arg2: root::locale_t,
            arg3: *const u32,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn fwscanf_l(
            arg1: *mut root::FILE,
            arg2: root::locale_t,
            arg3: *const u32,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn getwc_l(arg1: *mut root::FILE, arg2: root::locale_t) -> root::wint_t;
    }
    extern "C" {
        pub fn getwchar_l(arg1: root::locale_t) -> root::wint_t;
    }
    extern "C" {
        pub fn mbrlen_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: usize,
            arg3: *mut root::mbstate_t,
            arg4: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn mbrtowc_l(
            arg1: *mut u32,
            arg2: *const ::std::os::raw::c_char,
            arg3: usize,
            arg4: *mut root::mbstate_t,
            arg5: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn mbsinit_l(
            arg1: *const root::mbstate_t,
            arg2: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mbsrtowcs_l(
            arg1: *mut u32,
            arg2: *mut *const ::std::os::raw::c_char,
            arg3: usize,
            arg4: *mut root::mbstate_t,
            arg5: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn putwc_l(arg1: u32, arg2: *mut root::FILE, arg3: root::locale_t) -> root::wint_t;
    }
    extern "C" {
        pub fn putwchar_l(arg1: u32, arg2: root::locale_t) -> root::wint_t;
    }
    extern "C" {
        pub fn swprintf_l(
            arg1: *mut u32,
            n: usize,
            arg2: root::locale_t,
            arg3: *const u32,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn swscanf_l(
            arg1: *const u32,
            arg2: root::locale_t,
            arg3: *const u32,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn ungetwc_l(
            arg1: root::wint_t,
            arg2: *mut root::FILE,
            arg3: root::locale_t,
        ) -> root::wint_t;
    }
    extern "C" {
        pub fn vfwprintf_l(
            arg1: *mut root::FILE,
            arg2: root::locale_t,
            arg3: *const u32,
            arg4: root::__darwin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vswprintf_l(
            arg1: *mut u32,
            n: usize,
            arg2: root::locale_t,
            arg3: *const u32,
            arg4: root::__darwin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vwprintf_l(
            arg1: root::locale_t,
            arg2: *const u32,
            arg3: root::__darwin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcrtomb_l(
            arg1: *mut ::std::os::raw::c_char,
            arg2: u32,
            arg3: *mut root::mbstate_t,
            arg4: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn wcscoll_l(
            arg1: *const u32,
            arg2: *const u32,
            arg3: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcsftime_l(
            arg1: *mut u32,
            arg2: usize,
            arg3: *const u32,
            arg4: *const root::tm,
            arg5: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn wcsrtombs_l(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *mut *const u32,
            arg3: usize,
            arg4: *mut root::mbstate_t,
            arg5: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn wcstod_l(arg1: *const u32, arg2: *mut *mut u32, arg3: root::locale_t) -> f64;
    }
    extern "C" {
        pub fn wcstol_l(
            arg1: *const u32,
            arg2: *mut *mut u32,
            arg3: ::std::os::raw::c_int,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn wcstoul_l(
            arg1: *const u32,
            arg2: *mut *mut u32,
            arg3: ::std::os::raw::c_int,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_ulong;
    }
    extern "C" {
        pub fn wcswidth_l(
            arg1: *const u32,
            arg2: usize,
            arg3: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcsxfrm_l(
            arg1: *mut u32,
            arg2: *const u32,
            arg3: usize,
            arg4: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn wctob_l(arg1: root::wint_t, arg2: root::locale_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcwidth_l(arg1: u32, arg2: root::locale_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wprintf_l(arg1: root::locale_t, arg2: *const u32, ...) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wscanf_l(arg1: root::locale_t, arg2: *const u32, ...) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vfwscanf_l(
            arg1: *mut root::FILE,
            arg2: root::locale_t,
            arg3: *const u32,
            arg4: root::__darwin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vswscanf_l(
            arg1: *const u32,
            arg2: root::locale_t,
            arg3: *const u32,
            arg4: root::__darwin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn vwscanf_l(
            arg1: root::locale_t,
            arg2: *const u32,
            arg3: root::__darwin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcstof_l(arg1: *const u32, arg2: *mut *mut u32, arg3: root::locale_t) -> f32;
    }
    extern "C" {
        pub fn wcstold_l(arg1: *const u32, arg2: *mut *mut u32, arg3: root::locale_t) -> f64;
    }
    extern "C" {
        pub fn wcstoll_l(
            arg1: *const u32,
            arg2: *mut *mut u32,
            arg3: ::std::os::raw::c_int,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn wcstoull_l(
            arg1: *const u32,
            arg2: *mut *mut u32,
            arg3: ::std::os::raw::c_int,
            arg4: root::locale_t,
        ) -> ::std::os::raw::c_ulonglong;
    }
    extern "C" {
        pub fn mbsnrtowcs_l(
            arg1: *mut u32,
            arg2: *mut *const ::std::os::raw::c_char,
            arg3: usize,
            arg4: usize,
            arg5: *mut root::mbstate_t,
            arg6: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn wcscasecmp_l(
            arg1: *const u32,
            arg2: *const u32,
            arg3: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcsncasecmp_l(
            arg1: *const u32,
            arg2: *const u32,
            n: usize,
            arg3: root::locale_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn wcsnrtombs_l(
            arg1: *mut ::std::os::raw::c_char,
            arg2: *mut *const u32,
            arg3: usize,
            arg4: usize,
            arg5: *mut root::mbstate_t,
            arg6: root::locale_t,
        ) -> usize;
    }
    extern "C" {
        pub fn fgetwln_l(arg1: *mut root::FILE, arg2: *mut usize, arg3: root::locale_t)
            -> *mut u32;
    }
    extern "C" {
        pub fn nextwctype_l(
            arg1: root::wint_t,
            arg2: root::wctype_t,
            arg3: root::locale_t,
        ) -> root::wint_t;
    }
    extern "C" {
        pub fn towctrans_l(
            arg1: root::wint_t,
            arg2: root::wctrans_t,
            arg3: root::locale_t,
        ) -> root::wint_t;
    }
    extern "C" {
        pub fn wctrans_l(
            arg1: *const ::std::os::raw::c_char,
            arg2: root::locale_t,
        ) -> root::wctrans_t;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __nl_cat_d {
        pub __data: *mut ::std::os::raw::c_void,
        pub __size: ::std::os::raw::c_int,
    }
    const _: () = {
        ["Size of __nl_cat_d"][::std::mem::size_of::<__nl_cat_d>() - 16usize];
        ["Alignment of __nl_cat_d"][::std::mem::align_of::<__nl_cat_d>() - 8usize];
        ["Offset of field: __nl_cat_d::__data"]
            [::std::mem::offset_of!(__nl_cat_d, __data) - 0usize];
        ["Offset of field: __nl_cat_d::__size"]
            [::std::mem::offset_of!(__nl_cat_d, __size) - 8usize];
    };
    pub type nl_catd = *mut root::__nl_cat_d;
    pub type nl_item = root::__darwin_nl_item;
    extern "C" {
        pub fn catopen(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_int,
        ) -> root::nl_catd;
    }
    extern "C" {
        pub fn catgets(
            arg1: root::nl_catd,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn catclose(arg1: root::nl_catd) -> ::std::os::raw::c_int;
    }
    pub type InlineRefCounts = root::swift::InlineRefCounts;
    pub type __swift_size_t = ::std::os::raw::c_ulong;
    pub type __swift_ssize_t = u64;
    pub mod heap_object_abi {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub const LeastValidPointerValue: root::__swift_uintptr_t = 4294967296;
        extern "C" {
            #[link_name = "\u{1}__ZN15heap_object_abiL18SwiftSpareBitsMaskE"]
            pub static SwiftSpareBitsMask: root::__swift_uintptr_t;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN15heap_object_abiL20ObjCReservedBitsMaskE"]
            pub static ObjCReservedBitsMask: root::__swift_uintptr_t;
        }
        pub const ObjCReservedLowBits: ::std::os::raw::c_uint = 0;
        extern "C" {
            #[link_name = "\u{1}__ZN15heap_object_abiL23BridgeObjectTagBitsMaskE"]
            pub static BridgeObjectTagBitsMask: root::__swift_uintptr_t;
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct objc_method {
        _unused: [u8; 0],
    }
    pub type Method = *mut root::objc_method;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct objc_ivar {
        _unused: [u8; 0],
    }
    pub type Ivar = *mut root::objc_ivar;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct objc_category {
        _unused: [u8; 0],
    }
    pub type Category = *mut root::objc_category;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct objc_property {
        _unused: [u8; 0],
    }
    pub type objc_property_t = *mut root::objc_property;
    pub type Protocol = root::objc_object;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct objc_method_description {
        pub name: root::SEL,
        pub types: *mut ::std::os::raw::c_char,
    }
    const _: () = {
        ["Size of objc_method_description"]
            [::std::mem::size_of::<objc_method_description>() - 16usize];
        ["Alignment of objc_method_description"]
            [::std::mem::align_of::<objc_method_description>() - 8usize];
        ["Offset of field: objc_method_description::name"]
            [::std::mem::offset_of!(objc_method_description, name) - 0usize];
        ["Offset of field: objc_method_description::types"]
            [::std::mem::offset_of!(objc_method_description, types) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct objc_property_attribute_t {
        pub name: *const ::std::os::raw::c_char,
        pub value: *const ::std::os::raw::c_char,
    }
    const _: () = {
        ["Size of objc_property_attribute_t"]
            [::std::mem::size_of::<objc_property_attribute_t>() - 16usize];
        ["Alignment of objc_property_attribute_t"]
            [::std::mem::align_of::<objc_property_attribute_t>() - 8usize];
        ["Offset of field: objc_property_attribute_t::name"]
            [::std::mem::offset_of!(objc_property_attribute_t, name) - 0usize];
        ["Offset of field: objc_property_attribute_t::value"]
            [::std::mem::offset_of!(objc_property_attribute_t, value) - 8usize];
    };
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct mach_header {
        _unused: [u8; 0],
    }
    extern "C" {
        pub fn object_copy(obj: root::id, size: usize) -> root::id;
    }
    extern "C" {
        pub fn object_dispose(obj: root::id) -> root::id;
    }
    extern "C" {
        pub fn object_getClass(obj: root::id) -> root::Class;
    }
    extern "C" {
        pub fn object_setClass(obj: root::id, cls: root::Class) -> root::Class;
    }
    extern "C" {
        pub fn object_isClass(obj: root::id) -> root::BOOL;
    }
    extern "C" {
        pub fn object_getIvar(obj: root::id, ivar: root::Ivar) -> root::id;
    }
    extern "C" {
        pub fn object_setIvar(obj: root::id, ivar: root::Ivar, value: root::id);
    }
    extern "C" {
        pub fn object_setIvarWithStrongDefault(obj: root::id, ivar: root::Ivar, value: root::id);
    }
    extern "C" {
        pub fn object_setInstanceVariable(
            obj: root::id,
            name: *const ::std::os::raw::c_char,
            value: *mut ::std::os::raw::c_void,
        ) -> root::Ivar;
    }
    extern "C" {
        pub fn object_setInstanceVariableWithStrongDefault(
            obj: root::id,
            name: *const ::std::os::raw::c_char,
            value: *mut ::std::os::raw::c_void,
        ) -> root::Ivar;
    }
    extern "C" {
        pub fn object_getInstanceVariable(
            obj: root::id,
            name: *const ::std::os::raw::c_char,
            outValue: *mut *mut ::std::os::raw::c_void,
        ) -> root::Ivar;
    }
    extern "C" {
        pub fn objc_getClass(name: *const ::std::os::raw::c_char) -> root::Class;
    }
    extern "C" {
        pub fn objc_getMetaClass(name: *const ::std::os::raw::c_char) -> root::Class;
    }
    extern "C" {
        pub fn objc_lookUpClass(name: *const ::std::os::raw::c_char) -> root::Class;
    }
    extern "C" {
        pub fn objc_getRequiredClass(name: *const ::std::os::raw::c_char) -> root::Class;
    }
    extern "C" {
        pub fn objc_getClassList(
            buffer: *mut root::Class,
            bufferCount: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn objc_copyClassList(outCount: *mut ::std::os::raw::c_uint) -> *mut root::Class;
    }
    extern "C" {
        pub fn objc_enumerateClasses(
            image: *const ::std::os::raw::c_void,
            namePrefix: *const ::std::os::raw::c_char,
            conformingTo: *mut root::Protocol,
            subclassing: root::Class,
            block: *mut ::std::os::raw::c_void,
        );
    }
    extern "C" {
        pub fn class_getName(cls: root::Class) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn class_isMetaClass(cls: root::Class) -> root::BOOL;
    }
    extern "C" {
        pub fn class_getSuperclass(cls: root::Class) -> root::Class;
    }
    extern "C" {
        pub fn class_setSuperclass(cls: root::Class, newSuper: root::Class) -> root::Class;
    }
    extern "C" {
        pub fn class_getVersion(cls: root::Class) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn class_setVersion(cls: root::Class, version: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn class_getInstanceSize(cls: root::Class) -> usize;
    }
    extern "C" {
        pub fn class_getInstanceVariable(
            cls: root::Class,
            name: *const ::std::os::raw::c_char,
        ) -> root::Ivar;
    }
    extern "C" {
        pub fn class_getClassVariable(
            cls: root::Class,
            name: *const ::std::os::raw::c_char,
        ) -> root::Ivar;
    }
    extern "C" {
        pub fn class_copyIvarList(
            cls: root::Class,
            outCount: *mut ::std::os::raw::c_uint,
        ) -> *mut root::Ivar;
    }
    extern "C" {
        pub fn class_getInstanceMethod(cls: root::Class, name: root::SEL) -> root::Method;
    }
    extern "C" {
        pub fn class_getClassMethod(cls: root::Class, name: root::SEL) -> root::Method;
    }
    extern "C" {
        pub fn class_getMethodImplementation(cls: root::Class, name: root::SEL) -> root::IMP;
    }
    extern "C" {
        pub fn class_getMethodImplementation_stret(cls: root::Class, name: root::SEL) -> root::IMP;
    }
    extern "C" {
        pub fn class_respondsToSelector(cls: root::Class, sel: root::SEL) -> root::BOOL;
    }
    extern "C" {
        pub fn class_copyMethodList(
            cls: root::Class,
            outCount: *mut ::std::os::raw::c_uint,
        ) -> *mut root::Method;
    }
    extern "C" {
        pub fn class_conformsToProtocol(
            cls: root::Class,
            protocol: *mut root::Protocol,
        ) -> root::BOOL;
    }
    extern "C" {
        pub fn class_copyProtocolList(
            cls: root::Class,
            outCount: *mut ::std::os::raw::c_uint,
        ) -> *mut *mut root::Protocol;
    }
    extern "C" {
        pub fn class_getProperty(
            cls: root::Class,
            name: *const ::std::os::raw::c_char,
        ) -> root::objc_property_t;
    }
    extern "C" {
        pub fn class_copyPropertyList(
            cls: root::Class,
            outCount: *mut ::std::os::raw::c_uint,
        ) -> *mut root::objc_property_t;
    }
    extern "C" {
        pub fn class_getIvarLayout(cls: root::Class) -> *const u8;
    }
    extern "C" {
        pub fn class_getWeakIvarLayout(cls: root::Class) -> *const u8;
    }
    extern "C" {
        pub fn class_addMethod(
            cls: root::Class,
            name: root::SEL,
            imp: root::IMP,
            types: *const ::std::os::raw::c_char,
        ) -> root::BOOL;
    }
    extern "C" {
        pub fn class_replaceMethod(
            cls: root::Class,
            name: root::SEL,
            imp: root::IMP,
            types: *const ::std::os::raw::c_char,
        ) -> root::IMP;
    }
    extern "C" {
        pub fn class_addIvar(
            cls: root::Class,
            name: *const ::std::os::raw::c_char,
            size: usize,
            alignment: u8,
            types: *const ::std::os::raw::c_char,
        ) -> root::BOOL;
    }
    extern "C" {
        pub fn class_addProtocol(cls: root::Class, protocol: *mut root::Protocol) -> root::BOOL;
    }
    extern "C" {
        pub fn class_addProperty(
            cls: root::Class,
            name: *const ::std::os::raw::c_char,
            attributes: *const root::objc_property_attribute_t,
            attributeCount: ::std::os::raw::c_uint,
        ) -> root::BOOL;
    }
    extern "C" {
        pub fn class_replaceProperty(
            cls: root::Class,
            name: *const ::std::os::raw::c_char,
            attributes: *const root::objc_property_attribute_t,
            attributeCount: ::std::os::raw::c_uint,
        );
    }
    extern "C" {
        pub fn class_setIvarLayout(cls: root::Class, layout: *const u8);
    }
    extern "C" {
        pub fn class_setWeakIvarLayout(cls: root::Class, layout: *const u8);
    }
    extern "C" {
        pub fn objc_getFutureClass(name: *const ::std::os::raw::c_char) -> root::Class;
    }
    extern "C" {
        pub fn class_createInstance(cls: root::Class, extraBytes: usize) -> root::id;
    }
    extern "C" {
        pub fn objc_constructInstance(
            cls: root::Class,
            bytes: *mut ::std::os::raw::c_void,
        ) -> root::id;
    }
    extern "C" {
        pub fn objc_destructInstance(obj: root::id) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn objc_allocateClassPair(
            superclass: root::Class,
            name: *const ::std::os::raw::c_char,
            extraBytes: usize,
        ) -> root::Class;
    }
    extern "C" {
        pub fn objc_registerClassPair(cls: root::Class);
    }
    extern "C" {
        pub fn objc_duplicateClass(
            original: root::Class,
            name: *const ::std::os::raw::c_char,
            extraBytes: usize,
        ) -> root::Class;
    }
    extern "C" {
        pub fn objc_disposeClassPair(cls: root::Class);
    }
    extern "C" {
        pub fn method_getName(m: root::Method) -> root::SEL;
    }
    extern "C" {
        pub fn method_getImplementation(m: root::Method) -> root::IMP;
    }
    extern "C" {
        pub fn method_getTypeEncoding(m: root::Method) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn method_getNumberOfArguments(m: root::Method) -> ::std::os::raw::c_uint;
    }
    extern "C" {
        pub fn method_copyReturnType(m: root::Method) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn method_copyArgumentType(
            m: root::Method,
            index: ::std::os::raw::c_uint,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn method_getReturnType(
            m: root::Method,
            dst: *mut ::std::os::raw::c_char,
            dst_len: usize,
        );
    }
    extern "C" {
        pub fn method_getArgumentType(
            m: root::Method,
            index: ::std::os::raw::c_uint,
            dst: *mut ::std::os::raw::c_char,
            dst_len: usize,
        );
    }
    extern "C" {
        pub fn method_getDescription(m: root::Method) -> *mut root::objc_method_description;
    }
    extern "C" {
        pub fn method_setImplementation(m: root::Method, imp: root::IMP) -> root::IMP;
    }
    extern "C" {
        pub fn method_exchangeImplementations(m1: root::Method, m2: root::Method);
    }
    extern "C" {
        pub fn ivar_getName(v: root::Ivar) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn ivar_getTypeEncoding(v: root::Ivar) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn ivar_getOffset(v: root::Ivar) -> isize;
    }
    extern "C" {
        pub fn property_getName(property: root::objc_property_t) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn property_getAttributes(
            property: root::objc_property_t,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn property_copyAttributeList(
            property: root::objc_property_t,
            outCount: *mut ::std::os::raw::c_uint,
        ) -> *mut root::objc_property_attribute_t;
    }
    extern "C" {
        pub fn property_copyAttributeValue(
            property: root::objc_property_t,
            attributeName: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn objc_getProtocol(name: *const ::std::os::raw::c_char) -> *mut root::Protocol;
    }
    extern "C" {
        pub fn objc_copyProtocolList(
            outCount: *mut ::std::os::raw::c_uint,
        ) -> *mut *mut root::Protocol;
    }
    extern "C" {
        pub fn protocol_conformsToProtocol(
            proto: *mut root::Protocol,
            other: *mut root::Protocol,
        ) -> root::BOOL;
    }
    extern "C" {
        pub fn protocol_isEqual(
            proto: *mut root::Protocol,
            other: *mut root::Protocol,
        ) -> root::BOOL;
    }
    extern "C" {
        pub fn protocol_getName(proto: *mut root::Protocol) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn protocol_getMethodDescription(
            proto: *mut root::Protocol,
            aSel: root::SEL,
            isRequiredMethod: root::BOOL,
            isInstanceMethod: root::BOOL,
        ) -> root::objc_method_description;
    }
    extern "C" {
        pub fn protocol_copyMethodDescriptionList(
            proto: *mut root::Protocol,
            isRequiredMethod: root::BOOL,
            isInstanceMethod: root::BOOL,
            outCount: *mut ::std::os::raw::c_uint,
        ) -> *mut root::objc_method_description;
    }
    extern "C" {
        pub fn protocol_getProperty(
            proto: *mut root::Protocol,
            name: *const ::std::os::raw::c_char,
            isRequiredProperty: root::BOOL,
            isInstanceProperty: root::BOOL,
        ) -> root::objc_property_t;
    }
    extern "C" {
        pub fn protocol_copyPropertyList(
            proto: *mut root::Protocol,
            outCount: *mut ::std::os::raw::c_uint,
        ) -> *mut root::objc_property_t;
    }
    extern "C" {
        pub fn protocol_copyPropertyList2(
            proto: *mut root::Protocol,
            outCount: *mut ::std::os::raw::c_uint,
            isRequiredProperty: root::BOOL,
            isInstanceProperty: root::BOOL,
        ) -> *mut root::objc_property_t;
    }
    extern "C" {
        pub fn protocol_copyProtocolList(
            proto: *mut root::Protocol,
            outCount: *mut ::std::os::raw::c_uint,
        ) -> *mut *mut root::Protocol;
    }
    extern "C" {
        pub fn objc_allocateProtocol(name: *const ::std::os::raw::c_char) -> *mut root::Protocol;
    }
    extern "C" {
        pub fn objc_registerProtocol(proto: *mut root::Protocol);
    }
    extern "C" {
        pub fn protocol_addMethodDescription(
            proto: *mut root::Protocol,
            name: root::SEL,
            types: *const ::std::os::raw::c_char,
            isRequiredMethod: root::BOOL,
            isInstanceMethod: root::BOOL,
        );
    }
    extern "C" {
        pub fn protocol_addProtocol(proto: *mut root::Protocol, addition: *mut root::Protocol);
    }
    extern "C" {
        pub fn protocol_addProperty(
            proto: *mut root::Protocol,
            name: *const ::std::os::raw::c_char,
            attributes: *const root::objc_property_attribute_t,
            attributeCount: ::std::os::raw::c_uint,
            isRequiredProperty: root::BOOL,
            isInstanceProperty: root::BOOL,
        );
    }
    extern "C" {
        pub fn objc_copyImageNames(
            outCount: *mut ::std::os::raw::c_uint,
        ) -> *mut *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn class_getImageName(cls: root::Class) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn objc_copyClassNamesForImage(
            image: *const ::std::os::raw::c_char,
            outCount: *mut ::std::os::raw::c_uint,
        ) -> *mut *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn sel_isEqual(lhs: root::SEL, rhs: root::SEL) -> root::BOOL;
    }
    extern "C" {
        pub fn objc_enumerationMutation(obj: root::id);
    }
    extern "C" {
        pub fn objc_setEnumerationMutationHandler(
            handler: ::std::option::Option<unsafe extern "C" fn(arg1: root::id)>,
        );
    }
    extern "C" {
        pub fn objc_setForwardHandler(
            fwd: *mut ::std::os::raw::c_void,
            fwd_stret: *mut ::std::os::raw::c_void,
        );
    }
    extern "C" {
        pub fn imp_implementationWithBlock(block: root::id) -> root::IMP;
    }
    extern "C" {
        pub fn imp_getBlock(anImp: root::IMP) -> root::id;
    }
    extern "C" {
        pub fn imp_removeBlock(anImp: root::IMP) -> root::BOOL;
    }
    extern "C" {
        pub fn objc_loadWeak(location: *mut root::id) -> root::id;
    }
    extern "C" {
        pub fn objc_storeWeak(location: *mut root::id, obj: root::id) -> root::id;
    }
    pub const objc_AssociationPolicy_OBJC_ASSOCIATION_ASSIGN: root::objc_AssociationPolicy = 0;
    pub const objc_AssociationPolicy_OBJC_ASSOCIATION_RETAIN_NONATOMIC:
        root::objc_AssociationPolicy = 1;
    pub const objc_AssociationPolicy_OBJC_ASSOCIATION_COPY_NONATOMIC: root::objc_AssociationPolicy =
        3;
    pub const objc_AssociationPolicy_OBJC_ASSOCIATION_RETAIN: root::objc_AssociationPolicy = 769;
    pub const objc_AssociationPolicy_OBJC_ASSOCIATION_COPY: root::objc_AssociationPolicy = 771;
    pub type objc_AssociationPolicy = usize;
    extern "C" {
        pub fn objc_setAssociatedObject(
            object: root::id,
            key: *const ::std::os::raw::c_void,
            value: root::id,
            policy: root::objc_AssociationPolicy,
        );
    }
    extern "C" {
        pub fn objc_getAssociatedObject(
            object: root::id,
            key: *const ::std::os::raw::c_void,
        ) -> root::id;
    }
    extern "C" {
        pub fn objc_removeAssociatedObjects(object: root::id);
    }
    pub type objc_hook_getImageName = ::std::option::Option<
        unsafe extern "C" fn(
            cls: root::Class,
            outImageName: *mut *const ::std::os::raw::c_char,
        ) -> root::BOOL,
    >;
    extern "C" {
        pub fn objc_setHook_getImageName(
            newValue: root::objc_hook_getImageName,
            outOldValue: *mut root::objc_hook_getImageName,
        );
    }
    pub type objc_hook_getClass = ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            outClass: *mut root::Class,
        ) -> root::BOOL,
    >;
    extern "C" {
        pub fn objc_setHook_getClass(
            newValue: root::objc_hook_getClass,
            outOldValue: *mut root::objc_hook_getClass,
        );
    }
    pub type objc_func_loadImage =
        ::std::option::Option<unsafe extern "C" fn(header: *const root::mach_header)>;
    extern "C" {
        pub fn objc_addLoadImageFunc(func: root::objc_func_loadImage);
    }
    pub type objc_hook_lazyClassNamer = ::std::option::Option<
        unsafe extern "C" fn(cls: root::Class) -> *const ::std::os::raw::c_char,
    >;
    extern "C" {
        pub fn objc_setHook_lazyClassNamer(
            newValue: root::objc_hook_lazyClassNamer,
            oldOutValue: *mut root::objc_hook_lazyClassNamer,
        );
    }
    pub type _objc_swiftMetadataInitializer = ::std::option::Option<
        unsafe extern "C" fn(cls: root::Class, arg: *mut ::std::os::raw::c_void) -> root::Class,
    >;
    extern "C" {
        pub fn _objc_realizeClassFromSwift(
            cls: root::Class,
            previously: *mut ::std::os::raw::c_void,
        ) -> root::Class;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct objc_method_list {
        _unused: [u8; 0],
    }
    extern "C" {
        pub fn _objc_flush_caches(cls: root::Class);
    }
    extern "C" {
        pub fn class_lookupMethod(cls: root::Class, sel: root::SEL) -> root::IMP;
    }
    extern "C" {
        pub fn class_respondsToMethod(cls: root::Class, sel: root::SEL) -> root::BOOL;
    }
    extern "C" {
        pub fn object_copyFromZone(
            anObject: root::id,
            nBytes: usize,
            zone: *mut ::std::os::raw::c_void,
        ) -> root::id;
    }
    extern "C" {
        pub fn class_createInstanceFromZone(
            arg1: root::Class,
            idxIvars: usize,
            zone: *mut ::std::os::raw::c_void,
        ) -> root::id;
    }
    extern "C" {
        pub fn objc_retain(arg1: root::id) -> root::id;
    }
    extern "C" {
        pub fn objc_release(arg1: root::id);
    }
    extern "C" {
        pub fn _objc_rootAutorelease(arg1: root::id) -> root::id;
    }
    extern "C" {
        pub fn objc_moveWeak(arg1: *mut root::id, arg2: *mut root::id);
    }
    extern "C" {
        pub fn objc_copyWeak(arg1: *mut root::id, arg2: *mut root::id);
    }
    extern "C" {
        pub fn objc_initWeak(arg1: *mut root::id, arg2: root::id) -> root::id;
    }
    extern "C" {
        pub fn objc_destroyWeak(arg1: *mut root::id);
    }
    extern "C" {
        pub fn objc_loadWeakRetained(arg1: *mut root::id) -> root::id;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct objc_image_info {
        pub version: u32,
        pub flags: u32,
    }
    const _: () = {
        ["Size of objc_image_info"][::std::mem::size_of::<objc_image_info>() - 8usize];
        ["Alignment of objc_image_info"][::std::mem::align_of::<objc_image_info>() - 4usize];
        ["Offset of field: objc_image_info::version"]
            [::std::mem::offset_of!(objc_image_info, version) - 0usize];
        ["Offset of field: objc_image_info::flags"]
            [::std::mem::offset_of!(objc_image_info, flags) - 4usize];
    };
    extern "C" {
        pub fn objc_readClassPair(
            cls: root::Class,
            info: *const root::objc_image_info,
        ) -> root::Class;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _bindgen_ty_5 {
        pub c: ::std::os::raw::c_char,
    }
    const _: () = {
        ["Size of _bindgen_ty_5"][::std::mem::size_of::<_bindgen_ty_5>() - 1usize];
        ["Alignment of _bindgen_ty_5"][::std::mem::align_of::<_bindgen_ty_5>() - 1usize];
        ["Offset of field: _bindgen_ty_5::c"][::std::mem::offset_of!(_bindgen_ty_5, c) - 0usize];
    };
    extern "C" {
        pub static objc_absolute_packed_isa_class_mask: root::_bindgen_ty_5;
    }
    extern "C" {
        #[doc = " Return the path of the libswiftCore library.\n\n This can be used to locate files that are installed alongside the Swift\n runtime library.\n\n \\return A string containing the full path to libswiftCore.  The string is\n         owned by the runtime and should not be freed."]
        pub fn swift_getRuntimeLibraryPath() -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[doc = " Return the path of the Swift root.\n\n If the path to libswiftCore is `/usr/local/swift/lib/libswiftCore.dylib`,\n this function would return `/usr/local/swift`.\n\n The path returned here can be overridden by setting the environment variable\n SWIFT_ROOT.\n\n \\return A string containing the full path to the Swift root directory, based\n         either on the location of the Swift runtime, or on the `SWIFT_ROOT`\n         environment variable if set.  The string is owned by the runtime\n         and should not be freed."]
        pub fn swift_getRootPath() -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[doc = " Return the path of the specified auxiliary executable.\n\n This function will search for the auxiliary executable in the following\n paths:\n\n   <swift-root>/libexec/swift/<platform>/<name>\n   <swift-root>/libexec/swift/<name>\n   <swift-root>/bin/<name>\n   <swift-root>/<name>\n\n It will return the first of those that exists, but it does not test that\n the file is indeed executable.\n\n On Windows, it will automatically add `.exe` to the name, which means you\n do not need to special case the name for Windows.\n\n If you are using this function to locate a utility program for use by the\n runtime, you should provide a way to override its location using an\n environment variable.\n\n If the executable cannot be found, it will return nullptr.\n\n \\param name      The name of the executable to locate.\n\n \\return A string containing the full path to the executable.  This string\n         should be released with `free()` when no longer required."]
        pub fn swift_copyAuxiliaryExecutablePath(
            name: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn swift_dtoa_optimal_binary16_p(
            arg1: *const ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_char,
            length: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn swift_dtoa_optimal_binary32_p(
            arg1: *const ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_char,
            length: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn swift_dtoa_optimal_float(
            arg1: f32,
            dest: *mut ::std::os::raw::c_char,
            length: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn swift_dtoa_optimal_binary64_p(
            arg1: *const ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_char,
            length: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn swift_dtoa_optimal_double(
            arg1: f64,
            dest: *mut ::std::os::raw::c_char,
            length: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn swift_dtoa_optimal_long_double(
            arg1: f64,
            dest: *mut ::std::os::raw::c_char,
            length: usize,
        ) -> usize;
    }
    pub type os_function_t =
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
    pub type os_block_t = *mut ::std::os::raw::c_void;
    extern "C" {
        pub fn os_retain(object: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn os_release(object: *mut ::std::os::raw::c_void);
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct voucher_s {
        _unused: [u8; 0],
    }
    pub type voucher_t = *mut root::voucher_s;
    extern "C" {
        pub fn voucher_copy() -> root::voucher_t;
    }
    extern "C" {
        pub fn voucher_adopt(voucher: root::voucher_t) -> root::voucher_t;
    }
    pub type __builtin_va_list = *mut ::std::os::raw::c_char;
    pub type __uint128_t = u128;
    pub type type_ = ::std::os::raw::c_uint;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _bindgen_ty_1 {
        pub _address: u8,
    }
    pub type rep = ::std::os::raw::c_longlong;
    // pub type rep = ::std::os::raw::c_longlong;
    pub type __int128_t = i128;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _bindgen_ty_2 {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _bindgen_ty_3 {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _bindgen_ty_4 {
        pub _address: u8,
    }
    pub type char_type = u16;
    // pub type char_type = u32;
    pub type value_type = bool;
    // const _: () = {
    //     ["Size of template specialization: RefCounts_open0_InlineRefCountBits_close0"]
    //         [::std::mem::size_of::<root::swift::RefCounts>() - 8usize];
    //     ["Align of template specialization: RefCounts_open0_InlineRefCountBits_close0"]
    //         [::std::mem::align_of::<root::swift::RefCounts>() - 8usize];
    // };
    // const _: () = {
    //     ["Size of template specialization: RefCounts_open0_SideTableRefCountBits_close0"]
    //         [::std::mem::size_of::<root::swift::RefCounts>() - 16usize];
    //     ["Align of template specialization: RefCounts_open0_SideTableRefCountBits_close0"]
    //         [::std::mem::align_of::<root::swift::RefCounts>() - 16usize];
    // };
    pub const Immortal_t_Immortal: root::Immortal_t = 0;
    pub type Immortal_t = ::std::os::raw::c_uint;
}
