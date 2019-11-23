use std::cmp::{max, min};
use std::io::{self, Read};

struct Dimensions(u32, u32, u32);

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input.split("\n").collect();
    let box_sizes: Vec<Dimensions> = lines.into_iter().filter_map(parse_to_dimensions).collect();

    let wrapping_paper_needed: u32 = box_sizes.iter().map(&calc_wrapping_paper).sum();
    let ribbon_needed: u32 = box_sizes.iter().map(&calc_ribbon_length).sum();

    println!(
        "Total wrapping paper need = {} square feet",
        wrapping_paper_needed
    );

    println!("Total ribbon needed = {} feet", ribbon_needed);

    Ok(())
}

fn parse_to_dimensions(s: &str) -> Option<Dimensions> {
    let d: Vec<&str> = s.split("x").collect();

    match d.len() {
        3 => Some(Dimensions(
            d[0].parse::<u32>().expect("to be an integer"),
            d[1].parse::<u32>().expect("to be an integer"),
            d[2].parse::<u32>().expect("to be an integer"),
        )),
        _ => None,
    }
}

fn calc_wrapping_paper(d: &Dimensions) -> u32 {
    let lw = d.0 * d.1;
    let wh = d.1 * d.2;
    let hl = d.2 * d.0;

    let min = min(min(lw, wh), min(wh, hl));

    (2 * lw) + (2 * wh) + (2 * hl) + min
}

fn calc_ribbon_length(d: &Dimensions) -> u32 {
    let a = min(min(d.0, d.1), min(d.1, d.2));
    let b = {
        if d.0 >= min(d.1, d.2) && d.0 <= max(d.1, d.2) {
            d.0
        } else if d.1 >= min(d.0, d.2) && d.1 <= max(d.0, d.2) {
            d.1
        } else {
            d.2
        }
    };

    (2 * a) + (2 * b) + (d.0 * d.1 * d.2)
}
