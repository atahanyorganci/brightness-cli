use clap::{Parser, Subcommand};
use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use std::ffi::CString;
use brightness_cli::*;

fn get_display_service() -> Option<io_service_t> {
    let service_name = CString::new("IODisplayConnect").unwrap();
    let service = unsafe {
        IOServiceGetMatchingService(
            kIOMainPortDefault,
            IOServiceMatching(service_name.as_ptr()),
        )
    };
    if service == 0 {
        None
    } else {
        Some(service)
    }
}

fn brightness_key_cfstring() -> CFString {
    CFString::new("brightness")
}

fn get_brightness() -> Result<f32, String> {
    let service = get_display_service().ok_or("No display service found")?;
    let mut brightness = 0f32;
    let key = brightness_key_cfstring();
    let ret = unsafe {
        IODisplayGetFloatParameter(
            service,
            0,
            key.as_concrete_TypeRef() as *const brightness_cli::__CFString,
            &mut brightness,
        )
    };
    unsafe { IOObjectRelease(service) };
    if ret == 0 {
        Ok(brightness)
    } else {
        Err(format!("IODisplayGetFloatParameter failed: {ret}"))
    }
}

fn set_brightness(value: f32) -> Result<f32, String> {
    let service = get_display_service().ok_or("No display service found")?;
    let v = value.clamp(0.0, 1.0);
    let key = brightness_key_cfstring();
    let ret = unsafe {
        IODisplaySetFloatParameter(
            service,
            0,
            key.as_concrete_TypeRef() as *const brightness_cli::__CFString,
            v,
        )
    };
    unsafe { IOObjectRelease(service) };
    if ret == 0 {
        Ok(v)
    } else {
        Err(format!("IODisplaySetFloatParameter failed: {ret}"))
    }
}

#[derive(Parser)]
#[command(name = "brightness", about = "Control display brightness via IOKit")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Print current brightness (0–100%)
    Get,
    /// Set brightness (0.0–1.0)
    Set { value: f32 },
    /// Increase brightness by STEP (default 0.1)
    Up {
        #[arg(default_value = "0.1")]
        step: f32,
    },
    /// Decrease brightness by STEP (default 0.1)
    Down {
        #[arg(default_value = "0.1")]
        step: f32,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.cmd {
        Cmd::Get => get_brightness().map(|b| println!("{:.0}%", b * 100.0)),
        Cmd::Set { value } => {
            set_brightness(value).map(|v| println!("{:.0}%", v * 100.0))
        }
        Cmd::Up { step } => get_brightness()
            .and_then(|b| set_brightness(b + step))
            .map(|v| println!("{:.0}%", v * 100.0)),
        Cmd::Down { step } => get_brightness()
            .and_then(|b| set_brightness(b - step))
            .map(|v| println!("{:.0}%", v * 100.0)),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
