//! Public constants for all supported MIME types.
//!
//! This module provides public string constants for all MIME types supported by the library.
//! These constants can be used for comparisons, pattern matching, and configuration.
//!
//! # Usage
//!
//! ```rust
//! use mimetype_detector::{detect, constants::*};
//!
//! let data = b"\x89PNG\r\n\x1a\n";
//! let mime_type = detect(data);
//!
//! if mime_type.is(IMAGE_PNG) {
//!     println!("Found a PNG image!");
//! }
//! ```
//!
//! # Categories
//!
//! Constants are organized by format category:
//! - **Text formats**: HTML, XML, UTF variants
//! - **Document formats**: PDF, PostScript, Office documents
//! - **Archive formats**: ZIP, TAR, 7Z, RAR, and others
//! - **Image formats**: PNG, JPEG, GIF, WebP, and many more
//! - **Audio formats**: MP3, FLAC, WAV, OGG, and others
//! - **Video formats**: MP4, WebM, AVI, MKV, and others
//! - **Executable formats**: ELF, PE/EXE, Java CLASS, WASM
//! - **Font formats**: TTF, OTF, WOFF, WOFF2
//! - **Web & multimedia**: SWF, CRX
//! - **Specialized formats**: DICOM, SQLite, CAD, eBooks

// ============================================================================
// TEXT FORMATS
// ============================================================================

/// HTML document with UTF-8 encoding
pub const TEXT_HTML: &str = "text/html; charset=utf-8";

/// HTML document with UTF-16 encoding
pub const TEXT_HTML_UTF16: &str = "text/html; charset=utf-16";

/// XML document with UTF-8 encoding
pub const TEXT_XML: &str = "text/xml; charset=utf-8";

/// XML document with UTF-16 encoding
pub const TEXT_XML_UTF16: &str = "text/xml; charset=utf-16";

/// Alternative XML MIME type
pub const APPLICATION_XML: &str = "application/xml";

/// Alternative XML MIME type with UTF-16 encoding
pub const APPLICATION_XML_UTF16: &str = "application/xml; charset=utf-16";

/// Plain text with UTF-8 encoding and BOM
pub const TEXT_UTF8_BOM: &str = "text/plain; charset=utf-8";

/// Plain text with UTF-16 Big Endian encoding
pub const TEXT_UTF16_BE: &str = "text/plain; charset=utf-16be";

/// Plain text with UTF-16 Little Endian encoding
pub const TEXT_UTF16_LE: &str = "text/plain; charset=utf-16le";

/// Plain text with UTF-8 encoding
pub const TEXT_UTF8: &str = "text/plain; charset=utf-8";

/// Generic plain text
pub const TEXT_PLAIN: &str = "text/plain";

/// WebAssembly Text format (WAT)
pub const TEXT_WASM: &str = "text/wasm";

// ============================================================================
// DOCUMENT FORMATS
// ============================================================================

/// Adobe Portable Document Format
pub const APPLICATION_PDF: &str = "application/pdf";

/// Alternative PDF MIME type
pub const APPLICATION_X_PDF: &str = "application/x-pdf";

/// Forms Data Format (PDF variant)
pub const APPLICATION_VND_FDF: &str = "application/vnd.fdf";

/// PostScript document
pub const APPLICATION_POSTSCRIPT: &str = "application/postscript";

/// Encapsulated PostScript
pub const APPLICATION_EPS: &str = "application/eps";

/// ClarisWorks document
pub const APPLICATION_X_CLARISWORKS: &str = "application/x-clarisworks";

/// Quark Express document
pub const APPLICATION_VND_QUARK_QUARKXPRESS: &str = "application/vnd.quark.quarkxpress";

/// Microsoft OLE storage (legacy Office documents)
pub const APPLICATION_X_OLE_STORAGE: &str = "application/x-ole-storage";

/// Compound File Binary (OLE storage variant)
pub const APPLICATION_X_CFB: &str = "application/x-cfb";

/// Advanced Authoring Format
pub const APPLICATION_X_AAF: &str = APPLICATION_OCTET_STREAM;

// ============================================================================
// ARCHIVE & COMPRESSION FORMATS
// ============================================================================

/// 7-Zip archive
pub const APPLICATION_X_7Z_COMPRESSED: &str = "application/x-7z-compressed";

/// ZIP archive
pub const APPLICATION_ZIP: &str = "application/zip";

/// Alternative ZIP MIME types
pub const APPLICATION_X_ZIP: &str = "application/x-zip";
pub const APPLICATION_X_ZIP_COMPRESSED: &str = "application/x-zip-compressed";

/// RAR archive
pub const APPLICATION_X_RAR_COMPRESSED: &str = "application/x-rar-compressed";
pub const APPLICATION_X_RAR: &str = "application/x-rar";

/// GZIP compression
pub const APPLICATION_GZIP: &str = "application/gzip";
pub const APPLICATION_X_GZIP: &str = "application/x-gzip";
pub const APPLICATION_X_GUNZIP: &str = "application/x-gunzip";
pub const APPLICATION_GZIPPED: &str = "application/gzipped";
pub const APPLICATION_GZIP_COMPRESSED: &str = "application/gzip-compressed";
pub const APPLICATION_X_GZIP_COMPRESSED: &str = "application/x-gzip-compressed";
pub const GZIP_DOCUMENT: &str = "gzip/document";

/// TAR archive
pub const APPLICATION_X_TAR: &str = "application/x-tar";

/// BZIP compression
pub const APPLICATION_X_BZIP: &str = "application/x-bzip";

/// BZIP2 compression
pub const APPLICATION_X_BZIP2: &str = "application/x-bzip2";

/// XZ compression
pub const APPLICATION_X_XZ: &str = "application/x-xz";

/// Zstandard compression
pub const APPLICATION_ZSTD: &str = "application/zstd";

/// LZIP compression
pub const APPLICATION_LZIP: &str = "application/lzip";

/// LZIP compression (alias)
pub const APPLICATION_X_LZIP: &str = "application/x-lzip";

/// LZ4 Compression
pub const APPLICATION_X_LZ4: &str = "application/x-lz4";

/// Microsoft Cabinet archive
pub const APPLICATION_VND_MS_CAB_COMPRESSED: &str = "application/vnd.ms-cab-compressed";

/// InstallShield Cabinet
pub const APPLICATION_X_INSTALLSHIELD: &str = "application/x-installshield";

/// CPIO archive
pub const APPLICATION_X_CPIO: &str = "application/x-cpio";

/// Unix AR archive
pub const APPLICATION_X_ARCHIVE: &str = "application/x-archive";
pub const APPLICATION_X_UNIX_ARCHIVE: &str = "application/x-unix-archive";

/// Red Hat Package Manager
pub const APPLICATION_X_RPM: &str = "application/x-rpm";

/// BitTorrent metadata
pub const APPLICATION_X_BITTORRENT: &str = "application/x-bittorrent";

/// FITS (Flexible Image Transport System)
pub const APPLICATION_FITS: &str = "application/fits";

/// FITS (alias)
pub const IMAGE_FITS: &str = "image/fits";

/// XAR archive
pub const APPLICATION_X_XAR: &str = "application/x-xar";

/// ARJ archive
pub const APPLICATION_ARJ: &str = "application/arj";
pub const APPLICATION_X_ARJ: &str = "application/x-arj";

/// LHA/LZH archive
pub const APPLICATION_X_LZH_COMPRESSED: &str = "application/x-lzh-compressed";
pub const APPLICATION_X_LHA: &str = "application/x-lha";

/// LArc/LZS archive
pub const APPLICATION_X_LZS_COMPRESSED: &str = "application/x-lzh-compressed";

/// Debian package
pub const APPLICATION_VND_DEBIAN_BINARY_PACKAGE: &str = "application/vnd.debian.binary-package";

/// Web ARChive format
pub const APPLICATION_WARC: &str = "application/warc";

/// ACE Archive
pub const APPLICATION_X_ACE_COMPRESSED: &str = "application/x-ace-compressed";

/// ISO 9660 CD/DVD Image
pub const APPLICATION_X_ISO9660_IMAGE: &str = "application/x-iso9660-image";

/// ALZ Archive
pub const APPLICATION_X_ALZ_COMPRESSED: &str = "application/x-alz-compressed";

/// StuffIt Archive
pub const APPLICATION_X_STUFFIT: &str = "application/x-stuffit";

/// StuffIt X Archive
pub const APPLICATION_X_STUFFITX: &str = "application/x-stuffitx";

/// Mozilla Archive (Firefox/Thunderbird updates)
pub const APPLICATION_X_MOZILLA_ARCHIVE: &str = "application/x-mozilla-archive";

/// RZIP Archive (long-range compression)
pub const APPLICATION_X_RZIP: &str = "application/x-rzip";

/// LRZIP Archive (long-range ZIP)
pub const APPLICATION_X_LRZIP: &str = "application/x-lrzip";

// ============================================================================
// IMAGE FORMATS
// ============================================================================

/// Portable Network Graphics
pub const IMAGE_PNG: &str = "image/png";

/// Animated PNG
pub const IMAGE_VND_MOZILLA_APNG: &str = "image/vnd.mozilla.apng";

/// JPEG image
pub const IMAGE_JPEG: &str = "image/jpeg";

/// JPEG 2000
pub const IMAGE_JP2: &str = "image/jp2";

