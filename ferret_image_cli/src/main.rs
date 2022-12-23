use std::ops::Not;

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use ferret_image::{ImageInfo, Sex, Color, Pattern, License};
use anyhow::Result;

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
    Verify
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Create => {
            let name: Option<String> = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Ferret's name")
                .interact()
                .map(|s: String| s.is_empty().not().then(|| s))
                .unwrap();
            
            let sex: usize = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Ferret's sex")
                .default(2)
                .items(&["Male", "Female", "Prefer not to say"])
                .interact()
                .unwrap();
            
            let sex = match sex {
                0 => Some(Sex::Male),
                1 => Some(Sex::Female),
                2 => None,
                _ => unreachable!()
            };

        },
        Subcommand::Verify => {
            println!("Verify");
        }
    };

    Ok(())
}
