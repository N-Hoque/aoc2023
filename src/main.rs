use aoc_2023::read_file;

macro_rules! solve_day {
    ($day:expr,$day_mod:ident) => {
        let data = read_file($day);
        let solver = aoc_2023::$day_mod::Solver::new(data);
        println!("Day {} [1/2]: {}", $day, solver.solve_first());
        println!("Day {} [2/2]: {}", $day, solver.solve_second());
    };
}

fn main() {
    solve_day!(1, day_1);
    solve_day!(2, day_2);
    solve_day!(3, day_3);
    solve_day!(4, day_4);
    solve_day!(5, day_5);
    solve_day!(6, day_6);
    solve_day!(7, day_7);
    solve_day!(8, day_8);
}