/// JPEG 2000 Extended
pub const IMAGE_JPX: &str = "image/jpx";

/// JPEG 2000 Multi-part
pub const IMAGE_JPM: &str = "image/jpm";

/// JPEG 2000 Multi-part (alias)
pub const VIDEO_JPM: &str = "video/jpm";

/// JPEG 2000 Codestream (raw codestream without container)
pub const IMAGE_X_JP2_CODESTREAM: &str = "image/x-jp2-codestream";

/// JPEG XS
pub const IMAGE_JXS: &str = "image/jxs";

/// JPEG XR
pub const IMAGE_JXR: &str = "image/jxr";

/// JPEG XR (alias)
pub const IMAGE_VND_MS_PHOTO: &str = "image/vnd.ms-photo";

/// JPEG XL
pub const IMAGE_JXL: &str = "image/jxl";

/// Graphics Interchange Format
pub const IMAGE_GIF: &str = "image/gif";

/// WebP image format
pub const IMAGE_WEBP: &str = "image/webp";

/// Tagged Image File Format
pub const IMAGE_TIFF: &str = "image/tiff";

/// Windows Bitmap
pub const IMAGE_BMP: &str = "image/bmp";
pub const IMAGE_X_BMP: &str = "image/x-bmp";
pub const IMAGE_X_MS_BMP: &str = "image/x-ms-bmp";

/// Windows Icon
pub const IMAGE_X_ICON: &str = "image/x-icon";

/// Apple Icon Image
pub const IMAGE_X_ICNS: &str = "image/x-icns";

/// Adobe Photoshop Document
pub const IMAGE_VND_ADOBE_PHOTOSHOP: &str = "image/vnd.adobe.photoshop";
pub const IMAGE_X_PSD: &str = "image/x-psd";
pub const APPLICATION_PHOTOSHOP: &str = "application/photoshop";

/// High Efficiency Image Container
pub const IMAGE_HEIC: &str = "image/heic";

/// HEIC sequence
pub const IMAGE_HEIC_SEQUENCE: &str = "image/heic-sequence";

/// High Efficiency Image Format
pub const IMAGE_HEIF: &str = "image/heif";

/// HEIF sequence
pub const IMAGE_HEIF_SEQUENCE: &str = "image/heif-sequence";

/// AV1 Image File Format Sequence
pub const IMAGE_AVIF_SEQUENCE: &str = "image/avif-sequence";

/// Better Portable Graphics
pub const IMAGE_BPG: &str = "image/bpg";

/// GIMP native format
pub const IMAGE_X_XCF: &str = "image/x-xcf";

/// GIMP pattern
pub const IMAGE_X_GIMP_PAT: &str = "image/x-gimp-pat";

/// GIMP brush
pub const IMAGE_X_GIMP_GBR: &str = "image/x-gimp-gbr";

/// OpenRaster layered image format
pub const IMAGE_OPENRASTER: &str = "image/openraster";

/// Radiance HDR
pub const IMAGE_VND_RADIANCE: &str = "image/vnd.radiance";

/// X11 Pixmap
pub const IMAGE_X_XPIXMAP: &str = "image/x-xpixmap";

/// Portable Bitmap (Netpbm)
pub const IMAGE_X_PORTABLE_BITMAP: &str = "image/x-portable-bitmap";

/// Portable Graymap (Netpbm)
pub const IMAGE_X_PORTABLE_GRAYMAP: &str = "image/x-portable-graymap";

/// Portable Pixmap (Netpbm)
pub const IMAGE_X_PORTABLE_PIXMAP: &str = "image/x-portable-pixmap";

/// Portable Arbitrary Map (Netpbm)
pub const IMAGE_X_PORTABLE_ARBITRARYMAP: &str = "image/x-portable-arbitrarymap";

/// AutoCAD Drawing
pub const IMAGE_VND_DWG: &str = "image/vnd.dwg";
pub const IMAGE_X_DWG: &str = "image/x-dwg";
pub const APPLICATION_ACAD: &str = "application/acad";
pub const APPLICATION_X_ACAD: &str = "application/x-acad";
pub const APPLICATION_AUTOCAD_DWG: &str = "application/autocad_dwg";
pub const APPLICATION_DWG: &str = "application/dwg";
pub const APPLICATION_X_DWG: &str = "application/x-dwg";
pub const APPLICATION_X_AUTOCAD: &str = "application/x-autocad";
pub const DRAWING_DWG: &str = "drawing/dwg";

/// AutoCAD Drawing Exchange Format (DXF) ASCII
pub const IMAGE_VND_DXF: &str = "image/vnd.dxf";

/// AutoCAD Drawing Exchange Format (DXF) Binary
pub const APPLICATION_X_DXF: &str = "application/x-dxf";

/// DjVu document format
pub const IMAGE_VND_DJVU: &str = "image/vnd.djvu";

/// AVIF Image Sequence
pub const IMAGE_AVIF: &str = "image/avif";

/// Animated PNG
pub const IMAGE_APNG: &str = "image/apng";

/// Quite OK Image Format
pub const IMAGE_X_QOI: &str = "image/x-qoi";

/// Free Lossless Image Format
pub const IMAGE_FLIF: &str = "image/flif";

/// Khronos Texture 2.0
pub const IMAGE_KTX2: &str = "image/ktx2";

/// OpenEXR High Dynamic Range
pub const IMAGE_X_EXR: &str = "image/x-exr";

/// Enhanced Metafile
pub const IMAGE_EMF: &str = "image/emf";

/// Windows Metafile
pub const IMAGE_WMF: &str = "image/wmf";

/// DirectDraw Surface
pub const IMAGE_VND_MS_DDS: &str = "image/vnd-ms.dds";

/// PC Paintbrush
pub const IMAGE_X_PCX: &str = "image/x-pcx";

/// Khronos Texture
pub const IMAGE_KTX: &str = "image/ktx";

/// ARM Texture Compression
pub const IMAGE_X_ASTC: &str = "image/x-astc";

/// Windows Animated Cursor
pub const APPLICATION_X_NAVI_ANIMATION: &str = "application/x-navi-animation";

/// CorelDRAW
pub const APPLICATION_VND_COREL_DRAW: &str = "application/vnd.corel-draw";
pub const APPLICATION_CDR: &str = "application/cdr";
pub const APPLICATION_X_CDR: &str = "application/x-cdr";

/// IFF/ILBM (Amiga)
pub const IMAGE_X_ILBM: &str = "image/x-ilbm";
pub const IMAGE_X_IFF: &str = "image/x-iff";

/// Truevision TGA (Targa)
pub const IMAGE_X_TGA: &str = "image/x-tga";

/// Sun Raster
pub const IMAGE_X_SUN_RASTER: &str = "image/x-sun-raster";

/// Silicon Graphics Image
pub const IMAGE_X_SGI: &str = "image/x-sgi";

/// Farbfeld lossless image format
pub const IMAGE_X_FARBFELD: &str = "image/x-ff";

/// JPEG-LS lossless/near-lossless compression
pub const IMAGE_JLS: &str = "image/jls";

/// Magick Image File Format (ImageMagick)
pub const IMAGE_X_MIFF: &str = "image/x-miff";

/// Portable FloatMap (HDR Netpbm format)
pub const IMAGE_X_PFM: &str = "image/x-pfm";

/// Multiple-image Network Graphics (animated PNG-like)
pub const IMAGE_X_MNG: &str = "image/x-mng";

/// JPEG Network Graphics (JPEG with PNG-style chunks)
pub const IMAGE_X_JNG: &str = "image/x-jng";

/// Sketch design file (Bohemian Coding)
pub const IMAGE_X_SKETCH: &str = "image/x-sketch";

// ============================================================================
// AUDIO FORMATS
// ============================================================================

/// MPEG Audio Layer 3
pub const AUDIO_MPEG: &str = "audio/mpeg";
pub const AUDIO_X_MPEG: &str = "audio/x-mpeg";
pub const AUDIO_MP3: &str = "audio/mp3";

/// MPEG-1/2 Audio Layer 2
pub const AUDIO_MP2: &str = "audio/mpeg";

/// Free Lossless Audio Codec
pub const AUDIO_FLAC: &str = "audio/flac";
pub const AUDIO_X_FLAC: &str = "audio/x-flac";

/// Waveform Audio File Format
pub const AUDIO_WAV: &str = "audio/wav";
pub const AUDIO_X_WAV: &str = "audio/x-wav";
pub const AUDIO_VND_WAVE: &str = "audio/vnd.wave";
pub const AUDIO_WAVE: &str = "audio/wave";

/// Audio Interchange File Format
pub const AUDIO_AIFF: &str = "audio/aiff";

/// Audio Interchange File Format (alias)
pub const AUDIO_X_AIFF: &str = "audio/x-aiff";

/// Musical Instrument Digital Interface
pub const AUDIO_MIDI: &str = "audio/midi";
pub const AUDIO_MID: &str = "audio/mid";

/// Ogg container format
pub const APPLICATION_OGG: &str = "application/ogg";

/// Ogg container format (alias)
pub const APPLICATION_X_OGG: &str = "application/x-ogg";

/// OGG Audio
pub const AUDIO_OGG: &str = "audio/ogg";

/// OGG Video
pub const VIDEO_OGG: &str = "video/ogg";

/// OGG Media (video container)
pub const VIDEO_OGG_MEDIA: &str = "video/ogg";

