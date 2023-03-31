use strum_macros::EnumString;

#[derive(Debug)]
pub enum DirectionUD {
    Up,
    Down,
}

#[derive(Debug, EnumString)]
pub enum SyllabicType {
    #[strum(serialize = "begin")]
    Begin,
    #[strum(serialize = "end")]
    End,
    #[strum(serialize = "single")]
    Single,
}

#[derive(Debug, EnumString)]
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
