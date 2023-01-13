mod dialogue_utils;

use anyhow::Result;
use clap::Parser;
use dialogue_utils::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use ferret_image::{BiologicalInfo, Color, ImageInfo, License, Pattern, Sex};
use seek_bufread::BufReader;
use std::io::{BufRead, Cursor, Seek};
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

fn biologocal_info() -> Result<BiologicalInfo> {
    let name = optional_input("Ferret's name")?;

    let sex = enum_select_default::<Sex>("Ferret's sex", "Unspecified")?;

    let color = enum_select_default::<Color>("Ferret's color", "Unspecified")?;

    let pattern = enum_select_default::<Pattern>("Ferret's pattern", "Unspecified")?;

    let alt = optional_input("Alternative description")?;

    Ok(BiologicalInfo {
        name,
        sex,
        color,
        pattern,
        alt,
    })
}

fn collect_ferret_info() -> Result<ImageInfo> {
    println!("Legal and credit information");

    let is_public_domain = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is the image in the public domain?")
        .interact()?;

    let source = optional_input("Source of image [e.g. URL] (optional, specify if applicable)")?;

    let bio = biologocal_info()?;

    if is_public_domain {
        let author = optional_input("Original author's name (optional)")?;

        Ok(ImageInfo::PublicDomain {
            info: bio,
            author,
            source,
        })
    } else {
        let author: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Original author's name")
            .interact()?;

        let license = enum_select::<License>("License")?;

        Ok(ImageInfo::Licensed {
            info: bio,
            author,
            license,
            source,
        })
    }
}

trait BufReadSeek: BufRead + Seek {}
impl<T: BufRead + Seek> BufReadSeek for T {}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Create { source } => {
            let mut file: Box<dyn BufReadSeek> = if std::path::Path::new(&source).exists() {
                // source is a file
                Box::new(BufReader::new(std::fs::File::open(&source)?))
            } else if reqwest::Url::parse(&source).is_ok() {
                let request = reqwest::blocking::get(&source)?;
                println!("Request code is {}", request.status());
                let bytes = request.bytes()?;
                Box::new(Cursor::new(bytes.to_vec()))
            } else {
                println!("Source must be a file or URL");
                std::process::exit(1)
            };

            let id = Uuid::new_v4();
            println!("UUID is {id}");

            let mut ferret_path = std::path::PathBuf::from("./ferret_images/collection");
            ferret_path.push(id.to_string());
            std::fs::create_dir_all(&ferret_path)?;

            let mut image_path = ferret_path.clone();
            image_path.push("image.jpg");

            // Convert to PNG
            println!("Converting image to JPG (if applicable)");
            let image = image::io::Reader::new(&mut file)
                .with_guessed_format()?
                .decode()?;

            // then save the image to the image file
            println!("Saving...");
            image.save(image_path)?;

            let ferret_info = collect_ferret_info()?;

            // Save the metadata
            println!("Saving metadata...");
            let mut metadata_path = ferret_path.clone();
            metadata_path.push("image.json");
            let mut metadata_file = std::fs::File::create(&metadata_path)?;
            serde_json::to_writer_pretty(&mut metadata_file, &ferret_info)?;

            println!("Done! PR your new changes!");
        }
        Subcommand::Verify => {
            println!("Verify");
        }
    };

    Ok(())
}
