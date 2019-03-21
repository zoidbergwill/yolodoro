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
use mac_notification_sys::*;

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

#[cfg(target_os = "macos")]
fn notify(msg: &str) {
    send_notification("Yolodoro",
                      &Some(msg),
                      "Run away as fast as you can",
                      &Some("Blow"))
        .unwrap();
}

fn main() -> CliResult {
    let bundle = get_bundle_identifier_or_default("firefox");
    set_application(&bundle).unwrap();
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
		notify(&pomodoro_msg);
        println!("Thread is going to sleep");
        thread::sleep(pomodoro);

        println!("Pomodoro done");
        count += 1;

        let duration = if count % 4 == 0 {
            let pause_msg = format!("Ready for a long pause: {}m", (long_pause.as_secs() / 60));
            println!("{}", pause_msg);
            notify(&pause_msg);
            long_pause
        } else {
            let pause_msg = format!("Ready for a short pause: {}m", (short_pause.as_secs() / 60));
            println!("{}", pause_msg);
            notify(&pause_msg);
            short_pause
        };
        println!("Thread is going to sleep");
        thread::sleep(duration);
    });
    while running.load(Ordering::SeqCst) {}
    println!("Got it! Exiting...");
    Ok(())
}
