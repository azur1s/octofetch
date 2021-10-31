use std::fmt;

pub struct ContentBox {
  pub pushed_lines: Vec<String>,
  pub longest_line: usize,
}

const COLOR_OFFSET: usize = 17;

impl fmt::Display for ContentBox {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let dashes = "─".repeat(self.longest_line - COLOR_OFFSET);

    writeln!(f, "╭{}╮", dashes)?;

    for pushed_line in self.pushed_lines.iter() {
      writeln!(
        f,
        "│ {}{} │",
        pushed_line,
        " ".repeat(self.longest_line - pushed_line.len() + 1)
      )?;
    }

    writeln!(f, "╰{}╯", dashes)
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
