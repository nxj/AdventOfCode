use reqwest::{cookie::Jar, Url};
use std::sync::Arc;
use std::fs;
use std::io::ErrorKind;

fn download_aoc_inputs(day: i32, year: i32, session : &String) -> reqwest::Result<String>{

    let cookie = format!("session={}",session);

    let url = format!("https://adventofcode.com/{}/day/{}/input",year, day).parse::<Url>().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    let arc_jar  = Arc::new(jar);
    
    let client = reqwest::blocking::Client::builder()
    .cookie_store(true)
    .cookie_provider(arc_jar)
    .build()?;

    let res = client
    .get(url)
    .send()?
    .error_for_status();

    return res?.text();
}

pub fn aoc_get_input(day: i32, year: i32, session : &String) -> String {
    
    let input_file_path = format!("inputs/day_{}", day);
    let input_data = fs::read_to_string(&input_file_path);

    let data = match input_data {
        Ok(data) => data,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    let downloaded_input = download_aoc_inputs(day, year, session);
                    let downloaded_input = match downloaded_input {
                        Ok(data) => {
                            fs::write(input_file_path, &data).expect("Unable to write aoc input data to file");
                            data
                        },
                        Err(error) => panic!("Could not download inputs for aoc (day: {:?}, year: {:?}): {:?}", day, year, error),
                    };
                    downloaded_input
                },
                _ => {
                    panic!("Could not open file: {:?}", error);
                },
            }
        }
    };

    return data
}