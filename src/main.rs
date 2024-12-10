use clap::Parser;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

#[derive(Parser)]
struct Cli {
    day: u8,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    match args.day {
        1 => day1::solve(args.path),
        2 => day2::solve(args.path),
        3 => day3::solve(args.path),
        4 => day4::solve(args.path),
        5 => day5::solve(args.path),
        6 => day6::solve(args.path),
        7 => day7::solve(args.path),
        8 => day8::solve(args.path),
        9 => day9::solve(args.path),
        10 => day10::solve(args.path),
        _ => println!("This day has not been implemented yet!"),
    }
}
