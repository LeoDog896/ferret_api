use std::ops::Not;

use anyhow::Result;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use ferret_image::{BiologicalInfo, Color, ImageInfo, License, Pattern, Sex};
use strum::IntoEnumIterator;

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
    Create,
    /// Verifies the /images directory
    Verify,
}

fn enum_select_default<E>(prompt: &str, default: &str) -> Option<E>
where
    E: IntoEnumIterator + std::fmt::Display,
{
    let mut items: Vec<String> = E::iter().map(|e| e.to_string()).collect();
    items.push(default.to_string());
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(items.len() - 1)
        .items(items.as_slice())
        .interact()
        .unwrap();
    if selection == items.len() - 1 {
        None
    } else {
        Some(
            E::iter()
                .nth(selection)
                .expect("Past bounds of enum iterator. Dialoguer may have changed."),
        )
    }
}

fn enum_select<E>(prompt: &str) -> E
where
    E: IntoEnumIterator + std::fmt::Display,
{
    let items: Vec<String> = E::iter().map(|e| e.to_string()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items.as_slice())
        .interact()
        .unwrap();
    E::iter()
        .nth(selection)
        .expect("Past bounds of enum iterator. Dialoguer may have changed.")
}

fn biologocal_info() -> BiologicalInfo {
    let name: Option<String> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Ferret's name")
        .interact()
        .map(|s: String| s.is_empty().not().then(|| s))
        .unwrap();

    let sex = enum_select_default::<Sex>("Ferret's sex", "Unspecified");

    let color = enum_select_default::<Color>("Ferret's color", "Unspecified");

    let pattern = enum_select_default::<Pattern>("Ferret's pattern", "Unspecified");

    BiologicalInfo {
        name,
        sex,
        color,
        pattern,
    }
}

fn collect_ferret_info() -> Result<ImageInfo> {
    println!("Legal and credit information");

    let is_public_domain = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is the image in the public domain?")
        .interact()
        .unwrap();

    let source: Option<String> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Source of image [e.g. URL] (optional, specify if applicable)")
        .interact()
        .map(|s: String| s.is_empty().not().then(|| s))
        .unwrap();

    let bio = biologocal_info();

    if is_public_domain {
        let author: Option<String> = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Original author's name (optional)")
            .interact()
            .map(|s: String| s.is_empty().not().then(|| s))
            .unwrap();

        return Ok(ImageInfo::PublicDomain {
            info: bio,
            author,
            source,
        });
    } else {
        let author: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Original author's name")
            .interact()
            .unwrap();

        let license = enum_select::<License>("License");

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
        Subcommand::Create => {
            let ferret_info = collect_ferret_info()?;
        }
        Subcommand::Verify => {
            println!("Verify");
        }
    };

    Ok(())
}
