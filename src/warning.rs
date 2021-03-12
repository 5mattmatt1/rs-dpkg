pub struct Warning
{
    message: String
}

impl Warning
{
    pub fn new(message: &str) -> Self
    {
        Self {
            message: message.to_string()
        }
    }
}

impl std::fmt::Display for Warning
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        use termion::color::{Fg, Bg, Rgb, Reset};
        let bright_white = Rgb(255, 255, 255).fg_string();
        let bright_yellow = Rgb(255, 255, 0).fg_string();
        let grey = Rgb(128, 128, 128).fg_string();
        write!(formatter, "{}dpkg: {}warning:{} {}", 
                bright_white,
                bright_yellow, grey, 
                self.message)
    }
}