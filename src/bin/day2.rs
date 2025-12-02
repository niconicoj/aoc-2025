fn main() {

    let input = include_str!("../../inputs/day2");

    let parts: usize = input.split(',').map(|part| {
        let (start, end) = part.split_once('-').expect("incorrect ID range format");
        let (start, end) = (start.trim().parse::<usize>().expect("incorrect start range format"), end.trim().parse::<usize>().expect("incorrect end range format"));
        let sum: usize = (start..=end).filter(|id| id.ilog10() % 2 == 1).map(|id| {
            let digits = id.ilog10() + 1;
            let halved_dozens = 10usize.pow(digits / 2);

            let high_part = id / halved_dozens;
            let low_part = id - high_part * halved_dozens;

            return if high_part == low_part {
                println!("{id} ({halved_dozens}) : {high_part}|{low_part}");
                id
            } else {0};
        }).sum();
        return sum;
    }).sum();

    println!("sum : {parts}");
}

#[cfg(test)]
mod tests {
    use super::*;
    
}
