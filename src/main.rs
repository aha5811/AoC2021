mod util;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    util::do_day(day01::main, "day01");
    util::do_day(day02::main, "day02");
    util::do_day(day03::main, "day03");
    util::do_day(day04::main, "day04");
    util::do_day(day05::main, "day05");
    util::do_day(day06::main, "day06");
    util::do_day(day07::main, "day07");
    util::do_day(day08::main, "day08");
    util::do_day(day09::main, "day09");
    util::do_day(day10::main, "day10");
}
