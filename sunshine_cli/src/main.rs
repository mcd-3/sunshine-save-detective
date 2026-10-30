extern crate sunshine_lib;

use clap::Parser;

use sunshine_lib::{
    enums::file_error::FileError,
    utils::report::write_report,
    models::mem_card::MemoryCard,
};

/// Sunshine Save Detective:
/// Analyze Super Mario Sunshine save files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to save file
    #[arg(short, long)]
    path: String,

    /// Save files. Options: A,B,C
    #[arg(short, long, value_delimiter = ',')]
    files: Vec<char>,

    /// Show the game's region (NTSC-U, PAL, NTSC-J)
    #[arg(short, long)]
    region: bool,

    /// Show number of 1ups collected per save
    #[arg(short, long)]
    lives: bool,

    /// Show total shines collected per save
    #[arg(short, long)]
    shines: bool,

    /// Show total blue coins collected per save
    #[arg(short, long)]
    blue_coins: bool,

    /// Show coin highscores per world
    #[arg(short, long)]
    coin_hs: bool,

    /// Show list of shines collected per world
    #[arg(short = 'x', long)]
    world_shines: bool,

    /// Show list of blue coins collected per world
    #[arg(short = 'o', long)]
    world_blue_coins: bool,

    /// Show list of nozzles collected per world
    #[arg(short, long)]
    nozzles: bool,
}

fn main() {
    let args = Args::parse();

    let mem_card =
        MemoryCard::read_from_file(args.path.as_str());

    let read_is_okay = match mem_card.file_error {
        FileError::NoError => true,
        FileError::FileRead => false,
        FileError::NotProperGame => false,
        FileError::NotValidSave => false,
        FileError::ReadCancelled => false,
    };

    if !read_is_okay {
        return;
    }

    let saves = if args.files.is_empty() {
        vec!['A', 'B', 'C']
    } else {
        args.files.clone()
    };

    // Special case where if no options are set, show everything
    if !args.region
        && !args.shines
        && !args.blue_coins
        && !args.world_shines
        && !args.world_blue_coins
        && !args.coin_hs
        && !args.nozzles
        && !args.lives
    {
        write_report(
            &mem_card,
            saves,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            None,
        );
        return;
    }

    write_report(
        &mem_card,
        saves,
        args.region,
        args.shines,
        args.blue_coins,
        args.world_shines,
        args.world_blue_coins,
        args.coin_hs,
        args.nozzles,
        args.lives,
        None,
    );
}
