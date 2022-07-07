use image;
use clap::{ArgEnum, Parser, Subcommand};
use ricencrypt;
use std::convert::Into;

#[derive(Parser)]
struct Arg {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clone, ArgEnum)]
enum Direction {
    Row,
    Col,
    Both,
}

impl Into<ricencrypt::Direction> for Direction {
    fn into(self) -> ricencrypt::Direction {
        match self {
            Direction::Col => ricencrypt::Direction::Col,
            Direction::Row => ricencrypt::Direction::Row,
            Direction::Both => ricencrypt::Direction::Both,
        }
    }
}

#[derive(Subcommand)]
enum Command {
    Encrypt {
        #[clap(arg_enum, value_parser)]
        mode: Direction,
        #[clap(value_parser)]
        input_file: String,
        #[clap(value_parser, default_value_t = String::from("a.jpg"))]
        output_file: String,
        #[clap(value_parser, default_value_t = 0.666)]
        x: f64,
        #[clap(value_parser, default_value_t = 3.99999)]
        u: f64,
    },
    Dencrypt {
        #[clap(arg_enum, value_parser)]
        mode: Direction,
        #[clap(value_parser)]
        input_file: String,
        #[clap(value_parser, default_value_t = String::from("a.jpg"))]
        output_file: String,
        #[clap(value_parser, default_value_t = 0.666)]
        x: f64,
        #[clap(value_parser, default_value_t = 3.99999)]
        u: f64,
    },
}

fn main() -> () {
    let arg = Arg::parse();

    match arg.command {
        Command::Encrypt {
            mode,
            input_file,
            output_file,
            x,
            u,
        } => {
            let pic = image::io::Reader::open(input_file).expect("Open file err").decode().expect("Decode err");
            let pic = ricencrypt::encrypt(pic, mode.into(), x, u);
            pic.save(output_file).expect("Save err");
        }
        Command::Dencrypt {
            mode,
            input_file,
            output_file,
            x,
            u,
        } => {
            let pic = image::io::Reader::open(input_file).expect("Open file err").decode().expect("Decode err");
            let pic = ricencrypt::dencrypt(pic, mode.into(), x, u);
            pic.save(output_file).expect("Save err");
        }
    }
}
