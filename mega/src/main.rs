// This is a command line tool to gather the metadata of a file and then place
// it onto the system clipboard.

// All of the neccessary crates.
use std::{path, fs::metadata, collections::HashMap};
use structopt::StructOpt;
use chrono::{offset::Local, DateTime};
use clipboard::{ClipboardProvider, ClipboardContext};

// A struct to collect the user input.
#[derive(StructOpt)]
struct Cli {
    /// Path.
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
    /// Narrow output to the time of last access.
    #[structopt(short, long)]
    accessed: bool,
    /// Narrow output to the time of creation.
    #[structopt(short, long)]
    created: bool,
    /// Narrow output to the time of last modification.
    #[structopt(short, long)]
    modified: bool,
    /// Narrow output to is_symlink.
    #[structopt(short, long)]
    symlink: bool,
    /// Narrow output to is_directory.
    #[structopt(short, long)]
    directory: bool,
    /// Narrow output to is_file.
    #[structopt(short, long)]
    file: bool,
    /// Narrow output to is_readonly.
    #[structopt(short, long)]
    readonly: bool,
    /// Narrow output to the length of the file.
    #[structopt(short, long)]
    length: bool,
}

// Function to gather the metadata of the file.
fn gather_metadata(path: path::PathBuf) -> std::fs::Metadata {
    let metadata = match metadata(path) {
        Ok(metadata) => metadata,
        Err(_error) => panic!("The file couldn't be found. You probably gave an invalid/nonexistant file name."),
    };

    return metadata;
}

fn collect_flags(args: Cli) -> std::collections::HashMap<&'static str, bool> {
    let mut flags = HashMap::new(); 
    flags.insert("a", args.accessed);
    flags.insert("c", args.created);
    flags.insert("d", args.directory);
    flags.insert("f", args.file);
    flags.insert("l", args.length);
    flags.insert("m", args.modified);
    flags.insert("r", args.readonly);
    flags.insert("s", args.symlink);

    return flags;
}

// Function to convert the gathered metadata into a string.
fn metadata_to_string(metadata: std::fs::Metadata, flags: std::collections::HashMap<&str, bool>) -> std::string::String {
    // Convert all of the system time data into a string.
    let accessed_time: DateTime<Local> = metadata.accessed().unwrap().into();
    let accessed_meta = accessed_time.format("%d/%m/%Y %T").to_string();
    let created_time: DateTime<Local> = metadata.created().unwrap().into();
    let created_meta = created_time.format("%d/%m/%Y %T").to_string();
    let modified_time: DateTime<Local> = metadata.modified().unwrap().into();
    let modified_meta = modified_time.format("%d/%m/%Y %T").to_string();

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

    // Decide which data to present and which data not to.
    let mut final_meta = String::from("");
    let mut true_flags = HashMap::new();

    for (key, value) in flags {
        if value == true {
            true_flags.insert(key, value);
        }
    }

    if true_flags.is_empty() == false {
        for (key, _value) in true_flags {
            // Only print the neccessary data, based on the flags that returned
            // true.
            match key {
                "a" => {final_meta.push_str("Date & time accessed: "); final_meta.push_str(&accessed_meta); final_meta.push_str("\n")},
                "c" => {final_meta.push_str("Date & time created: "); final_meta.push_str(&created_meta); final_meta.push_str("\n")},
                "d" => {final_meta.push_str(&is_dir_meta); final_meta.push_str("\n")},
                "f" => {final_meta.push_str(&is_file_meta); final_meta.push_str("\n")},
                "l" => {final_meta.push_str(&len_meta); final_meta.push_str("\n")},
                "m" => {final_meta.push_str("Date & time modified: "); final_meta.push_str(&modified_meta); final_meta.push_str("\n")},
                "r" => {final_meta.push_str(&is_readonly_meta); final_meta.push_str("\n")},
                "s" => {final_meta.push_str(&is_sym_meta); final_meta.push_str("\n")},
                _ => println!("This should never be triggered."),
            }
        }
    } else {
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
        final_meta.push_str(" characters");
        final_meta.push_str("\n");
        final_meta.push_str("Date & time accessed: ");
        final_meta.push_str(&accessed_meta);
        final_meta.push_str("\n");
        final_meta.push_str("Date & time created: ");
        final_meta.push_str(&created_meta);
        final_meta.push_str("\n");
        final_meta.push_str("Date & time modified: ");
        final_meta.push_str(&modified_meta);
    }

    return final_meta;
}

// Function to copy the metadata to the user's clipboard
fn clip_copy(meta_string: &std::string::String) {
    let mut clip_context: ClipboardContext = ClipboardProvider::new().unwrap();
    clip_context.set_contents(meta_string.to_owned()).unwrap();
}

fn main() {
    // Collect all of the flags. 
    let flags = collect_flags(Cli::from_args());

    // Collect the metadata of the file by running the above functions, then add
    // it to the clipboard.
    let meta: std::string::String = metadata_to_string(gather_metadata(Cli::from_args().path), flags);
    clip_copy(&meta);

    // Output the metadata of the file to the console. I might want to use a
    // BufWriter in the future, but I'm only printing once, so this seems
    // pointless.
    println!("{}", &meta);
}
