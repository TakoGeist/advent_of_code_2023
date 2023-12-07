
fn scratch(input: &str) -> u32{
    input.replace("  ", " ").lines().map(|line| 
            line.split(":").nth(1).unwrap().split_once("|").unwrap())
         .map(|(left, right)| 
            (left.trim().split(" ").collect::<Vec<_>>(), right.trim().split(" ")))
         .map(|(x, y)| 
            y.filter(move |elem| x.contains(elem)).collect::<Vec<_>>())
         .filter(|wins| wins.len() != 0).map(|x| 1 << (x.len() - 1)).sum()
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part1 -> {}", scratch(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let res ="Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(scratch(res), 13);
    }
}
