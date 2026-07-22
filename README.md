<!-- PROJECT LOGO / BANNER -->
<p align="center">
  <img src="assets/logo.svg" alt="obmc" width="150">
</p>

<p align="center">
  <strong>obmc.rs</strong> — Order By Media Creation
  <br>
  <a href="https://crates.io/crates/obmc"><img src="https://img.shields.io/crates/v/obmc" alt="crates.io"></a>
  <a href="https://github.com/MrDwarf7/obmc.rs/actions/workflows/build.yml"><img src="https://github.com/MrDwarf7/obmc.rs/actions/workflows/build.yml/badge.svg" alt="build"></a>
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT-blue" alt="license"></a>
  <a href="https://github.com/MrDwarf7/obmc.rs/releases"><img src="https://img.shields.io/github/v/release/MrDwarf7/obmc.rs" alt="release"></a>
</p>

<!-- TAGLINE + DESCRIPTION -->
## obmc

Prefix media filenames with their creation date. Uses EXIF/container metadata (nom-exif) for images and video, then filesystem timestamps, then epoch as fallback. Same name format on Linux and Windows.

```bash
# Quick example
obmc -f ./media
```

## Features

- **EXIF/Container metadata first** — Extracts creation date from JPEG, PNG, HEIC, AVIF, TIFF, RAW (Canon CR3, Fujifilm RAF, Phase One IIQ), MP4, MOV, MKV, AVI, WebM, 3GP
- **Filesystem fallback** — Uses birthtime or mtime when media metadata is missing
- **Idempotent** — Re-running skips already-prefixed files
- **Parallel by default** — Rayon-powered multi-threaded crawl
- **Cross-platform** — Linux, macOS, Windows
- **Zero dependencies** — Single binary, no runtime

## Installation

### Cargo (Recommended)

```bash
cargo install obmc
```

### One-liner (Linux/macOS/Windows Git Bash)

```bash
curl -fsSL https://github.com/MrDwarf7/obmc.rs/raw/main/build/install.sh | sh
```

Installs to `/usr/local/bin` (or `~/.local/bin` if not writable). Set `OBMC_VERSION=vX.Y.Z` to pin a version.

### System Packages

| OS | Command |
|----|---------|
| Arch | `pacman -S obmc` |
| macOS | `brew install obmc` |
| Fedora | `dnf copr enable MrDwarf7/obmc && dnf install obmc` |
| NixOS | `nix-shell -p obmc` |
| Windows | `winget install obmc` |

### Release Archives

Download from [Releases](https://github.com/MrDwarf7/obmc.rs/releases/latest).

Each archive contains:

```
obmc-<target>-<tag>.zip
  obmc[.exe]
  README.md
  LICENSE-MIT
  LICENSE-APACHE
  THIRD_PARTY_NOTICES.md
```

### Supported Targets

| OS | Arch | Triple |
|----|------|--------|
| Linux | x86_64 | `x86_64-unknown-linux-gnu` |
| Linux | arm64 | `aarch64-unknown-linux-gnu` |
| macOS | Intel | `x86_64-apple-darwin` |
| macOS | Apple Silicon | `aarch64-apple-darwin` |
| Windows | x86_64 | `x86_64-pc-windows-msvc` |

## Usage

```bash
obmc [OPTIONS]
```

### Examples

```bash
# Process media in default folder (./data or <exe>/data)
obmc

# Specify target directory
obmc -f ./media

# Dry run - show what would happen
obmc -f ./media -d

# Single-threaded crawl
obmc -f ./media --serial

# Skip files that already exist at destination
obmc -f ./media --skip-existing

# Quiet mode - only print errors
obmc -f ./media -q

# Exit with code 1 on any rename failure (for scripts)
obmc -f ./media --strict

# Print version
obmc -V
```

### Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--help` | `-h` | Show help |
| `--version` | `-V` | Print version |
| `--folder` | `-f` | Target directory (default: `./data` debug, `<exe>/data` release) |
| `--dry-run` | `-d` | Show what would happen without renaming |
| `--serial` | | Single-threaded crawl (default: parallel via rayon) |
| `--skip-existing` | | Don't rename if destination already exists |
| `--quiet` | `-q` | Suppress non-error output |
| `--strict` | | Exit with code 1 on any rename failure |

Logging: `RUST_LOG=debug obmc ...` (default: `info`)

## Name Format

```
YYYY_MM_DD HH.MM HH.MMAP original_name.ext
```

- 24h time first (sort order), 12h time second (readability)
- Missing creation data falls back silently to filesystem time

## How It Picks the Date

1. EXIF / container metadata (nom-exif)
2. Filesystem birthtime or mtime (whichever is earlier)
3. Epoch (1970-01-01) -- prints a warning

## Supported Formats

Images: JPEG, PNG, HEIC, AVIF, TIFF, RAW (Canon CR3, Fujifilm RAF, Phase One IIQ)

Video/Audio: MP4, MOV, MKV, AVI, WebM, 3GP

## Build

```bash
# Release binary
make build          # or: cargo build --release

# Run tests
make test           # or: cargo test

# Run locally
make run            # or: cargo run -- <args>
```

## License

MIT OR Apache-2.0 — see [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).