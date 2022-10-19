use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::process::exit;

mod algorithms;

fn main() -> std::io::Result<()> {
    let filepath = std::env::args().nth(1);
    if filepath == None {
        exit(1);
    }
    let filepath = filepath.as_deref().unwrap();

    // The available hash algorithms
    let hash_algorithms = vec!["SHA-256", "MD5"];

    // Makes a selection menu in the terminal with the available algs
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&hash_algorithms)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            if hash_algorithms[index] == "SHA-256" {
                println!(
                    "The SHA-256 checksum for the file is: {}",
                    algorithms::calc_sha256(filepath)
                );
            } else if hash_algorithms[index] == "MD5" {
                println!(
                    "The MD5 checksum for the file is: {}",
                    algorithms::calc_md5(filepath)
                );
            }
        }
        None => println!("Aborted: You did not select an algorithm"),
    }

    Ok(())
}