/// OGG Multiplexed (audio+video+text)
pub const APPLICATION_OGG_MULTIPLEXED: &str = "application/ogg";

/// Monkey's Audio
pub const AUDIO_APE: &str = "audio/ape";

/// Musepack
pub const AUDIO_MUSEPACK: &str = "audio/musepack";

/// Audio Codec 3 (Dolby Digital)
pub const AUDIO_AC3: &str = "audio/ac3";

/// DTS Audio (Digital Theater Systems)
pub const AUDIO_DTS: &str = "audio/vnd.dts";
pub const AUDIO_DTS_HD: &str = "audio/vnd.dts.hd";

/// Ogg Opus
pub const AUDIO_OPUS: &str = "audio/opus";

/// Sun/NeXT Audio
pub const AUDIO_BASIC: &str = "audio/basic";

/// Adaptive Multi-Rate
pub const AUDIO_AMR: &str = "audio/amr";

/// Adaptive Multi-Rate (alias)
pub const AUDIO_AMR_NB: &str = "audio/amr-nb";

/// Creative Voice File
pub const AUDIO_X_UNKNOWN: &str = "audio/x-unknown";

/// M3U playlist
pub const AUDIO_X_MPEGURL: &str = "audio/x-mpegurl";

/// M3U playlist (alias)
pub const AUDIO_MPEGURL: &str = "audio/mpegurl";

/// Advanced Audio Coding
pub const AUDIO_AAC: &str = "audio/aac";

/// Qualcomm PureVoice
pub const AUDIO_QCELP: &str = "audio/qcelp";

/// MPEG-4 Audio
pub const AUDIO_MP4: &str = "audio/mp4";
pub const AUDIO_X_M4A: &str = "audio/x-m4a";
pub const AUDIO_X_MP4A: &str = "audio/x-mp4a";

/// WavPack Lossless Audio
pub const AUDIO_X_WAVPACK: &str = "audio/x-wavpack";

/// True Audio Lossless
pub const AUDIO_X_TTA: &str = "audio/x-tta";

/// DSD Stream File
pub const AUDIO_X_DSF: &str = "audio/x-dsf";

/// DSD Interchange File Format
pub const AUDIO_X_DFF: &str = "audio/x-dff";

/// Scream Tracker 3 Module
pub const AUDIO_S3M: &str = "audio/s3m";

/// Shoutcast Playlist
pub const AUDIO_X_SCPLS: &str = "audio/x-scpls";

// ============================================================================
// VIDEO FORMATS
// ============================================================================

/// MPEG-4 Part 14
pub const VIDEO_MP4: &str = "video/mp4";

/// WebM video format
pub const VIDEO_WEBM: &str = "video/webm";
pub const AUDIO_WEBM: &str = "audio/webm";

/// Matroska video
pub const VIDEO_X_MATROSKA: &str = "video/x-matroska";

/// Audio Video Interleave
pub const VIDEO_X_MSVIDEO: &str = "video/x-msvideo";
pub const VIDEO_AVI: &str = "video/avi";
pub const VIDEO_MSVIDEO: &str = "video/msvideo";

/// MPEG video
pub const VIDEO_MPEG: &str = "video/mpeg";

/// QuickTime movie
pub const VIDEO_QUICKTIME: &str = "video/quicktime";

/// Flash Video
pub const VIDEO_X_FLV: &str = "video/x-flv";

/// Advanced Systems Format
pub const VIDEO_X_MS_ASF: &str = "video/x-ms-asf";
pub const VIDEO_ASF: &str = "video/asf";
pub const VIDEO_X_MS_WMV: &str = "video/x-ms-wmv";

/// Windows Media Audio
pub const AUDIO_X_MS_WMA: &str = "audio/x-ms-wma";

/// Advanced Stream Redirector (ASF playlist)
pub const VIDEO_X_MS_ASX: &str = "video/x-ms-asx";

/// Microsoft Digital Video Recording
pub const VIDEO_X_MS_DVR: &str = "video/x-ms-asf";

/// CD Audio track
pub const APPLICATION_X_CDF: &str = "application/x-cdf";

/// iTunes Video
pub const VIDEO_X_M4V: &str = "video/x-m4v";

/// MTV video format
pub const VIDEO_X_MTV: &str = "video/x-mtv";

/// MPEG-2 Transport Stream
pub const VIDEO_MP2T: &str = "video/mp2t";

/// Actions Media Video
pub const VIDEO_X_AMV: &str = "video/x-amv";

/// 3rd Generation Partnership Project (3GPP)
pub const VIDEO_3GPP: &str = "video/3gpp";
pub const AUDIO_3GPP: &str = "audio/3gpp";
pub const VIDEO_3GP: &str = "video/3gp";

/// 3rd Generation Partnership Project 2 (3GPP2)
pub const VIDEO_3GPP2: &str = "video/3gpp2";
pub const AUDIO_3GPP2: &str = "audio/3gpp2";
pub const VIDEO_3G2: &str = "video/3g2";

/// RealMedia
pub const APPLICATION_VND_RN_REALMEDIA: &str = "application/vnd.rn-realmedia";

/// RealMedia Variable Bitrate
pub const APPLICATION_VND_RN_REALMEDIA_VBR: &str = "application/vnd.rn-realmedia-vbr";

/// RealVideo
pub const VIDEO_X_PN_REALVIDEO: &str = "video/x-pn-realvideo";

/// Silicon Graphics Movie
pub const VIDEO_X_SGI_MOVIE: &str = "video/x-sgi-movie";

/// Motion JPEG 2000
pub const VIDEO_MJ2: &str = "video/mj2";

/// Digital Video Broadcasting
pub const VIDEO_VND_DVB_FILE: &str = "video/vnd.dvb.file";

/// Autodesk FLIC Animation (FLI)
pub const VIDEO_FLI: &str = "video/fli";

/// Autodesk FLIC Animation (FLC)
pub const VIDEO_FLC: &str = "video/flc";

/// Fast Search and Transfer Video
pub const VIDEO_VND_FVT: &str = "video/vnd.fvt";

/// Material Exchange Format (professional video)
pub const APPLICATION_MXF: &str = "application/mxf";

/// MPEG-2 Program Stream
pub const VIDEO_MP2P: &str = "video/mp2p";

/// Windows Recorded TV Show
pub const VIDEO_X_WTV: &str = "video/x-wtv";

// ============================================================================
// EXECUTABLE & BINARY FORMATS
// ============================================================================

/// Windows Portable Executable
pub const APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE: &str =
    "application/vnd.microsoft.portable-executable";

/// Executable and Linkable Format
pub const APPLICATION_X_ELF: &str = "application/x-elf";

/// Java Class file
pub const APPLICATION_X_JAVA_APPLET: &str = "application/x-java-applet";
pub const APPLICATION_X_JAVA_APPLET_BINARY: &str = "application/x-java-applet; charset=binary";

/// WebAssembly binary
pub const APPLICATION_WASM: &str = "application/wasm";

/// Amiga Hunk Executable
pub const APPLICATION_X_AMIGA_EXECUTABLE: &str = "application/x-amiga-executable";

/// Xbox Executable
pub const APPLICATION_X_XBOX_EXECUTABLE: &str = "application/x-xbox-executable";

/// Xbox 360 Executable
pub const APPLICATION_X_XBOX360_EXECUTABLE: &str = "application/x-xbox360-executable";

/// AppImage Linux Application
pub const APPLICATION_X_APPIMAGE: &str = "application/x-appimage";

/// LLVM Bitcode
pub const APPLICATION_X_LLVM: &str = "application/x-llvm";

/// Apache Arrow Columnar Format
pub const APPLICATION_VND_APACHE_ARROW_FILE: &str = "application/vnd.apache.arrow.file";

/// Apache Avro
pub const APPLICATION_VND_APACHE_AVRO: &str = "application/vnd.apache.avro";

/// ID3v2 Audio Metadata
pub const APPLICATION_X_ID3V2: &str = "application/x-id3v2";

/// ICC Color Profile
pub const APPLICATION_VND_ICCPROFILE: &str = "application/vnd.iccprofile";

/// PEM Certificate/Key File
pub const APPLICATION_X_PEM_FILE: &str = "application/x-pem-file";

/// Age Encryption
pub const APPLICATION_X_AGE_ENCRYPTION: &str = "application/x-age-encryption";

/// Extensible Binary Meta Language
pub const APPLICATION_X_EBML: &str = "application/x-ebml";

// ============================================================================
// FONT FORMATS
// ============================================================================

/// TrueType Font
pub const FONT_TTF: &str = "font/ttf";
pub const FONT_SFNT: &str = "font/sfnt";
pub const APPLICATION_X_FONT_TTF: &str = "application/x-font-ttf";
pub const APPLICATION_FONT_SFNT: &str = "application/font-sfnt";

/// Web Open Font Format
pub const FONT_WOFF: &str = "font/woff";

/// Web Open Font Format 2
pub const FONT_WOFF2: &str = "font/woff2";

/// OpenType Font
pub const FONT_OTF: &str = "font/otf";

/// Embedded OpenType
pub const APPLICATION_VND_MS_FONTOBJECT: &str = "application/vnd.ms-fontobject";

/// TrueType Collection
pub const FONT_COLLECTION: &str = "font/collection";

/// AngelCode BMFont binary format
pub const APPLICATION_X_ANGELCODE_BMFONT: &str = "application/x-angelcode-bmfont";

