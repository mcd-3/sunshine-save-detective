use std::fs;

use crate::{
    sunshine_lib::{
        models::{
            save_file::SaveFile,
            world::World,
            mem_card::MemoryCard
        },
        utils::{
            string::padded_string,
            data::{
                get_blue_coin_count,
                get_worlds_data_list
            }
        },
    },
};

const PADDING_NONE: u16 = 0;
const PADDING_SMALL: u16 = 2;
const PADDING_MEDIUM: u16 = 4;
const PADDING_LARGE: u16 = 6;

fn print_lines(lines: &Vec<String>) {
    for line in lines {
        println!("{}", line);
    }
}

fn add_formated_string_with_count(
    lines: &mut Vec<String>,
    string: &String,
    count: u16,
    padding: u16,
) {
    let str = format!(
        "{}{}",
        string.clone(),
        count
    );
    lines.push(
        String::from(
            padded_string(&str, padding, 0)
        )
    );
}

fn add_formated_string(
    lines: &mut Vec<String>,
    string: &String,
    padding: u16,
) {
    let str = format!(
        "{}",
        string.clone(),
    );
    lines.push(
        String::from(
            padded_string(&str, padding, 0)
        )
    );
}

fn add_formated_string_with_collected_flag(
    lines: &mut Vec<String>,
    string: &String,
    is_collected: bool,
    padding: u16,
) {
    let str = format!(
        "[{}] {}",
        if is_collected { '1' } else { '0' },
        string.clone(),
    );
    lines.push(
        String::from(
            padded_string(&str, padding, 0)
        )
    );
}

// TODO: Move a lot of this logic into separate functions
/// Write a report for the save data, either to a txt file or to the terminal
/// 
/// Returns true if successful and false if failure
pub fn write_report(
    mem_card: &MemoryCard,
    save_file_labels: Vec<char>,
    show_region: bool,
    show_shine_total: bool,
    show_bc_total: bool,
    show_world_shines: bool,
    show_world_bcs: bool,
    show_world_coin_hs: bool,
    show_nozzles: bool,
    show_lives_total: bool,
    file_path: Option<String>,
) -> bool {
    let mut lines: Vec<String> = vec![
        String::from("=== Sunshine Save Detective ===")
    ];

    if show_region {
        add_formated_string(
            &mut lines,
            &String::from(
                format!("Region: {}", &mem_card.get_region_name())
            ),
            PADDING_NONE,
        );
    }

    for label in &save_file_labels {
        if label.to_ascii_uppercase() != 'A'
            && label.to_ascii_uppercase() != 'B'
            && label.to_ascii_uppercase() != 'C'
        {
            lines.push(String::from(format!("{} is not a valid save file", label)));
            print_lines(&lines);
            break;
        }

        let save = SaveFile::new_by_label(*label);
        let worlds = get_worlds_data_list(&mem_card, &save);

        add_formated_string(
            &mut lines,
            &String::from(
                format!("Save File {}:", label)
            ),
            PADDING_NONE,
        );

        if show_shine_total {
            add_formated_string_with_count(
                &mut lines,
                &String::from("Total Shine Count: "),
                save.get_shine_count(&mem_card),
                PADDING_SMALL,
            );
        }

        if show_bc_total {
            add_formated_string_with_count(
                &mut lines,
                &String::from("Total Blue Coin Count: "),
                get_blue_coin_count(&worlds),
                PADDING_SMALL,
            );
        }

        if show_lives_total {
            add_formated_string_with_count(
                &mut lines,
                &String::from("1Ups Collected: "),
                save.get_lives_count(&mem_card),
                PADDING_SMALL,
            );
        }

        for world in worlds {
            add_formated_string(
                &mut lines,
                &String::from(
                    format!(
                        "{}:",
                        World::get_name_from_identifier(world.identifier)
                    )
                ),
                PADDING_SMALL,
            );

            if show_world_coin_hs {
                add_formated_string_with_count(
                    &mut lines,
                    &String::from("Coin HighScore: "),
                    world.get_coin_hs(),
                    PADDING_MEDIUM,
                );
            }

            if show_world_shines {
                add_formated_string_with_count(
                    &mut lines,
                    &String::from("Shines Collected: "),
                    world.shines.len() as u16,
                    PADDING_MEDIUM,
                );
            }

            if show_world_bcs {
                add_formated_string_with_count(
                    &mut lines,
                    &String::from("Blue Coins Collected: "),
                    world.blue_coins.len() as u16,
                    PADDING_MEDIUM,
                );
            }

            if show_world_shines {
                add_formated_string(&mut lines, &String::from("Shines:"), PADDING_MEDIUM);
                for shine in world.shines {
                    add_formated_string_with_collected_flag(
                        &mut lines,
                        &String::from(shine.get_description()),
                        shine.get_is_collected(),
                        PADDING_LARGE
                    );
                }
            }

            if show_world_bcs {
                add_formated_string(&mut lines, &String::from("Blue Coins:"), PADDING_MEDIUM);
                for bc in world.blue_coins {
                    add_formated_string_with_collected_flag(
                        &mut lines,
                        &String::from(bc.get_description()),
                        bc.get_is_collected(),
                        PADDING_LARGE
                    );
                }
            }

            if show_nozzles && world.nozzles.len() > 0 {
                add_formated_string(&mut lines, &String::from("Nozzles:"), PADDING_MEDIUM);
                for nozzle in world.nozzles {
                    add_formated_string_with_collected_flag(
                        &mut lines,
                        &String::from(nozzle.get_nozzle_name()),
                        nozzle.get_is_collected(),
                        PADDING_LARGE
                    );
                }
            }
        }
    }

    if file_path.is_none() {
        print_lines(&lines);
        return true;
    } else {
        let contents = lines.join("\n");
        return match fs::write(
            format!(
                "{}",
                file_path.unwrap()
            ),
            contents
        ) {
            Ok(_res) => true,
            Err(_err) => false,
        }
    }
}