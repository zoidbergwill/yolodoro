extern crate notify_rust;

use notify_rust::Notification;

use std::time::Duration;
use std::thread;

const POMODORO_SECONDS: u64 = 60*24;
// const POMODORO_SECONDS: u64 = 3;
const SHORT_PAUSE_SECONDS: u64 = 60*5;
const LONG_PAUSE_SECONDS: u64 = 60*20;

fn blocking_notification(summary: &str, body: &str) {
  #[allow(unused_variables)]
  let notification_handle = Notification::new()
    .summary(summary)
    .body(body)
    .show()
    .unwrap();

  #[cfg(all(unix, not(target_os = "macos")))]
  notification_handle.wait_for_action(|_action| {
    println!("Notification was closed");
  });
}

fn main() {
  let pomodoro = Duration::new(POMODORO_SECONDS, 0);
  let short_pause = Duration::new(SHORT_PAUSE_SECONDS, 0);
  let long_pause = Duration::new(LONG_PAUSE_SECONDS, 0);

  let mut count = 0;
  loop {
    let pomodoro_msg = format!("Starting new pomodoro: {}m", (POMODORO_SECONDS / 60));
    blocking_notification(&pomodoro_msg.to_owned(), "");
    println!("{}", pomodoro_msg);
    thread::sleep(pomodoro);

    println!("Pomodoro done");
    count += 1;

    let duration = if count % 4 == 0 {
      let pause_msg = format!("Ready for a long pause: {}m", (LONG_PAUSE_SECONDS / 60));
      println!("{}", pause_msg);
      blocking_notification(&pause_msg.to_owned(), "");
      long_pause
    } else {
      let pause_msg = format!("Ready for a short pause: {}m", (SHORT_PAUSE_SECONDS / 60));
      println!("{}", pause_msg);
      blocking_notification(&pause_msg.to_owned(), "");
      short_pause
    };
    thread::sleep(duration);
  }
}
