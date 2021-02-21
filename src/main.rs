use clap::App;
use clap::Arg;
use serde::Deserialize;
use std::fs::create_dir_all;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    Directory,
    File,
    Report,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(rename = "lowercase", alias = "type")]
    fs_type: EntryType,
    name: Option<String>,
    contents: Option<Vec<Self>>,

    directories: Option<u64>,
    files: Option<u64>,
}

pub fn walk_entry(entry: Entry, mut root: PathBuf) {
    if let Some(name) = entry.name {
        root.push(name);
    }

    match entry.fs_type {
        EntryType::Directory => {
            create_dir_all(&root);
        }
        EntryType::File => {
            File::create(&root);
        }
        _ => {}
    }

    if let Some(contents) = entry.contents {
        for e in contents {
            walk_entry(e, root.clone());
        }
    }
}

fn main() {
    let matches = App::new("recon")
        .version("1.0")
        .author("Valerian G. <valerian.garleanu@pm.me>")
        .about("Reconstructs a dir tree from a treedump")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Specifiy the input file containing a tree dump")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out_dir")
                .short("o")
                .long("out_dir")
                .value_name("DIR")
                .help("Specify a directory into which to reconstruct the tree")
                .takes_value(true),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let out_dir = matches.value_of("out_dir").unwrap();

    let root = PathBuf::from(out_dir);

    let entry: Vec<Entry> =
        serde_json::from_reader(File::open(input_file).expect("Could not open the input file"))
            .expect("Got invalid json");

    for e in entry {
        if let EntryType::Report = e.fs_type {
            println!(
                "Written {} directories and {} files.",
                e.directories.unwrap(),
                e.files.unwrap()
            );
            continue;
        }

        walk_entry(e, root.clone());
    }
}
