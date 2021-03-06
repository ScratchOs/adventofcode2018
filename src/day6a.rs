use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::cmp;

#[derive(Clone, Copy, Debug)]
struct Loc {
    claimed: bool,
    id: usize,
    distance: u32,
    contested: bool,
}

pub fn day6a() {
    let path = Path::new("data/day6.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => {}
    }

    let re = Regex::new(r"(\d+), (\d+)").unwrap();
    let mut coords = vec![];

    let (mut x, mut y, mut width, mut height) = (0,0,0,0);
    let mut bounds_defined = false;
    for cap in re.captures_iter(&s) {
        let c = (
            cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<usize>().unwrap()
        );
        coords.push(c);
        if !bounds_defined {
            x = c.0;
            y = c.1;
            width = c.0;
            height = c.1;
            bounds_defined = true;
        }
        if c.0 < x {
            x = c.0;
        }
        if c.1 < y {
            y = c.1;
        }
        if c.0 > width {
            width = c.0;
        }
        if c.1 > height {
            height = c.1;
        }
    }
    
    for coord in &mut coords {
        coord.0 -= x;
        coord.1 -= y;
    }
    width -= x;
    height -= y;

    let mut area = vec![vec![Loc{claimed: false, contested: false, distance: 0, id: 0}; height + 1]; width + 1];

    for (x, row) in area.iter_mut().enumerate() {
        for (y, location) in row.iter_mut().enumerate() {
            for (id, coord) in coords.iter().enumerate() {
                let dist = (coord.0 as i32 - x as i32).abs() as u32
                    + (coord.1 as i32 - y as i32).abs() as u32;
                if !location.claimed {
                    location.distance = dist;
                    location.claimed = true;
                } else if location.distance > dist {
                    location.contested = false;
                    location.distance = dist;
                    location.id = id;
                } else if location.distance == dist {
                    location.contested = true;
                }
            }
        }
    }

    let mut counts = vec![(0, true); coords.len()];

    for (x, row) in area.iter().enumerate() {
        for (y, location) in row.iter().enumerate() {
            if !location.contested {
                if x == 0 || y == 0 || x == width || y == height {
                    counts[location.id].1 = false;
                }
                if counts[location.id].1 {
                    counts[location.id].0 += 1;
                }
            }
        }
    }

    let max = counts.into_iter().filter(|s|s.1).fold(0, |current, s| cmp::max(current, s.0));

    println!("{}", max);
}
