# Sunshine Save Detective
![sun](./src/assets/logo.png)

**A save file analyzer for the Gamecube game Super Mario Sunshine**

## Features
- Reads NTSC-U, NTSC-J, and PAL `.gci` save files
- Swap between save files A, B, and C
- Check which shines and blue coins were collected per world
- See each world's high score count and if nozzles were collected
- See how many lives are left on a save file

## Building and Bundling
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

## Important Notes
This program only currently supports North American save files. It does not support Japanese or European saves.

This program also only currently support reading `.gci` save formats.

## Disclaimer
***No LLM-generated content, whether code, vector graphics, digital images, text, or other content was used in the making of this program.***