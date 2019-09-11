use dirs::home_dir;
use std::{fs, io};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long = "movies")]
    movies: bool,
    #[structopt(long = "tags", required = true)]
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
            "\ncategories = ['movies']"
        } else {
            ""
        }
    );
    let output = output.trim();
    let path = dir.join(cli.title).with_extension("md");
    eprintln!("{}", path.display());
    if path.exists() {
        eprintln!("path exists already, opening ...");
    } else {
        fs::write(&path, output)?;
    }
    std::process::Command::new("editor").arg(path).status()?;

    Ok(())
}

fn main() {
    if let Err(why) = run() {
        eprintln!("{}", why);
    }
}
