
fn almanac(input: &str) -> i64{
    let mut guide = input.split("\n\n");
    let mut seeds = 
        guide.next().unwrap().split_ascii_whitespace()
            .filter_map(|x| x.parse::<i64>().ok()).collect::<Vec<_>>();
    let maps = guide
        .map(|x| x.split_once(":").unwrap().1.trim().lines()
            .map(|line| line.trim().split(" ")
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>())
            .map(|x| [x[0],x[1],x[1] + x[2]])
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for step in maps{
        for ind in 0..seeds.len(){
            for rule in step.iter(){
                if seeds[ind] < rule[2] && seeds[ind] >= rule[1]{
                    seeds[ind] = rule[0] + seeds[ind] - rule[1];
                    break;
                }
            }
        }
    }

    seeds.into_iter().fold(i64::MAX, |acc, x| if x < acc {x} else {acc})
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part1 -> {}", almanac(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let res = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(almanac(res), 35);
    }
}
