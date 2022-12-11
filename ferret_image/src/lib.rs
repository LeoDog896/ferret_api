use serde::{Serialize, Deserialize};

/// Colors of a ferret.
/// Referenced from https://www.ferret.org/pdfs/Ferret_Colors_and_Patterns.pdf
#[derive(Serialize, Deserialize, Debug)]
pub enum Color {
    Albino,
    Black,
    BlackSable,
    Champagne,
    Chocolate,
    Cinnamon,
    DarkEyedWhite,
    Sable,
}

/// Patterns of a ferret.
/// Referenced from https://www.ferret.org/pdfs/Ferret_Colors_and_Patterns.pdf
#[derive(Serialize, Deserialize, Debug)]
pub enum Pattern {
    Blaze,
    Mitt,
    Mutt,
    Panda,
    Point,
    Roan,
    Solid,
    Standard,
    Patterned,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Sex {
    Male,
    Female
}

/// Information about a ferret.
#[derive(Serialize, Deserialize, Debug)]
pub struct FerretInfo {
    pub name: Option<String>,
    pub sex: Option<Sex>,
    pub color: Option<Color>,
    pub pattern: Option<Pattern>,
}
