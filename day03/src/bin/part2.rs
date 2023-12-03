use std::collections::{HashSet, HashMap};


const INDEX: [(isize,isize);8] = [(1,-1), (1,0), (1,1),
                                  (0,-1),        (0,1),
                                  (-1,-1),(-1,0),(-1,1)];

#[derive(PartialEq)]
enum Node{
    Number(u32),
    Dot,
    Symbol((usize, char))
}

impl From<(usize, char)> for Node{
    fn from(input: (usize, char)) -> Node{
        match input.1{
            '0'..='9' => Node::Number(input.1.to_digit(10).unwrap() as u32),
            '.' => Node::Dot,
            _ => Node::Symbol(input)
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
    let grid = input.lines().enumerate().map(|(ind, line)| line.chars().enumerate().map(|(a, b)| (a + ind * 1000, b).into()).collect::<Vec<Node>>()).collect::<Vec<_>>();
    let max_col = grid.len();
    let max_lin = grid[0].len();
    let mut numbers = HashMap::new();
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
                        if let Node::Symbol(s) = grid[y][x]{
                            numbers.entry(s.0).or_insert(Vec::new()).push(num);
                            break;
                        }
                  }
                }
                num = 0;
                neigh_num.clear();
            }
        }
    }
    numbers.into_iter().filter(|(_, a)| a.len() == 2).map(|(_, a)| a.iter().product::<u32>()).sum()
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part2 -> {}", gear(input));
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
        assert_eq!(gear(res), 467835);
    }
}
