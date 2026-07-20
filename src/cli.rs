use std::path::PathBuf;

use clap::Parser;

use crate::folders::CrawlType;

#[rustfmt::skip]
#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    author,
    version,
    about,
    long_about = "\n\
Organize media files by prefixing filenames with their media creation date.\n\
Uses EXIF / container metadata via nom-exif (Windows and Unix).\n\
",
    // Custom -v / -V below
    disable_version_flag = true,
    styles = get_styles()
)]
pub struct Cli {
    /// Folder to organize (default: ./data in debug, <exe_dir>/data in release)
    #[arg(
        short = 'f',
        long = "folder",
        value_name = "FOLDER",
        value_hint = clap::ValueHint::DirPath
    )]
    pub folder: Option<PathBuf>,

    /// Print what would be renamed without touching the filesystem
    #[arg(short = 'd', long = "dry-run", alias = "dry_run", default_value_t = false)]
    pub dry_run: bool,

    /// Force serial directory crawl (default is parallel via rayon).
    /// Stored as [`CrawlType`] via [`CrawlType::from`] (`true` -> Serial).
    #[arg(long = "serial", default_value_t = false)]
    serial: bool,

    /// Print version and exit
    #[arg(short = 'v', short_alias = 'V', long = "version")]
    pub version: bool,
}

impl Cli {
    pub fn new() -> Self {
        let s = Self::parse();
        if s.version {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        s
    }

    /// Crawl strategy for this invocation (`--serial` -> [`CrawlType::Serial`]).
    #[inline]
    pub fn crawl_type(&self) -> CrawlType {
        CrawlType::from(self.serial)
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue))),
        )
        .literal(anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightWhite))))
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)))
                .effects(anstyle::Effects::ITALIC),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
        )
        .placeholder(anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))))
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn serial_flag_maps_to_crawl_type() {
        assert_eq!(CrawlType::from(true), CrawlType::Serial);
        assert_eq!(CrawlType::from(false), CrawlType::Parallel);
        assert_eq!(CrawlType::default(), CrawlType::Parallel);

        let parallel = Cli {
            folder:  None,
            dry_run: false,
            serial:  false,
            version: false,
        };
        assert_eq!(parallel.crawl_type(), CrawlType::Parallel);

        let serial = Cli {
            serial: true,
            ..parallel
        };
        assert_eq!(serial.crawl_type(), CrawlType::Serial);
    }
}
