fn main() {

    let input = include_str!("../../inputs/day4");

    let mut grid = Grid::from_str(input);
    let mut total = 0;
    
    while let Some(to_delete) = grid.list_accessible_cells() {
        total += to_delete.len();        
        to_delete.iter().for_each(|&(x, y)| {
            grid.clear(x, y);
        }); 
    }

    println!("sum : {total}");
}

struct Grid {
    grid: Vec<Vec<char>>,
    w: usize,
    h: usize
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        Self {
            w: grid.get(0).map(|row| row.len()).unwrap_or_default(),
            h: grid.len(),
            grid, 
        } 
    }

    pub fn list_accessible_cells(&self) -> Option<Vec<(usize, usize)>> {
        let mut list = Vec::new();
        (0..self.h).for_each(|y| {
            (0..self.w).for_each(|x| {
                if self.count_adjacent(x, y) < 4 && self.has_tp_roll(x, y) {
                    list.push((x, y));
                }
            })
        });

        if list.len() != 0 {
            Some(list)
        }else {
            None
        }
    }

    fn count_adjacent(&self, x: usize, y: usize) -> usize {
          self.has_tp_roll(x.wrapping_sub(1),   y.wrapping_sub(1)) as usize
        + self.has_tp_roll(x,     y.wrapping_sub(1)) as usize
        + self.has_tp_roll(x + 1, y.wrapping_sub(1)) as usize
        + self.has_tp_roll(x.wrapping_sub(1), y) as usize
        + self.has_tp_roll(x + 1, y) as usize
        + self.has_tp_roll(x.wrapping_sub(1), y + 1) as usize
        + self.has_tp_roll(x,     y + 1) as usize
        + self.has_tp_roll(x + 1, y + 1) as usize
    }

    fn has_tp_roll(&self, x: usize, y: usize) -> bool {
        self.grid.get(y).and_then(|row| row.get(x)).filter(|&c| c == &'@').is_some()
    }

    fn clear(&mut self, x: usize, y: usize) {
        if let Some(c) = self.grid.get_mut(y).and_then(|row| row.get_mut(x)) {
            *c = '.'
        }
    }
}

#[cfg(test)]
mod tests {}
