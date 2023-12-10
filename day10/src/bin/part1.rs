
#[derive(Clone, Copy)]
enum Direction{
    Up,
    Down,
    Left, 
    Right
}

fn maze(input: &str) -> usize{
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

    let mut res = 1;

    while maze[current] != 'S'{
        match (maze[current], from){
            ('|', Direction::Up) => current -= line,
            ('|', Direction::Down) => current += line,
            ('-', Direction::Right) => current += 1,
            ('-', Direction::Left) => current -= 1,
            ('7', Direction::Up) | ('J', Direction::Down) => {current -= 1; from = Direction::Left}
            ('7', Direction::Right) | ('F', Direction::Left) => {current += line; from = Direction::Down}
            ('L', Direction::Down) | ('F', Direction::Up) => {current += 1; from = Direction::Right}
            ('L', Direction::Left) | ('J', Direction::Right) => {current -= line; from = Direction::Up}
            _ => panic!(),
        }
        res += 1;
    }
    res / 2
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part1 -> {}",maze(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_one() {
        let res = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(maze(res), 4);
    }

    #[test]
    fn test_two() {
        let res = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(maze(res), 8);
    }
}
