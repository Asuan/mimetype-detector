# Supported File Formats

This document provides a comprehensive listing of all 450+ file formats supported by the mimetype-detector library.

**Latest additions (2025-11-05):**

**Batch 28 (Latest):**
- Autodesk 3D Studio Max (.max)

**Batch 27:**
- Circuit Diagram Document (.cddx)
- Design Web Format XPS (.dwfx)
- FictionBook ZIP (.fbz)

**Batch 26:**
- Tiled Map XML (.tmx)
- Tiled Tileset XML (.tsx)
- MPEG-DASH MPD (.mpd)
- MusicXML ZIP (.mxl)

**Batch 25:**
- MathML (.mathml)
- MusicXML (.musicxml)
- TTML (.ttml)
- SOAP (.soap)

**Batch 24:**
- draw.io (.drawio)
- XML Shareable Playlist Format (.xspf)
- XSLT (.xsl)
- Figma (.fig)

**Batch 23:**
- Model 3D ASCII (.a3d)
- Autodesk 123D (.123dx)
- Fusion 360 (.f3d)

**Batch 22:**
- Inter-Quake Export (.iqe)
- Model 3D Binary (.m3d)
- SpaceClaim Document (.scdoc)

**Batch 21:**
- Autodesk Inventor Assembly (.iam)
- Autodesk Inventor Drawing (.idw)
- Autodesk Inventor Presentation (.ipn)
- Autodesk Inventor Part (.ipt)

**Batch 20:**
- SolidWorks Assembly (.sldasm)
- SolidWorks Drawing (.slddrw)
- SolidWorks Part (.sldprt)

**Batch 19:**
- Sketch 43 (.sketch)

**Batch 18:**
- Universal Scene Description ZIP (.usdz)

**Batch 17:**
- Initial Graphics Exchange Specification (.iges/.igs)

**Batch 16:**
- Uniform Office Format Presentation (.uop)
- Uniform Office Format Spreadsheet (.uos)
- Uniform Office Format Text (.uot)

**Batch 15:**
- Sun XML Writer Global (.sgw)
- WordPerfect Graphics (.wpg)
- WordPerfect Presentations (.shw)
- WordPerfect Macro (.wpm)

**Batch 14:**
- Sun XML Calc Template (.stc)
- Sun XML Draw Template (.std)
- Sun XML Impress Template (.sti)
- Sun XML Writer Template (.stw)

**Batch 13:**
- Sun XML Draw (.sxd)
- Sun XML Impress (.sxi)
- Sun XML Math (.sxm)
- Sun XML Writer (.sxw)

**Batch 12:**
- StarDraw (.sda)
- StarCalc (.sdc)
- StarImpress (.sdd)
- StarChart (.sds)
- StarWriter (.sdw)
- StarMath (.smf)

**Batch 11:**
- Mozilla XPInstall (.xpi)
- OpenXPS (.xps)
- Microsoft Works Word Processor (.wps)
- Microsoft Works 6 Spreadsheet (.xlr)
- vCalendar 1.0 (.vcs)
- Universal Subtitle Format (.usf)

**Batch 10:**
- Empty file (.empty)
- Multi Layer Archive (.mla)
- PMarc Archive (.pma)
- Nintendo Switch ROM (.xci)
- Material Exchange Format (.mxf)
- Windows Recorded TV Show (.wtv)

**Batch 9:**
- MTV Video (.mtv)
- AbiWord Template (.awt)
- Ogg Speex (.spx)
- macOS Alias (no extension)
- PEM Certificate Signing Request (.csr)
- ActiveMime (.mso)

**Batch 8:**
- Web Application Archive (.war)
- QEMU Copy-On-Write (.qcow)
- Windows Media Audio (.wma)
- Windows Media Video (.wmv)
- RealVideo (.rv)
- Visual Studio Extension (.vsix)

**Batch 7:**
- Windows App Bundle (.appxbundle)
- Microsoft Outlook Personal Storage Table (.pst)
- Microsoft Project Plan (.mpp)
- LArc/LZS Archive (.lzs)
- MPEG-1/2 Audio Layer 2 (.mp2)

**Batch 6:**
- Adobe Illustrator (.ai)
- Adobe AIR Application (.air)
- Microsoft Digital Video Recording (.dvr-ms)
- Adobe Flash Project (.fla)
- InDesign Markup Language (.idml)
- AbiWord Document (.abw)

**Batch 5:**
- Android App Bundle (.aab)
- Windows App Package (.appx)
- iOS App Store Package (.ipa)
- Advanced Stream Redirector (.asx)
- CD Audio track (.cda)

**Batch 4:**
- Enterprise Application Archive (.ear)
- Ogg Media (.ogm)
- Ogg Multiplexed (.ogx)
- Common Object File Format (.o)
- OpenRaster (.ora)
- OpenDocument Text Master Template (.otm)

**Batch 3:**
- BZIP compression (.bz)
- Microsoft Visual Studio Solution (.sln)
- LaTeX Document (.tex)
- Clojure Script (.clj)
- OpenDocument Database (.odb)
- OpenDocument Text Master (.odm)

**Batch 2:**
- Apple iTunes Audiobook (.m4b)
- Apple iTunes Protected Audio (.m4p)
- Flash MP4 Audio (.f4a)
- Flash MP4 Audiobook (.f4b)
- Flash MP4 Video (.f4v)
- Flash MP4 Protected Video (.f4p)
- FictionBook e-book (.fb2)

