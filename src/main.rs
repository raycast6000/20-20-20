#![feature(duration_constructors)]
use notify_rust::Notification;
use std::{
    thread,
    time::Duration
};

fn main() {
    println!("The 20-20-20 rule reminder is on.");

    loop {
        thread::sleep(Duration::from_mins(20));

        Notification::new()
            .summary("The 20-20-20 rule")
            .body("Look at something 20-feet away or more.")
            .timeout(30)
            .show()
            .unwrap();
    }
}
