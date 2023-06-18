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

#[derive(Debug, EnumString, PartialEq, Clone)]
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
    Rehersal { text: String },
    Coda,
    Segno,
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
    pub step: Step,
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
    Tied { s: StartStop },
    Slur { s: StartStop, number: u8 },
    Articulations(ArticulationType, Option<Placement>),
}

#[derive(Debug, EnumString, PartialEq)]
pub enum Step {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}
#[derive(Debug, EnumString)]
pub enum StartStop {
    #[strum(serialize = "start")]
    Start,
    #[strum(serialize = "stop")]
    Stop,
}

#[derive(Debug)]
pub enum HarmonyItem {
    Root { step: Step, alter: Option<u8> },
    Kind { text: String, kind: HarmonyKind },
    Bass { step: String, alter: Option<u8> },
}

#[derive(Debug, EnumString)]
pub enum HarmonyKind {
    #[strum(serialize = "major")]
    Major,
    #[strum(serialize = "minor")]
    Minor,
    #[strum(serialize = "augmented")]
    Augmented,
    #[strum(serialize = "diminished")]
    Diminished,
    #[strum(serialize = "dominant")]
    Dominant,
    #[strum(serialize = "major-seventh")]
    MajorSeventh,
    #[strum(serialize = "minor-seventh")]
    MinorSeventh,
    #[strum(serialize = "diminished-seventh")]
    DiminishedSeventh,
    #[strum(serialize = "half-diminished-seventh")]
    HalfDiminishedSeventh,
    #[strum(serialize = "augmented-seventh")]
    AugmentedSeventh,
    #[strum(serialize = "major-minor-seventh")]
    MajorMinorSeventh,
    #[strum(serialize = "dominant-ninth")]
    DominantNinth,
    #[strum(serialize = "major-ninth")]
    MajorNinth,
    #[strum(serialize = "minor-ninth")]
    MinorNinth,
    #[strum(serialize = "dominant-11th")]
    Dominant11th,
    #[strum(serialize = "major-11th")]
    Major11th,
    #[strum(serialize = "minor-11th")]
    Minor11th,
    #[strum(serialize = "dominant-13th")]
    Dominant13th,
    #[strum(serialize = "major-13th")]
    Major13th,
    #[strum(serialize = "minor-13th")]
    Minor13th,
    #[strum(serialize = "suspended-second")]
    SuspendedSecond,
    #[strum(serialize = "suspended-fourth")]
    SuspendedFourth,
    #[strum(serialize = "neapolitan")]
    Neapolitan,
    #[strum(serialize = "italian")]
    Italian,
    #[strum(serialize = "french")]
    French,
    #[strum(serialize = "german")]
    German,
    #[strum(serialize = "pedal")]
    Pedal,
    #[strum(serialize = "power")]
    Power,
    #[strum(serialize = "tristan")]
    Tristan,
    #[strum(serialize = "other")]
    Other,
}

#[derive(Debug, EnumString, PartialEq, Clone)]
pub enum ArticulationType {
    #[strum(serialize = "staccato")]
    Staccato,
    #[strum(serialize = "tenuto")]
    Tenuto,
    #[strum(serialize = "accent")]
    Accent,
    #[strum(serialize = "strong-accent")]
    StrongAccent,
    #[strum(serialize = "detached-legato")]
    DetachedLegato,
}
#[derive(Debug, PartialEq)]
pub struct Articulation(pub ArticulationType, pub Option<Placement>, pub String);
