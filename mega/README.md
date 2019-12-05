# [mega](https://github.com/MetallicSquid/rust-cmd-line-tools/tree/master/mega)

## **MEtadata GAtherer**

This is a simple command line tool that takes a file path as an argument, gathers the avaliable metadata of the file and places it inside of the user's clipboard.

This command line tool uses three external crates:

1.  [Structopt](https://github.com/TeXitoi/structopt) - This is a crate that (in this case) allows for the command line tool to take in a file path as an argument and then use and manipulate it.

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
![cli_mega-demonstration](https://github.com/MetallicSquid/rust-cmd-line-tools/blob/master/cli_mega-demonstration.PNG)

ToDo:

 - [x]  Collect the metadata of the file.
 
 - [x]  Add support for the timestamps. 
 
 - [x]  Convert all of the collected metadata into `std::string::String`.
 
 - [x]  Place this metadata string into the user's clipboard.
 
 - [ ]  Add a help command.
 
 - [ ]  Allow the user to select the metadata they want to receive.
 
 - [ ]  Allow for a file that is not in the immediate directory to be analysed. 
 
 - [ ]  Completely error-proof the whole tool.

### License:

This project is licensed under the MIT license. See [LICENSE](https://github.com/MetallicSquid/rust-cmd-line-tools/blob/master/LICENSE) for more details.
