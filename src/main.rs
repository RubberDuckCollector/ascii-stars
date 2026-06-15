use ansi_term::Color;
use ansi_term::Style;
use rand::prelude::*;
use std::io::Write;

mod terminal_management;

fn clear_screen() {
    std::process::Command::new("clear");
}

#[derive(Debug)]
struct Stars {
    dot: String,
    asterisk: String,
    o: String,
}

impl Stars {
    fn instantiate_stars(param_dot: String, param_asterisk: String, param_o: String) -> Stars {
        Stars {
            dot: param_dot,
            asterisk: param_asterisk,
            o: param_o,
        }
    }
}

fn main() {
    let terminal_size = terminal_management::get_terminal_size();

    println!("Terminal columns: {}", terminal_size.columns);
    println!("Terminal lines: {}", terminal_size.lines);
    println!("Terminal area: {}", terminal_size.get_area());

    let mut rng = rand::rng(); // get rng object

    // let mut dot_symbol = String::new();
    // let mut asterisk_symbol = String::new();
    // let mut o_symbol = String::new();

    println!("About to generate screen. Press enter to continue.");

    let dot_symbol = ".".to_string();
    let asterisk_symbol = "*".to_string();
    let o_symbol = "o".to_string();

    let mut star_symbols = vec![dot_symbol, asterisk_symbol, o_symbol];

    let my_stars = Stars::instantiate_stars(
        star_symbols[0].clone(),
        star_symbols[1].clone(),
        star_symbols[2].clone(),
    );
    println!("my_stars: {:#?}", my_stars);

    println!("About to generate screen. Press enter to continue.");
    std::io::stdout().flush().unwrap(); // makes the text print immediately

    let mut msg = "".to_string();
    std::io::stdin().read_line(&mut msg).unwrap(); // input to pause execution after the colored text is done (can verify the screen works)

    clear_screen();

    for _ in 1..=terminal_size.lines {
        for _ in 1..=terminal_size.columns {
            let is_filled_in_with_star: u8 = rand::random_range(0..=2);
            match is_filled_in_with_star {
                0 => match star_symbols.choose(&mut rand::rng()) {
                    Some(i) => print!("{}", Style::new().on(Color::Black).paint(i)),
                    None => panic!("Error while deciding what star should go in the space."),
                },
                1 => print!("{}", Style::new().on(Color::Black).paint(" ")),
                2 => print!("{}", Style::new().on(Color::Black).paint(" ")),
                _ => panic!("Error while matching whether a space is filled in with a star."),
            }
        }
    }
    std::io::stdout().flush().unwrap(); // makes the text print immediately

    let mut msg = "".to_string();
    std::io::stdin().read_line(&mut msg).unwrap(); // input to pause execution after the colored text is done (can verify the screen works)

    // let x: f32 = rng.random();
    // println!("rand f32 0.0..=1.0: {}", x);
}
