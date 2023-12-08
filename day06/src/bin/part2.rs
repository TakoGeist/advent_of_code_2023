
fn boat_race(input: &str) -> u64{
    input.split_once("\n").into_iter()
        .map(|(a, b)| (a.split_once(":").unwrap().1.replace(" ", "").trim().parse::<u64>().unwrap(),
                                         b.split_once(":").unwrap().1.replace(" ", "").trim().parse::<u64>().unwrap()))
        .map(|(a, b)| (a, f64::sqrt((a * a - 4 * b) as f64)))
        .map(|(a,c)| ((a as f64 - c) / 2f64, (a as f64 + c) / 2f64))
        .map(|(a, b)| if a.round() == a && b.round() == b {b.ceil() as u64 - a.ceil() as u64 - 1}
                                else {b.ceil() as u64 - a.ceil() as u64})
        .product()
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part2 -> {}", boat_race(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let res = "Time:      7  15   30
        Distance:  9  40  200";
        assert_eq!(boat_race(res), 71503);
    }
}
