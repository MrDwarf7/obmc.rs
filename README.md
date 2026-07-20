# obmc.rs

**obmc** (Order By Media Creation)

Prefix media filenames with their creation date. Uses EXIF/container metadata
(nom-exif) for images and video, then filesystem timestamps, then epoch as
fallback. Same name format on Linux and Windows.

```
screenshot.png  -->  2023_10_07 15.24 03.24PM screenshot.png
```

## How it picks the date

1. EXIF / container metadata (nom-exif)
2. Filesystem birthtime or mtime (whichever is earlier)
3. Epoch (1970-01-01) -- prints a warning

## Supported formats

Images: JPEG, PNG, HEIC, AVIF, TIFF, RAW (Canon CR3, Fujifilm RAF, Phase One IIQ)
Video/Audio: MP4, MOV, MKV, AVI, WebM, 3GP

## Usage

```
obmc -f ./media
```

| Flag | Description |
|------|-------------|
| `-f <dir>` | Target directory (default: `./data` debug, `<exe>/data` release) |
| `-d` | Dry-run: print what would happen |
| `--serial` | Single-threaded crawl (default: parallel via rayon) |
| `--skip-existing` | Don't rename if destination already exists |
| `-v` | Print version |
| `-q` | Quiet: only print errors |
| `--strict` | Exit with code 1 on any rename failure (for scripts) |

## Name format

`YYYY_MM_DD HH.MM HH.MMAP original_name.ext`

- 24h time first (sort order), 12h time second (readability)
- Missing creation data falls back silently to filesystem time

## Install

### cargo

```
cargo install obmc
```

### From source

```
git clone https://github.com/MrDwarf7/obmc.rs
cd obmc.rs
make build
```

## Build

```
make build        # release binary at target/release/obmc
make test         # unit + integration
make run          # default folder
```

## License

MIT
