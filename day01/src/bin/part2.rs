use std::collections::HashMap;

fn trebuchet(input: &str) -> u32{
    let numbers = HashMap::from([("1", 1),("2", 2),("3", 3),("4", 4),("5", 5),("6", 6),("7", 7),("8", 8),("9", 9),
                ("one", 1),("two", 2),("three", 3),("four", 4),("five", 5),("six", 6),("seven", 7),("eight", 8),("nine", 9)]);
    let mut occur = input.lines()
        .map(|line| numbers.keys()
        .map(|num| line.match_indices(num)).flatten()
        .collect::<Vec<_>>()).collect::<Vec<_>>();
    for line in occur.iter_mut(){
        line.sort_by(|(a, _),(b, _)| a.cmp(b));
    }
    occur.iter().map(|x| numbers.get(x.first().unwrap().1).unwrap() * 10 + numbers.get(x.last().unwrap().1).unwrap()).sum()
}


fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part2 -> {}", trebuchet(input));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let res = trebuchet(input);
        assert_eq!(res, 281);
    }
}