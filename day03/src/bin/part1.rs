use std::collections::HashSet;


const INDEX: [(isize,isize);8] = [(1,-1), (1,0), (1,1),
                                  (0,-1),        (0,1),
                                  (-1,-1),(-1,0),(-1,1)];

#[derive(PartialEq)]
enum Node{
    Number(u32),
    Dot,
    Symbol
}

impl From<char> for Node{
    fn from(input: char) -> Node{
        match input{
            '0'..='9' => Node::Number(input.to_digit(10).unwrap() as u32),
            '.' => Node::Dot,
            _ => Node::Symbol
        }
    }
}

fn neighbours(col: usize, lin: usize, max_col: usize, max_lin: usize) -> Vec<(usize,usize)>{
    INDEX.iter().map(|&(y, x)| (col as isize + y, lin as isize  + x))
        .filter(|&(x,y)| x >= 0 && x < max_col as isize && y >= 0 && y < max_lin as isize)
        .map(|(a,b)| (a as usize, b as usize))
        .collect()
}

fn gear(input: &str) -> u32{
    let mut res = 0;
    let grid = input.lines().map(|line| line.chars().map(|x| x.into()).collect::<Vec<Node>>()).collect::<Vec<_>>();
    let max_col = grid.len();
    let max_lin = grid[0].len();
    let mut num = 0;
    let mut neigh_num = HashSet::new();
    for (col, line) in grid.iter().enumerate(){
        for (lin, node) in line.iter().enumerate(){
            if let Node::Number(x) = *node{
                num *= 10; 
                num += x; 
                neigh_num.extend(neighbours(col, lin, max_col, max_lin));
            }
            else{
                if num == 0 {continue;}
                else {
                    for &(y, x) in neigh_num.iter(){
                        if grid[y][x] == Node::Symbol{
                            res += num;
                            break;
                        }
                  }
                }
                num = 0;
                neigh_num.clear();
            }
        }
    }
    res
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part1 -> {}", gear(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let res = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!(gear(res), 4361);
    }
}
