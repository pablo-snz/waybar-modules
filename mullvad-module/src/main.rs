extern crate clap;
use clap::{App, Arg};
use serde_json::json;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

struct MullvadStatus {
    connected: bool,
}

impl MullvadStatus {
    fn new() -> Self {
        let mullvad_status_output = Command::new("mullvad")
            .arg("status")
            .output()
            .expect("Error executing 'mullvad status'");

        let mullvad_status = String::from_utf8_lossy(&mullvad_status_output.stdout);
        let connected = mullvad_status.contains("Connected to");

        MullvadStatus { connected }
    }

    fn toggle(&mut self) {
        if self.connected {
            let _ = Command::new("mullvad")
                .arg("disconnect")
                .status();
        } else {
            let _ = Command::new("mullvad")
                .arg("connect")
                .status();
        }
        
        sleep(Duration::from_millis(500));

        *self = MullvadStatus::new();
    }

    fn get_output(&self) -> String {
        let status_text = if self.connected {
            "on"
        } else {
            "off"
        };

        json!({
            "alt": status_text,
            "tooltip": self.get_mullvad_status(),
            "class": status_text
        })
        .to_string()
    }

    fn get_mullvad_status(&self) -> String {
        let mullvad_status_output = Command::new("mullvad")
            .arg("status")
            .output()
            .expect("Error executing 'mullvad status'");

        String::from_utf8_lossy(&mullvad_status_output.stdout).trim().to_string()
    }
}

fn main() {
    let matches = App::new("Waybar Mullvad Status")
        .version("1.0")
        .author("pablo-snz")
        .about("Get Mullvad status and toggle for Waybar")
        .arg(Arg::with_name("toggle")
            .short('t')
            .long("toggle")
            .help("Connect or disconnect Mullvad")
            .takes_value(false)
        )
        .get_matches();

    let mut mullvad_status = MullvadStatus::new();

    if matches.is_present("toggle") {
        mullvad_status.toggle();
    }

    let output = mullvad_status.get_output();

    println!("{}", output);
}

