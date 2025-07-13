use lazy_static::lazy_static;
use crate::models::colors::color::Color;

pub struct Red {
    pub s_50: &'static Color,
    pub s_100: &'static Color,
    pub s_200: &'static Color,
    pub s_300: &'static Color,
    pub s_400: &'static Color,
    pub s_500: &'static Color,
    pub s_600: &'static Color,
    pub s_700: &'static Color,
    pub s_800: &'static Color,
    pub s_900: &'static Color,
}

lazy_static! {
    static ref RED_50: Color = Color::rgb(254, 242, 242);
    static ref RED_100: Color = Color::rgb(254, 226, 226);
    static ref RED_200: Color = Color::rgb(252, 165, 165);
    static ref RED_300: Color = Color::rgb(248, 113, 113);
    static ref RED_400: Color = Color::rgb(239, 68, 68);
    static ref RED_500: Color = Color::rgb(239, 68, 68);
    static ref RED_600: Color = Color::rgb(220, 38, 38);
    static ref RED_700: Color = Color::rgb(185, 28, 28);
    static ref RED_800: Color = Color::rgb(153, 27, 27);
    static ref RED_900: Color = Color::rgb(127, 29, 29);
}

impl Red {
    pub fn new() -> Self {
        Red {
            s_50: &RED_50,
            s_100: &RED_100,
            s_200: &RED_200,
            s_300: &RED_300,
            s_400: &RED_400,
            s_500: &RED_500,
            s_600: &RED_600,
            s_700: &RED_700,
            s_800: &RED_800,
            s_900: &RED_900,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_red_colors() {
        let red = Red::new();
        assert_eq!(red.s_50, &Color::rgb(254, 242, 242));
        assert_eq!(red.s_100, &Color::rgb(254, 226, 226));
        assert_eq!(red.s_200, &Color::rgb(252, 165, 165));
        assert_eq!(red.s_300, &Color::rgb(248, 113, 113));
        assert_eq!(red.s_400, &Color::rgb(239, 68, 68));
        assert_eq!(red.s_500, &Color::rgb(239, 68, 68));
        assert_eq!(red.s_600, &Color::rgb(220, 38, 38));
        assert_eq!(red.s_700, &Color::rgb(185, 28, 28));
        assert_eq!(red.s_800, &Color::rgb(153, 27, 27));
        assert_eq!(red.s_900, &Color::rgb(127, 29, 29));
    }
}