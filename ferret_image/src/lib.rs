pub enum FerretColor {
    Albino,
    Black,
    BlackSable,
    Champagne,
    Chocolate,
    Cinnamon,
    DarkEyedWhite,
    Sable
}

pub enum FerretPattern {
    Blaze,
    Mitt,
    Mutt,
    Panda,
    Point,
    Roan,
    Solid,
    Standard,
    Patterned
}

pub struct FerretInfo {
    pub name: Option<String>,
    pub color: Option<FerretColor>,
    pub pattern: Option<FerretPattern>,
}