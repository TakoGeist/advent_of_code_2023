use std::collections::HashMap;

fn wasteland(input: &str) -> u32{
    let input = input.replace("(", "").replace(")", "");
    let mut lines = input.lines();
    let mut moves = lines.next().unwrap().chars().cycle();
    let map = lines.skip(1)
        .map(|x| x.split_once("=").unwrap())
        .map(|(a, b)| (a, b.split_once(",").unwrap()))
        .map(|(x, (a,b))| (x.trim(),[a.trim(), b.trim()])).collect::<HashMap<_,_>>();
    let mut current = "AAA";
    let mut res = 0;
    while current != "ZZZ"{
        if moves.next().unwrap() == 'L'{
            current = map.get(current).unwrap()[0];
        }
        else{
            current = map.get(current).unwrap()[1];
        }
        res += 1;
    }
    res
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part1 -> {}", wasteland(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let res = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(wasteland(res), 2);
    }

    #[test]
    fn test_two() {
        let res = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(wasteland(res), 6);
    }
}
