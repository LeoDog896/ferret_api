use std::{ops::Not, io::Read};

use anyhow::Result;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use ferret_image::{BiologicalInfo, Color, ImageInfo, License, Pattern, Sex};
use strum::IntoEnumIterator;
use uuid::Uuid;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser, Debug)]
enum Subcommand {
    /// Creates a ferret image
    Create {
        /// The source image file / URL
        #[clap(short, long)]
        source: String,
    },
    /// Verifies the /images directory
    Verify,
}

fn enum_select_default<E>(prompt: &str, default: &str) -> Result<Option<E>>
where
    E: IntoEnumIterator + std::fmt::Display,
{
    let mut items: Vec<String> = E::iter().map(|e| e.to_string()).collect();
    items.push(default.to_string());
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(items.len() - 1)
        .items(items.as_slice())
        .interact()?;
    if selection == items.len() - 1 {
        Ok(None)
    } else {
        Ok(Some(E::iter().nth(selection).expect(
            "Past bounds of enum iterator. Dialoguer may have changed.",
        )))
    }
}

fn enum_select<E>(prompt: &str) -> Result<E>
where
    E: IntoEnumIterator + std::fmt::Display,
{
    let items: Vec<String> = E::iter().map(|e| e.to_string()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items.as_slice())
        .interact()?;

    Ok(E::iter()
        .nth(selection)
        .expect("Past bounds of enum iterator. Dialoguer may have changed."))
}

fn biologocal_info() -> Result<BiologicalInfo> {
    let name: Option<String> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Ferret's name")
        .interact()
        .map(|s: String| s.is_empty().not().then(|| s))?;

    let sex = enum_select_default::<Sex>("Ferret's sex", "Unspecified")?;

    let color = enum_select_default::<Color>("Ferret's color", "Unspecified")?;

    let pattern = enum_select_default::<Pattern>("Ferret's pattern", "Unspecified")?;

    Ok(BiologicalInfo {
        name,
        sex,
        color,
        pattern,
    })
}

fn collect_ferret_info() -> Result<ImageInfo> {
    println!("Legal and credit information");

    let is_public_domain = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is the image in the public domain?")
        .interact()?;

    let source: Option<String> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Source of image [e.g. URL] (optional, specify if applicable)")
        .interact()
        .map(|s: String| s.is_empty().not().then(|| s))?;

    let bio = biologocal_info()?;

    if is_public_domain {
        let author: Option<String> = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Original author's name (optional)")
            .interact()
            .map(|s: String| s.is_empty().not().then(|| s))?;

        return Ok(ImageInfo::PublicDomain {
            info: bio,
            author,
            source,
        });
    } else {
        let author: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Original author's name")
            .interact()?;

        let license = enum_select::<License>("License")?;

        return Ok(ImageInfo::Licensed {
            info: bio,
            author,
            license,
            source,
        });
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Create { source } => { 
            let mut file: Box<dyn Read> = if std::path::Path::new(&source).exists() {
                // source is a file
                Box::new(std::fs::File::open(&source)?)
            } else if reqwest::Url::parse(&source).is_ok() {
                // source is a URL, download it
                Box::new(reqwest::blocking::get(&source)?)
            } else {
                println!("Source must be a file or URL");
                std::process::exit(1)
            };

            let ferret_info = collect_ferret_info()?;

            let id = Uuid::new_v4();

            let mut ferret_path = std::path::PathBuf::from("./images");
            ferret_path.push(id.to_string());
            std::fs::create_dir_all(ferret_path.parent().unwrap())?;

            // Save the image
            let mut image_path = ferret_path.clone();
            image_path.push("image.png");
            let mut image_file = std::fs::File::create(&image_path)?;
            std::io::copy(&mut file, &mut image_file)?;

            // Save the metadata
            let mut metadata_path = ferret_path.clone();
            metadata_path.push("image.json");
            let mut metadata_file = std::fs::File::create(&metadata_path)?;
            serde_json::to_writer_pretty(&mut metadata_file, &ferret_info)?;

        }
        Subcommand::Verify => {
            println!("Verify");
        }
    };

    Ok(())
}
