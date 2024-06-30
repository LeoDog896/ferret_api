mod dialogue_utils;

use anyhow::{bail, Context, Result};
use clap::Parser;
use dialogue_utils::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use ferret_image::{BiologicalInfo, Color, ImageInfo, License, Pattern, Sex};
use seek_bufread::BufReader;
use std::{fs::read_dir, io::{BufRead, Cursor, Seek}, path::Path, process::Command};
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
        source: String,
    },
    /// Verifies the /images directory
    Verify,
    /// Get general statistics on stored images
    Statistics
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
    let collections_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().join("ferret_images").join("collection");
    let args = Args::parse();

    // First, check if the collections directory exists
    if !collections_path.exists() {
        eprintln!("No collections directory found. Did you clone the submodules?");
        std::process::exit(1);
    }

    match args.subcommand {
        Subcommand::Create { source } => {
            // Check if jpegoptim is installed
            println!("Checking if jpegoptim is installed...");
            which::which("jpegoptim")
                .context("jpegoptim is not installed. Please install it to continue.")?;
            
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

            let mut ferret_path = collections_path;
            ferret_path.push(id.to_string());
            std::fs::create_dir_all(&ferret_path)?;

            let mut image_path = ferret_path.clone();
            image_path.push("image.jpg");

            // Convert to PNG
            println!("Converting image to JPG (if applicable)");
            let image = image::io::Reader::new(&mut file)
                .with_guessed_format()?
                .decode()
                .context("Error decoding image")?;

            // then save the image to the image file
            println!("Saving...");
            image.save(&image_path)?;

            let ferret_info = collect_ferret_info()?;

            // Save the metadata
            println!("Saving metadata...");
            let mut metadata_path = ferret_path.clone();
            metadata_path.push("image.json");
            let mut metadata_file = std::fs::File::create(&metadata_path)?;
            serde_json::to_writer_pretty(&mut metadata_file, &ferret_info)?;

            // Run jpegotim on the image
            println!("Optimizing image...");
            let output = Command::new("jpegoptim")
                .arg("--strip-all")
                .arg("--all-progressive")
                .arg(&image_path)
                .output()?;
            
            if !output.status.success() {
                std::fs::remove_dir_all(&ferret_path)?;
                bail!("Error optimizing image: {}", String::from_utf8_lossy(&output.stderr));
            }

            // Check the file size -- warn if over 300k
            let file_size = std::fs::metadata(&image_path)?.len();
            if file_size > 1024 * 300 {
                eprintln!("WARNING: File size is over 300k ({} bytes). You may need to resize your images.", file_size);
            } else {
                println!("File size is {}kb", file_size / 1024);
            }

            println!("Done! PR your new changes!");
        }
        Subcommand::Verify => {
            // Loop through all the directories
            for entry in read_dir(collections_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    // Check if the image exists
                    let mut image_path = path.clone();
                    image_path.push("image.jpg");
                    if !image_path.exists() {
                        eprintln!("Image does not exist for {}", path.display());
                        std::process::exit(1);
                    }

                    // Check if the metadata exists
                    let mut metadata_path = path.clone();
                    metadata_path.push("image.json");
                    if !metadata_path.exists() {
                        eprintln!("Metadata does not exist for {}", path.display());
                        std::process::exit(1);
                    }

                    // Check if the metadata is an ImageInfo struct
                    let metadata = std::fs::read_to_string(&metadata_path)?;
                    let is_valid = serde_json::from_str::<ImageInfo>(&metadata);
                    if is_valid.is_err() {
                        eprintln!("Metadata is not valid for {}", path.display());
                        std::process::exit(1);
                    }

                    // Check if the image is a valid JPEG
                    let image = image::io::Reader::open(&image_path)?
                        .with_guessed_format()?
                        .decode()?;
                    if image.color() != image::ColorType::Rgb8 {
                        eprintln!("Image is not RGB8 for {}", path.display());
                        std::process::exit(1);
                    }
                }

                println!("Verified {}", path.display());
            }

            println!("All images verified!");
        },
        Subcommand::Statistics => {
            let count = read_dir(collections_path)?.count();

            println!("Image count: {}", count);
        }
    };

    Ok(())
}
