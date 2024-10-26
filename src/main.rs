mod proverb;

use colored::Colorize;
use proverb::bestow_wisdom;

fn main() {
    println!("{}", bestow_wisdom().white());
}
