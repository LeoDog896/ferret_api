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

/// All creative commons licenses.
/// Referenced from https://creativecommons.org/about/cclicenses/
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum License {
    Attribution,
    AttributionShareAlike,
    AttributionNoDerivatives,
    AttributionNonCommercial,
    AttributionNonCommercialShareAlike,
    AttributionNonCommercialNoDerivatives,
    /// CC0 -- essentially in public domain
    Zero,
}

/// Information about a ferret.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageInfo {
    /// The ferret's name.
    pub name: Option<String>,
    pub sex: Option<Sex>,
    pub color: Option<Color>,
    pub pattern: Option<Pattern>,
    /// The name of the author of the image.
    pub author: Option<String>,
    /// Where the image was retrieved from.
    pub source: Option<String>,
    /// The license of the image.
    pub license: License,
}

#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    #[error("Image is not in the public domain, but doesn't have an author")]
    MissingAuthor,
}

impl ImageInfo {
    pub fn verify(&self) -> Result<(), ImageError> {
        if self.license == License::Zero {
            return Ok(());
        }

        if self.author.is_none() {
            return Err(ImageError::MissingAuthor);
        }

        Ok(())
    }
}

#[test]
fn test_deserialize_json() {
    let json_string = r#"{
        "name": "Fluffy",
        "sex": "Male",
        "color": "Black",
        "pattern": "Solid",
        "author": "Rusty",
        "source": "https://www.example.com",
        "license": "Attribution"
    }"#;

    let ferret_info: ImageInfo = serde_json::from_str(json_string).unwrap();

    assert_eq!(ferret_info.name, Some("Fluffy".to_string()));
    assert_eq!(ferret_info.sex, Some(Sex::Male));
    assert_eq!(ferret_info.color, Some(Color::Black));
    assert_eq!(ferret_info.pattern, Some(Pattern::Solid));
    assert_eq!(ferret_info.author, Some("Rusty".to_string()));
    assert_eq!(ferret_info.source, Some("https://www.example.com".to_string()));
    assert_eq!(ferret_info.license, License::Attribution);
}

#[test]
fn test_unlicensed() {
    let json_string = r#"{ }"#;

    let ferret_info: serde_json::Result<ImageInfo> = serde_json::from_str(json_string);

    assert!(ferret_info.is_err());
}

#[test]
fn test_license_zero() {
    let json_string = r#"{
        "license": "Zero"
    }"#;
    
    let ferret_info: ImageInfo = serde_json::from_str(json_string).unwrap();

    assert_eq!(ferret_info.license, License::Zero);
}

#[test]
fn test_non_public_domain() {
    let json_string = r#"{
        "license": "Attribution"
    }"#;

    let ferret_info: ImageInfo = serde_json::from_str(json_string).unwrap();

    assert_eq!(ferret_info.license, License::Attribution);
    assert!(ferret_info.verify().is_err());
}