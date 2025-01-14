#[cfg(target_os = "linux")]
mod linux_usbfs;

#[cfg(target_os = "linux")]
pub use linux_usbfs::*;

#[cfg(target_os = "windows")]
mod windows_winusb;

#[cfg(target_os = "windows")]
pub use windows_winusb::*;
