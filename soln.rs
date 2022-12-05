// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 5.  
//! Ben and Bart Massey 2022

use aoc::{*, reparse::*};
use regex::Regex;

fn add_row(rows: &mut [Vec<char>], line: &str) {
    let line: Vec<char> = line.chars().collect();
    for (i, col) in rows.iter_mut().enumerate() {
        let c = line[i * 4 + 1];
        if c != ' ' {
            col.push(c);
        }
    }
}

fn main() {
    let row_re = Regex::new(r"^(\[[A-Z]\]|   )( (\[[A-Z]\]|   ))*$").unwrap();

    // XXX We will take advantage of the input picture
    // formatting having padding spaces at the end of each
    // line.
    let mut lines = input_lines().peekable();
    let first_row = lines.next().unwrap();
    assert!(row_re.is_match(&first_row));
    let nchars = first_row.chars().count();
    assert_eq!(3, nchars % 4);
    let ncols = (nchars + 1) / 4;
    let mut rows: Vec<Vec<char>> = std::iter::repeat(vec![])
        .take(ncols)
        .collect();

    add_row(&mut rows, &first_row);
    while row_re.is_match(lines.peek().unwrap()) {
        add_row(&mut rows, &lines.next().unwrap());
    }
    for row in &mut rows {
        row.reverse();
    }

    let _ = lines.next();
    let _ = lines.next();

    let move_re = Reparse::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)");
    for line in lines {
        let parsed = move_re.parse(&line).unwrap();
        let count = parsed.get::<usize>(1);
        let source = parsed.get::<usize>(2) - 1;
        let dest = parsed.get::<usize>(3) - 1;
        match get_part() {
            Part1 => {
                for _ in 0..count {
                    let block = rows[source].pop().unwrap();
                    rows[dest].push(block);
                }
            }
            Part2 => {
                let nsource = rows[source].len();
                assert!(nsource >= count);
                let blocks: Vec<char> = rows[source]
                    .drain(nsource - count..)
                    .collect();
                rows[dest].extend_from_slice(&blocks);
            }
        }
    }

    let message: String =
        rows.iter_mut().flat_map(|r| r.pop()).collect();
    println!("{message}");
}
