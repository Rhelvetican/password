use clap::Parser;

const NUMBER: &str = "Number";
const T_F: &str = "True/False";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 8, value_name = NUMBER, display_order = 1)]
    number: u8,
    #[clap(short, long, value_parser, default_value_t = true, value_name = T_F, display_order = 2)]
    number_enabled: bool,
    #[clap(short, long, value_parser, default_value_t = false, value_name = T_F, display_order = 3)]
    capital_enabled: bool,
    #[clap(short, long, value_parser, default_value_t = false, value_name = T_F, display_order = 4)]
    special_enabled: bool,
}

fn main() {
}
