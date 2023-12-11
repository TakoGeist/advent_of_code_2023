
const EXP_FACTOR: usize = 999999;

fn offset(expand: & Vec<usize>, fst: usize, snd:usize) -> usize{
    let (start, end) = if fst < snd {(fst,snd)} else {(snd,fst)};
    let mut count = 0;
    let mut lower = 0;
    for &i in expand{
        if i < start{
            lower += 1;
        }
        if i < end{
            count += 1;
        }
        else{
            break;
        }
    }
    count - lower
}

fn expansion(input: &str) -> usize{
    let universe = input.lines().map(|line| line.chars().map(|x| x == '#')
        .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = (0..universe[0].len()).filter(|&x| (0..universe.len()).all(|y| !universe[y][x]))
        .collect::<Vec<_>>();
    
    let columns = (0..universe.len()).filter(|&x| universe[x].iter().fold(true, |acc, a| acc && !a))
        .collect::<Vec<_>>();
    
    let galaxies = universe.iter().enumerate()
        .map(|(y, line)| line.iter().enumerate()
        .filter(|(_, &a)| a).map(move |(x, _)| (y, x)).collect::<Vec<_>>())
        .flatten().collect::<Vec<_>>();

    let mut pairs = Vec::new();
    (0..galaxies.len()).for_each(|x| (x + 1..galaxies.len())
        .for_each(|y| pairs.push((galaxies[x], galaxies[y]))));

    pairs.into_iter().map(|((x1, x2),(y1, y2))| 
        (x1).abs_diff(y1) + offset(&columns, x1 ,y1) * EXP_FACTOR 
        + x2.abs_diff(y2) + offset(&rows, x2, y2) * EXP_FACTOR).sum()
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part2 -> {}", expansion(input));
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
        assert_eq!(expansion(res), 8410);
    }
}
