mod utils;
mod day1;
mod day2;

use day1::{part_1 as day1_part1, part_2 as day1_part2};
use day2::{part_1 as day2_part1, part_2 as day2_part2};

fn main() {
    day1_part1();
    day1_part2();

    day2_part1();
    day2_part2();
}
