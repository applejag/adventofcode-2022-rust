use clap::Parser;
use day::Part;

mod day;
mod day01;
mod day02;

#[derive(Debug, clap::Parser)]
#[clap(color = concolor_clap::color_choice())]
struct Cli {
    #[command(flatten)]
    color: concolor_clap::Color,

    #[arg(value_parser = clap::value_parser!(u32).range(1..24))]
    day: u32,

    #[arg(value_enum, default_value_t = Part::Part1)]
    part: Part,
}

fn main() {
    let cli = Cli::parse();

    match cli.day {
        1 => day01::run(cli.part),
        2 => day02::run(cli.part),
        _ => panic!("Day not yet implemented")
    }
}