/// Glyphs font editor format
pub const FONT_X_GLYPHS: &str = "font/x-glyphs";

// ============================================================================
// WEB & MULTIMEDIA FORMATS
// ============================================================================

/// Adobe Flash
pub const APPLICATION_X_SHOCKWAVE_FLASH: &str = "application/x-shockwave-flash";

/// Chrome Extension
pub const APPLICATION_X_CHROME_EXTENSION: &str = "application/x-chrome-extension";

/// PKCS#7 Signature
pub const APPLICATION_PKCS7_SIGNATURE: &str = "application/pkcs7-signature";

// ============================================================================
// SPECIALIZED FORMATS
// ============================================================================

/// DICOM medical imaging
pub const APPLICATION_DICOM: &str = "application/dicom";

/// Mobipocket eBook
pub const APPLICATION_X_MOBIPOCKET_EBOOK: &str = "application/x-mobipocket-ebook";

/// Fasoo document protection
pub const APPLICATION_X_FASOO: &str = "application/x-fasoo";

/// Adobe InDesign Document
pub const APPLICATION_X_INDESIGN: &str = "application/x-indesign";

/// Adobe InDesign Markup Language
pub const APPLICATION_VND_ADOBE_INDESIGN_IDML_PACKAGE: &str =
    "application/vnd.adobe.indesign-idml-package";

/// Adobe Illustrator Artwork
pub const APPLICATION_VND_ADOBE_ILLUSTRATOR: &str = "application/vnd.adobe.illustrator";

/// Adobe Integrated Runtime
pub const APPLICATION_VND_ADOBE_AIR_APPLICATION_INSTALLER_PACKAGE_ZIP: &str =
    "application/vnd.adobe.air-application-installer-package+zip";

/// Adobe Flash Project
pub const APPLICATION_VND_ADOBE_FLA: &str = "application/vnd.adobe.fla";

/// Meta Information Encapsulation
pub const APPLICATION_X_MIE: &str = "application/x-mie";

/// PGP NetShare
pub const APPLICATION_X_PGP_NET_SHARE: &str = "application/x-pgp-net-share";

// ============================================================================
// MICROSOFT OFFICE & DOCUMENT FORMATS
// ============================================================================

/// Microsoft Word 2007+ Document
pub const APPLICATION_VND_OPENXML_WORDPROCESSINGML_DOCUMENT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";

/// Microsoft Excel 2007+ Spreadsheet
pub const APPLICATION_VND_OPENXML_SPREADSHEETML_SHEET: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";

/// Microsoft PowerPoint 2007+ Presentation
pub const APPLICATION_VND_OPENXML_PRESENTATIONML_PRESENTATION: &str =
    "application/vnd.openxmlformats-officedocument.presentationml.presentation";

/// Microsoft Visio Drawing 2007+
pub const APPLICATION_VND_MS_VISIO_DRAWING_MAIN_XML: &str =
    "application/vnd.ms-visio.drawing.main+xml";

/// Microsoft HTML Help
pub const APPLICATION_VND_MS_HTMLHELP: &str = "application/vnd.ms-htmlhelp";

/// Microsoft OneNote
pub const APPLICATION_ONENOTE: &str = "application/onenote";

/// EPUB Electronic Publication
pub const APPLICATION_EPUB_ZIP: &str = "application/epub+zip";

/// Java Archive
pub const APPLICATION_JAVA_ARCHIVE: &str = "application/java-archive";

/// Java Archive (aliases)
pub const APPLICATION_JAR: &str = "application/jar";
pub const APPLICATION_JAR_ARCHIVE: &str = "application/jar-archive";
pub const APPLICATION_X_JAVA_ARCHIVE: &str = "application/x-java-archive";

/// Enterprise Application Archive (Java EE)
pub const APPLICATION_X_EAR: &str = "application/x-ear";

/// Android Package
pub const APPLICATION_VND_ANDROID_PACKAGE_ARCHIVE: &str = "application/vnd.android.package-archive";

/// Android App Bundle
pub const APPLICATION_VND_ANDROID_AAB: &str = "application/vnd.android.aab";

/// iOS App Store Package
pub const APPLICATION_X_IOS_APP: &str = "application/x-ios-app";

/// Microsoft Excel legacy format
pub const APPLICATION_VND_MS_EXCEL: &str = "application/vnd.ms-excel";

/// Microsoft Excel alias
pub const APPLICATION_MSEXCEL: &str = "application/msexcel";

/// Microsoft Word legacy format
pub const APPLICATION_MSWORD: &str = "application/msword";

/// Microsoft Word alias
pub const APPLICATION_VND_MS_WORD: &str = "application/vnd.ms-word";

/// WordPerfect Document
pub const APPLICATION_VND_WORDPERFECT: &str = "application/vnd.wordperfect";

/// AbiWord Document
pub const APPLICATION_X_ABIWORD: &str = "application/x-abiword";

/// AbiWord Template
pub const APPLICATION_X_ABIWORD_TEMPLATE: &str = "application/x-abiword-template";

/// Microsoft PowerPoint legacy format
pub const APPLICATION_VND_MS_POWERPOINT: &str = "application/vnd.ms-powerpoint";

/// Microsoft PowerPoint alias
pub const APPLICATION_MSPOWERPOINT: &str = "application/mspowerpoint";

/// Microsoft Publisher
pub const APPLICATION_VND_MS_PUBLISHER: &str = "application/vnd.ms-publisher";

/// Microsoft Outlook Message
pub const APPLICATION_VND_MS_OUTLOOK: &str = "application/vnd.ms-outlook";

/// Microsoft Outlook Personal Storage Table
pub const APPLICATION_VND_MS_OUTLOOK_PST: &str = "application/vnd.ms-outlook";

/// Microsoft Project Plan
pub const APPLICATION_VND_MS_PROJECT: &str = "application/vnd.ms-project";

/// Microsoft Visio Drawing
pub const APPLICATION_VND_VISIO: &str = "application/vnd.visio";

/// Microsoft Works Database
pub const APPLICATION_VND_MS_WORKS_DB: &str = "application/vnd.ms-works-db";

/// Microsoft Works Spreadsheet
pub const APPLICATION_VND_MS_WORKS: &str = "application/vnd.ms-works";

/// Microsoft Write
pub const APPLICATION_X_MSWRITE: &str = "application/x-mswrite";

/// Windows Media Playlist
pub const APPLICATION_VND_MS_WPL: &str = "application/vnd.ms-wpl";

/// Microsoft Installer
pub const APPLICATION_X_MS_INSTALLER: &str = "application/x-ms-installer";

/// Microsoft Installer alias
pub const APPLICATION_X_WINDOWS_INSTALLER: &str = "application/x-windows-installer";

/// Microsoft Installer alias
pub const APPLICATION_X_MSI: &str = "application/x-msi";

/// Microsoft Installer Patch
pub const APPLICATION_X_MS_PATCH: &str = "application/x-ms-patch";

/// Microsoft Installer Patch alias
pub const APPLICATION_X_MSP: &str = "application/x-msp";

/// Windows App Package
pub const APPLICATION_VND_MS_APPX: &str = "application/vnd.ms-appx";

/// Windows App Bundle
pub const APPLICATION_VND_MS_APPX_BUNDLE: &str = "application/vnd.ms-appx.bundle";

/// Microsoft Reader eBook
pub const APPLICATION_X_MS_READER: &str = "application/x-ms-reader";

/// Microsoft Visual Studio Solution
pub const APPLICATION_VND_MS_DEVELOPER: &str = "application/vnd.ms-developer";

/// Microsoft Visual Studio Extension
pub const APPLICATION_VSIX: &str = "application/vsix";

// ============================================================================
// OPEN DOCUMENT FORMATS
// ============================================================================

/// OpenDocument Text
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT: &str = "application/vnd.oasis.opendocument.text";

/// OpenDocument Spreadsheet
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET: &str =
    "application/vnd.oasis.opendocument.spreadsheet";

/// OpenDocument Presentation
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION: &str =
    "application/vnd.oasis.opendocument.presentation";

/// OpenDocument Graphics
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS: &str =
    "application/vnd.oasis.opendocument.graphics";

/// OpenDocument Formula
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_FORMULA: &str =
    "application/vnd.oasis.opendocument.formula";

/// OpenDocument Chart
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_CHART: &str =
    "application/vnd.oasis.opendocument.chart";

/// OpenDocument Database
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_DATABASE: &str =
    "application/vnd.oasis.opendocument.database";

/// OpenDocument Text Master
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_MASTER: &str =
    "application/vnd.oasis.opendocument.text-master";

/// OpenOffice Calc Spreadsheet
pub const APPLICATION_VND_SUN_XML_CALC: &str = "application/vnd.sun.xml.calc";

/// Google Earth KMZ (Zipped KML)
pub const APPLICATION_VND_GOOGLE_EARTH_KMZ: &str = "application/vnd.google-earth.kmz";

/// OpenDocument Text Template
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_TEMPLATE: &str =
    "application/vnd.oasis.opendocument.text-template";

/// OpenDocument Spreadsheet Template
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET_TEMPLATE: &str =
    "application/vnd.oasis.opendocument.spreadsheet-template";

/// OpenDocument Presentation Template
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION_TEMPLATE: &str =
    "application/vnd.oasis.opendocument.presentation-template";

