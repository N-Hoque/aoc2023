use aoc_2023::read_file;

fn main() {
    let data = read_file(1);
    let solver = aoc_2023::day_1::Solver::new(data);
    println!("Day 1 [1/2]: {}", solver.solve_first());
    println!("Day 1 [2/2]: {}", solver.solve_second());

    let data = read_file(2);
    let solver = aoc_2023::day_2::Solver::new(data);
    println!("Day 2 [1/2]: {}", solver.solve_first());
    println!("Day 2 [2/2]: {}", solver.solve_second());
}
