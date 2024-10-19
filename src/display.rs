use colored::Colorize;
use std::io::{self, Write};

pub fn display_info(ascii_art: &[&str], info: &[String], show_colors: bool) {
    let mut stdout = io::stdout();
    let ascii_width = ascii_art
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let max_lines = std::cmp::max(ascii_art.len(), info.len());
    for i in 0..max_lines {
        let ascii_line = if i < ascii_art.len() {
            ascii_art[i]
        } else {
            ""
        };
        let info_line = if i < info.len() {
            &info[i]
        } else {
            ""
        };
        writeln!(
            stdout,
            "{:<ascii_width$} {}",
            ascii_line.truecolor(173, 216, 230), // Light blue color for ASCII art
            info_line
        )
        .unwrap();
    }
    if show_colors {
        writeln!(stdout).unwrap(); // Add a blank line before the color palette
        display_color_palette();
    }
}

fn display_color_palette() {
    let colors = [
        (0, 0, 0),       // Black
        (128, 0, 0),     // Red
        (0, 128, 0),     // Green
        (128, 128, 0),   // Yellow
        (0, 0, 128),     // Blue
        (128, 0, 128),   // Magenta
        (0, 128, 128),   // Cyan
        (192, 192, 192), // White
        (128, 128, 128), // Bright Black
        (255, 0, 0),     // Bright Red
        (0, 255, 0),     // Bright Green
        (255, 255, 0),   // Bright Yellow
        (0, 0, 255),     // Bright Blue
        (255, 0, 255),   // Bright Magenta
        (0, 255, 255),   // Bright Cyan
        (255, 255, 255), // Bright White
    ];

    for (r, g, b) in colors.iter() {
        print!("{} ", "██".truecolor(*r, *g, *b));
    }
    println!();
}
