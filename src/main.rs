extern crate ctrlc;
extern crate notify_rust;
extern crate quicli;
extern crate structopt;

use std::io;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;
use quicli::prelude::*;
use structopt::StructOpt;

fn blocking_notification(summary: &str, body: &str) {
    #[allow(unused_variables)]
    let notification_handle = Notification::new()
        .summary(summary)
        .body(body)
        .hint(Hint::Resident(true))
        .show()
        .unwrap();

    #[cfg(all(unix, not(target_os = "macos")))]
    notification_handle.wait_for_action(|_action| {
        println!("Notification was closed");
    });
}

/// Read some lines of a file
#[derive(Debug, StructOpt)]
#[structopt(
    name = "yolodoro",
    about = "Simplest Pomodoro timer you can think of, written in Rust."
)]
struct Cli {
    /// Length of pomodoro
    #[structopt(short = "l", default_value = "24")]
    length: u64,
    /// Length of short pause
    #[structopt(short = "s", default_value = "5")]
    short_pause: u64,
    /// Length of long pause
    #[structopt(short = "lp", default_value = "20")]
    long_pause: u64,
}

fn main() -> CliResult {
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
        let stdout = io::stdout();
        let _ = writeln!(&mut stdout.lock(), "{}", pomodoro_msg);
        blocking_notification(&pomodoro_msg.to_owned(), "");
        thread::sleep(pomodoro);

        println!("Pomodoro done");
        count += 1;

        let duration = if count % 4 == 0 {
            let pause_msg = format!("Ready for a long pause: {}m", (long_pause.as_secs() / 60));
            let _ = writeln!(&mut stdout.lock(), "{}", pause_msg);
            blocking_notification(&pause_msg.to_owned(), &pause_msg);
            long_pause
        } else {
            let pause_msg = format!("Ready for a short pause: {}m", (short_pause.as_secs() / 60));
            let _ = writeln!(&mut stdout.lock(), "{}", pause_msg);
            blocking_notification(&pause_msg.to_owned(), &pause_msg);
            short_pause
        };
        thread::sleep(duration);
    });
    while running.load(Ordering::SeqCst) {}
    println!("Got it! Exiting...");
    Ok(())
}