/// OpenDocument Graphics Template
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS_TEMPLATE: &str =
    "application/vnd.oasis.opendocument.graphics-template";

/// OpenDocument Text Master Template
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_MASTER_TEMPLATE: &str =
    "application/vnd.oasis.opendocument.text-master-template";

// ============================================================================
// DATABASE FORMATS
// ============================================================================

/// Microsoft Access Database
pub const APPLICATION_X_MSACCESS: &str = "application/x-msaccess";

/// dBase Database File
pub const APPLICATION_X_DBF: &str = "application/x-dbf";

/// Lotus 1-2-3 Spreadsheet
pub const APPLICATION_VND_LOTUS_1_2_3: &str = "application/vnd.lotus-1-2-3";

/// Lotus Notes Database
pub const APPLICATION_VND_LOTUS_NOTES: &str = "application/vnd.lotus-notes";

/// MARC Library Records
pub const APPLICATION_MARC: &str = "application/marc";

/// SQLite database
pub const APPLICATION_VND_SQLITE3: &str = "application/vnd.sqlite3";

/// SQLite database (alias)
pub const APPLICATION_X_SQLITE3: &str = "application/x-sqlite3";

// ============================================================================
// PROGRAMMING & TEXT FORMATS
// ============================================================================

/// PHP Script
pub const TEXT_X_PHP: &str = "text/x-php";

/// JavaScript
pub const TEXT_JAVASCRIPT: &str = "text/javascript";
pub const APPLICATION_JAVASCRIPT: &str = "application/javascript";

/// Python Script
pub const TEXT_X_PYTHON: &str = "text/x-python";

/// Python Script (aliases)
pub const TEXT_X_SCRIPT_PYTHON: &str = "text/x-script.python";
pub const APPLICATION_X_PYTHON: &str = "application/x-python";

/// Perl Script
pub const TEXT_X_PERL: &str = "text/x-perl";

/// Ruby Script
pub const TEXT_X_RUBY: &str = "text/x-ruby";

/// Ruby Script (alias)
pub const APPLICATION_X_RUBY: &str = "application/x-ruby";

/// Lua Script
pub const TEXT_X_LUA: &str = "text/x-lua";

/// Shell Script
pub const TEXT_X_SHELLSCRIPT: &str = "text/x-shellscript";

/// Shell Script (aliases)
pub const TEXT_X_SH: &str = "text/x-sh";
pub const APPLICATION_X_SHELLSCRIPT: &str = "application/x-shellscript";
pub const APPLICATION_X_SH: &str = "application/x-sh";

/// Tcl Script
pub const TEXT_X_TCL: &str = "text/x-tcl";

/// Tcl Script (alias)
pub const APPLICATION_X_TCL: &str = "application/x-tcl";

/// Clojure Script
pub const TEXT_X_CLOJURE: &str = "text/x-clojure";

/// LaTeX Document
pub const TEXT_X_TEX: &str = "text/x-tex";

/// JSON Data
pub const APPLICATION_JSON: &str = "application/json";

/// JSON Data with UTF-16 encoding
pub const APPLICATION_JSON_UTF16: &str = "application/json; charset=utf-16";

/// GeoJSON Geographic Data
pub const APPLICATION_GEO_JSON: &str = "application/geo+json";

/// Newline Delimited JSON
pub const APPLICATION_X_NDJSON: &str = "application/x-ndjson";

/// CSV Data
pub const TEXT_CSV: &str = "text/csv";

/// CSV Data with UTF-16 encoding
pub const TEXT_CSV_UTF16: &str = "text/csv; charset=utf-16";

/// Tab Separated Values
pub const TEXT_TAB_SEPARATED_VALUES: &str = "text/tab-separated-values";

/// Tab Separated Values with UTF-16 encoding
pub const TEXT_TAB_SEPARATED_VALUES_UTF16: &str = "text/tab-separated-values; charset=utf-16";

/// Pipe Separated Values
pub const TEXT_PIPE_SEPARATED_VALUES: &str = "text/pipe-separated-values";

/// Pipe Separated Values with UTF-16 encoding
pub const TEXT_PIPE_SEPARATED_VALUES_UTF16: &str = "text/pipe-separated-values; charset=utf-16";

/// Semicolon Separated Values
pub const TEXT_SEMICOLON_SEPARATED_VALUES: &str = "text/semicolon-separated-values";

/// Semicolon Separated Values with UTF-16 encoding
pub const TEXT_SEMICOLON_SEPARATED_VALUES_UTF16: &str =
    "text/semicolon-separated-values; charset=utf-16";

/// Rich Text Format
pub const TEXT_RTF: &str = "text/rtf";

/// Rich Text Format with UTF-16 encoding
pub const TEXT_RTF_UTF16: &str = "text/rtf; charset=utf-16";

/// Rich Text Format (alias)
pub const APPLICATION_RTF: &str = "application/rtf";

/// SubRip Subtitles
pub const APPLICATION_X_SUBRIP: &str = "application/x-subrip";

/// SubRip Subtitles with UTF-16 encoding
pub const APPLICATION_X_SUBRIP_UTF16: &str = "application/x-subrip; charset=utf-16";

/// SubRip Subtitles (aliases)
pub const APPLICATION_X_SRT: &str = "application/x-srt";
pub const TEXT_X_SRT: &str = "text/x-srt";

/// WebVTT Subtitles
pub const TEXT_VTT: &str = "text/vtt";

/// WebVTT Subtitles with UTF-16 encoding
pub const TEXT_VTT_UTF16: &str = "text/vtt; charset=utf-16";

/// vCard Contact
pub const TEXT_VCARD: &str = "text/vcard";

/// vCard Contact with UTF-16 encoding
pub const TEXT_VCARD_UTF16: &str = "text/vcard; charset=utf-16";

/// iCalendar
pub const TEXT_CALENDAR: &str = "text/calendar";

/// iCalendar with UTF-16 encoding
pub const TEXT_CALENDAR_UTF16: &str = "text/calendar; charset=utf-16";

/// Scalable Vector Graphics
pub const IMAGE_SVG_XML: &str = "image/svg+xml";

/// Scalable Vector Graphics with UTF-16 encoding
pub const IMAGE_SVG_XML_UTF16: &str = "image/svg+xml; charset=utf-16";

// ============================================================================
// BASE MIME TYPES WITHOUT CHARSET (for UTF-16 aliases)
// ============================================================================

/// Base HTML MIME type without charset
pub const TEXT_HTML_BASE: &str = "text/html";

/// Base XML MIME type without charset
pub const TEXT_XML_BASE: &str = "text/xml";

/// Base JSON MIME type without charset
pub const APPLICATION_JSON_BASE: &str = "application/json";

/// JSON Feed format
pub const APPLICATION_FEED_JSON: &str = "application/feed+json";

/// Base CSV MIME type without charset
pub const TEXT_CSV_BASE: &str = "text/csv";

/// Base TSV MIME type without charset
pub const TEXT_TAB_SEPARATED_VALUES_BASE: &str = "text/tab-separated-values";

/// Base PSV MIME type without charset
pub const TEXT_PIPE_SEPARATED_VALUES_BASE: &str = "text/pipe-separated-values";

/// Base SSV MIME type without charset
pub const TEXT_SEMICOLON_SEPARATED_VALUES_BASE: &str = "text/semicolon-separated-values";

/// Base SRT MIME type without charset
pub const APPLICATION_X_SUBRIP_BASE: &str = "application/x-subrip";

/// Base VTT MIME type without charset
pub const TEXT_VTT_BASE: &str = "text/vtt";

/// Base vCard MIME type without charset
pub const TEXT_VCARD_BASE: &str = "text/vcard";

/// Base iCalendar MIME type without charset
pub const TEXT_CALENDAR_BASE: &str = "text/calendar";

/// Base RTF MIME type without charset
pub const TEXT_RTF_BASE: &str = "text/rtf";

/// Base SVG MIME type without charset
pub const IMAGE_SVG_XML_BASE: &str = "image/svg+xml";

// ============================================================================
// XML-BASED FORMATS
// ============================================================================

/// RSS Feed
pub const APPLICATION_RSS_XML: &str = "application/rss+xml";

/// RSS Feed (alias)
pub const TEXT_RSS: &str = "text/rss";

/// Atom Feed  
pub const APPLICATION_ATOM_XML: &str = "application/atom+xml";

/// X3D 3D Graphics
pub const MODEL_X3D_XML: &str = "model/x3d+xml";

/// Google Earth KML
pub const APPLICATION_VND_GOOGLE_EARTH_KML_XML: &str = "application/vnd.google-earth.kml+xml";

/// XLIFF Translation
pub const APPLICATION_X_XLIFF_XML: &str = "application/x-xliff+xml";

/// Collada 3D Graphics
pub const MODEL_VND_COLLADA_XML: &str = "model/vnd.collada+xml";

/// Geography Markup Language
pub const APPLICATION_GML_XML: &str = "application/gml+xml";

/// GPS Exchange Format
pub const APPLICATION_GPX_XML: &str = "application/gpx+xml";

/// Training Center XML
pub const APPLICATION_VND_GARMIN_TCX_XML: &str = "application/vnd.garmin.tcx+xml";

/// Additive Manufacturing Format
pub const APPLICATION_X_AMF: &str = "application/x-amf";

/// 3D Manufacturing Format
pub const APPLICATION_VND_MS_PACKAGE_3DMANUFACTURING_3DMODEL_XML: &str =
    "application/vnd.ms-package.3dmanufacturing-3dmodel+xml";

