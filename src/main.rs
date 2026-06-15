use std::ops::Range;

use ansi_term::Color;
use ansi_term::Style;

mod terminal_management;

//  import os
//  import random
//  import readline
//  import subprocess
//  import my_modules
//
//
//  def main():
//      terminal_size = os.get_terminal_size()
//      print(terminal_size)
//      print(terminal_size.columns)
//      print(terminal_size.lines)
//
//      subprocess.run("clear")
//      print(my_modules.Color.BlackBg)
//      print(my_modules.Color.Dim)
//      for i in range(terminal_size.lines):
//          for j in range(terminal_size.columns):
//              print(f" ", end='')
//      print(my_modules.Color.Reset, end='')
//      input()  # locks the screen so you can review the output
//

//  if __name__ == "__main__":
//      main()

fn clear_screen() {
    // Source - https://stackoverflow.com/a/34837038
    // Posted by minghan, modified by community. See post 'Timeline' for change history
    // Retrieved 2026-06-15, License - CC BY-SA 4.0

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let terminal_size = terminal_management::get_terminal_size();

    println!("Terminal columns: {}", terminal_size.columns);
    println!("Terminal lines: {}", terminal_size.lines);
    println!("Terminal area: {}", terminal_size.get_area());

    clear_screen();

    for i in 1..=terminal_size.lines {
        for j in 1..=terminal_size.columns {
            print!("{}", Style::new().on(Color::Black).paint(" "));
        }
    }
}
