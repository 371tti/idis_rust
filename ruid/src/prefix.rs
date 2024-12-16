pub mod prefix {
    pub const UNCATEGORIZED_DATA: u16 = 0x0000;
    pub mod file {
        pub const FILE_UNCATEGORIZED: u16 = 0x1000;
        pub mod text {
            pub const FILE_TEXT_TXT: u16 = 0x1100;
            pub const FILE_TEXT_MARKDOWN: u16 = 0x1101;
            pub const FILE_TEXT_RTF: u16 = 0x1102;
            pub const FILE_TEXT_DOCX: u16 = 0x1103;
            pub const FILE_TEXT_PDF: u16 = 0x1104;
            pub const FILE_TEXT_ODT: u16 = 0x1105;
            pub const FILE_TEXT_LATEX: u16 = 0x1106;
            pub const FILE_TEXT_EPUB: u16 = 0x1107;
            pub const FILE_TEXT_CSV: u16 = 0x1108;
        }
        pub mod binary {
            pub const FILE_BINARY_EXECUTABLE: u16 = 0x1200;
            pub const FILE_BINARY_BIN: u16 = 0x1201;
            pub const FILE_BINARY_DLL: u16 = 0x1202;
            pub const FILE_BINARY_UNIX_SHARED_LIB: u16 = 0x1203;
            pub const FILE_BINARY_MAC_OSDISK_IMAGE: u16 = 0x1204;
            pub const FILE_BINARY_ISODISK_IMAGE: u16 = 0x1205;
            pub const FILE_BINARY_DISK_IMAGE: u16 = 0x1206;
        }
        pub mod configuration {
            pub const FILE_CONFIGURATION_CFG: u16 = 0x1300;
            pub const FILE_CONFIGURATION_INI: u16 = 0x1301;
            pub const FILE_CONFIGURATION_JSON: u16 = 0x1302;
            pub const FILE_CONFIGURATION_XML: u16 = 0x1303;
            pub const FILE_CONFIGURATION_YAML: u16 = 0x1304;
            pub const FILE_CONFIGURATION_TOML: u16 = 0x1305;
        }
        pub mod cache {
            pub const FILE_CACHE_CACHE: u16 = 0x1400;
            pub const FILE_CACHE_TMP: u16 = 0x1401;
            pub const FILE_CACHE_SWP: u16 = 0x1402;
        }
        pub mod log {
            pub const FILE_LOG_LOG: u16 = 0x1500;
            pub const FILE_LOG_OUT: u16 = 0x1501;
        }
    }
    pub mod metadata {
        pub const METADATA_UNCATEGORIZED: u16 = 0x2000;
        pub mod user {
            pub const METADATA_USER_EXAMPLE: u16 = 0x2100;
            pub const METADATA_USER_ID: u16 = 0x2101;
        }
        pub mod permission {
            pub const METADATA_PERMISSION_UNCATEGORIZED: u16 = 0x2200;
            pub const METADATA_PERMISSION_EVERYONE: u16 = 0x2201;
        }
    }
    pub const LOG_DATA: u16 = 0x3000;
    pub const CACHE_DATA: u16 = 0x4000;
    pub const DEVICE_DATA: u16 = 0x5000;
    pub const OBJECT_DATA: u16 = 0x6000;
    pub const OTHER_DATA: u16 = 0xF000;
}