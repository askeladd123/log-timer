use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use std::{env, fs, io::Error, path::Path};

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found");
    let completions_dir = Path::new(&manifest_dir).join("completions");
    fs::create_dir_all(&completions_dir).expect("Failed to create completions directory");

    let mut cmd = Cli::command();
    for shell in Shell::value_variants() {
        generate_to(*shell, &mut cmd, "log", &completions_dir)
            .expect("Failed to generate completion script");
    }

    println!(
        "cargo:warning=Completions for {:?} generated at {:?}.",
        Shell::value_variants(),
        completions_dir
    );
    Ok(())
}
