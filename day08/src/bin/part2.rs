use std::collections::HashMap;

fn gcd(mut x: u64, mut y: u64) -> u64{
    while y != 0{
        let z = y;
        y = x % y;
        x = z;
    }
    x
}

fn wasteland(input: &str) -> u64{
    let input = input.replace("(", "").replace(")", "");
    let mut lines = input.lines();
    let moves = lines.next().unwrap().chars().cycle();
    let map = lines.skip(1)
        .map(|x| x.split_once("=").unwrap())
        .map(|(a, b)| (a, b.split_once(",").unwrap()))
        .map(|(x, (a,b))| (x.trim().chars(),[a.trim().chars(), b.trim().chars()]))
        .map(|(mut a, [mut b, mut c])| 
            ([a.next().unwrap(), a.next().unwrap(), a.next().unwrap()],
            [[b.next().unwrap(), b.next().unwrap(), b.next().unwrap()],
            [c.next().unwrap(), c.next().unwrap(), c.next().unwrap()]]))
        .collect::<HashMap<_,_>>();

    let current = map.keys().filter(|&x| x[2] == 'A').collect::<Vec<_>>();
    let mut steps = Vec::new();

    for mut elem in current{
        let mut mov = moves.clone();
        let mut step = 0;
        while elem[2] != 'Z'{
            if mov.next().unwrap() == 'L'{
                elem = &map.get(elem).unwrap()[0];
            }
            else{
                elem = &map.get(elem).unwrap()[1];
            }
            step += 1;
        }
        steps.push(step);
    }
    steps.into_iter().reduce(|acc, x| acc * x / gcd(acc, x)).unwrap()
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
        let res = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        assert_eq!(wasteland(res), 6);
    }
}
