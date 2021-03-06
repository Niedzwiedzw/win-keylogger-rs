use crate::get_mouse_pos;

pub type Position = (i32, i32);

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Key {
    Char(char),
    Unknown(u8),

    LButton(Position),
    RButton(Position),
    MButton(Position),
    XButton1(Position),
    XButton2(Position),

    Cancel,
    Back,
    Tab,
    Clear,
    Return,
    Shift,
    Control,
    Menu,
    Pause,
    Capital,
    Kana,  //Kana,Hanguel,Hangul
    Junja,
    Final,
    Hanja,  //Hanja,Kanji
    Escape,
    Convert,
    Nonconvert,
    Accept,
    Modechange,
    Space,
    Prior,
    Next,
    End,
    Home,
    Left,
    Up,
    Right,
    Down,
    Select,
    Print,
    Execute,
    Snapshot,
    Insert,
    Delete,
    Help,
    Lwin,
    Rwin,
    Apps,
    Sleep,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Multiply,
    Add,
    Separator,
    Subtract,
    Decimal,
    Divide,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Numlock,
    Scroll,
    Lshift,
    Rshift,
    Lcontrol,
    Rcontrol,
    Lmenu,
    Rmenu,
    BrowserBack,
    BrowserForward,
    BrowserRefresh,
    BrowserStop,
    BrowserSearch,
    BrowserFavorites,
    BrowserHome,
    VolumeMute,
    VolumeDown,
    VolumeUp,
    MediaNextTrack,
    MediaPrevTrack,
    MediaStop,
    MediaPlayPause,
    LaunchMail,
    LaunchMediaSelect,
    LaunchApp1,
    LaunchApp2,
    Oem1,
    OemPlus,
    OemComma,
    OemMinus,
    OemPeriod,
    Oem2,
    Oem3,
    Oem4,
    Oem5,
    Oem6,
    Oem7,
    Oem8,
    Oem102,
    Processkey,
    Attn,
    Crsel,
    Exsel,
    Ereof,
    Play,
    Zoom,
    Noname,
    Pa1,
    OemClear,
}

impl Key {
    pub fn from<T>(code: T) -> Self
    where T: Into<u8> + Into<char>
    {
        let code: u8 = code.into();
        println!("{}", code);
        match code {
            65 ... 90 => Key::Char(code.into()),

            0x01 => Key::LButton(get_mouse_pos()),
            0x02 => Key::RButton(get_mouse_pos()),
            0x03 => Key::Cancel,
            0x04 => Key::MButton(get_mouse_pos()),
            0x05 => Key::XButton1(get_mouse_pos()),
            0x06 => Key::XButton2(get_mouse_pos()),
            0x08 => Key::Back,
            0x09 => Key::Tab,
            0x0C => Key::Clear,
            0x0D => Key::Return,
            0x10 => Key::Shift,
            0x11 => Key::Control,
            0x12 => Key::Menu,
            0x13 => Key::Pause,
            0x14 => Key::Capital,
            0x15 => Key::Kana,
            0x17 => Key::Junja,
            0x18 => Key::Final,

            0x19 => Key::Hanja,
            0x1B => Key::Escape,
            0x1C => Key::Convert,
            0x1D => Key::Nonconvert,
            0x1E => Key::Accept,
            0x1F => Key::Modechange,
            0x20 => Key::Space,
            0x21 => Key::Prior,
            0x22 => Key::Next,
            0x23 => Key::End,
            0x24 => Key::Home,
            0x25 => Key::Left,
            0x26 => Key::Up,
            0x27 => Key::Right,
            0x28 => Key::Down,
            0x29 => Key::Select,
            0x2A => Key::Print,
            0x2B => Key::Execute,
            0x2C => Key::Snapshot,
            0x2D => Key::Insert,
            0x2E => Key::Delete,
            0x2F => Key::Help,
            0x5B => Key::Lwin,
            0x5C => Key::Rwin,
            0x5D => Key::Apps,
            0x5F => Key::Sleep,
            0x60 => Key::Numpad0,
            0x61 => Key::Numpad1,
            0x62 => Key::Numpad2,
            0x63 => Key::Numpad3,
            0x64 => Key::Numpad4,
            0x65 => Key::Numpad5,
            0x66 => Key::Numpad6,
            0x67 => Key::Numpad7,
            0x68 => Key::Numpad8,
            0x69 => Key::Numpad9,
            0x6A => Key::Multiply,
            0x6B => Key::Add,
            0x6C => Key::Separator,
            0x6D => Key::Subtract,
            0x6E => Key::Decimal,
            0x6F => Key::Divide,
            0x70 => Key::F1,
            0x71 => Key::F2,
            0x72 => Key::F3,
            0x73 => Key::F4,
            0x74 => Key::F5,
            0x75 => Key::F6,
            0x76 => Key::F7,
            0x77 => Key::F8,
            0x78 => Key::F9,
            0x79 => Key::F10,
            0x7A => Key::F11,
            0x7B => Key::F12,
            0x7C => Key::F13,
            0x7D => Key::F14,
            0x7E => Key::F15,
            0x7F => Key::F16,
            0x80 => Key::F17,
            0x81 => Key::F18,
            0x82 => Key::F19,
            0x83 => Key::F20,
            0x84 => Key::F21,
            0x85 => Key::F22,
            0x86 => Key::F23,
            0x87 => Key::F24,
            0x90 => Key::Numlock,
            0x91 => Key::Scroll,
            0xa0 => Key::Lshift,
            0xa1 => Key::Rshift,
            0xa2 => Key::Lcontrol,
            0xa3 => Key::Rcontrol,
            0xa4 => Key::Lmenu,
            0xa5 => Key::Rmenu,
            0xa6 => Key::BrowserBack,
            0xa7 => Key::BrowserForward,
            0xa8 => Key::BrowserRefresh,
            0xa9 => Key::BrowserStop,
            0xaa => Key::BrowserSearch,
            0xab => Key::BrowserFavorites,
            0xac => Key::BrowserHome,
            0xad => Key::VolumeMute,
            0xae => Key::VolumeDown,
            0xaf => Key::VolumeUp,
            0xb0 => Key::MediaNextTrack,
            0xb1 => Key::MediaPrevTrack,
            0xb2 => Key::MediaStop,
            0xb3 => Key::MediaPlayPause,
            0xb4 => Key::LaunchMail,
            0xb5 => Key::LaunchMediaSelect,
            0xb6 => Key::LaunchApp1,
            0xb7 => Key::LaunchApp2,
            0xba => Key::Oem1,
            0xbb => Key::OemPlus,
            0xbc => Key::OemComma,
            0xbd => Key::OemMinus,
            0xbe => Key::OemPeriod,
            0xbf => Key::Oem2,
            0xc0 => Key::Oem3,
            0xdb => Key::Oem4,
            0xdc => Key::Oem5,
            0xdd => Key::Oem6,
            0xde => Key::Oem7,
            0xdf => Key::Oem8,
            0xe2 => Key::Oem102,
            0xe5 => Key::Processkey,
            0xf6 => Key::Attn,
            0xf7 => Key::Crsel,
            0xf8 => Key::Exsel,
            0xf9 => Key::Ereof,
            0xfa => Key::Play,
            0xfb => Key::Zoom,
            0xfc => Key::Noname,
            0xfd => Key::Pa1,
            0xfe => Key::OemClear,

            _ => Key::Unknown(code.into())
        }
    }
}
