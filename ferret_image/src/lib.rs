use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

/// Colors of a ferret.
/// Referenced from https://www.ferret.org/pdfs/Ferret_Colors_and_Patterns.pdf
#[derive(Serialize, Deserialize, Debug, PartialEq, EnumIter, Display)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, EnumIter, Display)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, EnumIter, Display)]
pub enum Sex {
    Male,
    Female,
}

/// All creative commons licenses.
/// Referenced from https://creativecommons.org/about/cclicenses/
#[derive(Serialize, Deserialize, Debug, PartialEq, EnumIter, Display)]
pub enum License {
    Attribution,
    AttributionShareAlike,
    AttributionNoDerivatives,
    AttributionNonCommercial,
    AttributionNonCommercialShareAlike,
    AttributionNonCommercialNoDerivatives,
}

/// Information about a ferret.
#[derive(Serialize, Deserialize, Debug)]
pub struct BiologicalInfo {
    /// The ferret's name.
    pub name: Option<String>,
    pub sex: Option<Sex>,
    pub color: Option<Color>,
    pub pattern: Option<Pattern>,
    /// The alternative description for the image.
    pub alt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ImageInfo {
    PublicDomain {
        info: BiologicalInfo,
        /// The name of the author of the image.
        author: Option<String>,
        /// Where the image was retrieved from.
        source: Option<String>,
    },
    Licensed {
        info: BiologicalInfo,
        /// The name of the author of the image.
        author: String,
        /// Where the image was retrieved from.
        source: Option<String>,
        /// The license of the image.
        license: License,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_licensed() {
        let json_string = r#"{
            "info": {
                "name": "Fluffy",
                "sex": "Male",
                "color": "Black",
                "pattern": "Solid"
            },
            "author": "Rusty",
            "source": "https://www.example.com",
            "license": "Attribution"
        }"#;

        serde_json::from_str::<ImageInfo>(json_string).unwrap();
    }

    #[test]
    fn test_bad_licensed() {
        let json_string = r#"{
            "info": {
                "name": "Fluffy",
                "sex": "Male",
                "color": "Black",
                "pattern": "Solid"
            },
        }"#;

        assert!(serde_json::from_str::<ImageInfo>(json_string).is_err());
    }
}
