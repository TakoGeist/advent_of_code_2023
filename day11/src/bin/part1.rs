
fn expansion(input: &str) -> usize{
    let mut universe = input.lines().map(|line| line.chars().map(|x| x == '#')
        .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    {
        let clone = universe.clone();
        (0..clone[0].len()).filter(|&x| (0..clone.len()).all(|y| !clone[y][x])).enumerate()
            .for_each(|(a, x)| (0..clone.len()).for_each(|y| universe[y].insert(x + a, false)));
        
        (0..clone.len()).filter(|&x| clone[x].iter().fold(true, |acc, a| acc && !a)).enumerate()
            .for_each(|(a, x)| universe.insert(x + a, vec![false; universe[0].len()]));
    }
    
    let galaxies = universe.iter().enumerate()
    .map(|(y, line)| line.iter().enumerate()
    .filter(|(_, &a)| a).map(move |(x, _)| (y, x)).collect::<Vec<_>>())
    .flatten().collect::<Vec<_>>();

    let mut pairs = Vec::new();
    (0..galaxies.len()).for_each(|x| (x + 1..galaxies.len())
        .for_each(|y| pairs.push((galaxies[x], galaxies[y]))));

    pairs.into_iter().map(|((x1, x2),(y1, y2))| (x1).abs_diff(y1) + x2.abs_diff(y2)).sum()
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part1 -> {}", expansion(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let res = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(expansion(res), 374);
    }
}
