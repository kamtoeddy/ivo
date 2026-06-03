use std::fmt;

pub enum TextStyle {
    ColorBlue,
    // ColorGreen,
    // ColorYellow,
    ColorRed,
    // ColorInversed,
    FontBold,
    // FontUnderlined,
    Reset,
}

impl TextStyle {
    pub fn to_ansi(&self) -> &'static str {
        match self {
            Self::ColorRed => "\x1b[31m",
            // Self::ColorGreen => "\x1b[32m",
            // Self::ColorYellow => "\x1b[33m",
            Self::ColorBlue => "\x1b[34m",
            Self::FontBold => "\x1b[1m",
            // Self::FontUnderlined => "\x1b[4m",
            // Self::ColorInversed => "\x1b[7m",
            Self::Reset => "\x1b[0m",
        }
    }
}

pub struct Styled<'a, T: fmt::Display> {
    style: TextStyle,
    text: &'a T,
}

impl<'a, T: fmt::Display> fmt::Display for Styled<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.style.to_ansi(),
            self.text,
            TextStyle::Reset.to_ansi()
        )
    }
}

pub trait Stylable: fmt::Display + Sized {
    fn colored(&self, color: TextStyle) -> Styled<'_, Self> {
        Styled {
            style: color,
            text: self,
        }
    }

    fn colored_blue(&self) -> Styled<'_, Self> {
        self.colored(TextStyle::ColorBlue)
    }

    // fn colored_green(&self) -> Styled<'_, Self> {
    //     self.colored(TextStyle::ColorGreen)
    // }

    // fn colored_yellow(&self) -> Styled<'_, Self> {
    //     self.colored(TextStyle::ColorYellow)
    // }

    fn colored_red(&self) -> Styled<'_, Self> {
        self.colored(TextStyle::ColorRed)
    }

    fn font_bold(&self) -> Styled<'_, Self> {
        self.colored(TextStyle::FontBold)
    }

    // fn font_underlined(&self) -> Styled<'_, Self> {
    //     self.colored(TextStyle::FontUnderlined)
    // }
}

impl<T: fmt::Display> Stylable for T {}
