mod proverb;

use colored::Colorize;
use proverb::bestow_wisdom;
use terminal_size::{terminal_size, Height, Width};

fn main() {
  let dimensions = terminal_size();

  if let Some((Width(w), Height(_h))) = dimensions {
    let mut proverb: String = bestow_wisdom();
    while proverb.len() > w as usize {
      proverb = bestow_wisdom();
    }

    println!("{}", proverb.white());
  } else {
    println!("unable to bestow wisdom")
  }
}
