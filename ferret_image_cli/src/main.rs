use anyhow::Result;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use ferret_image::{BiologicalInfo, Color, ImageInfo, License, Pattern, Sex};
use seek_bufread::BufReader;
use std::io::{BufRead, Cursor, Seek};
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
    let name = optional_input("Ferret's name")?;

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

fn optional_input(prompt: &str) -> Result<Option<String>> {
    Ok(Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default("None_".into())
        .interact()
        .map(|s: String| (s == "None_".to_string()).then(|| s))?)
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
                let bytes = reqwest::blocking::get(&source)?.bytes()?;
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
