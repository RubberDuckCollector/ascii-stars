mod terminal_management;

fn main() {
    let terminal_size = terminal_management::get_terminal_size();


    println!("Terminal width: {}", terminal_size.width);
    println!("Terminal height: {}", terminal_size.height);
    println!("Terminal area: {}", terminal_size.get_area());
}
