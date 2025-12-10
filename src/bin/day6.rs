#![feature(strip_circumfix)]

use std::str::Chars;
fn main() {

    let input = include_str!("../../inputs/day6");

    let mut lines = input.lines()
        .map(|line| line.chars())
        .collect::<Vec<Chars<'_>>>();

    let operators = lines.pop().expect("no operator list").filter(|c| !c.is_whitespace()).collect::<Vec<char>>();


    let res = operators.iter().fold(0, |acc, &op| {
        let mut cols:  Vec<usize> = Vec::new();
        while let Some(col) = map_col(&mut lines) {
            cols.push(col);
        }

        match op {
            '+' => {
                let sum: usize = cols.iter().sum();
                acc + sum
            },
            '*' => {
                let prod: usize = cols.iter().product();
                acc + prod
            },
            _ => acc,
        }
    });
    

    println!("Result: {}", res);
}

fn map_col(lines: &mut Vec<Chars<'_>>) -> Option<usize> {
    let col = (0..lines.len()).map(|line_idx| {
        lines[line_idx].next().and_then(|c| c.to_digit(10)).map(|v| v as usize)
    }).collect::<Vec<Option<usize>>>();

    if col.iter().all(|d| d.is_none()) {
        return None;
    } else {
        let v = col.iter().rev().filter_map(|&v| v).enumerate().fold(0usize, |acc, (i, v)| {
            acc + v * 10usize.pow(i as u32)
        });
        Some(v) 
    }
}

#[cfg(test)]
mod tests {}
