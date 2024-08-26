use colored::{ColoredString, Colorize};

#[derive(Clone, Copy)]
pub enum Color {
    RED,
    YELLOW,
    WHITE,
    GREEN,
    BLUE
}


pub enum MessageRegistry {
    InvalidChar(char)
}

fn paint_text(msg: &str, color: Color) -> ColoredString {
    match color {
        Color::RED => ColoredString::from(msg).red(),
        Color::YELLOW => ColoredString::from(msg).yellow(),
        Color::WHITE => ColoredString::from(msg).white(),
        Color::GREEN => ColoredString::from(msg).green(),
        Color::BLUE => ColoredString::from(msg).blue(),
    }
}

pub fn registry_log(file: &str, line: i32, column: i32, msg: MessageRegistry, color: Color) {
    let msg_boilerplate = format!("({}) at [{} | {}]:", file, line, column);

    let found_message = "Unpredicted character.";

    println!("{} {}", paint_text(msg_boilerplate.as_str(), color), paint_text(found_message, color));
}

pub fn custom_log(msg: &str, color: Color) {
    println!("{}", paint_text(msg, color));
}

pub fn custom_positional_log(file: &str, line: i32, column: i32, msg: &str, color: Color) {

}