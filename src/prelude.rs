// in-crate Error type
// pub use crate::error::Error;

pub use std::path::PathBuf;
pub use std::sync::LazyLock;

use eyre::Error;

// in-crate result type
pub type Result<T> = std::result::Result<T, Error>;

// Wrapper struct
pub struct W<T>(pub T);

pub const DATE_SEP: &str = "_";
pub const DATE_TIME_SEP: &str = " ";
pub const TIME_SEP: &str = ".";
pub const AM_PM_SEP: &str = "";

// $File = Get-Item "E:\GitHub\Rust\order_by_media_creation\data\test.mp4"
// $ShellApplication = New-Object -ComObject Shell.Application
// $ShellFolder = $ShellApplication.Namespace($File.Directory.FullName)
// $ShellFile = $ShellFolder.ParseName($File.Name)
//
// # 208 = MediaCreated
// $v = $ShellFolder.GetDetailsOf($ShellFile, 208)
// Write-Host $v
