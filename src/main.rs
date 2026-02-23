use brightness_cli::{get_brightness, set_brightness};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "brightness",
    about = "Control display brightness via DisplayServices"
)]
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
        Cmd::Set { value } => set_brightness(value).map(|v| println!("{:.0}%", v * 100.0)),
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
