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

| Flag              | Description                                                      |
| ----------------- | ---------------------------------------------------------------- |
| `-f <dir>`        | Target directory (default: `./data` debug, `<exe>/data` release) |
| `-d`              | Dry-run: print what would happen                                 |
| `--serial`        | Single-threaded crawl (default: parallel via rayon)              |
| `--skip-existing` | Don't rename if destination already exists                       |
| `-v`              | Print version                                                    |
| `-q`              | Quiet: only print errors                                         |
| `--strict`        | Exit with code 1 on any rename failure (for scripts)             |

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

### System (curl pipe)

```
curl -fsSL https://github.com/MrDwarf7/obmc.rs/releases/latest/download/install.sh | bash
```

Detects your OS and architecture, downloads the right asset from the
latest release, and installs to `/usr/local/bin`.

### Manual download

Grab the archive for your platform from the
[releases page](https://github.com/MrDwarf7/obmc.rs/releases/latest).

| OS                    | Archive                                   |
| --------------------- | ----------------------------------------- |
| Linux (x86_64)        | `obmc-x86_64-unknown-linux-gnu-<tag>.zip` |
| macOS (x86_64)        | `obmc-x86_64-apple-darwin-<tag>.zip`      |
| macOS (Apple Silicon) | `obmc-aarch64-apple-darwin-<tag>.zip`     |
| Windows (x86_64)      | `obmc-x86_64-pc-windows-msvc-<tag>.zip`   |

Extract the archive and place the `obmc` (or `obmc.exe`) binary somewhere
in your `PATH`.

## Build

```
make build        # release binary at target/release/obmc
make test         # unit + integration
make run          # default folder
```

## License

MIT
