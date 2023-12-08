
fn almanac(input: &str) -> isize{
    let mut guide = input.split("\n\n");
    
    let mut seeds = guide.next().unwrap().split_ascii_whitespace()
        .filter_map(|x| x.parse::<isize>().ok())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|a| a[0]..(a[0] + a[1]))
        .collect::<Vec<_>>();

    let maps = guide
        .map(|x| x.split_once(":").unwrap().1.trim().lines()
            .map(|line| line.trim().split(" ")
                .map(|x| x.trim().parse::<isize>().unwrap())
                .collect::<Vec<_>>())
            .map(|x| [x[0], x[1], x[1] + x[2]])
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut idx = 0;

    for mut step in maps{
        step.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        while idx < seeds.len(){
            for rule in step.iter(){
                let (start, end) = (seeds[idx].start, seeds[idx].end - 1);
                let (left_delta, right_delta) = (start.saturating_sub(rule[1]), end.saturating_sub(rule[1]));
                let (lower, upper) = 
                ((rule[1]..rule[2]).contains(&start), (rule[1]..rule[2]).contains(&(end)));
                match (lower, upper){
                    (true, true) => {
                        seeds[idx] = (rule[0] + left_delta)..(rule[0] + right_delta);
                        break;
                    }
                    (true, false) => {
                        seeds[idx] = (rule[0] + left_delta)..(rule[0] + rule[2] - rule[1]);
                        seeds.insert(idx + 1, rule[2]..(end + 1));
                        break;
                    }
                    (false, true) => {
                        seeds[idx] = rule[0]..(rule[0] + right_delta);
                        seeds.insert(idx + 1, start..rule[1]);
                        break;
                    }
                    _ => ()
                }
            }
            idx += 1;
        }
        idx = 0;
    }

    seeds.into_iter().map(|x| x.start).min().unwrap()
}
fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part2 -> {}", almanac(input));
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
        assert_eq!(almanac(res), 46);
    }
}
