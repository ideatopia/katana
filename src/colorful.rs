/*
    Why ? 'cause style and colored terminal output are more interesting and good-looking for users.
    Link bellow are for better understanding of how its works.

    https://chrisyeh96.github.io/2020/03/28/terminal-colors.html
    https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
    https://jvns.ca/blog/2024/10/01/terminal-colours/
 */

use std::{env, fmt};

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Default,
    Black,
    White,
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy)]
pub enum Style {
    Default,
    Bold,
    Italic,
}

/// A struct that allows you to print styled and colored text in the terminal.
/// This example demonstrates how you can use the `Colorful` struct for adding colors and styles.
///
/// # Examples
///
/// Basic usage of the `Colorful` struct for various text styles and colors.
///
/// ```rust
/// use katana::colorful::{Color, Colorful, Style};
///
/// fn main() {
///     let no_style = Colorful::new("no_style");
///
///     // default style (no style, no colors)
///     println!("{}", no_style);
///
///     // red
///     let red_text = Colorful::new("red_text")
///         .foreground(Color::Red);
///     println!("{}", red_text);
///
///     // blue on red
///     let blue_on_red = Colorful::new("blue_on_red")
///         .background(Color::Red)
///         .foreground(Color::Blue);
///     println!("{}", blue_on_red);
///
///     // green on black, bold
///     let hacker_style = Colorful::new("hacker_style")
///         .background(Color::Black)
///         .foreground(Color::Green)
///         .style(Style::Bold);
///     println!("{}", hacker_style);
/// }
/// ```
pub struct Colorful {
    text: String, // itself
    style: Option<Style>,
    foreground: Option<Color>,
    background: Option<Color>,
}

impl Colorful {
    // https://doc.rust-lang.org/rust-by-example/generics/bounds.html
    // https://www.youtube.com/watch?v=t25vayJ8LVg
    pub fn new<T: fmt::Display>(text: T) -> Self {
        Self {
            text: text.to_string(),
            style: None,
            foreground: None,
            background: None,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
    
    pub fn foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
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

    pub fn get_ansi_color(color: Color, is_background: bool) -> String {
        // @todo: add more color later
        let base = if is_background { 40 } else { 30 };
        let code = match color {
            Color::Default => 0,
            Color::Black => base,
            Color::White => base + 7,
            Color::Red => base + 1,
            Color::Green => base + 2,
            Color::Blue => base + 4,
        };

        format!("\x1b[{}m", code)
    }

    pub fn get_ansi_style(style: Style) -> String {
        let code = match style {
            Style::Default => 0,
            Style::Bold => 1,
            Style::Italic => 3,
        };

        format!("\x1b[{}m", code)
    }
}

// More details on https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
impl fmt::Display for Colorful {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !Self::is_colors_supported() {
            return write!(f, "{}", self.text);
        }

        let mut result = String::new();

        if let Some(style) = self.style {
            result.push_str(&Self::get_ansi_style(style));
        }
        if let Some(fg) = self.foreground {
            result.push_str(&Self::get_ansi_color(fg, false));
        }
        if let Some(bg) = self.background {
            result.push_str(&Self::get_ansi_color(bg, true));
        }

        result.push_str(&self.text);
        result.push_str("\x1b[0m"); // reset styles & colors
        write!(f, "{}", result)
    }
}

/// A trait to add colored output capabilities to any type that implements Display.
///
/// # Examples
///
///  ```rust
/// use katana::colorful::Colored;
///
/// fn main() {
///     println!(
///         "{}{}{}.",
///         "Katana Colorful".black().white_background(),
///         ": ".blue().red_background(),
///         "the Hacker Way".green().black_background().bold(),
///     );
/// }
///  ```
pub trait Colored: fmt::Display {
    fn colored(&self) -> Colorful {
        Colorful::new(self)
    }

    fn style(&self, style: Style) -> Colorful {
        self.colored().style(style)
    }

    fn foreground(&self, color: Color) -> Colorful {
        self.colored().foreground(color)
    }

    fn background(&self, color: Color) -> Colorful {
        self.colored().background(color)
    }

    fn black(&self) -> Colorful { self.colored().foreground(Color::Black) }
    fn red(&self) -> Colorful { self.colored().foreground(Color::Red) }
    fn green(&self) -> Colorful { self.colored().foreground(Color::Green) }
    fn blue(&self) -> Colorful { self.colored().foreground(Color::Blue) }
    fn white(&self) -> Colorful { self.colored().foreground(Color::White) }

    fn black_background(&self) -> Colorful { self.colored().background(Color::Black) }
    fn red_background(&self) -> Colorful { self.colored().background(Color::Red) }
    fn green_background(&self) -> Colorful { self.colored().background(Color::Green) }
    fn blue_background(&self) -> Colorful { self.colored().background(Color::Blue) }
    fn white_background(&self) -> Colorful { self.colored().background(Color::White) }

    fn bold(&self) -> Colorful { self.colored().style(Style::Bold) }
    fn italic(&self) -> Colorful { self.colored().style(Style::Italic) }
}

// Implement Colored for all types that implement Display
impl<T: fmt::Display> Colored for T {}