/// Adobe XFDF
pub const APPLICATION_VND_ADOBE_XFDF: &str = "application/vnd.adobe.xfdf";

/// OWL Ontology
pub const APPLICATION_OWL_XML: &str = "application/owl+xml";

/// XHTML
pub const APPLICATION_XHTML_XML: &str = "application/xhtml+xml";

/// HTTP Archive Format
pub const APPLICATION_JSON_HAR: &str = "application/json";

// ============================================================================
// 3D & GEOSPATIAL FORMATS
// ============================================================================

/// ESRI Shapefile
pub const APPLICATION_VND_SHP: &str = "application/vnd.shp";

/// ESRI Shapefile Index
pub const APPLICATION_VND_SHX: &str = "application/vnd.shx";

/// glTF Binary
pub const MODEL_GLTF_BINARY: &str = "model/gltf-binary";

/// glTF JSON
pub const MODEL_GLTF_JSON: &str = "model/gltf+json";

/// Universal 3D
pub const MODEL_U3D: &str = "model/u3d";

/// Autodesk FBX (Filmbox)
pub const APPLICATION_VND_AUTODESK_FBX: &str = "application/vnd.autodesk.fbx";

/// STL 3D Model (STereoLithography)
pub const MODEL_STL: &str = "model/stl";

/// STL ASCII variant
pub const MODEL_X_STL_ASCII: &str = "model/x.stl-ascii";

/// Autodesk Maya Binary
pub const APPLICATION_X_MAYA_BINARY: &str = "application/x-maya-binary";

/// Autodesk Maya ASCII
pub const APPLICATION_X_MAYA_ASCII: &str = "application/x-maya-ascii";

/// InterQuake Model format
pub const MODEL_X_IQM: &str = "model/x-iqm";

/// MagicaVoxel voxel format
pub const MODEL_X_VOX: &str = "model/x-vox";

/// Google Draco 3D compression
pub const MODEL_X_DRACO: &str = "model/x-draco";

/// STEP 3D model (ISO 10303-21)
pub const MODEL_STEP: &str = "model/step";

/// Initial Graphics Exchange Specification (IGES)
pub const MODEL_IGES: &str = "model/iges";

/// Virtual Reality Modeling Language
pub const MODEL_VRML: &str = "model/vrml";

/// Cinema 4D 3D model format
pub const MODEL_X_C4D: &str = "model/x-c4d";

/// Autodesk Alias (WIRE) 3D model format
pub const MODEL_X_WIRE: &str = "model/x-wire";

/// Design Web Format (Autodesk DWF)
pub const MODEL_VND_DWF: &str = "model/vnd.dwf";

/// OpenNURBS 3D model format (Rhino)
pub const MODEL_X_3DM: &str = "model/x-3dm";

/// Universal Scene Description Binary (Pixar)
pub const MODEL_X_USD: &str = "model/x-usd";

/// Universal Scene Description ASCII (Pixar)
pub const MODEL_X_USD_ASCII: &str = "model/x-usd-ascii";

/// Universal Scene Description ZIP (Pixar)
pub const MODEL_VND_USDZ_ZIP: &str = "model/vnd.usdz+zip";

/// Model3D binary format
pub const MODEL_X_3D_BINARY: &str = "model/x-3d-binary";

/// SketchUp 3D model format
pub const APPLICATION_VND_SKETCHUP_SKP: &str = "application/vnd.sketchup.skp";

/// SolidWorks Assembly
pub const MODEL_X_SLDASM: &str = "model/x-sldasm";

/// SolidWorks Drawing
pub const MODEL_X_SLDDRW: &str = "model/x-slddrw";

/// SolidWorks Part
pub const MODEL_X_SLDPRT: &str = "model/x-sldprt";

/// Autodesk Inventor Assembly
pub const MODEL_X_IAM: &str = "model/x-iam";

/// Autodesk Inventor Drawing
pub const MODEL_X_IDW: &str = "model/x-idw";

/// Autodesk Inventor Presentation
pub const MODEL_X_IPN: &str = "model/x-ipn";

/// Autodesk Inventor Part
pub const MODEL_X_IPT: &str = "model/x-ipt";

/// Inter-Quake Export
pub const MODEL_X_IQE: &str = "model/x-iqe";

/// Model 3D Binary
pub const MODEL_X_3D_MODEL: &str = "model/x-3d-model";

/// SpaceClaim Document
pub const MODEL_X_SCDOC: &str = "model/x-scdoc";

/// Autodesk 123D
pub const MODEL_X_123DX: &str = "model/x-123dx";

/// Fusion 360
pub const MODEL_X_F3D: &str = "model/x-f3d";

/// Model 3D ASCII
pub const TEXT_X_3D_MODEL: &str = "text/x-3d-model";

/// draw.io diagram
pub const APPLICATION_VND_JGRAPH_MXFILE: &str = "application/vnd.jgraph.mxfile";

/// XML Shareable Playlist Format
pub const APPLICATION_XSPF_XML: &str = "application/xspf+xml";

/// XSLT stylesheet
pub const APPLICATION_XSLT_XML: &str = "application/xslt+xml";

/// Figma design file
pub const IMAGE_X_FIGMA: &str = "image/x-figma";

/// Mathematical Markup Language
pub const APPLICATION_MATHML_XML: &str = "application/mathml+xml";

/// MusicXML
pub const APPLICATION_VND_RECORDARE_MUSICXML_XML: &str = "application/vnd.recordare.musicxml+xml";

/// Timed Text Markup Language
pub const APPLICATION_TTML_XML: &str = "application/ttml+xml";

/// Simple Object Access Protocol
pub const APPLICATION_SOAP_XML: &str = "application/soap+xml";

/// Tiled Map XML (game development)
pub const APPLICATION_X_TMX_XML: &str = "application/x-tmx+xml";

/// Tiled Tileset XML (game development)
pub const APPLICATION_X_TSX_XML: &str = "application/x-tsx+xml";

/// MPEG-DASH Media Presentation Description
pub const APPLICATION_DASH_XML: &str = "application/dash+xml";

/// MusicXML ZIP (compressed music notation)
pub const APPLICATION_VND_RECORDARE_MUSICXML: &str = "application/vnd.recordare.musicxml";

/// Circuit Diagram Document (XML)
pub const APPLICATION_VND_CIRCUITDIAGRAM_DOCUMENT_MAIN_XML: &str =
    "application/vnd.circuitdiagram.document.main+xml";

/// Design Web Format XPS (XML)
pub const MODEL_VND_DWFX_XPS: &str = "model/vnd.dwfx+xps";

/// FictionBook ZIP (compressed e-book)
pub const APPLICATION_X_FBZ: &str = "application/x-fbz";

// ============================================================================
// VIRTUAL MACHINE & DISK IMAGE FORMATS
// ============================================================================

/// QEMU Copy-on-Write version 2
pub const APPLICATION_X_QEMU_DISK: &str = "application/x-qemu-disk";

/// Microsoft Virtual Hard Disk
pub const APPLICATION_X_VHD: &str = "application/x-vhd";

/// Microsoft Virtual Hard Disk v2
pub const APPLICATION_X_VHDX: &str = "application/x-vhdx";

/// VMware Virtual Disk
pub const APPLICATION_X_VMDK: &str = "application/x-vmdk";

/// VirtualBox Virtual Disk Image
pub const APPLICATION_X_VIRTUALBOX_VDI: &str = "application/x-virtualbox-vdi";

/// Windows Imaging Format
pub const APPLICATION_X_MS_WIM: &str = "application/x-ms-wim";

// ============================================================================
// FILESYSTEM FORMATS
// ============================================================================

/// Squashfs compressed filesystem
pub const APPLICATION_X_SQUASHFS: &str = "application/x-squashfs";

// ============================================================================
// NINTENDO & GAMING FORMATS
// ============================================================================

/// Nintendo Entertainment System ROM
pub const APPLICATION_VND_NINTENDO_SNES_ROM: &str = "application/vnd.nintendo.snes.rom";

// ============================================================================
// NETWORK & DEBUGGING FORMATS
// ============================================================================

/// Packet Capture (libpcap)
pub const APPLICATION_VND_TCPDUMP_PCAP: &str = "application/vnd.tcpdump.pcap";

/// Packet Capture Next Generation
pub const APPLICATION_X_PCAPNG: &str = "application/x-pcapng";

// ============================================================================
// 3D & CAD FORMATS
// ============================================================================

/// Blender 3D File
pub const APPLICATION_X_BLENDER: &str = "application/x-blender";

/// Autodesk 3D Studio Max mesh format
pub const APPLICATION_X_3DS: &str = "application/x-3ds";

/// Autodesk 3D Studio Max project file
pub const APPLICATION_X_MAX: &str = "application/x-max";

/// Polygon File Format
pub const APPLICATION_PLY: &str = "application/ply";

// ============================================================================
// MISCELLANEOUS FORMATS
// ============================================================================

/// Hierarchical Data Format
pub const APPLICATION_X_HDF: &str = "application/x-hdf";

/// CBOR Binary Data
pub const APPLICATION_CBOR: &str = "application/cbor";

/// Apache Parquet
pub const APPLICATION_VND_APACHE_PARQUET: &str = "application/vnd.apache.parquet";

/// Apache Parquet (alias)
pub const APPLICATION_X_PARQUET: &str = "application/x-parquet";

