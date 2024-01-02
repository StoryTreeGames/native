#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseButton {
    Control = 0x08,
    Shift = 0x04,
    Left = 0x01,
    Right = 0x02,
    Middle = 0x10,
    X1 = 0x20,
    X2 = 0x40,
}

#[derive(Debug, Clone)]
pub enum MouseEventType {
    Ignore,
    Down(MouseButton),
    Up(MouseButton),
    Double(MouseButton),
    Move,
    Hover,
    Scroll(i16),
    HScroll(i16),
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub x: u16,
    pub y: u16,
    pub etype: MouseEventType,
}