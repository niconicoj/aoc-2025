use std::collections::{BTreeMap, HashMap};

fn main() {

    let input = include_str!("../../inputs/day3");

    let joltage = input.lines().map(|bank| {
        let mut map: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        bank.char_indices().for_each(|(i, c)| {
            map.entry(c.to_digit(10).expect("integer base 10")).or_default().push(i);
        });

        match map.pop_last() {
            Some((c, indices)) if indices.len() < 2 => {
                map.pop_last().map(|(c, indices)| c).or
            }
            Some((c, indices)) => {
                c * 10 + c
            }
            None => 0u32
            
        }
    }).sum::<u32>();

    println!("sum : {joltage}");
}

#[cfg(test)]
mod tests {}
