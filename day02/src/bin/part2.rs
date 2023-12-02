
fn conundrum(input: &str) -> usize{
    input.lines().map(|line| line.split(":").nth(1).unwrap())
    .map(|game| game.replace(";", ",").split(",")
        .fold((0,0,0), |(red, green, blue), dice| {
            match dice.trim().split_once(" "){
                Some((num, "red")) => if num.parse::<usize>().unwrap() > red {(num.parse::<usize>().unwrap(), green, blue)} else {(red, green, blue)}
                Some((num, "green")) => if num.parse::<usize>().unwrap() > green {(red, num.parse::<usize>().unwrap(), blue)} else {(red, green, blue)}
                Some((num, "blue")) => if num.parse::<usize>().unwrap() > blue {(red, green, num.parse::<usize>().unwrap())} else {(red, green, blue)}
                _ => panic!("Unexpected color value")
            }
        }))
    .fold(0, |acc, (red, green, blue)| acc + red * green * blue)
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part2 -> {}", conundrum(input));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        let res = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(conundrum(res), 2286);
    }
}