**Batch 1:**
- MS-DOS Batch files (.bat, .cmd)
- Windows DLL/SYS/SCR extensions (PE format)
- ELF .elf extension
- HEIC/HEIF Sequence .heics/.heifs extensions

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
- [Virtual Machine & Disk Image Formats](#virtual-machine--disk-image-formats)
- [Filesystem Formats](#filesystem-formats)
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
| Windows Registry | `text/plain` | `.reg` | | ASCII or UTF-16 format |

---

## Document Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| PDF | `application/pdf` | `.pdf` | `application/x-pdf` | Adobe Portable Document Format |
| FDF | `application/vnd.fdf` | `.fdf` | | Forms Data Format (PDF variant) |
| PostScript | `application/postscript` | `.ps` | | |
| Encapsulated PostScript | `application/eps` | `.eps` | | Binary EPS with TIFF/WMF preview |
| Word 2007+ | `application/vnd.openxmlformats-officedocument.wordprocessingml.document` | `.docx` | | |
| Word 97-2003 | `application/msword` | `.doc` | | Legacy format (OLE-based) |
| Excel 2007+ | `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` | `.xlsx` | | |
| Excel 97-2003 | `application/vnd.ms-excel` | `.xls` | | Legacy format (OLE-based) |
| PowerPoint 2007+ | `application/vnd.openxmlformats-officedocument.presentationml.presentation` | `.pptx` | | |
| PowerPoint 97-2003 | `application/vnd.ms-powerpoint` | `.ppt` | | Legacy format (OLE-based) |
| Visio 2007+ | `application/vnd.ms-visio.drawing.main+xml` | `.vsdx` | | |
| Publisher | `application/vnd.ms-publisher` | `.pub` | | |
| Outlook Message | `application/vnd.ms-outlook` | `.msg` | | |
| Outlook Personal Storage Table | `application/vnd.ms-outlook` | `.pst` | | Personal email archive (OLE-based) |
| Project Plan | `application/vnd.ms-project` | `.mpp` | | Microsoft Project file |
| InDesign | `application/x-indesign` | `.indd` | | Adobe InDesign Document |
| InDesign Markup Language | `application/vnd.adobe.indesign-idml-package` | `.idml` | | ZIP-based InDesign format |
| Illustrator | `application/vnd.adobe.illustrator` | `.ai` | | Adobe Illustrator Artwork (PDF-based) |
| Flash Project | `application/vnd.adobe.fla` | `.fla` | | Adobe Flash Project (ZIP-based, CS5+) |
| Works Database | `application/vnd.ms-works-db` | `.wdb` | | Microsoft Works Database (v1-2) |
| Works Spreadsheet | `application/vnd.ms-works` | `.wks` | | Microsoft Works Spreadsheet |
| Write | `application/x-mswrite` | `.wri` | | Microsoft Write (v3.0 and v3.1) |
| OneNote | `application/onenote` | `.one` | | |
| WordPerfect | `application/vnd.wordperfect` | `.wpd` | | |
| AbiWord | `application/x-abiword` | `.abw` | | Gzip-compressed XML document |
| AbiWord Template | `application/x-abiword-template` | `.awt` | | AbiWord template format |
| ClarisWorks | `application/x-clarisworks` | `.cwk` | | Apple legacy document format |
| Quark Express | `application/vnd.quark.quarkxpress` | `.qxd` | | Professional publishing software |
| RTF | `text/rtf` | `.rtf` | `application/rtf` | Rich Text Format |
| RTF (UTF-16) | `text/rtf; charset=utf-16` | `.rtf` | | UTF-16 BE/LE variants |
| EPUB | `application/epub+zip` | `.epub` | | Electronic Publication |
| Mobipocket eBook | `application/x-mobipocket-ebook` | `.mobi` | | |
| Microsoft Reader | `application/x-ms-reader` | `.lit` | | eBook format |
| BroadBand eBook | `application/x-lrf` | `.lrf` | | Sony Reader format |
| Amiga Disk File | `application/x-amiga-disk-format` | `.adf` | | Amiga floppy disk image (DOS0-DOS5) |
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
| BZIP | `application/x-bzip` | `.bz` | | Legacy BZIP compression (BZ0) |
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
| ACE | `application/x-ace-compressed` | `.ace` | | ACE Archive |
| Zoo | `application/x-zoo` | `.zoo` | | Zoo Archive |
| ZPAQ | `application/x-zpaq` | `.zpaq` | | ZPAQ Archive |
| Unix Compress | `application/x-compress` | `.Z` | | Unix compress format |
| Snappy Framed | `application/x-snappy-framed` | `.sz` | | Snappy compressed |
| SeqBox | `application/x-seqbox` | `.sbx` | | Sequence Box container |
| PAK | `application/x-pak` | `.pak` | | PAK Archive (games) |
| TASTY | `application/octet-stream` | `.tasty` | | TASTY format |
| LZMA | `application/x-lzma` | `.lzma` | | LZMA compressed |
| LZOP | `application/x-lzop` | `.lzo` | | LZOP compressed |
| LZFSE | `application/x-lzfse` | `.lzfse` | | Apple's Lempel-Ziv Finite State Entropy |
| BZIP3 | `application/x-bzip3` | `.bz3` | | BZIP3 compressed |
| ARJ | `application/arj` | `.arj` | `application/x-arj` | Legacy DOS compression |
| LHA/LZH | `application/x-lzh-compressed` | `.lzh` | `application/x-lha` | Japanese compression |
| LArc/LZS | `application/x-lzh-compressed` | `.lzs` | | Legacy Japanese compression (similar to LZH) |
| WARC | `application/warc` | `.warc` | | Web ARChive format |
| BitTorrent | `application/x-bittorrent` | `.torrent` | | Torrent metadata |
| Mozilla Archive | `application/x-mozilla-archive` | `.mar` | | Firefox/Thunderbird update archive |
| RZIP | `application/x-rzip` | `.rz` | | Long-range compression format |
| LRZIP | `application/x-lrzip` | `.lrz` | | Long-range ZIP compression |
| FITS | `application/fits` | `.fits` | `image/fits` | Flexible Image Transport System |

---

## Image Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| PNG | `image/png` | `.png` | | Portable Network Graphics |
| APNG | `image/vnd.mozilla.apng` | `.apng` | | Animated PNG |
| MNG | `image/x-mng` | `.mng` | | Multiple-image Network Graphics (animated PNG-like) |
| JNG | `image/x-jng` | `.jng` | | JPEG Network Graphics (JPEG with PNG-style chunks) |
| JPEG | `image/jpeg` | `.jpg` | | `.jpeg`, `.jpe`, `.jif`, `.jfif`, `.jfi` |
| JPEG 2000 | `image/jp2` | `.jp2` | | |
| JPEG 2000 Extended | `image/jpx` | `.jpx` | | |
| JPEG 2000 Multi-part | `image/jpm` | `.jpm` | `video/jpm` | |
| JPEG 2000 Codestream | `image/x-jp2-codestream` | `.j2c` | | `.jpc`, `.j2k` - Raw codestream without container |
| JPEG XS | `image/jxs` | `.jxs` | | |
| JPEG XR | `image/jxr` | `.jxr` | `image/vnd.ms-photo` | |
| JPEG XL | `image/jxl` | `.jxl` | | |
| GIF | `image/gif` | `.gif` | | Graphics Interchange Format |
| WebP | `image/webp` | `.webp` | | |
| TIFF | `image/tiff` | `.tiff` | | `.tif` |
| Adobe DNG | `image/x-adobe-dng` | `.dng` | | Digital Negative (TIFF-based) |
| Sony ARW | `image/x-sony-arw` | `.arw` | | Sony RAW format (TIFF-based) |
| Sony SR2 | `image/x-sony-sr2` | `.sr2` | | Sony RAW format, older (TIFF-based) |
| Pentax PEF | `image/x-pentax-pef` | `.pef` | | Pentax RAW format (TIFF-based) |
| Hasselblad 3FR | `image/x-hasselblad-3fr` | `.3fr` | | Hasselblad RAW format (TIFF-based) |
| Minolta MRW | `image/x-minolta-mrw` | `.mrw` | | Minolta RAW format |
| Kodak KDC | `image/x-kodak-kdc` | `.kdc` | | Kodak RAW format |
| Kodak DCR | `image/x-kodak-dcr` | `.dcr` | | Kodak RAW format |
| Cineon | `image/cineon` | `.cin` | | Digital cinema format |
| DPX | `image/x-dpx` | `.dpx` | | Digital Picture Exchange (cinema) |
| BMP | `image/bmp` | `.bmp` | `image/x-bmp`, `image/x-ms-bmp` | `.dib` |
| Windows Icon | `image/x-icon` | `.ico` | | |
| Windows Cursor | `image/x-win-cursor` | `.cur` | | Windows static cursor |
| Apple Icon | `image/x-icns` | `.icns` | | |
| Photoshop | `image/vnd.adobe.photoshop` | `.psd` | `image/x-psd`, `application/photoshop` | |
| Sketch | `image/x-sketch` | `.sketch` | | Design tool by Bohemian Coding (ZIP-based) |
| Figma | `image/x-figma` | `.fig` | | Figma design file (ZIP-based) |
| HEIC | `image/heic` | `.heic` | | High Efficiency Image Container |
| HEIC Sequence | `image/heic-sequence` | `.heic` | `.heics` | |
| HEIF | `image/heif` | `.heif` | | High Efficiency Image Format |
| HEIF Sequence | `image/heif-sequence` | `.heif` | `.heifs` | |
| AVIF | `image/avif` | `.avif` | | AV1 Image File Format |
| AVIF Sequence | `image/avif-sequence` | `.avifs` | | Animated AVIF images |
| BPG | `image/bpg` | `.bpg` | | Better Portable Graphics |
| GIMP XCF | `image/x-xcf` | `.xcf` | | GIMP native format |
| GIMP Pattern | `image/x-gimp-pat` | `.pat` | | |
| GIMP Brush | `image/x-gimp-gbr` | `.gbr` | | |
| OpenRaster | `image/openraster` | `.ora` | | Layered image format (ZIP-based) |
| Radiance HDR | `image/vnd.radiance` | `.hdr` | | High Dynamic Range |
| X11 Pixmap | `image/x-xpixmap` | `.xpm` | | |
| X11 Bitmap | `image/x-xbitmap` | `.xbm` | | Legacy X11 bitmap format |
| Portable Bitmap | `image/x-portable-bitmap` | `.pbm` | | Netpbm format |
| Portable Graymap | `image/x-portable-graymap` | `.pgm` | | Netpbm format |
| Portable Pixmap | `image/x-portable-pixmap` | `.ppm` | | Netpbm format |
| Portable Arbitrary Map | `image/x-portable-arbitrarymap` | `.pam` | | Netpbm format |
| QOI (Quite OK Image) | `image/x-qoi` | `.qoi` | | Quite OK Image format |
| FLIF | `image/flif` | `.flif` | | Free Lossless Image Format (deprecated) |
| OpenEXR | `image/x-exr` | `.exr` | | High dynamic range format |
| Khronos Texture 2 | `image/ktx2` | `.ktx2` | | KTX2 texture format |
| PCX | `image/x-pcx` | `.pcx` | | Picture Exchange / PC Paintbrush |
| PICtor | `image/x-pictor` | `.pic` | | PICtor/PC Paint DOS graphics format |
| AutoCAD Drawing | `image/vnd.dwg` | `.dwg` | `image/x-dwg`, `application/acad`, `application/x-acad`, `application/autocad_dwg`, `application/dwg`, `application/x-dwg`, `application/x-autocad`, `drawing/dwg` | |
| AutoCAD DXF ASCII | `image/vnd.dxf` | `.dxf` | | Drawing Exchange Format (ASCII) |
| AutoCAD DXF Binary | `application/x-dxf` | `.dxf` | | Drawing Exchange Format (Binary) |
| DirectDraw Surface | `image/vnd-ms.dds` | `.dds` | | Game textures |
| Khronos Texture | `image/ktx` | `.ktx` | | OpenGL/Vulkan textures |
| ASTC | `image/x-astc` | `.astc` | | ARM Texture Compression |
| Truevision TGA | `image/x-tga` | `.tga` | | Targa format |
| Sun Raster | `image/x-sun-raster` | `.ras` | | Legacy Unix format |
| Silicon Graphics | `image/x-sgi` | `.sgi` | | Film/VFX format |
| Farbfeld | `image/x-ff` | `.ff` | | Suckless lossless image format |
| JPEG-LS | `image/jls` | `.jls` | | Lossless/near-lossless JPEG (ISO-14495-1) |
| MIFF | `image/x-miff` | `.miff` | | ImageMagick native format |
| PFM | `image/x-pfm` | `.pfm` | | Portable FloatMap (HDR Netpbm) |
| SVG | `image/svg+xml` | `.svg` | | Scalable Vector Graphics |
| SVG (UTF-16) | `image/svg+xml; charset=utf-16` | `.svg` | | UTF-16 BE/LE variants |
| DICOM | `application/dicom` | `.dcm` | | Medical imaging |

---

## Audio Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| MP3 | `audio/mpeg` | `.mp3` | `audio/x-mpeg`, `audio/mp3` | MPEG Audio Layer 3 |
| MP2 | `audio/mpeg` | `.mp2` | | MPEG-1/2 Audio Layer 2 |
| FLAC | `audio/flac` | `.flac` | | Free Lossless Audio Codec |
| WAV | `audio/wav` | `.wav` | `audio/x-wav`, `audio/vnd.wave`, `audio/wave` | Waveform Audio File |
| AIFF | `audio/aiff` | `.aiff` | | `.aif` |
| MIDI | `audio/midi` | `.midi` | `audio/mid` | `.mid` |
| OGG | `application/ogg` | `.ogg` | | Container format |
| OGG Audio | `audio/ogg` | `.oga` | | `.opus` |
| OGG Video | `video/ogg` | `.ogv` | | |
| OGG Media | `video/ogg` | `.ogm` | | Ogg Media (video with subtitles) |
| OGG Multiplexed | `application/ogg` | `.ogx` | | Ogg Multiplexed (audio+video+text) |
| Ogg Speex | `audio/ogg` | `.spx` | | Voice codec in Ogg container |
| Monkey's Audio | `audio/ape` | `.ape` | | |
| Musepack | `audio/musepack` | `.mpc` | | |
| Sun/NeXT Audio | `audio/basic` | `.au` | | `.snd` |
| AMR | `audio/amr` | `.amr` | `audio/amr-nb` | Adaptive Multi-Rate |
| Creative Voice | `audio/x-voc` | `.voc` | | DOS/Sound Blaster audio |
| CD Audio | `application/x-cdf` | `.cda` | | CD Audio track (RIFF CDDA) |
| RealAudio | `audio/x-realaudio` | `.ra` | | Legacy streaming audio |
| M3U | `audio/x-mpegurl` | `.m3u` | `audio/mpegurl` | `.m3u8` |
| PLS | `audio/x-scpls` | `.pls` | | Shoutcast Playlist |
| WPL | `application/vnd.ms-wpl` | `.wpl` | | Windows Media Playlist (XML-based) |
| AAC | `audio/aac` | `.aac` | | Advanced Audio Coding |
| DTS | `audio/vnd.dts` | `.dts` | `audio/vnd.dts.hd` | Digital Theater Systems surround sound |
| Qualcomm PureVoice | `audio/qcelp` | `.qcp` | | |
| M4A | `audio/x-m4a` | `.m4a` | | |
| MPEG-4 Audio | `audio/mp4` | `.mp4` | `audio/x-m4a`, `audio/x-mp4a` | |
| WavPack | `audio/x-wavpack` | `.wv` | | Lossless/lossy compression |
| True Audio | `audio/x-tta` | `.tta` | | Lossless codec |
| DSD Stream | `audio/x-dsf` | `.dsf` | | Direct Stream Digital |
| DSD Interchange | `audio/x-dff` | `.dff` | | Direct Stream Digital |
| Scream Tracker 3 | `audio/s3m` | `.s3m` | | Module format |
| SoundFont 2 | `audio/x-soundfont` | `.sf2` | | MIDI instrument sample format |
| Quite OK Audio | `audio/x-qoa` | `.qoa` | | Modern lossless audio format |
| 8SVX Audio | `audio/x-8svx` | `.8svx` | | `.8sv` | Amiga IFF audio format |
| Audio Visual Research | `audio/x-avr` | `.avr` | | Atari ST audio format |

---

## Video Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| MP4 | `video/mp4` | `.mp4` | | MPEG-4 Part 14 |
| WebM | `video/webm` | `.webm` | `audio/webm` | |
| Matroska | `video/x-matroska` | `.mkv` | | `.mk3d`, `.mka`, `.mks` |
| AVI | `video/x-msvideo` | `.avi` | `video/avi`, `video/msvideo` | Audio Video Interleave |
| MPEG | `video/mpeg` | `.mpeg` | | Generic MPEG format |
| MPEG Video | `video/mpeg` | `.mpg` | | MPEG-1/2 Video |
| DVD Video Object | `video/mpeg` | `.vob` | | `.m2p` | MPEG-2 Program Stream |
| QuickTime | `video/quicktime` | `.mov` | | |
| QuickTime MQV | `video/quicktime` | `.mqv` | | |
| Flash Video | `video/x-flv` | `.flv` | | |
| ASF/WMV | `video/x-ms-asf` | `.asf` | `video/asf`, `video/x-ms-wmv` | Advanced Systems Format |
| Windows Media Audio | `audio/x-ms-wma` | `.wma` | | ASF-based audio format |
| Windows Media Video | `video/x-ms-wmv` | `.wmv` | | ASF-based video format |
| ASX | `video/x-ms-asx` | `.asx` | | Advanced Stream Redirector (ASF playlist) |
| DVR-MS | `video/x-ms-asf` | `.dvr-ms` | | Microsoft Digital Video Recording (ASF-based) |
| M4V | `video/x-m4v` | `.m4v` | | iTunes Video |
| MTV | `video/x-mtv` | `.mtv` | | MTV video format (RIFF-based) |
| RealMedia | `application/vnd.rn-realmedia` | `.rm` | | Legacy streaming media |
| RealVideo | `video/x-pn-realvideo` | `.rv` | | RealNetworks video format |
| RealMedia VBR | `application/vnd.rn-realmedia-vbr` | `.rmvb` | | Variable bitrate variant |
| Silicon Graphics Movie | `video/x-sgi-movie` | `.sgi` | | SGI movie format from IRIX |
| 3GPP | `video/3gpp` | `.3gp` | `video/3gp`, `audio/3gpp` | 3GPP Multimedia |
| 3GPP2 | `video/3gpp2` | `.3g2` | `video/3g2`, `audio/3gpp2` | 3GPP2 Multimedia |
| Motion JPEG 2000 | `video/mj2` | `.mj2` | | |
| DVB | `video/vnd.dvb.file` | `.dvb` | | Digital Video Broadcasting |
| FLIC (FLI) | `video/fli` | `.fli` | | Autodesk animation |
| FLIC (FLC) | `video/flc` | `.flc` | | Autodesk animation |
| Fast Search & Transfer | `video/vnd.fvt` | `.fvt` | | Surveillance video |
| MXF | `application/mxf` | `.mxf` | | Material Exchange Format (SMPTE standard) |
| WTV | `video/x-wtv` | `.wtv` | | Windows Recorded TV Show (successor to DVR-MS) |

---

## Executable & Binary Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Windows PE | `application/vnd.microsoft.portable-executable` | `.exe` | `.dll`, `.sys`, `.scr` | Portable Executable |
| ELF | `application/x-elf` | | `.so` | Executable and Linkable Format |
| ELF Object | `application/x-object` | | | |
| ELF Executable | `application/x-executable` | `.elf` | | |
| ELF Shared Library | `application/x-sharedlib` | `.so` | | |
| ELF Core Dump | `application/x-coredump` | | | |
| COFF | `application/x-coff` | `.o` | | Common Object File Format (i386) |
| Java Class | `application/x-java-applet; charset=binary` | `.class` | `application/x-java-applet` | |
| Java Archive | `application/java-archive` | `.jar` | `application/jar`, `application/jar-archive`, `application/x-java-archive` | |
| Web Application Archive | `application/java-archive` | `.war` | | Java web app (ZIP-based) |
| Enterprise Application Archive | `application/x-ear` | `.ear` | | Java EE application archive |
| Android Package | `application/vnd.android.package-archive` | `.apk` | | |
| Android App Bundle | `application/vnd.android.aab` | `.aab` | | ZIP-based app distribution |
| iOS App Store Package | `application/x-ios-app` | `.ipa` | | ZIP-based iOS app |
| Windows App Package | `application/vnd.ms-appx` | `.appx` | | ZIP-based Windows app |
| Windows App Bundle | `application/vnd.ms-appx.bundle` | `.appxbundle` | | ZIP-based Windows app bundle |
| Adobe AIR Application | `application/vnd.adobe.air-application-installer-package+zip` | `.air` | | ZIP-based Adobe AIR app |
| WebAssembly | `application/wasm` | `.wasm` | | |
| WebAssembly Text | `text/wasm` | `.wat` | | Human-readable WebAssembly text format |
| Amiga Hunk Executable | `application/x-amiga-executable` | `.amiga` | | Amiga legacy executable format |
| Xbox Executable | `application/x-xbox-executable` | `.xbe` | | Original Xbox executable |
| Xbox 360 Executable | `application/x-xbox360-executable` | `.xex` | | Xbox 360 executable (XEX1/XEX2) |
| AppImage | `application/x-appimage` | `.appimage` | | Linux application packaging format |
| COFF | `application/x-coff` | `.o` | | Common Object File Format (i386) |
| LLVM Bitcode | `application/x-llvm` | `.bc` | | LLVM compiler intermediate representation |
| Mach-O | `application/x-mach-binary` | `.macho` | | macOS/iOS executable |
| Microsoft Installer | `application/x-ms-installer` | `.msi` | | |
| Windows Shortcut | `application/x-ms-shortcut` | `.lnk` | | |
| Windows Animated Cursor | `application/x-navi-animation` | `.ani` | | Animated cursor format |
| Windows Help | `application/winhelp` | `.hlp` | | Legacy Windows help format |
| Windows Event Log | `application/x-ms-evt` | `.evt` | | Windows event log format |
| Windows Event Log XML | `application/x-ms-evtx` | `.evtx` | | XML-based Windows event log |
| Dalvik Executable | `application/vnd.android.dex` | `.dex` | | Android bytecode |
| Optimized Dalvik | `application/vnd.android.dey` | `.dey` | | Optimized Android bytecode |
| Lua Bytecode | `application/x-lua-bytecode` | `.luac` | | Compiled Lua |
| Java Keystore | `application/x-java-keystore` | `.jks` | | Java keystore |
| PEM Certificate | `application/x-pem-file` | `.pem` | | PEM format certificate |
| PEM Private Key | `application/x-pem-file` | `.pem` | | PEM format private key |
| PEM Public Key | `application/x-pem-file` | `.pem` | | PEM format public key |
| PEM CSR | `application/x-pem-file` | `.csr` | `.pem` | Certificate Signing Request |
| DER Certificate | `application/x-x509-ca-cert` | `.der` | | DER format certificate |
| PGP Message | `application/pgp` | `.pgp` | | PGP encrypted message |
| PGP Signature | `application/pgp-signature` | `.sig` | | PGP signature |
| PGP Public Key | `application/pgp-keys` | `.asc` | | PGP public key block |
| PGP Private Key | `application/pgp-keys` | `.asc` | | PGP private key block |
| Gettext MO | `application/x-gettext-translation` | `.mo` | | Compiled translation file (little-endian) |
| Age Encryption | `application/x-age-encryption` | `.age` | | Modern file encryption format |

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
| BMFont Binary | `application/x-angelcode-bmfont` | `.fnt` | | AngelCode bitmap font generator binary format |
| Glyphs | `font/x-glyphs` | `.glyphs` | | Glyphs font editor format |

---

## Web & Multimedia Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Adobe Flash | `application/x-shockwave-flash` | `.swf` | | |
| Chrome Extension | `application/x-chrome-extension` | `.crx` | | |
| PKCS#7 Signature | `application/pkcs7-signature` | `.p7s` | | |
| JSON Feed | `application/feed+json` | `.json` | | RSS/Atom alternative in JSON format |

---

## Database Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| SQLite | `application/vnd.sqlite3` | `.sqlite` | `application/x-sqlite3` | |
| MS Access (MDB) | `application/x-msaccess` | `.mdb` | | |
| MS Access (ACCDB) | `application/x-msaccess` | `.accdb` | | |
| dBase | `application/x-dbf` | `.dbf` | | |
| Lotus 1-2-3 v2/v9 | `application/vnd.lotus-1-2-3` | `.123` | | Legacy spreadsheet |
| Lotus 1-2-3 v1 | `application/vnd.lotus-1-2-3` | `.wk1` | | Legacy spreadsheet (version 1) |
| Lotus 1-2-3 v3 | `application/vnd.lotus-1-2-3` | `.wk3` | | Legacy spreadsheet (version 3) |
| Lotus 1-2-3 v4/v5 | `application/vnd.lotus-1-2-3` | `.wk4` | `.wk5` | Legacy spreadsheet (versions 4/5) |
| Lotus Notes | `application/vnd.lotus-notes` | `.nsf` | | Enterprise collaboration database |
| MARC | `application/marc` | `.mrc` | | Library records |
| HDF4 | `application/x-hdf` | `.hdf` | `.hdf4` | Hierarchical Data Format version 4 |
| HDF5 | `application/x-hdf5` | `.hdf5` | `.h5` | Hierarchical Data Format version 5 |
| Apache Parquet | `application/vnd.apache.parquet` | `.parquet` | `application/x-parquet` | |

---

## Programming & Scripting Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| PHP | `text/x-php` | `.php` | | |
| JavaScript | `text/javascript` | `.js` | `application/javascript` | |
| Python | `text/x-python` | `.py` | `text/x-script.python`, `application/x-python` | |
| Python Pickle | `application/x-pickle` | `.pkl` | `.pickle` | Serialization format (protocols 2-5) |
| Python Bytecode | `application/x-python-bytecode` | `.pyc` | | Compiled Python modules |
| Perl | `text/x-perl` | `.pl` | | |
| Ruby | `text/x-ruby` | `.rb` | `application/x-ruby` | |
| Lua | `text/x-lua` | `.lua` | | |
| Shell Script | `text/x-shellscript` | `.sh` | `text/x-sh`, `application/x-shellscript`, `application/x-sh` | |
| MS-DOS Batch | `text/x-msdos-batch` | `.bat` | `.cmd` | Windows batch scripts |
| Tcl | `text/x-tcl` | `.tcl` | `application/x-tcl` | |
| Clojure | `text/x-clojure` | `.clj` | | Clojure script with shebang detection |
| LaTeX | `text/x-tex` | `.tex` | | LaTeX document source |
| Visual Studio Solution | `application/vnd.ms-developer` | `.sln` | | Microsoft Visual Studio solution file |
| Visual Studio Extension | `application/vsix` | `.vsix` | | ZIP-based VS extension |
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
| draw.io | `application/vnd.jgraph.mxfile` | `.drawio` | | XML-based diagramming format |
| XSPF | `application/xspf+xml` | `.xspf` | | XML Shareable Playlist Format |
| XSLT | `application/xslt+xml` | `.xsl` | | Extensible Stylesheet Language Transformations |
| MathML | `application/mathml+xml` | `.mathml` | | Mathematical Markup Language |
| MusicXML | `application/vnd.recordare.musicxml+xml` | `.musicxml` | | Music notation format |
| TTML | `application/ttml+xml` | `.ttml` | | Timed Text Markup Language (subtitles) |
| SOAP | `application/soap+xml` | `.soap` | | Simple Object Access Protocol |
| TMX | `application/x-tmx+xml` | `.tmx` | | Tiled Map XML (game development) |
| TSX | `application/x-tsx+xml` | `.tsx` | | Tiled Tileset XML (game development) |
| MPD | `application/dash+xml` | `.mpd` | | MPEG-DASH Media Presentation Description |
| MXL | `application/vnd.recordare.musicxml` | `.mxl` | | MusicXML ZIP (compressed music notation, ZIP-based) |
| CDDX | `application/vnd.circuitdiagram.document.main+xml` | `.cddx` | | Circuit Diagram Document (electronic circuit diagrams) |
| DWFX | `model/vnd.dwfx+xps` | `.dwfx` | | Design Web Format XPS (Autodesk CAD exchange format) |
| FBZ | `application/x-fbz` | `.fbz` | | FictionBook ZIP (compressed e-book, ZIP-based) |
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
| ODB | `application/vnd.oasis.opendocument.database` | `.odb` | `application/x-vnd.oasis.opendocument.database` | OpenDocument Database |
| ODM | `application/vnd.oasis.opendocument.text-master` | `.odm` | `application/x-vnd.oasis.opendocument.text-master` | OpenDocument Text Master |
| OTT | `application/vnd.oasis.opendocument.text-template` | `.ott` | `application/x-vnd.oasis.opendocument.text-template` | Text Template |
| OTS | `application/vnd.oasis.opendocument.spreadsheet-template` | `.ots` | `application/x-vnd.oasis.opendocument.spreadsheet-template` | Spreadsheet Template |
| OTP | `application/vnd.oasis.opendocument.presentation-template` | `.otp` | `application/x-vnd.oasis.opendocument.presentation-template` | Presentation Template |
| OTG | `application/vnd.oasis.opendocument.graphics-template` | `.otg` | `application/x-vnd.oasis.opendocument.graphics-template` | Graphics Template |
| OTM | `application/vnd.oasis.opendocument.text-master-template` | `.otm` | `application/x-vnd.oasis.opendocument.text-master-template` | Text Master Template |
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
| 3D Studio Max | `application/x-3ds` | `.3ds` | | Autodesk 3DS mesh format |
| 3D Studio Max Project | `application/x-max` | `.max` | | Autodesk 3DS Max project file (OLE-based) |
| Autodesk 123D | `model/x-123dx` | `.123dx` | | Autodesk 123D 3D modeling (ZIP-based) |
| PLY | `application/ply` | `.ply` | | Polygon File Format |
| FBX (Autodesk Filmbox) | `application/vnd.autodesk.fbx` | `.fbx` | | 3D interchange format |
| Fusion 360 | `model/x-f3d` | `.f3d` | | Autodesk Fusion 360 CAD format (ZIP-based) |
| STL ASCII | `model/x.stl-ascii` | `.stl` | `model/stl` | STereoLithography (3D printing) |
| Maya Binary | `application/x-maya-binary` | `.mb` | | Autodesk Maya binary scene |
| Maya ASCII | `application/x-maya-ascii` | `.ma` | | Autodesk Maya ASCII scene |
| InterQuake Model | `model/x-iqm` | `.iqm` | | 3D model format for games |
| MagicaVoxel | `model/x-vox` | `.vox` | | Voxel model format |
| Google Draco | `model/x-draco` | `.drc` | | 3D geometry compression |
| STEP | `model/step` | `.stp` | | ISO 10303-21 CAD data exchange |
| IGES | `model/iges` | `.iges` | `.igs` | Initial Graphics Exchange Specification - CAD data exchange |
| VRML | `model/vrml` | `.wrl` | | Virtual Reality Modeling Language (VRML 1.0 and 2.0) |
| Cinema 4D | `model/x-c4d` | `.c4d` | | Maxon Cinema 4D 3D model format |
| Autodesk Alias | `model/x-wire` | `.wire` | | Autodesk Alias 3D modeling format |
| Design Web Format | `model/vnd.dwf` | `.dwf` | | Autodesk DWF CAD format |
| OpenNURBS | `model/x-3dm` | `.3dm` | | Rhino 3D model format |
| Universal Scene Description Binary | `model/x-usd` | `.usd` | | Pixar USD format |
| Universal Scene Description ASCII | `model/x-usd-ascii` | `.usda` | | Pixar USD text format |
| Universal Scene Description ZIP | `model/vnd.usdz+zip` | `.usdz` | | Pixar USD in ZIP container (AR/VR format) |
| Model3D Binary | `model/x-3d-binary` | `.3d` | | Binary 3D model format |
| SketchUp | `application/vnd.sketchup.skp` | `.skp` | | Trimble SketchUp 3D model format |
| SolidWorks Assembly | `model/x-sldasm` | `.sldasm` | | SolidWorks CAD assembly file |
| SolidWorks Drawing | `model/x-slddrw` | `.slddrw` | | SolidWorks CAD drawing file |
| SolidWorks Part | `model/x-sldprt` | `.sldprt` | | SolidWorks CAD part file |
| Autodesk Inventor Assembly | `model/x-iam` | `.iam` | | Autodesk Inventor CAD assembly file |
| Autodesk Inventor Drawing | `model/x-idw` | `.idw` | | Autodesk Inventor CAD drawing file |
| Autodesk Inventor Presentation | `model/x-ipn` | `.ipn` | | Autodesk Inventor CAD presentation file |
| Autodesk Inventor Part | `model/x-ipt` | `.ipt` | | Autodesk Inventor CAD part file |
| Inter-Quake Export | `model/x-iqe` | `.iqe` | | Text-based 3D model format for games |
| Model 3D ASCII | `text/x-3d-model` | `.a3d` | | Text-based 3D model format |
| Model 3D Binary | `model/x-3d-model` | `.m3d` | | Binary 3D model format |
| SpaceClaim Document | `model/x-scdoc` | `.scdoc` | | SpaceClaim CAD document file |

---

## Gaming Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Nintendo NES ROM | `application/vnd.nintendo.snes.rom` | `.nes` | | |
| Game Boy ROM | `application/x-gameboy-rom` | `.gb` | | Nintendo Game Boy |
| Game Boy Color ROM | `application/x-gameboy-color-rom` | `.gbc` | | Nintendo Game Boy Color |
| Game Boy Advance ROM | `application/x-gba-rom` | `.gba` | | Nintendo Game Boy Advance |
| Commodore 64 Program | `application/x-commodore-64-program` | `.prg` | | C64 executable |
| Commodore 64 Cartridge | `application/x-commodore-64-cartridge` | `.crt` | | C64 cartridge |
| Atari 7800 ROM | `application/x-atari-7800-rom` | `.a78` | | Atari 7800 game |

---

## Network & Debugging Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| PCAP | `application/vnd.tcpdump.pcap` | `.pcap` | | Packet Capture (libpcap) |
| PCAPNG | `application/x-pcapng` | `.pcapng` | | Next Generation PCAP |

---

## Virtual Machine & Disk Image Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| QCOW | `application/x-qemu-disk` | `.qcow` | | QEMU Copy-on-Write v1 disk image |
| QCOW2 | `application/x-qemu-disk` | `.qcow2` | | QEMU Copy-on-Write v2 disk image |
| VHD | `application/x-vhd` | `.vhd` | | Microsoft Virtual Hard Disk (legacy) |
| VHDX | `application/x-vhdx` | `.vhdx` | | Microsoft Virtual Hard Disk v2 |
| VMDK | `application/x-vmdk` | `.vmdk` | | VMware Virtual Disk (KDMV/COWD/descriptor variants) |
| VDI | `application/x-virtualbox-vdi` | `.vdi` | | VirtualBox Virtual Disk Image |
| WIM | `application/x-ms-wim` | `.wim` | | Windows Imaging Format |

---

## Filesystem Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Squashfs | `application/x-squashfs` | `.squashfs` | | Compressed read-only filesystem for embedded systems |

---

## Miscellaneous Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Email Message | `message/rfc822` | `.eml` | | RFC822 email message |
| CBOR | `application/cbor` | `.cbor` | | Concise Binary Object Representation |
| TZif | `application/tzif` | | | Time Zone Information Format |
| OLE Storage | `application/x-ole-storage` | | | Microsoft OLE (legacy Office) |
| AAF | `application/octet-stream` | `.aaf` | | Advanced Authoring Format |
| Fasoo | `application/x-fasoo` | | | Document protection |
| PGP NetShare | `application/x-pgp-net-share` | | | |
| MIE | `application/x-mie` | `.mie` | | Meta Information Encapsulation |
| FigletFont | `application/x-figlet-font` | `.flf` | | ASCII art fonts |
| FIT | `application/x-fit` | `.fit` | | Flexible and Interoperable Data Transfer (Garmin fitness/GPS) |
| GRIB | `application/x-grib` | `.grib` | | Weather data format (meteorology) |
| EBML | `application/x-ebml` | `.ebml` | | Extensible Binary Meta Language |
| ID3v2 | `application/x-id3v2` | | | MP3 metadata |
| ISO 9660 | `application/x-iso9660-image` | `.iso` | | CD/DVD image format |
| ICC Profile | `application/vnd.iccprofile` | `.icc` | | Color profile |
| Android Binary XML | `application/vnd.android.axml` | `.axml` | | Compiled Android XML |
| Android Resource | `application/vnd.android.arsc` | `.arsc` | | Android resource container |
| Stuffit | `application/x-stuffit` | `.sit` | | Mac archive format |
| Stuffit X | `application/x-stuffitx` | `.sitx` | | Mac archive format |
| macOS Alias | `application/x-apple-alias` | | | macOS Finder alias file |
| ActiveMime | `application/x-mso` | `.mso` | | Microsoft Office embedded OLE object |
| Enhanced Metafile | `image/emf` | `.emf` | | Windows vector format |
| Windows Metafile | `image/wmf` | `.wmf` | | Windows vector format |

---

## Batch 10 - Archive, Gaming, and System Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Empty File | `application/x-empty` | `.empty` | | Zero-length file detection |
| MLA | `application/x-mla` | `.mla` | | Multi Layer Archive |
| PMA | `application/x-lzh-compressed` | `.pma` | | PMarc archive (LZH variant with -pm0-/-pm1-/-pm2- signatures) |
| Microsoft Visio | `application/vnd.visio` | `.vsd` | | Microsoft Visio Drawing (OLE-based diagram format) |
| Silverlight App | `application/x-silverlight-app` | `.xap` | | Microsoft Silverlight Application (ZIP-based with AppManifest.xaml) |
| Nintendo Switch ROM | `application/x-nintendo-switch-rom` | `.xci` | | Nintendo Switch XCI - NX Card Image |

---

## Batch 11 - Office Documents, Subtitles, and Mozilla Extensions

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Mozilla XPInstall | `application/x-xpinstall` | `.xpi` | | Firefox/Thunderbird extension (ZIP-based with install.rdf or manifest.json) |
| OpenXPS | `application/oxps` | `.xps` | | XML Paper Specification (ZIP-based document format) |
| MS Works WPS | `application/vnd.ms-works` | `.wps` | | Microsoft Works Word Processor (OLE-based, extension-based detection) |
| MS Works XLR | `application/vnd.ms-works` | `.xlr` | | Microsoft Works 6 Spreadsheet |
| vCalendar 1.0 | `text/calendar` | `.vcs` | | Text-based calendar format (predecessor to iCalendar 2.0) |
| USF | `application/x-usf` | `.usf` | | Universal Subtitle Format (XML-based subtitle format) |

---

## Batch 12 - StarOffice/StarDivision Suite

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| StarDraw | `application/vnd.stardivision.draw` | `.sda` | | StarOffice/StarDivision Draw (graphics, ZIP-based) |
| StarCalc | `application/vnd.stardivision.calc` | `.sdc` | | StarOffice/StarDivision Calc (spreadsheet, ZIP-based) |
| StarImpress | `application/vnd.stardivision.impress` | `.sdd` | | StarOffice/StarDivision Impress (presentation, ZIP-based) |
| StarChart | `application/vnd.stardivision.chart` | `.sds` | | StarOffice/StarDivision Chart (ZIP-based) |
| StarWriter | `application/vnd.stardivision.writer` | `.sdw` | | StarOffice/StarDivision Writer (word processor, ZIP-based) |
| StarMath | `application/vnd.stardivision.math` | `.smf` | | StarOffice/StarDivision Math (mathematical formulas, ZIP-based) |

---

## Batch 13 - Sun XML Office Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Sun XML Draw | `application/vnd.sun.xml.draw` | `.sxd` | | Legacy Sun Microsystems graphics format (ZIP-based, detects via mimetype file) |
| Sun XML Impress | `application/vnd.sun.xml.impress` | `.sxi` | | Legacy Sun Microsystems presentation format (ZIP-based, detects via mimetype file) |
| Sun XML Math | `application/vnd.sun.xml.math` | `.sxm` | | Legacy Sun Microsystems mathematical formula format (ZIP-based, detects via mimetype file) |
| Sun XML Writer | `application/vnd.sun.xml.writer` | `.sxw` | | Legacy Sun Microsystems word processor format (ZIP-based, detects via mimetype file) |

---

## Batch 14 - Sun XML Office Template Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Sun XML Calc Template | `application/vnd.sun.xml.calc.template` | `.stc` | | Legacy Sun Microsystems spreadsheet template (ZIP-based, detects via mimetype file) |
| Sun XML Draw Template | `application/vnd.sun.xml.draw.template` | `.std` | | Legacy Sun Microsystems graphics template (ZIP-based, detects via mimetype file) |
| Sun XML Impress Template | `application/vnd.sun.xml.impress.template` | `.sti` | | Legacy Sun Microsystems presentation template (ZIP-based, detects via mimetype file) |
| Sun XML Writer Template | `application/vnd.sun.xml.writer.template` | `.stw` | | Legacy Sun Microsystems word processor template (ZIP-based, detects via mimetype file) |

---

## Batch 15 - Sun XML Writer Global and WordPerfect Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Sun XML Writer Global | `application/vnd.sun.xml.writer.global` | `.sgw` | | Legacy Sun Microsystems master document format (ZIP-based, detects via mimetype file) |
| WordPerfect Graphics | `application/vnd.wordperfect` | `.wpg` | | WordPerfect graphics format (child of WPD, extension-based detection) |
| WordPerfect Presentations | `application/vnd.wordperfect` | `.shw` | | WordPerfect presentation format (child of WPD, extension-based detection) |
| WordPerfect Macro | `application/vnd.wordperfect` | `.wpm` | | WordPerfect macro format (child of WPD, extension-based detection) |

---

## Batch 16 - Uniform Office Format (UOF)

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Uniform Office Format Presentation | `application/vnd.uof.presentation` | `.uop` | | Chinese office format (ZIP-based, detects via "uof:UOF" namespace and Chinese text) |
| Uniform Office Format Spreadsheet | `application/vnd.uof.spreadsheet` | `.uos` | | Chinese office format (ZIP-based, detects via "uof:UOF" namespace and Chinese text) |
| Uniform Office Format Text | `application/vnd.uof.text` | `.uot` | | Chinese office format (ZIP-based, detects via "uof:UOF" namespace and Chinese text) |

---

## Batch 17 - CAD Exchange Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Initial Graphics Exchange Specification | `model/iges` | `.iges` | `.igs` | CAD data exchange format - detects via 72 spaces + 'S' in column 73 |

---

## Batch 18 - Pixar USD ZIP Format

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Universal Scene Description ZIP | `model/vnd.usdz+zip` | `.usdz` | | Pixar's USD format in ZIP container - detects via .usda/.usdc filenames or #usda header |

---

## Batch 19 - Design Tool Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Sketch 43 | `image/x-sketch` | `.sketch` | | Design tool by Bohemian Coding - detects via document.json/meta.json with "_class" identifiers |

---

## Batch 20 - SolidWorks CAD Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| SolidWorks Assembly | `model/x-sldasm` | `.sldasm` | | OLE-based CAD assembly file - detects via "SolidWorks" and "Assembly"/"SLDASM" strings |
| SolidWorks Drawing | `model/x-slddrw` | `.slddrw` | | OLE-based CAD drawing file - detects via "SolidWorks" and "Drawing"/"SLDDRW" strings |
| SolidWorks Part | `model/x-sldprt` | `.sldprt` | | OLE-based CAD part file - detects via "SolidWorks" and "Part"/"SLDPRT" strings |

---

## Batch 21 - Autodesk Inventor CAD Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Autodesk Inventor Assembly | `model/x-iam` | `.iam` | | OLE-based CAD assembly file - detects via "Inventor" and "Assembly"/".iam" strings |
| Autodesk Inventor Drawing | `model/x-idw` | `.idw` | | OLE-based CAD drawing file - detects via "Inventor" and "Drawing"/".idw" strings |
| Autodesk Inventor Presentation | `model/x-ipn` | `.ipn` | | OLE-based CAD presentation file - detects via "Inventor" and "Presentation"/".ipn" strings |
| Autodesk Inventor Part | `model/x-ipt` | `.ipt` | | OLE-based CAD part file - detects via "Inventor" and "Part"/".ipt" strings |

---

## Batch 22 - Model/3D Binary Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Inter-Quake Export | `model/x-iqe` | `.iqe` | | Text-based 3D model format - detects via "# Inter-Quake Export" header |
| Model 3D Binary | `model/x-3d-model` | `.m3d` | | Binary 3D model format - detects via "3DMO" magic bytes |
| SpaceClaim Document | `model/x-scdoc` | `.scdoc` | | OLE-based CAD document - detects via "SpaceClaim" or "scdoc" strings |

---

## Batch 23 - CAD and 3D Modeling Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Model 3D ASCII | `text/x-3d-model` | `.a3d` | | Text-based 3D model format - detects via "3DGeometry" header |
| Autodesk 123D | `model/x-123dx` | `.123dx` | | ZIP-based 3D modeling format - detects via "123D" or "Autodesk.123D" strings |
| Fusion 360 | `model/x-f3d` | `.f3d` | | ZIP-based CAD format - detects via "Fusion360", "fusion360", or "Autodesk Fusion" strings |

---

## Batch 24 - XML-Based and Design Formats

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| draw.io | `application/vnd.jgraph.mxfile` | `.drawio` | | XML-based diagramming format - detects via "<mxfile" or "<mxGraphModel" elements |
| XSPF | `application/xspf+xml` | `.xspf` | | XML Shareable Playlist Format - detects via "<playlist" element with "xspf" namespace |
| XSLT | `application/xslt+xml` | `.xsl` | | Extensible Stylesheet Language Transformations - detects via "<xsl:stylesheet" or "<xsl:transform" with XSL namespace |
| Figma | `image/x-figma` | `.fig` | | ZIP-based design format - detects via "figma", "document", or "canvas" strings |

---

## Batch 25 - XML-Based Formats (Scientific, Music, Subtitles, Web Services)

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| MathML | `application/mathml+xml` | `.mathml` | | Mathematical Markup Language - detects via math/MathML elements with MathML namespace |
| MusicXML | `application/vnd.recordare.musicxml+xml` | `.musicxml` | | Music notation format - detects via score-partwise or score-timewise root elements |
| TTML | `application/ttml+xml` | `.ttml` | | Timed Text Markup Language (subtitles) - detects via tt element with TTML namespace |
| SOAP | `application/soap+xml` | `.soap` | | Simple Object Access Protocol - detects via Envelope element with SOAP namespace (1.1 or 1.2) |

---

## Batch 26 - XML-Based Formats (Game Development, Streaming, Music)

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| TMX | `application/x-tmx+xml` | `.tmx` | | Tiled Map XML - game development map format, detects via <map> element with version or orientation attributes |
| TSX | `application/x-tsx+xml` | `.tsx` | | Tiled Tileset XML - game development tileset format, detects via <tileset> element with version or tilewidth attributes |
| MPD | `application/dash+xml` | `.mpd` | | MPEG-DASH Media Presentation Description - streaming manifest, detects via <MPD> element with DASH namespace |
| MXL | `application/vnd.recordare.musicxml` | `.mxl` | | MusicXML ZIP - compressed music notation (ZIP-based), detects via .musicxml filename or META-INF/container.xml |

---

## Batch 27 - Final XML-Based Formats (Circuit Diagrams, CAD, E-books)

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| CDDX | `application/vnd.circuitdiagram.document.main+xml` | `.cddx` | | Circuit Diagram Document - electronic circuit diagrams, detects via <circuit> or <CircuitDocument> elements with circuitdiagram namespace |
| DWFX | `model/vnd.dwfx+xps` | `.dwfx` | | Design Web Format XPS - Autodesk CAD exchange format, detects via <DWFDocument> element or dwf/.dwfx strings |
| FBZ | `application/x-fbz` | `.fbz` | | FictionBook ZIP - compressed e-book (ZIP-based), detects via .fb2 filename or FictionBook namespace |

---

## Batch 28 - 3D Studio Max Project File

| Format | MIME Type | Extension(s) | Aliases | Notes |
|--------|-----------|--------------|---------|-------|
| Autodesk 3D Studio Max | `application/x-max` | `.max` | | OLE-based project file - detects via "3dsmax" or "3D Studio Max" strings in metadata |
