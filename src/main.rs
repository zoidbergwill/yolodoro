extern crate ctrlc;
extern crate quicli;
extern crate structopt;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use quicli::prelude::*;
use structopt::StructOpt;

#[cfg(target_os = "macos")]
extern crate mac_notification_sys;


#[cfg(target_os = "macos")]
struct MacOs;

trait Platform {
    fn setup() -> Self;
    fn notify(msg_title: &str, msg_body: &str);
    fn teardown(&mut self);
}

#[cfg(target_os = "macos")]
impl Platform for MacOs {
    fn setup() -> Self {
        MacOs
    }

    fn notify(msg_title: &str, msg_body: &str) {
        let bundle = mac_notification_sys::get_bundle_identifier("Script Editor").unwrap();
        mac_notification_sys::set_application(&bundle).unwrap();
        mac_notification_sys::send_notification(msg_title, &None, msg_body, &None).unwrap();
    }

    fn teardown(&mut self) {}
}

#[cfg(target_os = "macos")]
type CurrPlatform = MacOs;

/// Read some lines of a file
#[derive(Debug, StructOpt)]
#[structopt(
    name = "yolodoro",
    about = "Simplest Pomodoro timer you can think of, written in Rust."
)]
struct Cli {
    /// Length of pomodoro
    #[structopt(default_value = "24")]
    length: u64,
    /// Length of short pause
    #[structopt(default_value = "5")]
    short_pause: u64,
    /// Length of long pause
    #[structopt(default_value = "20")]
    long_pause: u64,
}

fn main() -> CliResult {
    let mut p = CurrPlatform::setup();
    let args = Cli::from_args();
    let pomodoro = Duration::new(args.length * 60, 0);
    let short_pause = Duration::new(args.short_pause * 60, 0);
    let long_pause = Duration::new(args.long_pause * 60, 0);

    let mut count = 0;
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    println!("Waiting for Ctrl-C...");
    thread::spawn(move || loop {
        let pomodoro_msg = format!("Starting new pomodoro: {}m", (pomodoro.as_secs() / 60));
        println!("{}", pomodoro_msg);
		CurrPlatform::notify("Yolodoro", &pomodoro_msg);
        println!("Thread is going to sleep");
        thread::sleep(pomodoro);

        println!("Pomodoro done");
        count += 1;

        let duration = if count % 4 == 0 {
            let pause_msg = format!("Ready for a long pause: {}m", (long_pause.as_secs() / 60));
            println!("{}", pause_msg);
			CurrPlatform::notify("Yolodoro", &pause_msg);
            long_pause
        } else {
            let pause_msg = format!("Ready for a short pause: {}m", (short_pause.as_secs() / 60));
            println!("{}", pause_msg);
			CurrPlatform::notify("Yolodoro", &pause_msg);
            short_pause
        };
        println!("Thread is going to sleep");
        thread::sleep(duration);
    });
    while running.load(Ordering::SeqCst) {}
    println!("Got it! Exiting...");
    p.teardown();
    Ok(())
}
