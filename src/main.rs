use ansi_term::Color;
use ansi_term::Style;
use std::io::Write;

mod terminal_management;

fn clear_screen() {
    std::process::Command::new("clear");
}

fn main() {
    let terminal_size = terminal_management::get_terminal_size();

    println!("Terminal columns: {}", terminal_size.columns);
    println!("Terminal lines: {}", terminal_size.lines);
    println!("Terminal area: {}", terminal_size.get_area());

    clear_screen();

    for _ in 1..=terminal_size.lines {
        for _ in 1..=terminal_size.columns {
            print!("{}", Style::new().on(Color::Black).paint(" "));
        }
    }
    std::io::stdout().flush().unwrap(); // makes the text print immediately

    let mut msg = "".to_string();
    std::io::stdin().read_line(&mut msg).unwrap(); // input to pause execution after the colored text is done (can verify the screen works)
}
