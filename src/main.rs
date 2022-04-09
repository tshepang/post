use std::{fs, process::Command};

use anyhow::Result;
use clap::Parser;
use dirs_next::home_dir;
use slug::slugify;

#[derive(Parser)]
struct Opt {
    #[clap(long)]
    movies: bool,
    #[clap(long, required = true)]
    tags: Vec<String>,
    title: String,
}

fn main() -> Result<()> {
    let cli = Opt::parse();
    let dir = home_dir().expect("No $HOME!?").join("blog/content");
    let format = time::format_description::parse("[year]-[month]-[day]")?;
    let today = time::OffsetDateTime::now_local()?;
    let today = today.format(&format)?;
    let output = format!(
        "
+++
title = {:?}
date = {}

[taxonomies]
tags = {:?}{}
+++
        ",
        cli.title,
        today,
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
    Command::new("editor").arg(path).status()?;

    Ok(())
}
