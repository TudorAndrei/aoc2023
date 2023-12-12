use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn parse_string1(text: String) -> u32 {
    let nrs: Vec<u32> = text.chars().filter_map(|x| x.to_digit(10)).collect();
    return 10 * nrs.first().unwrap() + nrs.last().unwrap();
}
fn parse_string2(text: String) -> u32 {
    let mut mod_text = (0..text.len()).filter_map(|index| {
        let subline = &text[index..];

        let result = if subline.starts_with("one") {
            '1'
        } else if subline.starts_with("two") {
            '2'
        } else if subline.starts_with("three") {
            '3'
        } else if subline.starts_with("four") {
            '4'
        } else if subline.starts_with("five") {
            '5'
        } else if subline.starts_with("six") {
            '6'
        } else if subline.starts_with("seven") {
            '7'
        } else if subline.starts_with("eight") {
            '8'
        } else if subline.starts_with("nine") {
            '9'
        } else {
            subline.chars().next().unwrap()
        };

        result.to_digit(10)
    });
    let nrs: Vec<u32> = mod_text.collect();
    // println!("{:?}", nrs);
    10 * nrs.first().unwrap() + nrs.last().unwrap()
}

fn main() {
    let mut acc = 0;
    if let Ok(lines) = read_lines("input/input.in") {
        for line in lines {
            let result = parse_string2(line.unwrap());
            acc += result;
        }
    }
    println!("{}", acc);
}
