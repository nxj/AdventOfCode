mod utils;
use regex::Regex;
use std::fmt;

fn day5(input_data: String) {

    println!("{:?}", input_data)

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = String::from("53616c7465645f5f7f29e338a08fa0b87bac819f29ede5ec8359127b634adf0427fb117dc07617f7598f6edcca9292e3");
    let input_data = utils::aoc_get_input(5, 2021, &session);
    day5(input_data);
    Ok(())
}
