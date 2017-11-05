extern crate rusting;
use rusting::Rust;

use std::thread;
use std::process::Child;
use std::io::{BufReader, Write, BufRead, Error};
use std::sync::mpsc::{Sender, Receiver, channel};

pub struct AsyncCommand {
	process: std::process::Child,
	tx: Sender<Option<String>>,
	rx: Receiver<Option<String>>,
}

fn e<T: ::std::fmt::Display>(e: T) { println!("doesn't rust: {}", e); }
fn o() { println!("no rusting for empty options"); }

impl AsyncCommand {
	pub fn new(c: Result<Child, Error>) -> AsyncCommand {
		let process = c.rust(e);

		let (tx, rx) = channel();
		
		AsyncCommand {
			process: process,
			tx: tx,
			rx: rx,
		}
	}

	pub fn run(&mut self) {
		let tx = self.tx.clone();
		let stdout = self.process.stdout.take().rust(o);

		thread::spawn(move || {
			let reader = BufReader::new(stdout);

			for line in reader.lines() {
				tx.send(Some(line.rust(e))).rust(e);
			}
		});
	}

	pub fn push(&mut self, buf: &[u8]) {
		let stdin = self.process.stdin.as_mut().rust(o);

		stdin.write(buf).rust(e);
		stdin.flush().rust(e);
	}

	pub fn packets(&mut self) -> AsyncCommandIntoIterator {
		AsyncCommandIntoIterator {
			subprocess: self,
		}
	}
}

pub struct AsyncCommandIntoIterator<'a> {
	subprocess: &'a mut AsyncCommand,
}

impl <'a>Iterator for AsyncCommandIntoIterator<'a> {
	type Item = String;
	fn next(&mut self) -> Option<String> {
		let data = self.subprocess.rx.try_recv();
		if data.is_ok() {
			data.rust(e)
		} else { None }
	}
}
