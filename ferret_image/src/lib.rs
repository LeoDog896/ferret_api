use serde::{Serialize, Deserialize};

/// Colors of a ferret.
/// Referenced from https://www.ferret.org/pdfs/Ferret_Colors_and_Patterns.pdf
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

// write a test for the FerretInfo struct (relating to serde)

#[test]
fn test_deserialize_json() {
    let json_string = r#"{
        "name": "Fluffy",
        "sex": "Male",
        "color": "Black",
        "pattern": "Solid"
    }"#;

    let ferret_info: FerretInfo = serde_json::from_str(json_string).unwrap();

    assert_eq!(ferret_info.name, Some("Fluffy".to_string()));
    assert_eq!(ferret_info.sex, Some(Sex::Male));
    assert_eq!(ferret_info.color, Some(Color::Black));
    assert_eq!(ferret_info.pattern, Some(Pattern::Solid));
}