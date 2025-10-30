# Supported File Formats

This document provides a comprehensive listing of all 228+ file formats supported by the mimetype-detector library.

## Table of Contents

- [Text Formats](#text-formats)
- [Document Formats](#document-formats)
- [Archive & Compression Formats](#archive--compression-formats)
- [Image Formats](#image-formats)
- [Audio Formats](#audio-formats)
- [Video Formats](#video-formats)
- [Executable & Binary Formats](#executable--binary-formats)
- [Font Formats](#font-formats)
- [Web & Multimedia Formats](#web--multimedia-formats)
- [Database Formats](#database-formats)
- [Programming & Scripting Formats](#programming--scripting-formats)
- [XML-Based Formats](#xml-based-formats)
- [3D & Geospatial Formats](#3d--geospatial-formats)
- [Gaming Formats](#gaming-formats)
- [Network & Debugging Formats](#network--debugging-formats)
- [Miscellaneous Formats](#miscellaneous-formats)

---

## Text Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| HTML | `text/html; charset=utf-8` | `.html` | `.htm` | Case-insensitive tag detection |
| HTML (UTF-16) | `text/html; charset=utf-16` | `.html` | | UTF-16 BE/LE variants |
| XML | `text/xml; charset=utf-8` | `.xml` | `application/xml` | |
| XML (UTF-16) | `text/xml; charset=utf-16` | `.xml` | `application/xml; charset=utf-16` | UTF-16 BE/LE variants |
| UTF-8 with BOM | `text/plain; charset=utf-8` | `.txt` | | |
| UTF-16 Big Endian | `text/plain; charset=utf-16be` | `.txt` | | |
| UTF-16 Little Endian | `text/plain; charset=utf-16le` | `.txt` | | |
| UTF-8 | `text/plain; charset=utf-8` | `.txt` | `text/plain` | |

---

## Document Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| PDF | `application/pdf` | `.pdf` | `application/x-pdf` | Adobe Portable Document Format |
| FDF | `application/vnd.fdf` | `.fdf` | | Forms Data Format (PDF variant) |
| PostScript | `application/postscript` | `.ps` | | |
| Word 2007+ | `application/vnd.openxmlformats-officedocument.wordprocessingml.document` | `.docx` | | |
| Word 97-2003 | `application/msword` | `.doc` | | Legacy format (OLE-based) |
| Excel 2007+ | `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` | `.xlsx` | | |
| Excel 97-2003 | `application/vnd.ms-excel` | `.xls` | | Legacy format (OLE-based) |
| PowerPoint 2007+ | `application/vnd.openxmlformats-officedocument.presentationml.presentation` | `.pptx` | | |
| PowerPoint 97-2003 | `application/vnd.ms-powerpoint` | `.ppt` | | Legacy format (OLE-based) |
| Visio 2007+ | `application/vnd.ms-visio.drawing.main+xml` | `.vsdx` | | |
| Publisher | `application/vnd.ms-publisher` | `.pub` | | |
| Outlook Message | `application/vnd.ms-outlook` | `.msg` | | |
| OneNote | `application/onenote` | `.one` | | |
| WordPerfect | `application/vnd.wordperfect` | `.wpd` | | |
| RTF | `text/rtf` | `.rtf` | `application/rtf` | Rich Text Format |
| RTF (UTF-16) | `text/rtf; charset=utf-16` | `.rtf` | | UTF-16 BE/LE variants |
| EPUB | `application/epub+zip` | `.epub` | | Electronic Publication |
| Mobipocket eBook | `application/x-mobipocket-ebook` | `.mobi` | | |
| Microsoft Reader | `application/x-ms-reader` | `.lit` | | eBook format |
| HTML Help | `application/vnd.ms-htmlhelp` | `.chm` | | |
| DjVu | `image/vnd.djvu` | `.djvu` | | Document imaging format |
| SubRip Subtitles | `application/x-subrip` | `.srt` | `application/x-srt`, `text/x-srt` | |
| SubRip (UTF-16) | `application/x-subrip; charset=utf-16` | `.srt` | | UTF-16 BE/LE variants |
| WebVTT | `text/vtt` | `.vtt` | | Web Video Text Tracks |
| WebVTT (UTF-16) | `text/vtt; charset=utf-16` | `.vtt` | | UTF-16 BE/LE variants |
| vCard | `text/vcard` | `.vcf` | | Contact card format |
| vCard (UTF-16) | `text/vcard; charset=utf-16` | `.vcf` | | UTF-16 BE/LE variants |
| iCalendar | `text/calendar` | `.ics` | | Calendar format |
| iCalendar (UTF-16) | `text/calendar; charset=utf-16` | `.ics` | | UTF-16 BE/LE variants |

---

## Archive & Compression Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| 7-Zip | `application/x-7z-compressed` | `.7z` | | |
| ZIP | `application/zip` | `.zip` | `application/x-zip`, `application/x-zip-compressed` | |
| RAR | `application/x-rar-compressed` | `.rar` | `application/x-rar` | |
| GZIP | `application/gzip` | `.gz` | `application/x-gzip`, `application/x-gunzip`, `application/gzipped`, `application/gzip-compressed`, `application/x-gzip-compressed`, `gzip/document` | `.tgz`, `.taz` |
| TAR | `application/x-tar` | `.tar` | | Uses checksum validation |
| BZIP2 | `application/x-bzip2` | `.bz2` | | |
| XZ | `application/x-xz` | `.xz` | | |
| Zstandard | `application/zstd` | `.zst` | | |
| LZIP | `application/lzip` | `.lz` | `application/x-lzip` | |
| LZ4 | `application/x-lz4` | `.lz4` | | Fast compression |
| Cabinet | `application/vnd.ms-cab-compressed` | `.cab` | | Microsoft Cabinet |
| InstallShield Cabinet | `application/x-installshield` | `.cab` | | |
| CPIO | `application/x-cpio` | `.cpio` | | Unix archive |
| AR | `application/x-archive` | `.a` | `application/x-unix-archive` | `.deb` |
| RPM | `application/x-rpm` | `.rpm` | | Red Hat Package Manager |
| Debian Package | `application/vnd.debian.binary-package` | `.deb` | | |
| XAR | `application/x-xar` | `.xar` | | |
| ARJ | `application/arj` | `.arj` | `application/x-arj` | Legacy DOS compression |
| LHA/LZH | `application/x-lzh-compressed` | `.lzh` | `application/x-lha` | Japanese compression |
| WARC | `application/warc` | `.warc` | | Web ARChive format |
| BitTorrent | `application/x-bittorrent` | `.torrent` | | Torrent metadata |
| FITS | `application/fits` | `.fits` | `image/fits` | Flexible Image Transport System |

---

## Image Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| PNG | `image/png` | `.png` | | Portable Network Graphics |
| APNG | `image/vnd.mozilla.apng` | `.apng` | | Animated PNG |
| JPEG | `image/jpeg` | `.jpg` | | `.jpeg`, `.jpe`, `.jif`, `.jfif`, `.jfi` |
| JPEG 2000 | `image/jp2` | `.jp2` | | |
| JPEG 2000 Extended | `image/jpx` | `.jpx` | | |
| JPEG 2000 Multi-part | `image/jpm` | `.jpm` | `video/jpm` | |
| JPEG XS | `image/jxs` | `.jxs` | | |
| JPEG XR | `image/jxr` | `.jxr` | `image/vnd.ms-photo` | |
| JPEG XL | `image/jxl` | `.jxl` | | |
| GIF | `image/gif` | `.gif` | | Graphics Interchange Format |
| WebP | `image/webp` | `.webp` | | |
| TIFF | `image/tiff` | `.tiff` | | `.tif` |
| BMP | `image/bmp` | `.bmp` | `image/x-bmp`, `image/x-ms-bmp` | `.dib` |
| Windows Icon | `image/x-icon` | `.ico` | | |
| Apple Icon | `image/x-icns` | `.icns` | | |
| Photoshop | `image/vnd.adobe.photoshop` | `.psd` | `image/x-psd`, `application/photoshop` | |
| HEIC | `image/heic` | `.heic` | | High Efficiency Image Container |
| HEIC Sequence | `image/heic-sequence` | `.heic` | | |
| HEIF | `image/heif` | `.heif` | | High Efficiency Image Format |
| HEIF Sequence | `image/heif-sequence` | `.heif` | | |
| AVIF | `image/avif` | `.avif` | | AV1 Image File Format |
| BPG | `image/bpg` | `.bpg` | | Better Portable Graphics |
| GIMP XCF | `image/x-xcf` | `.xcf` | | GIMP native format |
| GIMP Pattern | `image/x-gimp-pat` | `.pat` | | |
| GIMP Brush | `image/x-gimp-gbr` | `.gbr` | | |
| Radiance HDR | `image/vnd.radiance` | `.hdr` | | High Dynamic Range |
| X11 Pixmap | `image/x-xpixmap` | `.xpm` | | |
| Portable Bitmap | `image/x-portable-bitmap` | `.pbm` | | Netpbm format |
| Portable Graymap | `image/x-portable-graymap` | `.pgm` | | Netpbm format |
| Portable Pixmap | `image/x-portable-pixmap` | `.ppm` | | Netpbm format |
| Portable Arbitrary Map | `image/x-portable-arbitrarymap` | `.pam` | | Netpbm format |
| AutoCAD Drawing | `image/vnd.dwg` | `.dwg` | `image/x-dwg`, `application/acad`, `application/x-acad`, `application/autocad_dwg`, `application/dwg`, `application/x-dwg`, `application/x-autocad`, `drawing/dwg` | |
| AutoCAD DXF | `image/vnd.dxf` | `.dxf` | | Drawing Exchange Format |
| DirectDraw Surface | `image/vnd-ms.dds` | `.dds` | | Game textures |
| PC Paintbrush | `image/x-pcx` | `.pcx` | | Classic bitmap format |
| Khronos Texture | `image/ktx` | `.ktx` | | OpenGL/Vulkan textures |
| ASTC | `image/x-astc` | `.astc` | | ARM Texture Compression |
| Truevision TGA | `image/x-tga` | `.tga` | | Targa format |
| Sun Raster | `image/x-sun-raster` | `.ras` | | Legacy Unix format |
| Silicon Graphics | `image/x-sgi` | `.sgi` | | Film/VFX format |
| SVG | `image/svg+xml` | `.svg` | | Scalable Vector Graphics |
| SVG (UTF-16) | `image/svg+xml; charset=utf-16` | `.svg` | | UTF-16 BE/LE variants |
| DICOM | `application/dicom` | `.dcm` | | Medical imaging |

---

## Audio Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| MP3 | `audio/mpeg` | `.mp3` | `audio/x-mpeg`, `audio/mp3` | MPEG Audio Layer 3 |
| FLAC | `audio/flac` | `.flac` | | Free Lossless Audio Codec |
| WAV | `audio/wav` | `.wav` | `audio/x-wav`, `audio/vnd.wave`, `audio/wave` | Waveform Audio File |
| AIFF | `audio/aiff` | `.aiff` | | `.aif` |
| MIDI | `audio/midi` | `.midi` | `audio/mid` | `.mid` |
| OGG | `application/ogg` | `.ogg` | | Container format |
| OGG Audio | `audio/ogg` | `.oga` | | `.opus` |
| OGG Video | `video/ogg` | `.ogv` | | |
| Monkey's Audio | `audio/ape` | `.ape` | | |
| Musepack | `audio/musepack` | `.mpc` | | |
| Sun/NeXT Audio | `audio/basic` | `.au` | | `.snd` |
| AMR | `audio/amr` | `.amr` | `audio/amr-nb` | Adaptive Multi-Rate |
| Creative Voice | `audio/x-unknown` | `.voc` | | |
| M3U | `audio/x-mpegurl` | `.m3u` | `audio/mpegurl` | `.m3u8` |
| AAC | `audio/aac` | `.aac` | | Advanced Audio Coding |
| Qualcomm PureVoice | `audio/qcelp` | `.qcp` | | |
| M4A | `audio/x-m4a` | `.m4a` | | |
| MPEG-4 Audio | `audio/mp4` | `.mp4` | `audio/x-m4a`, `audio/x-mp4a` | |
| WavPack | `audio/x-wavpack` | `.wv` | | Lossless/lossy compression |
| True Audio | `audio/x-tta` | `.tta` | | Lossless codec |
| DSD Stream | `audio/x-dsf` | `.dsf` | | Direct Stream Digital |
| DSD Interchange | `audio/x-dff` | `.dff` | | Direct Stream Digital |
| Scream Tracker 3 | `audio/s3m` | `.s3m` | | Module format |

---

## Video Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| MP4 | `video/mp4` | `.mp4` | | MPEG-4 Part 14 |
| WebM | `video/webm` | `.webm` | `audio/webm` | |
| Matroska | `video/x-matroska` | `.mkv` | | `.mk3d`, `.mka`, `.mks` |
| AVI | `video/x-msvideo` | `.avi` | `video/avi`, `video/msvideo` | Audio Video Interleave |
| MPEG | `video/mpeg` | `.mpeg` | | |
| QuickTime | `video/quicktime` | `.mov` | | |
| QuickTime MQV | `video/quicktime` | `.mqv` | | |
| Flash Video | `video/x-flv` | `.flv` | | |
| ASF/WMV | `video/x-ms-asf` | `.asf` | `video/asf`, `video/x-ms-wmv` | Advanced Systems Format |
| M4V | `video/x-m4v` | `.m4v` | | iTunes Video |
| RealMedia VBR | `application/vnd.rn-realmedia-vbr` | `.rmvb` | | |
| 3GPP | `video/3gpp` | `.3gp` | `video/3gp`, `audio/3gpp` | 3GPP Multimedia |
| 3GPP2 | `video/3gpp2` | `.3g2` | `video/3g2`, `audio/3gpp2` | 3GPP2 Multimedia |
| Motion JPEG 2000 | `video/mj2` | `.mj2` | | |
| DVB | `video/vnd.dvb.file` | `.dvb` | | Digital Video Broadcasting |
| FLIC (FLI) | `video/fli` | `.fli` | | Autodesk animation |
| FLIC (FLC) | `video/flc` | `.flc` | | Autodesk animation |
| Fast Search & Transfer | `video/vnd.fvt` | `.fvt` | | Surveillance video |

---

## Executable & Binary Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Windows PE | `application/vnd.microsoft.portable-executable` | `.exe` | | Portable Executable |
| ELF | `application/x-elf` | | | Executable and Linkable Format |
| ELF Object | `application/x-object` | | | `.so` |
| ELF Executable | `application/x-executable` | | | |
| ELF Shared Library | `application/x-sharedlib` | `.so` | | |
| ELF Core Dump | `application/x-coredump` | | | |
| Java Class | `application/x-java-applet; charset=binary` | `.class` | `application/x-java-applet` | |
| Java Archive | `application/java-archive` | `.jar` | `application/jar`, `application/jar-archive`, `application/x-java-archive` | |
| Android Package | `application/vnd.android.package-archive` | `.apk` | | |
| WebAssembly | `application/wasm` | `.wasm` | | |
| Mach-O | `application/x-mach-binary` | `.macho` | | macOS/iOS executable |
| Microsoft Installer | `application/x-ms-installer` | `.msi` | | |
| Windows Shortcut | `application/x-ms-shortcut` | `.lnk` | | |

---

## Font Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| TrueType | `font/ttf` | `.ttf` | `font/sfnt`, `application/x-font-ttf`, `application/font-sfnt` | |
| OpenType | `font/otf` | `.otf` | | |
| WOFF | `font/woff` | `.woff` | | Web Open Font Format |
| WOFF2 | `font/woff2` | `.woff2` | | Web Open Font Format 2 |
| EOT | `application/vnd.ms-fontobject` | `.eot` | | Embedded OpenType |
| TrueType Collection | `font/collection` | `.ttc` | | |

---

## Web & Multimedia Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Adobe Flash | `application/x-shockwave-flash` | `.swf` | | |
| Chrome Extension | `application/x-chrome-extension` | `.crx` | | |
| PKCS#7 Signature | `application/pkcs7-signature` | `.p7s` | | |

---

## Database Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| SQLite | `application/vnd.sqlite3` | `.sqlite` | `application/x-sqlite3` | |
| MS Access (MDB) | `application/x-msaccess` | `.mdb` | | |
| MS Access (ACCDB) | `application/x-msaccess` | `.accdb` | | |
| dBase | `application/x-dbf` | `.dbf` | | |
| Lotus 1-2-3 | `application/vnd.lotus-1-2-3` | `.123` | | |
| MARC | `application/marc` | `.mrc` | | Library records |
| HDF | `application/x-hdf` | `.hdf` | | Hierarchical Data Format |
| Apache Parquet | `application/vnd.apache.parquet` | `.parquet` | `application/x-parquet` | |

---

## Programming & Scripting Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| PHP | `text/x-php` | `.php` | | |
| JavaScript | `text/javascript` | `.js` | `application/javascript` | |
| Python | `text/x-python` | `.py` | `text/x-script.python`, `application/x-python` | |
| Perl | `text/x-perl` | `.pl` | | |
| Ruby | `text/x-ruby` | `.rb` | `application/x-ruby` | |
| Lua | `text/x-lua` | `.lua` | | |
| Shell Script | `text/x-shellscript` | `.sh` | `text/x-sh`, `application/x-shellscript`, `application/x-sh` | |
| Tcl | `text/x-tcl` | `.tcl` | `application/x-tcl` | |
| JSON | `application/json` | `.json` | | |
| JSON (UTF-16) | `application/json; charset=utf-16` | `.json` | | UTF-16 BE/LE variants |
| GeoJSON | `application/geo+json` | `.geojson` | | Geographic data |
| NDJSON | `application/x-ndjson` | `.ndjson` | | Newline Delimited JSON |
| CSV | `text/csv` | `.csv` | | |
| CSV (UTF-16) | `text/csv; charset=utf-16` | `.csv` | | UTF-16 BE/LE variants |
| TSV | `text/tab-separated-values` | `.tsv` | | Tab Separated Values |
| TSV (UTF-16) | `text/tab-separated-values; charset=utf-16` | `.tsv` | | UTF-16 BE/LE variants |

---

## XML-Based Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| RSS | `application/rss+xml` | `.rss` | `text/rss` | RSS Feed |
| Atom | `application/atom+xml` | `.atom` | | Atom Feed |
| X3D | `model/x3d+xml` | `.x3d` | | 3D Graphics |
| KML | `application/vnd.google-earth.kml+xml` | `.kml` | | Google Earth |
| XLIFF | `application/x-xliff+xml` | `.xlf` | | Translation format |
| COLLADA | `model/vnd.collada+xml` | `.dae` | | 3D Graphics |
| GML | `application/gml+xml` | `.gml` | | Geography Markup |
| GPX | `application/gpx+xml` | `.gpx` | | GPS Exchange |
| TCX | `application/vnd.garmin.tcx+xml` | `.tcx` | | Training Center XML |
| AMF | `application/x-amf` | `.amf` | | Additive Manufacturing |
| 3MF | `application/vnd.ms-package.3dmanufacturing-3dmodel+xml` | `.3mf` | | 3D Manufacturing |
| XFDF | `application/vnd.adobe.xfdf` | `.xfdf` | | Adobe XFDF |
| OWL | `application/owl+xml` | `.owl` | | OWL Ontology |
| XHTML | `application/xhtml+xml` | `.html` | | XHTML |
| HAR | `application/json` | `.har` | | HTTP Archive |

---

## OpenDocument Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| ODT | `application/vnd.oasis.opendocument.text` | `.odt` | `application/x-vnd.oasis.opendocument.text` | OpenDocument Text |
| ODS | `application/vnd.oasis.opendocument.spreadsheet` | `.ods` | `application/x-vnd.oasis.opendocument.spreadsheet` | OpenDocument Spreadsheet |
| ODP | `application/vnd.oasis.opendocument.presentation` | `.odp` | `application/x-vnd.oasis.opendocument.presentation` | OpenDocument Presentation |
| ODG | `application/vnd.oasis.opendocument.graphics` | `.odg` | `application/x-vnd.oasis.opendocument.graphics` | OpenDocument Graphics |
| ODF | `application/vnd.oasis.opendocument.formula` | `.odf` | `application/x-vnd.oasis.opendocument.formula` | OpenDocument Formula |
| ODC | `application/vnd.oasis.opendocument.chart` | `.odc` | `application/x-vnd.oasis.opendocument.chart` | OpenDocument Chart |
| OTT | `application/vnd.oasis.opendocument.text-template` | `.ott` | `application/x-vnd.oasis.opendocument.text-template` | Text Template |
| OTS | `application/vnd.oasis.opendocument.spreadsheet-template` | `.ots` | `application/x-vnd.oasis.opendocument.spreadsheet-template` | Spreadsheet Template |
| OTP | `application/vnd.oasis.opendocument.presentation-template` | `.otp` | `application/x-vnd.oasis.opendocument.presentation-template` | Presentation Template |
| OTG | `application/vnd.oasis.opendocument.graphics-template` | `.otg` | `application/x-vnd.oasis.opendocument.graphics-template` | Graphics Template |
| SXC | `application/vnd.sun.xml.calc` | `.sxc` | | OpenOffice Calc |
| KMZ | `application/vnd.google-earth.kmz` | `.kmz` | | Zipped KML |

---

## 3D & Geospatial Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| ESRI Shapefile | `application/vnd.shp` | `.shp` | | |
| ESRI Shapefile Index | `application/vnd.shx` | `.shx` | | |
| glTF Binary | `model/gltf-binary` | `.glb` | | GL Transmission Format |
| glTF JSON | `model/gltf+json` | `.gltf` | | GL Transmission Format |
| Universal 3D | `model/u3d` | `.u3d` | | PDF 3D embedding |
| Blender | `application/x-blender` | `.blend` | | 3D modeling |
| PLY | `application/ply` | `.ply` | | Polygon File Format |

---

## Gaming Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Nintendo NES ROM | `application/vnd.nintendo.snes.rom` | `.nes` | | |

---

## Network & Debugging Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| PCAP | `application/vnd.tcpdump.pcap` | `.pcap` | | Packet Capture (libpcap) |
| PCAPNG | `application/x-pcapng` | `.pcapng` | | Next Generation PCAP |

---

## Miscellaneous Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| CBOR | `application/cbor` | `.cbor` | | Concise Binary Object Representation |
| TZif | `application/tzif` | | | Time Zone Information Format |
| OLE Storage | `application/x-ole-storage` | | | Microsoft OLE (legacy Office) |
| AAF | `application/octet-stream` | `.aaf` | | Advanced Authoring Format |
| Fasoo | `application/x-fasoo` | | | Document protection |
| PGP NetShare | `application/x-pgp-net-share` | | | |
