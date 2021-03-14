extern crate clap;
use clap::{crate_version, App, Arg};
use std::fs;
mod lib;

fn main() {
    let matches = App::new("tablify")
        .version(crate_version!())
        .author("Yuki Suzuki <y-suzuki@radiol.med.osaka-u.ac.jp>")
        .about("Load tabular data and turn it into a html file.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input csv file")
                .required(true),
        )
        .arg(
            Arg::with_name("TEMPLATE")
                .help("Sets the template file")
                .required(true),
        )
        .arg(
            Arg::with_name("autoescape")
                .help("Enable autoescaping")
                .short("a")
                .long("autoescape"),
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let raw_content = fs::read(input).unwrap();
    let template_filename = matches.value_of("TEMPLATE").unwrap();
    let template_content = fs::read_to_string(template_filename).unwrap();
    let html = lib::tablify(
        &template_content,
        &raw_content,
        input,
        matches.is_present("autoescape"),
    )
    .unwrap();
    println!("{}", html);

    std::process::exit(0)
}