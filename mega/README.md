# [mega](https://github.com/MetallicSquid/rust-cmd-line-tools/tree/master/mega)

## **MEtadata GAtherer**

This is a simple command line tool that takes a file path as an argument, gathers the avaliable metadata of the file and places it inside of the user's clipboard.

**NB: This is only temporary, but the output of the length function should be bytes instead of characters. I will change this when I also have a few other features to throw in.**

This command line tool uses three external crates:

1.  [Structopt](https://github.com/TeXitoi/structopt) - This is a crate that allows for the command line tool to take in a file path as an argument and then use and manipulate it.

2.  [Chrono](https://github.com/chronotope/chrono) - This is a crate that allows for the conversion between `std::time::SystemTime` to a `std::string::String`, so that it can be pasted onto the user's clipboard.

3.  [Clipboard](https://github.com/aweinstock314/rust-clipboard) - This is a crate that allows the tool to access the user's clipboard and fill it with the metadata of the file.

### Features:

* Can be used to gather metadata of files, directories and symlinks.
* Instantaneous.

### Installation:
Via cargo:

```
cargo install cli_mega
```

### Screenshot:
![cli_mega-demonstration](https://github.com/MetallicSquid/rust-cmd-line-tools/blob/master/mega/cli_mega-demonstration-2.png)

### Flags:

```
A tool to gather the metadata of a file

USAGE:
    cli_mega.exe [FLAGS] <path>

FLAGS:
    -a, --accessed     Narrow output to the time of last access.
    -c, --created      Narrow output to the time of creation.
    -d, --directory    Narrow output to is_directory.
    -f, --file         Narrow output to is_file.
    -h, --help         Prints help information
    -l, --length       Narrow output to the length of the file.
    -m, --modified     Narrow output to the time of last modification.
    -r, --readonly     Narrow output to is_readonly.
    -s, --symlink      Narrow output to is_symlink.
    -V, --version      Prints version information

ARGS:
    <path>    Path.
```

### License:

This project is licensed under the MIT license. See [LICENSE](https://github.com/MetallicSquid/rust-cmd-line-tools/blob/master/LICENSE) for more details.
