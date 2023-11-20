use std::fmt;

pub struct Pattern {
    // index: u8,
    rows: Vec<Option<Row>>,
}

impl Pattern {
    pub fn new(length: usize) -> Self {
        Pattern {
            rows: vec![None; length],
        }
    }

    pub fn add_row(&mut self, index: usize, row: Row) {
        if index < self.rows.len() {
            self.rows[index] = Some(row);
        }
    }

    pub fn remove_row(&mut self, index: usize) {
        if index < self.rows.len() {
            self.rows[index] = None;
        }
    }

    // You can also have methods to remove a row, update a row, etc.
}

#[derive(Clone)]
pub struct Row {
    channel: Option<u8>,
    note: Option<Note>,
    velocity: Option<u8>,
    effects: Vec<Effect>,
}

impl Row {
    pub fn new(
        channel: Option<u8>,
        note: Option<Note>,
        velocity: Option<u8>,
        effects: Vec<Effect>,
    ) -> Self {
        Row {
            channel,
            note,
            velocity,
            effects,
        }
    }
}

// a note has a midi value and string value that includes octave
#[derive(Copy, Clone)]
pub struct Note {
    // example "C#4"
    note_name: NoteName,
    octave: u8,
    midi: u8,
}

impl Note {
    // new needs to take just note_text and octave and translate that to midi
    pub fn new(note_name: NoteName, octave: u8) -> Self {
        Note {
            note_name,
            octave,
            midi: Self::map_note_to_midi(note_name, octave),
        }
    }
    //function to map note string and octave to midi value using enum of notes
    fn map_note_to_midi(note_name: NoteName, octave: u8) -> u8 {
        let base_midi = match note_name {
            NoteName::C => 0,
            NoteName::CSharp => 1,
            NoteName::D => 2,
            NoteName::DSharp => 3,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::FSharp => 6,
            NoteName::G => 7,
            NoteName::GSharp => 8,
            NoteName::A => 9,
            NoteName::ASharp => 10,
            NoteName::B => 11,
        };
        base_midi + (octave * 12)
    }
}
#[derive(Copy, Clone)]
pub enum NoteName {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl fmt::Display for NoteName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NoteName::C => "C",
                NoteName::CSharp => "C#",
                NoteName::D => "D",
                NoteName::DSharp => "D#",
                NoteName::E => "E",
                NoteName::F => "F",
                NoteName::FSharp => "F#",
                NoteName::G => "G",
                NoteName::GSharp => "G#",
                NoteName::A => "A",
                NoteName::ASharp => "A#",
                NoteName::B => "B",
            }
        )
    }
}

impl NoteName {
    fn from_str(note_str: &str) -> Option<NoteName> {
        match note_str {
            "C" => Some(NoteName::C),
            "C#" => Some(NoteName::CSharp),
            "D" => Some(NoteName::D),
            "D#" => Some(NoteName::DSharp),
            "E" => Some(NoteName::E),
            "F" => Some(NoteName::F),
            "F#" => Some(NoteName::FSharp),
            "G" => Some(NoteName::G),
            "G#" => Some(NoteName::GSharp),
            "A" => Some(NoteName::A),
            "A#" => Some(NoteName::ASharp),
            "B" => Some(NoteName::B),
            _ => None,
        }
    }

    fn next(&self) -> NoteName {
        match self {
            NoteName::C => NoteName::CSharp,
            NoteName::CSharp => NoteName::D,
            NoteName::D => NoteName::DSharp,
            NoteName::DSharp => NoteName::E,
            NoteName::E => NoteName::F,
            NoteName::F => NoteName::FSharp,
            NoteName::FSharp => NoteName::G,
            NoteName::G => NoteName::GSharp,
            NoteName::GSharp => NoteName::A,
            NoteName::A => NoteName::ASharp,
            NoteName::ASharp => NoteName::B,
            NoteName::B => NoteName::C,
        }
    }

    fn prev(&self) -> NoteName {
        match self {
            NoteName::C => NoteName::B,
            NoteName::CSharp => NoteName::C,
            NoteName::D => NoteName::CSharp,
            NoteName::DSharp => NoteName::D,
            NoteName::E => NoteName::DSharp,
            NoteName::F => NoteName::E,
            NoteName::FSharp => NoteName::F,
            NoteName::G => NoteName::FSharp,
            NoteName::GSharp => NoteName::G,
            NoteName::A => NoteName::GSharp,
            NoteName::ASharp => NoteName::A,
            NoteName::B => NoteName::ASharp,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Effect {}
