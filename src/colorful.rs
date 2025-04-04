/*
    Why ? 'cause style and colored terminal output are more interesting and good-looking for users.
    Link bellow are for better understanding of how its works.

    https://chrisyeh96.github.io/2020/03/28/terminal-colors.html
    https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
    https://jvns.ca/blog/2024/10/01/terminal-colours/
 */

use std::env;

pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
}

pub enum Style {
    Default,
    Bold,
    Italic,
}

pub struct Colorful {
    text: String, // itself
    style: Option<Style>,
    foreground: Option<Color>,
    background: Option<Color>,
}

impl Colorful {
    pub fn new() {
        //
    }

    pub fn get_text(mut self) -> String {
        self.text
    }

    pub fn set_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn get_style(mut self) -> Some<Style> {
        self.style
    }

    pub fn get_foreground(mut self) -> Some<Color> {
        self.foreground
    }

    pub fn set_foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    pub fn get_background(mut self) -> Some<Color> {
        self.foreground.unwrap()
    }

    pub fn set_background(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    pub fn is_colors_supported() -> bool {
        // explicitly disable colors
        if env::var("NO_COLOR").is_ok() {
            return false;
        }

        // only valid for unix shell
        // https://www.baeldung.com/linux/terminal-colors#3-term-variable
        if let Ok(term) = env::var("TERM") {
            // checking with dumb may not be the proper way but keep it for now
            // https://stackoverflow.com/questions/2465425/how-do-i-determine-if-a-terminal-is-color-capable
            if term == "dumb" {
                return false;
            }
        }

        #[cfg(windows)] {
            use std::io::IsTerminal;
            use std::io::stdout;
            // only if Windows supports ANSI
            if !stdout().is_terminal() {
                return false;
            }
        }

        true
    }
}
