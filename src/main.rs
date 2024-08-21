use std::vec::Vec;

mod colorband;
mod resistor;
use resistor::Resistor;

fn main() {
    let mut program_name = String::from("");
    let mut color_bands = Vec::with_capacity(6);

    for (i, arg) in std::env::args().enumerate() {
        if i == 0 {
            program_name = arg;
            continue;
        }

        if i > 6 {
            println!("Resistors can have a maximum of 6 bands.");
            return;
        }

        if arg == "-h" {
            help(program_name);
            return;
        }

        let color = get_colorband(&arg.to_lowercase());

        if color.is_none() {
            println!("{} is not a valid resistor color. Use -h for help.", arg);
            return;
        }

        color_bands.push(color.unwrap());
    }

    if color_bands.len() == 0 {
        help(program_name);
        return;
    }

    let resistor = Resistor::new(color_bands);
    println!("{}", resistor.to_string());
}

fn get_colorband(color: &str) -> Option<colorband::ColorBand> {
    match color {
        "black" | "k" => Some(colorband::BLACK),
        "brown" | "br" => Some(colorband::BROWN),
        "red" | "r" => Some(colorband::RED),
        "orange" | "o" | "or" => Some(colorband::ORANGE),
        "yellow" | "y" => Some(colorband::YELLOW),
        "green" | "gr" => Some(colorband::GREEN),
        "blue" | "bl" => Some(colorband::BLUE),
        "violet" | "v" | "purple" | "p" => Some(colorband::VIOLET),
        "grey" | "gray" | "gy" => Some(colorband::GREY),
        "white" | "w" => Some(colorband::WHITE),
        "gold" | "gd" | "go" => Some(colorband::GOLD),
        "silver" | "s" | "si" => Some(colorband::SILVER),
        _ => None,
    }
}

fn help(program_name: String) {
    println!("{} help", program_name);
    println!("Calculates resistance from color bands.");
    println!("Provide colors (full name or short form) as a list of arguments.");
    println!("Valid Colors:");
    println!("-------------");
    println!("black, k");
    println!("brown, br");
    println!("red, r");
    println!("orange, o, or");
    println!("yellow, y");
    println!("green, gr");
    println!("blue, bl");
    println!("violet, v, purple, p");
    println!("grey, gray, gy");
    println!("white, w");
    println!("gold, gd, go");
    println!("silver, s, si");
}
