use std::{fs::File, io::Read};
use std::fmt::Write;

fn main() {
    let mut data = Vec::<u8>::new();
    let mut file = File::open("HC32F005.SFR").unwrap();
    file.read_to_end(&mut data).unwrap();

    find_lenstrs(&data);
}

fn find_lenstrs(data: &[u8]) {
    let mut data = data;
    let mut leftovers = Vec::new();
    'outer: loop {
        loop {
            if let Some(eidx) = data.iter().position(|&b| b == 0) {
                // println!("!!!");
                // hexprint(&data[skip_idx..][..eidx + 1]);
                // println!();
                let looky = &data[..=eidx];
                let mut maybe = None;
                'search: for (i, b) in looky.iter().rev().enumerate().skip(1) {
                    if i > 255 {
                        // println!("FAIL3 {:02X}", b);
                        break 'search;
                    } else if *b == (i as u8) {
                        // println!("MAYBE");
                        maybe = Some(i);
                    } else if b.is_ascii_control() {
                        // println!("FAIL2 {:02X}", b);
                        break 'search;
                    } else if !b.is_ascii() {
                        // println!("FAIL1 {:02X}", b);
                        break 'search;
                    }
                }

                if let Some(found) = maybe.take() {
                    if found < 3 {
                        leftovers.extend_from_slice(&data[..=eidx]);
                    } else {
                        let len = looky.len();
                        leftovers.extend_from_slice(&data[..(eidx - found)]);
                        hexprint(&leftovers);
                        leftovers.clear();
                        println!();
                        print!("Bingo: [{}] ", found);
                        println!("{}", std::str::from_utf8(&looky[(len - found)..(len - 1)]).unwrap());
                        println!();
                    }
                } else {
                    leftovers.extend_from_slice(&data[..=eidx]);
                }
                data = &data[(eidx + 1)..];
            } else {
                hexprint(&leftovers);
                println!();
                println!("Done");
                return;
            }
        }
    }
}

fn hexprint(data: &[u8]) {
    let mut line = String::new();
    data.chunks(16).for_each(|c| {
        line.clear();
        for b in c {
            write!(&mut line, "{:02X} ", b).unwrap();
        }
        for b in c {
            let buh = &[*b];

            let chr = if !b.is_ascii_control() {
                std::str::from_utf8(buh).unwrap_or(".")
            } else {
                "."
            };

            write!(&mut line, "{}", chr).unwrap();
        }
        println!("{}", line);
    })
}
