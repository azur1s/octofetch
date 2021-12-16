use std::{fmt, process};

pub struct ContentBox {
	pub header: String,
	pub pushed_lines: Vec<String>,
	pub longest_line: usize,
	pub static_reduction: usize,
	pub border: bool,
}

impl fmt::Display for ContentBox {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.border {
			if self.header.len() > self.longest_line - self.static_reduction {
				println!(
					"The header is too long for it to display, try reducing the header length."
				);
				process::exit(1);
			}

			let dashes = "─";
			let top = format!(
				"╭{}{}╮",
				self.header,
				dashes.repeat(self.longest_line + 3 - self.static_reduction - self.header.len())
			);

			writeln!(f, "{}", top)?;

			for pushed_line in self.pushed_lines.iter() {
				writeln!(
					f,
					"│ {}{} │",
					pushed_line,
					" ".repeat(self.longest_line - pushed_line.len() + 1)
				)?;
			}

			writeln!(
				f,
				"╰{}╯",
				dashes.repeat(self.longest_line + 3 - self.static_reduction)
			)
		} else {
			for pushed_line in self.pushed_lines.iter() {
				writeln!(f, "{}", pushed_line)?;
			}

			writeln!(f)
		}
	}
}

impl ContentBox {
	pub fn push(&mut self, line: String) {
		let new_line = line.len();
		self.pushed_lines.push(line);
		if new_line > self.longest_line {
			self.longest_line = new_line;
		}
	}
}
