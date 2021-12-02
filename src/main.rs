mod util;
mod day01;
mod day02;

fn main() {
    println!("day01 #1: {}", day01::part1("day01"));
    util::test_i32("day01 #2 test", 5, day01::part2("day01_2_test"));
    println!("day01 #2: {}", day01::part2("day01"));
    println!("-------------------------------------");
    println!("day02 #1: {}", day02::part1("day02"));
    util::test_i32("day01 #2 test", 900, day02::part2("day02_2_test"));
    println!("day02 #2: {}", day02::part2("day02"));
    println!("-------------------------------------");
}
