use strum_macros::EnumString;


#[derive(Debug, EnumString, PartialEq)]
pub enum DurationType {
    #[strum(serialize = "64th")]
    Sixtyfourth,
    #[strum(serialize = "32nd")]
    Thirtysecond,
    #[strum(serialize = "16th")]
    Sixteenth,
    #[strum(serialize = "eighth")]
    Eighth,
    #[strum(serialize = "quarter")]
    Quarter,
    #[strum(serialize = "half")]
    Half,
    #[strum(serialize = "whole")]
    Whole,
    #[strum(serialize = "breve")]
    Breve,
}

#[derive(Debug)]
pub enum DirectionUD {
    Up,
    Down,
}

#[derive(Debug, EnumString, PartialEq)]
pub enum SyllabicType {
    #[strum(serialize = "begin")]
    Begin,
    #[strum(serialize = "end")]
    End,
    #[strum(serialize = "single")]
    Single,
}

#[derive(Debug, EnumString, PartialEq)]
pub enum Placement {
    #[strum(serialize = "above")]
    Above,
    #[strum(serialize = "below")]
    Below,
}

#[derive(Debug)]
pub enum DirectionType {
    Wedge { wedgetype: WedgeType, number: u8 },
    Dynamic(String),
    Words(String),
    Metronome { beat_unit: String, per_minute: u8 },
}

#[derive(Debug, EnumString)]
pub enum WedgeType {
    #[strum(serialize = "crescendo")]
    Crescendo,
    #[strum(serialize = "diminuendo")]
    Diminuendo,
    #[strum(serialize = "stop")]
    Stop,
}

#[derive(Debug, EnumString)]
pub enum Location {
    #[strum(serialize = "left")]
    Left,
    #[strum(serialize = "right")]
    Right,
}

#[derive(Debug, EnumString)]
pub enum RepeatDirection {
    #[strum(serialize = "forward")]
    Forward,
    #[strum(serialize = "backward")]
    Backward,
}

#[derive(Debug, EnumString)]
pub enum BarStyle {
    #[strum(serialize = "standard")]
    Standard,
    #[strum(serialize = "tick")]
    Tick,
    #[strum(serialize = "light-light")]
    LightLight,
    #[strum(serialize = "heavy-light")]
    HeavyLight,
    #[strum(serialize = "light-heavy")]
    LightHeavy,
}

#[derive(Debug)]
pub struct Pitch {
    pub step: char,
    pub octave: u8,
}

#[derive(Debug)]
pub struct Lyric {
    pub number: u8,
    pub placement: Placement,
    pub syllabic: SyllabicType,
    pub text: String,
    pub extend: bool,
}

#[derive(Debug)]
pub enum NotationType {
    Tied{s:StartStop},
    Slur{s:StartStop, number:u8},
}

#[derive(Debug, EnumString)]
pub enum StartStop {
    #[strum(serialize = "start")]
    Start,
    #[strum(serialize = "stop")]
    Stop,
}