fn main() {

    let input = include_str!("../../inputs/day5");

    let mut bounds = input.lines().take_while(|l| !l.is_empty()).flat_map(|l| {
        let (start, end) = l.split_once('-').expect("malformed range");
        let start = start.parse::<usize>().expect("bad start format");
        let end = end.parse::<usize>().expect("bad en format");
        [(start, Bound::Start), (end, Bound::End)]
    }).collect::<Vec<(usize, Bound)>>();

    bounds.sort();

    let mut bound_depth = 0;
    let mut prev_start_idx = 0;
    let mut sum = 0;

    bounds.iter().for_each(|(idx, bound)| {
        match bound {
            Bound::Start => {
                if bound_depth == 0 {
                    prev_start_idx = *idx;
                }
                bound_depth += 1;
            }
            Bound::End => {
                bound_depth -= 1;
                if bound_depth == 0 {
                    sum += idx - prev_start_idx + 1;
                }
            }
        };
    });

    println!("{sum}");

}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Bound {
    Start,
    End,
}

#[cfg(test)]
mod tests {}
