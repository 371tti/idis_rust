

### RUID (Readable Unique Identifier) v1 Format

The RUID (Readable Unique Identifier) is a proprietary format used by idis to identify data uniquely. The structure of the RUID v1 is as follows:

**Total: 128 bits**

| 16 bits | 4 bits       | 16 bits   | 48 bits         | 44 bits                            |
| ------- | ------------ | --------- | --------------- | ---------------------------------- |
| Prefix  | Version Code | Server ID | UNIX Time Stamp | Cryptographic Pseudo-Random Number |

- **Prefix**: Indicates what the ID represents.
- **Version Code**: RUID version code. For v1, the code is 0.e
- **Server ID**: The ID of the server managing the data.
- **UNIX Time Stamp**: UNIX timestamp in microseconds.
- **Cryptographic Pseudo-Random Number**: A cryptographically secure random number.

### Prefix Allocation

#### 0x0***: Uncategorized Data

- `0x0000` - Uncategorized Data

#### 0x1***: File Data

- **0x10**: Uncategorized File
  - `0x1000` - .- Uncategorized File
- **0x11**: Text Files
  - `0x1100` - .txt (Plain Text File)
  - `0x1101` - .md (Markdown File)
  - `0x1102` - .rtf (Rich Text File)
  - `0x1103` - .doc/.docx (Microsoft Word File)
  - `0x1104` - .pdf (PDF File)
  - `0x1105` - .odt (OpenDocument Text)
  - `0x1106` - .tex (LaTeX File)
  - `0x1107` - .epub (eBook File)
  - `0x1108` - .csv (Comma-Separated Values File)
- **0x12**: Binary Files
  - `0x1200` - .exe (Windows Executable File)
  - `0x1201` - .bin (Binary Data File)
  - `0x1202` - .dll (Windows Dynamic Link Library)
  - `0x1203` - .so (UNIX Shared Library)
  - `0x1204` - .dmg (macOS Disk Image)
  - `0x1205` - .iso (ISO Disk Image)
  - `0x1206` - .img (Disk Image File)
- **0x13**: Configuration Files
  - `0x1300` - .cfg/.conf (Configuration File)
  - `0x1301` - .ini (Initialization File)
  - `0x1302` - .json (JSON Configuration File)
  - `0x1303` - .xml (XML Configuration File)
  - `0x1304` - .yaml/.yml (YAML Configuration File)
  - `0x1305` - .toml (TOML Configuration File)
- **0x14**: Cache Files
  - `0x1400` - .cache (Cache File)
  - `0x1401` - .tmp (Temporary File)
  - `0x1402` - .swp (Swap File)
- **0x15**: Log Files
  - `0x1500` - .log (Log File)
  - `0x1501` - .out (Output File)
- **0x16**: Media Files
  - `0x1600` - .jpg/.jpeg (JPEG Image File)
  - `0x1601` - .png (PNG Image File)
  - `0x1602` - .gif (GIF Image File)
  - `0x1603` - .bmp (BMP Image File)
  - `0x1604` - .tiff (TIFF Image File)
  - `0x1605` - .svg (SVG Image File)
  - `0x1606` - .mp4 (MP4 Video File)
  - `0x1607` - .mp3 (MP3 Audio File)
  - `0x1608` - .wav (WAV Audio File)
  - `0x1609` - .mkv (MKV Video File)
  - `0x160A` - .avi (AVI Video File)
  - `0x160B` - .mov (QuickTime Video File)
  - `0x160C` - .flv (Flash Video File)
  - `0x160D` - .wmv (Windows Media Video File)
  - `0x160E` - .webm (WebM Video File)
  - `0x160F` - .ogg (Ogg Audio File)
  - `0x1610` - .flac (FLAC Audio File)
  - `0x1611` - .aac (AAC Audio File)
  - `0x1612` - .m4a (M4A Audio File)
- **0x17**: Compressed Files
  - `0x1700` - .zip (ZIP Compressed File)
  - `0x1701` - .rar (RAR Compressed File)
  - `0x1702` - .tar (TAR Archive File)
  - `0x1703` - .gz (GZIP Compressed File)
  - `0x1704` - .7z (7-Zip Compressed File)
  - `0x1705` - .bz2 (BZIP2 Compressed File)
  - `0x1706` - .xz (XZ Compressed File)
- **0x18**: Encrypted Files
  - `0x1800` - .enc (Encrypted File)
  - `0x1801` - .gpg (GPG Encrypted File)
  - `0x1802` - .aes (AES Encrypted File)
- **0x1E**: Folder
  - `0x1E00` - .ufile Uncategorized File
  - `0x1E01` - .nfile Normal File
- **0x1F**: Other Files
  - `0x1F00` - .+ Other Files

#### 0x2***: Metadata

- **0x20**

  - `0x2000` - Uncategorized Meta Data
- **0x21**: user id

  - `0x2100` - example user id
  - `0x2101` - user id
- **0x22**: permission id

  - `0x2200` - Uncategorized permission id
  - `0x2201` - EVERYONE PERMISSION

#### 0x3***: Log Data

(Currently not assigned)

#### 0x4***: Cache Data

(Currently not assigned)

#### 0x5***: Device Data

(Currently not assigned)

#### 0x6***: Object Data

(Currently not assigned)

#### 0xF***: Other Data

(Currently not assigned)

### Reserved for Future Use

- **0x7***: Reserved
- **0x8***: Reserved
- **0x9***: Reserved
- **0xA***: Reserved
- **0xB***: Reserved
- **0xC***: Reserved
- **0xD***: Reserved
- **0xE***: Reserved

This list will be updated as more categories and file types are added.
