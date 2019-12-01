// This is a command line tool to gather the metadata of a file and then place
// it onto the system clipboard.

// All of the neccessary crates.
use std::{path, fs::metadata};
use structopt::StructOpt;
use chrono::{offset::Local, DateTime};
use clipboard::{ClipboardProvider, ClipboardContext};

// A struct to collect the user input.
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
        Err(error) => panic!("The file couldn't be found. It's probably in another directory. ({})", error),
    };

    return metadata;
}

// Function to convert the gathered metadata into a string.
fn metadata_to_string(metadata: std::fs::Metadata) -> std::string::String{
    // Convert all of the system time data into a string.
    let accessed_time: DateTime<Local> = metadata.accessed().unwrap().into();
    let accessed_meta = accessed_time.format("%d/%m/%Y %T");
    let created_time: DateTime<Local> = metadata.created().unwrap().into();
    let created_meta = created_time.format("%d/%m/%Y %T");
    let modified_time: DateTime<Local> = metadata.modified().unwrap().into();
    let modified_meta = modified_time.format("%d/%m/%Y %T");

    // Convert all of the boolean data into a string.
    let is_sym_meta = match metadata.file_type().is_symlink() {
        true => "Is symlink? -> true".to_string(),
        false => "Is symlink? -> false".to_string(),
    };
    let is_dir_meta = match metadata.is_dir() {
        true => "Is directory? -> true".to_string(),
        false => "Is directory? -> false".to_string(),
    };
    let is_file_meta = match metadata.is_file() {
        true => "Is file? -> true".to_string(),
        false => "Is file? -> false".to_string(),
    };
    let is_readonly_meta = match metadata.permissions().readonly() {
        true => "Is readonly? -> true".to_string(),
        false => "Is readonly? -> false".to_string(),
    };

    // Convert all of the length data into a string.
    let len_meta = metadata.len().to_string();

    // Concatenate all of the string data into one big string.
    let mut final_meta = String::from("");
    final_meta.push_str(&is_sym_meta);
    final_meta.push_str("\n");
    final_meta.push_str(&is_dir_meta);
    final_meta.push_str("\n");
    final_meta.push_str(&is_file_meta);
    final_meta.push_str("\n");
    final_meta.push_str(&is_readonly_meta);
    final_meta.push_str("\n");
    final_meta.push_str("File length: ");
    final_meta.push_str(&len_meta);
    final_meta.push_str("\n");
    // final_meta.push_str(accessed_meta);
    // final_meta.push_str(created_meta);
    // final_meta.push_str(modified_meta);
    println!("{}", final_meta);

    return final_meta;
}

// Function to copy the metadata to the user's clipboard
fn clip_copy(meta_string: std::string::String) {
    let mut clip_context: ClipboardContext = ClipboardProvider::new().unwrap();
    clip_context.set_contents(meta_string.to_owned()).unwrap();
}

fn main() {
    // Collect the path of the file input.
    let args = Cli::from_args();

    // Collect the metadata of the file by running the above functions, then add
    // it to the clipboard.
    clip_copy(metadata_to_string(gather_metadata(args.path)));
}
