mod util;
mod day01;
mod day02;

fn main() {
    println!("day01_1: {}", day01::day01_1("day01"));
    util::test_i32("day01_2 test", 5, day01::day01_2("day01_2_test"));
    println!("day01_2: {}", day01::day01_2("day01"));
    println!("-------------------------------------");
    println!("day02_1: {}", day02::day02_1("day02"));
    util::test_i32("day01_2 test", 900, day02::day02_2("day02_2_test"));
    println!("day02_2: {}", day02::day02_2("day02"));
    println!("-------------------------------------");
}
