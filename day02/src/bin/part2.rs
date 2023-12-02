
fn conundrum(input: &str) -> usize{
    let mut res = 0;
    for game in input.lines().map(|line| line.split(":").nth(1).unwrap()){
        let (mut red, mut green, mut blue) = (0,0,0);
        for set in game.split(";").map(|x| x.trim().split(",")){
            for dice in set{
                match dice.trim().split_once(" "){
                    Some((num, "red")) => if num.parse::<usize>().unwrap() > red {red = num.parse::<usize>().unwrap();}
                    Some((num, "green")) => if num.parse::<usize>().unwrap() > green {green = num.parse::<usize>().unwrap();}
                    Some((num, "blue")) => if num.parse::<usize>().unwrap() > blue {blue = num.parse::<usize>().unwrap();}
                    _ => panic!("Unexpected color value")
                }
            }
        }
        res += red * green * blue;
    }
    res
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
