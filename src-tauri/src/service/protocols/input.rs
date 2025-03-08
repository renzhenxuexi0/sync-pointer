use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

impl KeyModifiers {
    pub fn none() -> Self {
        Self { shift: false, ctrl: false, alt: false, logo: false }
    }

    pub fn new(shift: bool, ctrl: bool, alt: bool, logo: bool) -> Self {
        Self { shift, ctrl, alt, logo }
    }
}

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Keyboard {
    KeyPress { key_code: u32, modifiers: KeyModifiers, text: Option<char> },
    KeyRelease { key_code: u32, modifiers: KeyModifiers },
}

impl Keyboard {
    pub fn press(
        key_code: u32,
        modifiers: KeyModifiers,
        text: Option<char>,
    ) -> Self {
        Self::KeyPress { key_code, modifiers, text }
    }

    pub fn release(key_code: u32, modifiers: KeyModifiers) -> Self {
        Self::KeyRelease { key_code, modifiers }
    }
}

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Mouse {
    Move { x: f32, y: f32 },
    Button { button: MouseButton, pressed: bool },
    Scroll(f32),
}

impl Mouse {
    pub fn move_to(x: f32, y: f32) -> Self {
        Self::Move { x, y }
    }

    pub fn button(button: MouseButton, pressed: bool) -> Self {
        Self::Button { button, pressed }
    }

    pub fn scroll(amount: f32) -> Self {
        Self::Scroll(amount)
    }
}