/// Flexible and Interoperable Data Transfer (Garmin FIT)
pub const APPLICATION_X_FIT: &str = "application/x-fit";

/// Unix Link File
pub const APPLICATION_X_MS_SHORTCUT: &str = "application/x-ms-shortcut";

/// Mach-O Binary
pub const APPLICATION_X_MACH_BINARY: &str = "application/x-mach-binary";

/// Time Zone Information Format
pub const APPLICATION_TZIF: &str = "application/tzif";

/// Amiga Disk File
pub const APPLICATION_X_AMIGA_DISK_FORMAT: &str = "application/x-amiga-disk-format";

/// Common Object File Format
pub const APPLICATION_X_COFF: &str = "application/x-coff";

/// Gettext Machine Object (compiled translation file)
pub const APPLICATION_X_GETTEXT_TRANSLATION: &str = "application/x-gettext-translation";

// ============================================================================
// ELF EXECUTABLE TYPES
// ============================================================================

/// ELF Object File
pub const APPLICATION_X_OBJECT: &str = "application/x-object";

/// ELF Executable
pub const APPLICATION_X_EXECUTABLE: &str = "application/x-executable";

/// ELF Shared Library  
pub const APPLICATION_X_SHAREDLIB: &str = "application/x-sharedlib";

/// ELF Core Dump
pub const APPLICATION_X_COREDUMP: &str = "application/x-coredump";

/// Empty file
pub const APPLICATION_X_EMPTY: &str = "application/x-empty";

/// Binary fallback
pub const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";

// ============================================================================
// ANDROID FORMATS
// ============================================================================

/// Dalvik Executable (Android)
pub const APPLICATION_VND_ANDROID_DEX: &str = "application/vnd.android.dex";

/// Optimized Dalvik Executable (Android)
pub const APPLICATION_VND_ANDROID_DEY: &str = "application/vnd.android.dey";

// ============================================================================
// ADDITIONAL COMPRESSION FORMATS
// ============================================================================

/// BZIP3 Compression
pub const APPLICATION_X_BZIP3: &str = "application/x-bzip3";

/// LZMA Compression
pub const APPLICATION_X_LZMA: &str = "application/x-lzma";

/// LZOP Compression
pub const APPLICATION_X_LZOP: &str = "application/x-lzop";

/// LZFSE Compression (Apple)
pub const APPLICATION_X_LZFSE: &str = "application/x-lzfse";

// ============================================================================
// GAME ROM FORMATS
// ============================================================================

/// GameBoy ROM
pub const APPLICATION_X_GAMEBOY_ROM: &str = "application/x-gameboy-rom";

/// GameBoy Color ROM
pub const APPLICATION_X_GAMEBOY_COLOR_ROM: &str = "application/x-gameboy-color-rom";

/// GameBoy Advance ROM
pub const APPLICATION_X_GBA_ROM: &str = "application/x-gba-rom";

/// Nintendo Entertainment System ROM
pub const APPLICATION_X_NINTENDO_NES_ROM: &str = "application/x-nintendo-nes-rom";

// ============================================================================
// CERTIFICATE AND KEY FORMATS
// ============================================================================

/// DER Certificate
pub const APPLICATION_X_X509_CA_CERT: &str = "application/x-x509-ca-cert";

/// Java Keystore
pub const APPLICATION_X_JAVA_KEYSTORE: &str = "application/x-java-keystore";

// ============================================================================
// BYTECODE FORMATS
// ============================================================================

/// Lua Bytecode
pub const APPLICATION_X_LUA_BYTECODE: &str = "application/x-lua-bytecode";

/// Python Bytecode (.pyc files)
pub const APPLICATION_X_PYTHON_BYTECODE: &str = "application/x-python-bytecode";

/// Python Pickle serialization (protocols 2-5)
pub const APPLICATION_X_PICKLE: &str = "application/x-pickle";

// ============================================================================
// PGP/GPG FORMATS
// ============================================================================

/// PGP Message
pub const APPLICATION_PGP: &str = "application/pgp";

/// PGP Signed Message
pub const APPLICATION_PGP_SIGNED: &str = "application/pgp-signature";

/// PGP Public Key Block
pub const APPLICATION_PGP_KEYS: &str = "application/pgp-keys";

/// PGP Signature
pub const APPLICATION_PGP_SIGNATURE: &str = "application/pgp-signature";

// ============================================================================
// ANDROID BINARY FORMATS
// ============================================================================

/// Android Binary XML (AXML)
pub const APPLICATION_VND_ANDROID_AXML: &str = "application/vnd.android.axml";

/// Android Resource Storage Container (ARSC)
pub const APPLICATION_VND_ANDROID_ARSC: &str = "application/vnd.android.arsc";

// ============================================================================
// CLASSIC EXECUTABLE FORMATS
// ============================================================================

/// MS-DOS Executable
pub const APPLICATION_X_DOSEXEC: &str = "application/x-dosexec";

// ============================================================================
// CAMERA RAW FORMATS
// ============================================================================

/// Canon Raw (original format)
pub const IMAGE_X_CANON_CRW: &str = "image/x-canon-crw";

/// Canon Raw 2 (DSLR format)
pub const IMAGE_X_CANON_CR2: &str = "image/x-canon-cr2";

/// Canon Raw 3 (mirrorless format)
pub const IMAGE_X_CANON_CR3: &str = "image/x-canon-cr3";

/// Nikon Electronic File (RAW)
pub const IMAGE_X_NIKON_NEF: &str = "image/x-nikon-nef";

/// Fujifilm RAW format
pub const IMAGE_X_FUJI_RAF: &str = "image/x-fuji-raf";

/// Olympus RAW format
pub const IMAGE_X_OLYMPUS_ORF: &str = "image/x-olympus-orf";

/// Panasonic RAW format
pub const IMAGE_X_PANASONIC_RW2: &str = "image/x-panasonic-rw2";

// ============================================================================
// AUDIO MODULE FORMATS
// ============================================================================

/// FastTracker 2 Extended Module
pub const AUDIO_X_XM: &str = "audio/x-xm";

/// Impulse Tracker Module
pub const AUDIO_X_IT: &str = "audio/x-it";

/// Scream Tracker 3 Module
pub const AUDIO_X_S3M: &str = "audio/x-s3m";

/// ProTracker Module
pub const AUDIO_X_MOD: &str = "audio/x-mod";

// ============================================================================
// APPLE FORMATS
// ============================================================================

/// Apple Disk Image
pub const APPLICATION_X_APPLE_DISKIMAGE: &str = "application/x-apple-diskimage";

/// macOS Alias File
pub const APPLICATION_X_APPLE_ALIAS: &str = "application/x-apple-alias";

// ============================================================================
// GAME ROM FORMATS (SEGA)
// ============================================================================

/// Sega Game Gear ROM
pub const APPLICATION_X_GAMEGEAR_ROM: &str = "application/x-gamegear-rom";

/// Sega Master System ROM
pub const APPLICATION_X_SMS_ROM: &str = "application/x-sms-rom";

/// Sega Genesis/Mega Drive ROM
pub const APPLICATION_X_GENESIS_ROM: &str = "application/x-genesis-rom";

// ============================================================================
// ARCHIVE FORMATS (ADDITIONAL)
// ============================================================================

/// Zoo archive format
pub const APPLICATION_X_ZOO: &str = "application/x-zoo";

/// ZPAQ archive format
pub const APPLICATION_X_ZPAQ: &str = "application/x-zpaq";

/// Unix compress format
pub const APPLICATION_X_COMPRESS: &str = "application/x-compress";

/// MS-DOS Batch file
pub const TEXT_X_MSDOS_BATCH: &str = "text/x-msdos-batch";

// ============================================================================
// RETRO GAMING FORMATS (ADDITIONAL)
// ============================================================================

/// Atari 7800 ROM
pub const APPLICATION_X_ATARI_7800_ROM: &str = "application/x-atari-7800-rom";

/// Commodore 64 Program
pub const APPLICATION_X_COMMODORE_64_PROGRAM: &str = "application/x-commodore-64-program";

/// Commodore 64 Cartridge
pub const APPLICATION_X_COMMODORE_64_CARTRIDGE: &str = "application/x-commodore-64-cartridge";

// ============================================================================
// NINTENDO ROM FORMATS (ADDITIONAL)
// ============================================================================

/// Nintendo 64 ROM
pub const APPLICATION_X_N64_ROM: &str = "application/x-n64-rom";

/// Nintendo DS ROM
pub const APPLICATION_X_NINTENDO_DS_ROM: &str = "application/x-nintendo-ds-rom";

/// Nintendo Switch Package (NSP)
pub const APPLICATION_X_NINTENDO_SWITCH_PACKAGE: &str = "application/x-nintendo-switch-package";

/// Nintendo Switch Relocatable Object (NRO)
pub const APPLICATION_X_NINTENDO_SWITCH_EXECUTABLE: &str =
    "application/x-nintendo-switch-executable";

/// Nintendo Switch Shared Object (NSO)
pub const APPLICATION_X_NINTENDO_SWITCH_SO: &str = "application/x-nintendo-switch-so";

/// Nintendo Switch ROM (XCI - NX Card Image)
pub const APPLICATION_X_NINTENDO_SWITCH_ROM: &str = "application/x-nintendo-switch-rom";

// ============================================================================
// NEO GEO ROM FORMATS
// ============================================================================

