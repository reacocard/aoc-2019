use std::fs;

fn calc_fuel(mass: &i32) -> i32 {
    let fuel: i32 = (mass / 3) - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + calc_fuel(&fuel)
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("File read error.");
    let masses: Vec<i32> = contents
        .trim()
        .split("\n")
        .map(|x| x.parse::<i32>().expect("Not an int."))
        .collect();
    let fuels: Vec<i32> = masses.iter().map(|x| (x / 3) - 2).collect();
    let sum: i32 = fuels.iter().sum();
    println!("{}", sum);  // part1 answer

    let fuels2: Vec<i32> = masses.iter().map(calc_fuel).collect();
    let sum2: i32 = fuels2.iter().sum();
    println!("{}", sum2);  // part2 answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2examples() {
        assert_eq!(calc_fuel(&14), 2);
        assert_eq!(calc_fuel(&1969), 966);
        assert_eq!(calc_fuel(&100756), 50346);
    }
}
