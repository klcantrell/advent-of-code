mod utils;
mod day1;
mod day2;
mod day3;

use day1::{part_1 as day1_part1, part_2 as day1_part2};
use day2::{part_1 as day2_part1, part_2 as day2_part2};
use day3::{part_1 as day3_part1, part_2 as day3_part2};

fn main() {
    day1_part1();
    day1_part2();

    day2_part1();
    day2_part2();

    day3_part1();
    day3_part2();
}
