use ansi_term::{Colour, Style};

fn pretty_print(bg: Colour, level: &str, content: &str) {
    println!(
        "{} {}",
        Style::new()
            .on(bg)
            .fg(Colour::RGB(255, 255, 255))
            .paint(format!(" {} ", level)),
        content
    );
}

pub fn info(content: &str) {
    pretty_print(Colour::Blue, "INFO", content);
}

pub fn success(content: &str) {
    pretty_print(Colour::Green, "SUCCESS", content);
}

pub fn error(content: &str) {
    pretty_print(Colour::Red, "ERROR", content);
}