/// Neo Geo Pocket ROM (Monochrome)
pub const APPLICATION_X_NEO_GEO_POCKET_ROM: &str = "application/x-neo-geo-pocket-rom";

/// Neo Geo Pocket Color ROM
pub const APPLICATION_X_NEO_GEO_POCKET_COLOR_ROM: &str = "application/x-neo-geo-pocket-color-rom";

// ============================================================================
// EBOOK AND DOCUMENT FORMATS
// ============================================================================

/// BroadBand eBook (Sony Reader)
pub const APPLICATION_X_LRF: &str = "application/x-lrf";

/// FictionBook e-book format (XML-based)
pub const APPLICATION_X_FB2_XML: &str = "application/x-fb2+xml";
/// FictionBook legacy MIME type
pub const APPLICATION_X_FICTIONBOOK_XML: &str = "application/x-fictionbook+xml";

// ============================================================================
// FONT FORMATS
// ============================================================================

/// FigletFont ASCII art font
pub const APPLICATION_X_FIGLET: &str = "application/x-figlet";
/// FigletFont legacy MIME type
pub const APPLICATION_X_FIGLET_FONT: &str = "application/x-figlet-font";

// ============================================================================
// OTHER FORMATS
// ============================================================================

/// SeqBox archive format
pub const APPLICATION_X_SBX: &str = "application/x-sbx";
/// SeqBox legacy MIME type
pub const APPLICATION_X_SEQBOX: &str = "application/x-seqbox";

/// Snappy framed format
pub const APPLICATION_X_SNAPPY_FRAMED: &str = "application/x-snappy-framed";

/// Tasty format
pub const APPLICATION_X_TASTY: &str = "application/x-tasty";

// ============================================================================
// ADDITIONAL ARCHIVE FORMATS
// ============================================================================

/// PAK archive format
pub const APPLICATION_X_PAK: &str = "application/x-pak";

// ============================================================================
// DATABASE FORMATS
// ============================================================================

/// dBASE database format
pub const APPLICATION_X_DBASE: &str = "application/x-dbase";

// ============================================================================
// ADDITIONAL IMAGE FORMATS
// ============================================================================

/// Adobe Digital Negative
pub const IMAGE_X_ADOBE_DNG: &str = "image/x-adobe-dng";

/// Sony ARW Raw format
pub const IMAGE_X_SONY_ARW: &str = "image/x-sony-arw";

/// Sony SR2 Raw format
pub const IMAGE_X_SONY_SR2: &str = "image/x-sony-sr2";

/// Pentax PEF Raw format
pub const IMAGE_X_PENTAX_PEF: &str = "image/x-pentax-pef";

/// Hasselblad 3FR Raw format
pub const IMAGE_X_HASSELBLAD_3FR: &str = "image/x-hasselblad-3fr";

/// Minolta MRW Raw format
pub const IMAGE_X_MINOLTA_MRW: &str = "image/x-minolta-mrw";

/// Kodak KDC Raw format
pub const IMAGE_X_KODAK_KDC: &str = "image/x-kodak-kdc";

/// Kodak DCR Raw format
pub const IMAGE_X_KODAK_DCR: &str = "image/x-kodak-dcr";

/// PICtor/PC Paint DOS graphics format
pub const IMAGE_X_PICTOR: &str = "image/x-pictor";

/// X11 Bitmap format
pub const IMAGE_X_XBITMAP: &str = "image/x-xbitmap";

// ============================================================================
// AUDIO FORMATS
// ============================================================================

/// Creative Voice audio format
pub const AUDIO_X_VOC: &str = "audio/x-voc";

/// RealAudio format
pub const AUDIO_X_REALAUDIO: &str = "audio/x-realaudio";

/// SoundFont 2 format
pub const AUDIO_X_SOUNDFONT: &str = "audio/x-soundfont";

/// Quite OK Audio format
pub const AUDIO_X_QOA: &str = "audio/x-qoa";

/// 8SVX Audio format (Amiga IFF)
pub const AUDIO_X_8SVX: &str = "audio/x-8svx";

/// Audio Visual Research format
pub const AUDIO_X_AVR: &str = "audio/x-avr";

// ============================================================================
// SCIENTIFIC DATA FORMATS
// ============================================================================

/// HDF5 Hierarchical Data Format
pub const APPLICATION_X_HDF5: &str = "application/x-hdf5";

/// GRIB weather data format
pub const APPLICATION_X_GRIB: &str = "application/x-grib";

// ============================================================================
// CINEMA FORMATS
// ============================================================================

/// Cineon digital cinema format
pub const IMAGE_CINEON: &str = "image/cineon";

/// Digital Picture Exchange
pub const IMAGE_X_DPX: &str = "image/x-dpx";

// ============================================================================
// WINDOWS FORMATS
// ============================================================================

/// Windows Help format
pub const APPLICATION_WINHELP: &str = "application/winhelp";

/// Windows Event Log
pub const APPLICATION_X_MS_EVT: &str = "application/x-ms-evt";

/// Windows Event Log XML
pub const APPLICATION_X_MS_EVTX: &str = "application/x-ms-evtx";

/// ActiveMime (Microsoft Office embedded OLE object)
pub const APPLICATION_X_MSO: &str = "application/x-mso";

/// Multi Layer Archive
pub const APPLICATION_X_MLA: &str = "application/x-mla";

/// Microsoft Silverlight Application
pub const APPLICATION_X_SILVERLIGHT_APP: &str = "application/x-silverlight-app";

/// Mozilla XPInstall
pub const APPLICATION_X_XPINSTALL: &str = "application/x-xpinstall";

/// OpenXPS (XML Paper Specification)
pub const APPLICATION_OXPS: &str = "application/oxps";

/// Universal Subtitle Format
pub const APPLICATION_X_USF: &str = "application/x-usf";

/// StarDraw (StarOffice/StarDivision Draw)
pub const APPLICATION_VND_STARDIVISION_DRAW: &str = "application/vnd.stardivision.draw";

/// StarCalc (StarOffice/StarDivision Calc)
pub const APPLICATION_VND_STARDIVISION_CALC: &str = "application/vnd.stardivision.calc";

/// StarImpress (StarOffice/StarDivision Impress)
pub const APPLICATION_VND_STARDIVISION_IMPRESS: &str = "application/vnd.stardivision.impress";

/// StarChart (StarOffice/StarDivision Chart)
pub const APPLICATION_VND_STARDIVISION_CHART: &str = "application/vnd.stardivision.chart";

/// StarWriter (StarOffice/StarDivision Writer)
pub const APPLICATION_VND_STARDIVISION_WRITER: &str = "application/vnd.stardivision.writer";

/// StarMath (StarOffice/StarDivision Math)
pub const APPLICATION_VND_STARDIVISION_MATH: &str = "application/vnd.stardivision.math";

/// Sun XML Draw (legacy Sun Microsystems format)
pub const APPLICATION_VND_SUN_XML_DRAW: &str = "application/vnd.sun.xml.draw";

/// Sun XML Impress (legacy Sun Microsystems format)
pub const APPLICATION_VND_SUN_XML_IMPRESS: &str = "application/vnd.sun.xml.impress";

/// Sun XML Math (legacy Sun Microsystems format)
pub const APPLICATION_VND_SUN_XML_MATH: &str = "application/vnd.sun.xml.math";

/// Sun XML Writer (legacy Sun Microsystems format)
pub const APPLICATION_VND_SUN_XML_WRITER: &str = "application/vnd.sun.xml.writer";

/// Sun XML Calc Template (legacy Sun Microsystems format)
pub const APPLICATION_VND_SUN_XML_CALC_TEMPLATE: &str = "application/vnd.sun.xml.calc.template";

/// Sun XML Draw Template (legacy Sun Microsystems format)
pub const APPLICATION_VND_SUN_XML_DRAW_TEMPLATE: &str = "application/vnd.sun.xml.draw.template";

/// Sun XML Impress Template (legacy Sun Microsystems format)
pub const APPLICATION_VND_SUN_XML_IMPRESS_TEMPLATE: &str =
    "application/vnd.sun.xml.impress.template";

/// Sun XML Writer Template (legacy Sun Microsystems format)
pub const APPLICATION_VND_SUN_XML_WRITER_TEMPLATE: &str = "application/vnd.sun.xml.writer.template";

/// Sun XML Writer Global (legacy Sun Microsystems format)
pub const APPLICATION_VND_SUN_XML_WRITER_GLOBAL: &str = "application/vnd.sun.xml.writer.global";

/// WordPerfect Graphics
pub const APPLICATION_VND_WORDPERFECT_GRAPHICS: &str = "application/vnd.wordperfect";

/// Uniform Office Format Presentation
pub const APPLICATION_VND_UOF_PRESENTATION: &str = "application/vnd.uof.presentation";

/// Uniform Office Format Spreadsheet
pub const APPLICATION_VND_UOF_SPREADSHEET: &str = "application/vnd.uof.spreadsheet";

/// Uniform Office Format Text
pub const APPLICATION_VND_UOF_TEXT: &str = "application/vnd.uof.text";

/// Windows Static Cursor
pub const IMAGE_X_WIN_CUR: &str = "image/x-win-cursor";

// ============================================================================
// EMAIL FORMATS
// ============================================================================

/// Email message (RFC822)
pub const MESSAGE_RFC822: &str = "message/rfc822";
