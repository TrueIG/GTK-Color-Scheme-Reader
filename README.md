# GTK Color Scheme Reader

This Rust project reads and displays the current GTK color scheme used on the system.

## Description

The program executes a command to get the current GTK theme and reads the associated `gtkrc` configuration file. It extracts color properties from the file and displays them in JSON format.

## Features

- Retrieves the current GTK theme using `gsettings`.
- Reads the `gtkrc` file of the current theme.
- Parses and extracts color properties.
- Displays the color properties in JSON format.

## Requirements

- **gsettings**: Ensure you have `gsettings` installed on your system.

## Installation

Clone this repository and build the project with Cargo:

```sh
git clone https://github.com/TrueIG/GTK-Color-Scheme-Reader.git
cd GTK-Color-Scheme-Reader 
cargo build --release
```

## Usage

Run the program:

```sh
./target/release/gtk-theme
```
