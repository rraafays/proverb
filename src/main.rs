use colored::Colorize;
use proverb::bestow_wisdom;

mod proverb;

fn main() {
  println!("{}", bestow_wisdom().white());
}
