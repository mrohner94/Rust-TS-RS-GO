fn get_input() -> &'static str {
	return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
}

fn main() {
    let ret = get_input()
        .lines()
        .enumerate()
        .flat_map(|(idx, line)| {
            return line.chars().nth(idx * 3 % line.len());
        }) //.collect()
        .filter(|x| *x == '#')
        .count();

        println!("treecount {:?}\n", ret);


}