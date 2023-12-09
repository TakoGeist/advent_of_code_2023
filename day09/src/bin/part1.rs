
fn mirage(input: &str) -> i32{
    input.lines()
        .map(|line| line.split(" ")
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<_>>())
        .fold(0,|acc,mut line| {
            let mut val = 0;
            while line.iter().any(|&x| x != 0){
                (0..line.len() - 1).for_each(|idx| line[idx] = line[idx + 1] - line[idx]);
                val += line.pop().unwrap();
            }
            acc + val
        })
}

fn main(){
    let input = include_str!("..\\input.txt");
    println!("Part1 -> {}", mirage(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let res = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(mirage(res), 114);
    }
}
