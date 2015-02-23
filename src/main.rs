#![feature(old_io)]

extern crate term;
use std::thread;

fn main() {
    let mut t = term::stdout().unwrap();

    for i in 0..1000 {
        thread::spawn(move || {
            println!("from thread {}", i);
        });
    }

    t.fg(term::color::RED).unwrap();
    writeln!(t, "this  is a test").unwrap();
}
