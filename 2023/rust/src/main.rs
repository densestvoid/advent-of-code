use log::{LevelFilter, error, debug};
use std::env;

use env_logger::Builder;

use advent_of_code_rust::cli;
use advent_of_code_rust::days::*;

fn main() {
    let mut day: cli::Arg<u32> = cli::Arg::new_positional();
    let mut part: cli::Arg<u32> = cli::Arg::new_positional();
    let mut file_contents: cli::Arg<cli::FileContents> = cli::Arg::new_positional();
    let mut debug: cli::Arg::<bool> = cli::Arg::new_optional(String::from("d"), false);

    let mut parser = cli::Parser::new();
    parser.add_argument(&mut day);
    parser.add_argument(&mut part);
    parser.add_argument(&mut file_contents);
    parser.add_argument(&mut debug);
    if let Err(e) = parser.parse(env::args().collect()) {
        println!("{}", e.reason);
        return
    }

    let mut logger = Builder::new();
    logger.filter_level(LevelFilter::Info);

    if let cli::Arg::Parsed(debug) = debug {
        if debug {
            logger.filter_level(LevelFilter::Debug);
        }
        logger.init();

        debug!("Debugging enabled: {}", debug);
    }

    if let cli::Arg::Parsed(file) = file_contents {
        if let cli::Arg::Parsed(day) = day {
            if let cli::Arg::Parsed(part) = part {
                match (day, part) {
                    (1, 1) => one::solve_part_1(&file.contents),
                    (1, 2) => one::solve_part_2(&file.contents),
                    (2, 1) => two::solve_part_1(&file.contents),
                    (2, 2) => two::solve_part_2(&file.contents),
                    (3, 1) => three::solve_part_1(&file.contents),
                    (3, 2) => three::solve_part_2(&file.contents),
                    (4, 1) => four::solve_part_1(&file.contents),
                    (4, 2) => four::solve_part_2(&file.contents),
                    (5, 1) => five::solve_part_1(&file.contents),
                    (5, 2) => five::solve_part_2(&file.contents),
                    (6, 1) => six::solve_part_1(&file.contents),
                    (6, 2) => six::solve_part_2(&file.contents),
                    (7, 1) => seven::solve_part_1(&file.contents),
                    (7, 2) => seven::solve_part_2(&file.contents),
                    (8, 1) => eight::solve_part_1(&file.contents),
                    (8, 2) => eight::solve_part_2(&file.contents),
                    (9, 1) => nine::solve_part_1(&file.contents),
                    (9, 2) => nine::solve_part_2(&file.contents),
                    (10, 1) => ten::solve_part_1(&file.contents),
                    (10, 2) => ten::solve_part_2(&file.contents),
                    (11, 1) => eleven::solve_part_1(&file.contents),
                    (11, 2) => eleven::solve_part_2(&file.contents),
                    (12, 1) => twelve::solve_part_1(&file.contents),
                    (12, 2) => twelve::solve_part_2(&file.contents),
                    (13, 1) => thirteen::solve_part_1(&file.contents),
                    (13, 2) => thirteen::solve_part_2(&file.contents),
                    (14, 1) => fourteen::solve_part_1(&file.contents),
                    (14, 2) => fourteen::solve_part_2(&file.contents),
                    (15, 1) => fifteen::solve_part_1(&file.contents),
                    (15, 2) => fifteen::solve_part_2(&file.contents),
                    _ => error!("day {} part {} not implemented", day, part),
                }
            }
        }
    }

}