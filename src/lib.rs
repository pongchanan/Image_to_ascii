use clap::{App, Arg};
use image::{DynamicImage, GenericImageView};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    input_files: Vec<String>,
    output_files: Vec<String>,
    write: bool,
    choose: usize,
}
///////////////////////////////////////////////////////////////
pub fn compose_ascii_atlas(path: Vec<String>) -> String {
    let mut dummy = Vec::new();
    for item in path {
        let img = image::open(item).unwrap();
        let img = resize_img(img, 100);
        let ascii_art = get_ascii_art(&img);
        dummy.push(ascii_art);
    }
    hcat(dummy.iter().map(|m| m.as_str()).collect())
}
pub fn resize_img(img: DynamicImage, new_width: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let img = img.thumbnail(new_width, (new_width * height) / width);
    return img;
}
pub fn hcat(data: Vec<&str>) -> String {
    let mut dummy: Vec<Vec<&str>> = Vec::new();
    let mut results = Vec::new();
    for item in data {
        dummy.push(item.split('\n').collect::<Vec<&str>>())
    }
    let size = dummy[0][0].len();
    for index in 0..size {
        let mut line: String = "".to_string();
        for ascii in dummy.clone() {
            line = format!("{}{}", line, ascii[index]);
        }
        results.push(line);
    }
    results.join("\n")
}
///////////////////////////////////////////////////////////////
pub fn get_ascii_art(img: &image::DynamicImage) -> String {
    let (width, height) = img.dimensions();
    let img = img.to_rgb8();
    let mut ascii_art = String::new();
    for _ in 0..width + 2 {
        ascii_art.push('+');
    }
    ascii_art.push('\n');
    for y in 0..height {
        ascii_art.push('+');
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let luminance = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
            let ascii_char = map_luminance_to_ascii(luminance);
            ascii_art.push(ascii_char);
        }
        ascii_art.push('+');
        ascii_art.push('\n');
    }
    for _ in 0..width + 2 {
        ascii_art.push('+');
    }
    ascii_art.push('\n');
    ascii_art
}
pub fn map_luminance_to_ascii(luminance: f32) -> char {
    let ascii_chars: [char; 5] = ['█', '▓', '▒', '░', ' '];
    let scale_factor = 255.0 / 4.0;
    let index = (luminance / scale_factor).round() as usize;
    ascii_chars[index]
}
///////////////////////////////////////////////////////////////
pub fn save_to(path: &str, data: String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
///////////////////////////////////////////////////////////////
pub fn find(path: &str, adress: usize) -> String {
    let mut res: Vec<Vec<&str>> = Vec::new();
    let mut result = Vec::new();
    let data = open(path);
    let line: Vec<&str> = data.split("\n").collect();
    for each_line in line {
        let mut data_line: Vec<&str> = each_line.split("+").collect();
        data_line.retain(|x| *x != "");
        res.push(data_line);
    }
    for item in res {
        if item.len() == 0 {
            continue;
        }
        result.push(item[adress]);
    }
    result.join("\n")
}
pub fn open(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
///////////////////////////////////////////////////////////////
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .long("files")
                .help("Input file(s)")
                .multiple(true),
        )
        .arg(
            Arg::with_name("write")
                .short("w")
                .help("write all in put to file")
                .takes_value(false)
                .conflicts_with("choose"),
        )
        .arg(
            Arg::with_name("choose")
                .short("c")
                .long("choose")
                .value_name("CHOOSE")
                .help("write choose picture to file")
                .default_value("0"),
        )
        .arg(
            Arg::with_name("target")
                .long("target")
                .value_name("Target")
                .help("Output file(s)")
                .multiple(true)
                .default_value("dummy.txt"),
        )
        .get_matches();

    Ok(Config {
        input_files: matches.values_of_lossy("files").unwrap(),
        output_files: matches.values_of_lossy("target").unwrap(),
        write: matches.is_present("write"),
        choose: matches
            .value_of("choose")
            .map(parse_positive_int)
            .transpose()
            .unwrap_or(Some(0))
            .unwrap_or(0),
    })
}
pub fn run(config: Config) -> MyResult<()> {
    if config.write == true {
        let ascii = compose_ascii_atlas(config.input_files);
        for bottom in config.output_files {
            let _ = save_to(&bottom, ascii.clone());
        }
    } else if config.write == false {
        let ascii = find(&config.input_files[0], config.choose);
        for bottom in config.output_files {
            let _ = save_to(&bottom, ascii.clone());
        }
    }
    Ok(())
}