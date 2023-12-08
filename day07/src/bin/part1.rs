use std::collections::HashMap;

fn get_score<'a>(input: impl Iterator<Item = usize>) -> usize{
    let mut cards = [0;13];
    for idx in input{
        cards[idx - 1] += 1;
    }
    let cards = cards.into_iter().filter(|&x| x != 0);
    match cards.clone().count(){
        1 => 60000000000usize,
        2 => if cards.into_iter().any(|x| x == 4) {50000000000usize} else {40000000000usize}
        3 => if cards.into_iter().any(|x| x == 3) {30000000000usize} else {20000000000usize}
        4 => 10000000000usize,
        _ => 0,
    }
}

fn camel(input: &str) -> usize{
    let score: HashMap<u8, u8> = 
        HashMap::from([(b'A',13),(b'K',12),(b'Q',11),(b'J',10),(b'T',9),
                       (b'9',8),(b'8',7),(b'7',6),(b'6',5),(b'5',4),
                       (b'4',3),(b'3',2),(b'2',1)]);
    let mut games = input.lines().filter_map(|line| line.split_once(" "))
        .map(|(a, b)| (a.as_bytes().iter().map(|x| *score.get(x).unwrap() as usize), b.trim().parse::<usize>().unwrap()))
        .map(|(a, b)| (a.clone().fold(0, |acc, x| acc * 100 + x) + get_score(a),b))
        .collect::<Vec<_>>();
    games.sort_by(|(a,_),(b,_)| a.cmp(b));
    games.iter().enumerate().fold(0, |acc, (idx, (_, bet))| bet * (idx + 1) + acc )
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part1 -> {}", camel(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let res = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(camel(res), 6440);
    }
}
