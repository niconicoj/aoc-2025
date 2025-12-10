fn main() {

    let input = include_str!("../../inputs/day2");

    let parts: usize = input.split(',').map(|part| {
        let (start, end) = part.split_once('-').expect("incorrect ID range format");
        let (start, end) = (start.trim().parse::<usize>().expect("incorrect start range format"), end.trim().parse::<usize>().expect("incorrect end range format"));
        let sum: usize = (start..=end).filter(|id| id.ilog10() % 2 == 1).map(|id| {
            let total_digits = id.ilog10();

            (0..=(total_digits / 2)).filter(|digits| total_digits % digits == 0).map(|digits| {

            });

            0
        }).sum();
        return sum;
    }).sum();

    println!("sum : {parts}");
}

#[cfg(test)]
mod tests {}
