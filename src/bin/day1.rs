fn main() {

    let mut dial = Dial::new();

    let input = include_str!("../../inputs/day1");

    let mut z_count = 0;

    input.lines().for_each(|line| {
        let (dir, amount) = line.split_at(1);
        let amount: i32 = amount.parse().expect("Failed to parse amount");

        match dir {
            "L" => {
                z_count += dial.turn_left(amount);
            }
            "R" => {
                z_count += dial.turn_right(amount);
            }
            _ => panic!("Invalid direction {dir}"),
        };
    });

    println!("Number of times dial hit zero: {z_count}");
}


struct Dial(i32);

impl Dial {
    fn new() -> Self {
        Dial(50)
    }

    fn turn_left(&mut self, amount: i32) -> i32 {
        let prev_pos = self.0;
        self.0 = (self.0 - amount).rem_euclid(100);
        return ((100 - prev_pos) % 100 + amount).div_euclid(100).abs();
    }

    fn turn_right(&mut self, amount: i32) -> i32 {
        let prev_pos = self.0;
        self.0 = (self.0 + amount).rem_euclid(100);
        return (prev_pos + amount).div_euclid(100).abs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dial_left_turns() {
        let mut dial = Dial::new();
        assert_eq!(dial.turn_left(100), 1);
        assert_eq!(dial.0, 50);
        assert_eq!(dial.turn_left(200), 2);
        assert_eq!(dial.0, 50);
        assert_eq!(dial.turn_left(360), 4);
        assert_eq!(dial.0, 90);
        assert_eq!(dial.turn_left(60), 0);
        assert_eq!(dial.0, 30);
        assert_eq!(dial.turn_left(250), 3);
        assert_eq!(dial.0, 80);
        assert_eq!(dial.turn_left(80), 1);
        assert_eq!(dial.0, 0);
        assert_eq!(dial.turn_left(10), 0);
        assert_eq!(dial.0, 90);
    }

    #[test]
    fn test_dial_right_turns() {
        let mut dial = Dial::new();
        assert_eq!(dial.turn_right(100), 1);
        assert_eq!(dial.0, 50);
        assert_eq!(dial.turn_right(200), 2);
        assert_eq!(dial.0, 50);
        assert_eq!(dial.turn_right(360), 4);
        assert_eq!(dial.0, 10);
        assert_eq!(dial.turn_right(60), 0);
        assert_eq!(dial.0, 70);
        assert_eq!(dial.turn_right(30), 1);
        assert_eq!(dial.0, 0);
        assert_eq!(dial.turn_right(30), 0);
        assert_eq!(dial.0, 30);
        assert_eq!(dial.turn_right(80), 1);
        assert_eq!(dial.0, 10);
    }

    #[test]
    fn test_dial_mixed_turns() {
        let mut dial = Dial::new();
        assert_eq!(dial.turn_left(68), 1);
        assert_eq!(dial.turn_left(30), 0);
        assert_eq!(dial.turn_right(48), 1);
        assert_eq!(dial.turn_left(5), 0);
        assert_eq!(dial.turn_right(60), 1);
        assert_eq!(dial.turn_left(55), 1);
        assert_eq!(dial.turn_left(1), 0);
        assert_eq!(dial.turn_left(99), 1);
        assert_eq!(dial.turn_right(14), 0);
        assert_eq!(dial.turn_left(82), 1);
    }
}    
