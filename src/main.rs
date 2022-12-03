use clap::Parser;
use day::{Day, Part};

mod day;
mod day01;
mod day02;

#[derive(Debug, clap::Parser)]
#[clap(color = concolor_clap::color_choice())]
struct Cli {
    #[command(flatten)]
    color: concolor_clap::Color,

    #[arg(value_enum)]
    day: Day,

    #[arg(value_enum, default_value_t = Part::Part1)]
    part: Part,
}

fn main() {
    let cli = Cli::parse();

    let file_path = format!("inputs/{}.txt", cli.day);
    println!(">>> {}, {}", cli.day, cli.part);
    println!(">>> file: {}", file_path);

    match cli.day {
        Day::Day01 => day01::run(cli.part, &file_path),
        Day::Day02 => day02::run(cli.part, &file_path),
    }
}
