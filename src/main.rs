mod util;
mod day01;
mod day02;
mod day03;

fn main() {
    println!("day01 #1: {}", day01::part1("day01"));
    util::test_i32("day01 #2 test", 5, day01::part2("day01_2_test"));
    println!("day01 #2: {}", day01::part2("day01"));
    println!("-------------------------------------");
    println!("day02 #1: {}", day02::part1("day02"));
    util::test_i32("day02 #2 test", 900, day02::part2("day02_2_test"));
    println!("day02 #2: {}", day02::part2("day02"));
    println!("-------------------------------------");
    println!("day03 #1: {}", day03::part1("day03"));
    util::test_i32("day03 #2 test", 230, day03::part2("day03_2_test"));
    println!("day03 #2: {}", day03::part2("day03"));
    println!("-------------------------------------");
}
