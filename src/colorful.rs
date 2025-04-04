/*
    Why ? 'cause style and colored terminal output are more interesting and good-looking for users.
    Link bellow are for better understanding of how its works.

    https://chrisyeh96.github.io/2020/03/28/terminal-colors.html
    https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
    https://jvns.ca/blog/2024/10/01/terminal-colours/
 */

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
    style: Style,
    foreground: Style,
    background: Style,
}

impl Colorful {
    pub fn new() {
        //
    }

    pub fn get_text() {
        //
    }

    pub fn set_style() {
        //
    }

    pub fn get_style() {
        //
    }

    pub fn get_foreground() {
        //
    }

    pub fn set_foreground() {
        //
    }

    pub fn get_background() {
        //
    }

    pub fn set_background() {
        //
    }
}
