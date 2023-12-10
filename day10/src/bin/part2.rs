
#[derive(Clone, Copy)]
enum Direction{
    Up,
    Down,
    Left, 
    Right
}

fn maze(input: &str) -> isize{
    let maze = input.chars().collect::<Vec<_>>();
    let line = maze.iter().position(|&x| x == '\n').unwrap();
    let maze = maze.into_iter().filter(|&x| x != '\n').collect::<Vec<_>>();
    let start = maze.iter().position(|&x| x == 'S').unwrap();

    let (mut current, mut from) = if matches!(maze.get(start + 1), Some(&'-') | Some(&'7') | Some(&'J')){
        (start + 1, Direction::Right)
    }
    else if matches!(maze.get(start - 1), Some(&'-') | Some(&'F') | Some(&'L')){
        (start - 1, Direction::Left)
    }
    else{
        (start - line, Direction::Up)
    };

    let mut vertices = vec![(start / line, start % line)];

    while maze[current] != 'S'{
        vertices.push((current / line, current % line));
        match (maze[current], from){
            ('|', Direction::Up) => current -= line,
            ('|', Direction::Down) => current += line,
            ('-', Direction::Right) => current += 1,
            ('-', Direction::Left) => current -= 1,
            ('7', Direction::Up) | ('J', Direction::Down) => { current -= 1; from = Direction::Left}
            ('7', Direction::Right) | ('F', Direction::Left) => { current += line; from = Direction::Down}
            ('L', Direction::Down) | ('F', Direction::Up) => { current += 1; from = Direction::Right}
            ('L', Direction::Left) | ('J', Direction::Right) => { current -= line; from = Direction::Up}
            _ => panic!(),
        }
    }

    (0..vertices.len()).map(|x| 
        (*vertices.get(x).unwrap(), *vertices.get(x + 1).or(vertices.get(0)).unwrap()))
    .map(|(i, j)| (i.0 * j.1) as isize - (i.1 * j.0) as isize)
    .sum::<isize>().abs() / 2 + 1 - vertices.len() as isize / 2
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part2 -> {}",maze(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_one() {
        let res = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(maze(res), 4);
    }
    
    #[test]
    fn test_two() {
        let res = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(maze(res), 8);
    }

    #[test]
    fn test_three() {
        let res = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(maze(res), 10);
    }
}
