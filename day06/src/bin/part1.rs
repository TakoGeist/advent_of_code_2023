
fn boat_race(input: &str) -> u32{
    input.split_once("\n").into_iter()
        .map(|(a, b)| (a.split_ascii_whitespace().filter_map(|elem| elem.parse::<u32>().ok()))
                                    .zip(b.split_ascii_whitespace().filter_map(|elem| elem.parse::<u32>().ok())))
        .flatten()
        .map(|(a, b)| (a, f32::sqrt((a * a - 4 * b) as f32)))
        .map(|(a,c)| ((a as f32 - c) / 2f32, (a as f32 + c) / 2f32))
        .map(|(a, b)| if a.round() == a && b.round() == b {b.ceil() as u32 - a.ceil() as u32 - 1}
                                else {b.ceil() as u32 - a.ceil() as u32})
        .product()
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part1 -> {}", boat_race(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let res = "Time:      7  15   30
        Distance:  9  40  200";
        assert_eq!(boat_race(res), 288);
    }
}
