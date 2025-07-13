use crate::models::colors::color::Color;

pub enum Red {
    _50,
    _100,
    _200,
    _300,
    _400,
    _500,
    _600,
    _700,
    _800,
    _900,
}

impl Red {
    pub fn to_color(&self) -> Color {
        match self {
            Red::_50 => Color::rgb(255, 235, 238),
            Red::_100 => Color::rgb(255, 205, 210),
            Red::_200 => Color::rgb(239, 154, 154),
            Red::_300 => Color::rgb(229, 115, 115),
            Red::_400 => Color::rgb(239, 83, 80),
            Red::_500 => Color::rgb(244, 67, 54),
            Red::_600 => Color::rgb(229, 57, 53),
            Red::_700 => Color::rgb(211, 47, 47),
            Red::_800 => Color::rgb(198, 40, 40),
            Red::_900 => Color::rgb(183, 28, 28),
        }
    }
}
