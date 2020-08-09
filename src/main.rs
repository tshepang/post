use dirs::home_dir;
use slug::slugify;
use std::{fs, io, process};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long)]
    movies: bool,
    #[structopt(long, required = true)]
    tags: Vec<String>,
    title: String,
}

fn run() -> io::Result<()> {
    let cli = Opt::from_args();
    let dir = home_dir().expect("No $HOME!?").join("blog/content");
    let today = chrono::Local::today().naive_local();
    let output = format!(
        "
+++
title = {:?}
date = {:?}

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
    process::Command::new("editor").arg(path).status()?;

    Ok(())
}

fn main() {
    if let Err(why) = run() {
        eprintln!("{}", why);
        process::exit(1);
    }
}
