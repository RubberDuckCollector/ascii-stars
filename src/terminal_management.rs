use terminal_size::{Width, Height, terminal_size};

// ALTERNATE IMPLMENTATION OF RETRIEVING TERMINAL DIMENSIONS
// let (w, h) = terminal_size
//     .map(|(Columns(w), Lines(h))| (w, h))
//     .unwrap_or((80, 24));
//
// if terminal_size.is_none() {
//     eprintln!("Could not detect a terminal size. Using fallback dimensions of 80x24")
// }

pub struct TerminalSize {
    pub columns: u16,
    pub lines: u16,
}

impl TerminalSize {
    pub fn get_area(&self) -> u32 {
        self.columns as u32 * self.lines as u32
    }
}

pub fn get_terminal_size() -> TerminalSize {
    let terminal_size = terminal_size(); // returns an Option

    // this destructs the Option and releases w and h into the current scope
    let (columns, lines) = terminal_size
        .map(|(Width(w), Height(h))| (w, h))
        .unwrap_or((80, 24));

    if terminal_size.is_none() {
        eprintln!("COULD NOT DETECT A TERMINAL SIZE. USING FALLBACK DIMENSIONS OF 80X24")
    }

    let user_terminal_size = TerminalSize {
        columns: columns,
        lines: lines,
    };

    return user_terminal_size;
}
