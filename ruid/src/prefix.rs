pub mod Prefix {
    pub const UncategorizedData: u16 = 0x0000;
    pub mod file {
        pub const File_Uncategorized: u16 = 0x1000;
        pub mod text {
            pub const File_Text_Txt: u16 = 0x1100;
            pub const File_Text_Markdown: u16 = 0x1101;
            pub const File_Text_Rtf: u16 = 0x1102;
            pub const File_Text_Docx: u16 = 0x1103;
            pub const File_Text_Pdf: u16 = 0x1104;
            pub const File_Text_Odt: u16 = 0x1105;
            pub const File_Text_Latex: u16 = 0x1106;
            pub const File_Text_Epub: u16 = 0x1107;
            pub const File_Text_Csv: u16 = 0x1108;
        }
        pub mod binary {
            pub const File_Binary_Executable: u16 = 0x1200;
            pub const File_Binary_Bin: u16 = 0x1201;
            pub const File_Binary_Dll: u16 = 0x1202;
            pub const File_Binary_UnixSharedLib: u16 = 0x1203;
            pub const File_Binary_MacOSDiskImage: u16 = 0x1204;
            pub const File_Binary_ISODiskImage: u16 = 0x1205;
            pub const File_Binary_DiskImage: u16 = 0x1206;
        }
        pub mod configuration {
            pub const File_Configuration_Cfg: u16 = 0x1300;
            pub const File_Configuration_Ini: u16 = 0x1301;
            pub const File_Configuration_Json: u16 = 0x1302;
            pub const File_Configuration_Xml: u16 = 0x1303;
            pub const File_Configuration_Yaml: u16 = 0x1304;
            pub const File_Configuration_Toml: u16 = 0x1305;
        }
        pub mod cache {
            pub const File_Cache_Cache: u16 = 0x1400;
            pub const File_Cache_Tmp: u16 = 0x1401;
            pub const File_Cache_Swp: u16 = 0x1402;
        }
        pub mod log {
            pub const File_Log_Log: u16 = 0x1500;
            pub const File_Log_Out: u16 = 0x1501;
        }
    }
    pub mod metadata {
        pub const Metadata_Uncategorized: u16 = 0x2000;
        pub mod user {
            pub const Metadata_User_Example: u16 = 0x2100;
            pub const Metadata_User_Id: u16 = 0x2101;
        }
        pub mod permission {
            pub const Metadata_Permission_Uncategorized: u16 = 0x2200;
            pub const Metadata_Permission_Everyone: u16 = 0x2201;
        }
    }
    pub const LogData: u16 = 0x3000;
    pub const CacheData: u16 = 0x4000;
    pub const DeviceData: u16 = 0x5000;
    pub const ObjectData: u16 = 0x6000;
    pub const OtherData: u16 = 0xF000;
}