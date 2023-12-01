fn trebuchet(input: &str) -> u32{
    input.lines().map(|x| x.chars()
            .filter_map(|x| x.to_digit(10)).collect::<Vec<_>>())
            .map(|x| x.first().unwrap() * 10 + x.last().unwrap()).sum()
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part1 -> {}", trebuchet(input));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let res = trebuchet(input);
        assert_eq!(res, 142);
    }
}
