use terminal_size::{Height, Width, terminal_size};

fn main() {
    let terminal_size = terminal_size();
    if let Some((Width(w), Height(h))) = terminal_size {
        println!("Your terminal is {} cols wide and {} lines tall", w, h);
    } else {
        println!("Unable to get terminal terminal_size");
    }
}
