mod terminal_management;

fn main() {
    let terminal_size = terminal_management::get_terminal_size();

    // let (w, h) = terminal_size
    //     .map(|(Width(w), Height(h))| (w, h))
    //     .unwrap_or((80, 24));

    // if terminal_size.is_none() {
    //     eprintln!("Could not detect a terminal size. Using fallback dimensions of 80x24")
    // }

    println!("Terminal width: {}", terminal_size.width);
    println!("Terminal height: {}", terminal_size.height);
    println!("Terminal area: {}", terminal_size.get_area());
}
