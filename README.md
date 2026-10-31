# Sunshine Save Detective
![sun](./sunshine_ui/src/assets/logo.png)

**A save file analyzer for the Gamecube game Super Mario Sunshine**

## Features
- Reads NTSC-U, NTSC-J, and PAL `.gci` save files
- Swap between save files A, B, and C
- Check which shines and blue coins were collected per world
- See each world's high score count and if nozzles were collected
- See how many lives are left on a save file

## Modules
This project is separated into 3 modules:
- sunshine_ui [GUI]
- sunshine_cli [CLI]
- sunshine_lib [Library]

## Building and Bundling GUI
This guide will show how to bundle and build the UI module `sunshine_ui`

For the CLI on all platforms, all you need to do is
```sh
cd sunshine_cli
cargo build --release
```

### Windows
Building for windows is relatively straight forward. Run the following command and it will produce a `.exe` file:
```sh
cargo build --release
```

### MacOS and Linux
You'll need to install [cargo-bundle](https://crates.io/crates/cargo-bundle) onto your machine if you want to create a mac app package or linux `.deb` package.

Once you have `cargo-bundle` installed, do the following command:
```sh
cargo bundle --release
```

## CLI
Provided in the latest release is a linux distribution of the CLI. Download it, then move it to your bin like so:
(alternatively, it is always recommended to build from source!)
```sh
mkdir -p ~/.local/bin
cp sunshine_cli ~/.local/bin/
```

Here is a list of options that can be used with the CLI.
```sh
Usage: sunshine_cli [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>       Path to save file
  -f, --files <FILES>     Save files. Options: A,B,C
  -r, --region            Show the game's region (NTSC-U, PAL, NTSC-J)
  -l, --lives             Show number of 1ups collected per save
  -s, --shines            Show total shines collected per save
  -b, --blue-coins        Show total blue coins collected per save
  -c, --coin-hs           Show coin highscores per world
  -x, --world-shines      Show list of shines collected per world
  -o, --world-blue-coins  Show list of blue coins collected per world
  -n, --nozzles           Show list of nozzles collected per world
  -h, --help              Print help
  -V, --version           Print version

Example:
sunshine_cli -p "./sms.gci" -f A,B,C -rlsbcxon
```

## Important Notes
This program only currently supports North American save files. It does not support Japanese or European saves.

This program also only currently support reading `.gci` save formats.

## Disclaimer
***No LLM-generated content, whether code, vector graphics, digital images, text, or other content was used in the making of this program.***