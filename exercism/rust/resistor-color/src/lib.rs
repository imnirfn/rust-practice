use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Copy, Eq, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => format!("{:?}", color),
        _ => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut c = ResistorColor::into_enum_iter().collect::<Vec<ResistorColor>>();
    c.sort();
    c
    // ResistorColor::into_enum_iter().collect()
}
