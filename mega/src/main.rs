// This is a command line tool to gather the metadata of a file and then place
// it onto the system clipboard.
use std::{path, fs::metadata};
use structopt::StructOpt;
use clipboard::ClipboardProvider;

#[derive(StructOpt)]
struct Cli {
    // The path for the tool to read.
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
}

// Function to gather the metadata of the file.
fn gather_metadata(path: path::PathBuf) -> std::fs::Metadata {
    let metadata = match metadata(path) {
        Ok(metadata) => metadata,
        Err(error) => panic!("The following error occured: {}", error),
    };

    return metadata;
}

// Function to convert the gathered metadata into a string.
fn metadata_to_string(metadata: std::fs::Metadata) {
    let accessed_meta = metadata.accessed().unwrap();
    let created_meta = metadata.created().unwrap();
    // let file_type_meta = metadata.file_type();
    let is_dir_meta = metadata.is_dir();
    let is_file_meta = metadata.is_file();
    let len_meta = metadata.len().to_string();
    let modified_meta = metadata.modified().unwrap();
    // let permissions_meta = metadata.permissions();

    let final_meta = String::from("");

}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", gather_metadata(args.path).file_type());

}
