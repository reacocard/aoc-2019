use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("File read error.");
    let base_code: Vec<usize> = contents
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().expect("Not an int."))
        .collect();
    let mut code = base_code.to_vec();
    code[1] = 12;
    code[2] = 2;
    let result = execute_intcode(code);
    println!("{}", result[0]); // part1 answer

    for amount in 0..1000 {
        for noun in 0..(amount + 1) {
            let verb = amount - noun;
            code = base_code.to_vec();
            code[1] = noun;
            code[2] = verb;
            let result = execute_intcode(code);
            if result[0] == 19690720 {
                println!("{}", 100 * noun + verb); // part2 answer
                std::process::exit(0);
            }
        }
    }
}

fn execute_intcode(mut code: Vec<usize>) -> Vec<usize> {
    let mut position: usize = 0;
    loop {
        let instruction = code[position];
        match instruction {
            99 => break,
            1 => {
                let arg1 = code[code[position + 1]];
                let arg2 = code[code[position + 2]];
                let arg3 = code[position + 3];
                code[arg3] = arg1 + arg2;
            }
            2 => {
                let arg1 = code[code[position + 1]];
                let arg2 = code[code[position + 2]];
                let arg3 = code[position + 3];
                code[arg3] = arg1 * arg2;
            }
            _ => panic!("Unexpected instruction {}.", instruction),
        }
        position += 4;
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1example1() {
        let code: Vec<usize> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected: Vec<usize> = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let result = execute_intcode(code);
        assert_eq!(result, expected);
    }
    #[test]
    fn part1example2() {
        let code: Vec<usize> = vec![1, 0, 0, 0, 99];
        let expected: Vec<usize> = vec![2, 0, 0, 0, 99];
        let result = execute_intcode(code);
        assert_eq!(result, expected);
    }
    #[test]
    fn part1example3() {
        let code: Vec<usize> = vec![2, 3, 0, 3, 99];
        let expected: Vec<usize> = vec![2, 3, 0, 6, 99];
        let result = execute_intcode(code);
        assert_eq!(result, expected);
    }
    #[test]
    fn part1example4() {
        let code: Vec<usize> = vec![2, 4, 4, 5, 99, 0];
        let expected: Vec<usize> = vec![2, 4, 4, 5, 99, 9801];
        let result = execute_intcode(code);
        assert_eq!(result, expected);
    }
    #[test]
    fn part1example5() {
        let code: Vec<usize> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected: Vec<usize> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let result = execute_intcode(code);
        assert_eq!(result, expected);
    }
}
