use std::fmt;

#[derive(Clone)]
pub enum Color {
    Red, Green, Blue, Yellow,
}

#[derive(Clone)]
pub enum Style {
    Bold,
    Italic,
    Underline,
}

impl Color {
    pub fn to_ansi_code(&self) -> &str {
        match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Blue => "\x1b[34m",
            Color::Yellow => "\x1b[33m"
        }
    }
}

impl Style {
    pub fn to_ansi_code(&self) -> &str {
        match self {
            Style::Bold => "\x1b[1m",
            Style::Italic => "\x1b[3m",
            Style::Underline => "\x1b[4m"
        }
    }
}

#[derive(Clone)]
pub struct PrintFormatter {
    text: String,
    color: Option<Color>,
    style: Option<Style>
}

impl PrintFormatter {
    pub fn new(text: &str) -> Self {
        PrintFormatter {
            text: text.to_string(),
            color: None,
            style: None
        }
    }

    pub fn color(&mut self, color: Color) -> Self {
        self.color = Some(color);
        self.clone()
    }

    pub fn style(&mut self, style: Style) -> Self {
        self.style = Some(style);
        self.clone()
    }

    pub fn format(&self) -> String {
        let mut formatted_text: String = String::new();

        if let Some(color) = &self.color {
            formatted_text.push_str(color.to_ansi_code());
        }

        if let Some(style) = &self.style {
            formatted_text.push_str(style.to_ansi_code());
        }

        formatted_text.push_str(&self.text);
        formatted_text.push_str("\x1b[0m");

        formatted_text
    }
}

impl fmt::Display for PrintFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

pub trait StringFormat {
    fn color(&mut self, color: Color) -> String;
    fn style(&mut self, style: Style) -> String;
    
    fn bold(&mut self) -> String;

    fn red(&mut self) -> String;
    fn blue(&mut self) -> String;
    fn yellow(&mut self) -> String;
}

impl StringFormat for &str {
    fn style(&mut self, style: Style) -> String {
        PrintFormatter::new(self).style(style).format()
    }
    fn color(&mut self, color: Color) -> String {
        PrintFormatter::new(self).color(color).format()
    }
    
    fn blue(&mut self) -> String {
        self.color(Color::Blue)
    }
    fn red(&mut self) -> String {
        self.color(Color::Red)
    }
    fn yellow(&mut self) -> String {
        self.color(Color::Yellow)
    }
    
    fn bold(&mut self) -> String {
        self.style(Style::Bold)
    }
}

impl StringFormat for String {
    fn color(&mut self, color: Color) -> String {
        let formatted_text = PrintFormatter::new(self).color(color).format();
        self.clear();
        self.push_str(&formatted_text);
        self.clone()
    }
    fn style(&mut self, style: Style) -> String {
        let formatted_text = PrintFormatter::new(self).style(style).format();
        self.clear();
        self.push_str(&formatted_text);
        self.clone()   
    }

    fn bold(&mut self) -> String {
        self.style(Style::Bold);
        self.clone()
    }

    fn blue(&mut self) -> String {
        self.color(Color::Blue);
        self.clone()
    }

    fn red(&mut self) -> String {
        self.color(Color::Red);
        self.clone()
    }

    fn yellow(&mut self) -> String {
        self.color(Color::Yellow);
        self.clone()
    }
}

#[test]
fn test_print_formatter() {
    let mut my_string: String = String::from("Hello, World!");
    my_string.style(Style::Italic);
    my_string.color(Color::Red);

    println!("{}", my_string);
}