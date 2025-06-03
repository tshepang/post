use std::{env, fs, process::Command};

use anyhow::Result;
use clap::Parser;
use slug::slugify;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long)]
    movies: bool,
    #[arg(long, num_args = 1.., default_value = "untagged")]
    tags: Vec<String>,
    title: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let dir = env::current_dir()?.join("content");
    let today = jiff::Zoned::now().strftime("%F");
    let output = format!(
        "
+++
title = {:?}
date = {today}

[taxonomies]
tags = {:?}{}
+++
        ",
        cli.title,
        cli.tags,
        if cli.movies {
            "\ncategories = [\"movies\"]"
        } else {
            ""
        }
    );
    let output = output.trim();
    let path = dir.join(slugify(cli.title)).with_extension("md");
    eprintln!("{}", path.display());
    if path.exists() {
        eprintln!("path exists already, opening ...");
    } else {
        fs::write(&path, output)?;
    }
    Command::new("hx").arg(path).status()?;

    Ok(())
}
