use terminal_size::{Height, Width, terminal_size};

pub struct TerminalSize {
    pub width: u16,
    pub height: u16,
}

impl TerminalSize {
    pub fn get_area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }
}

pub fn get_terminal_size() -> TerminalSize {
    let terminal_size = terminal_size(); // returns an Option

    // this releases w and h into the current scope
    let (width, height) = terminal_size
        .map(|(Width(w), Height(h))| (w, h))
        .unwrap_or((80, 24));

    if terminal_size.is_none() {
        eprintln!("COULD NOT DETECT A TERMINAL SIZE. USING FALLBACK DIMENSIONS OF 80X24")
    }

    let user_terminal_size = TerminalSize {
        width: width,
        height: height,
    };

    return user_terminal_size;
}
