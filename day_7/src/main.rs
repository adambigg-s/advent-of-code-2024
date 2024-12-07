


use std::{env, fs, process, time::Instant};



fn main() {
    let envs: Vec<String> = env::args().collect();
    let mut testing: bool = false;
    let file_path: &str = envs.get(1).map(|path| path.as_str()).unwrap_or_else(|| {
        testing = true;
        "testing.txt"
    });
    let buffer: String = fs::read_to_string(file_path).unwrap_or_else(|err| {
        println!("buffer read error: {err}");
        process::exit(3);
    });
    if testing { println!("{buffer}"); }

    let solution: Solution = Solution::construct(&buffer);

    let s1 = Instant::now();
    let part_one = solution.solve_one();
    let p1 = s1.elapsed();

    let s2 = Instant::now();
    let part_two = solution.solve_two();
    let p2 = s2.elapsed();

    println!("part one: {}\ntime one: {:#?}", part_one.to_print(), p1);
    println!();
    println!("part two: {}\ntime two: {:#?}", part_two.to_print(), p2);
    println!();
}

struct Equation {
    answer: BigInt,
    params: Vec<BigInt>,
}

impl Equation {
    fn construct(answer: BigInt, params: Vec<BigInt>) -> Equation {
        Equation { answer, params }
    }
}

struct Solution {
    calibrations: Vec<Equation>,
}

impl Solution {
    fn construct(buffer: &str) -> Solution {
        let calibrations = buffer
            .lines()
            .filter_map(|line| {
                let mut tokens = line.split(':');
                let answer = tokens.next()?.trim().parse::<u128>().ok()?;
                let params = tokens.next()?
                    .split_whitespace()
                    .filter_map(|x| x.trim().parse::<u128>().ok())
                    .collect::<Vec<_>>();
                let answer = BigInt::from_std_int(answer);
                let params = params.iter().map(|p| BigInt::from_std_int(*p)).collect();
                Some(Equation::construct(answer, params))
            }).collect();
        Solution { calibrations }
    }

    fn solve_one(&self) -> BigInt {
        let mut total_calibration_result: BigInt = BigInt::default();
        for calibration in &self.calibrations {
            let target = &calibration.answer;
            let params = &calibration.params;
            let operators: [Operator; 2] = [Operator::Add, Operator::Mul];
            let operator_combinations = generate_combinations(params.len()-1, &operators);
            for opcom in &operator_combinations {
                if self.is_valid_equation(target.clone(), params, opcom) {
                    total_calibration_result = BigInt::add(&total_calibration_result, target);
                    break;
                }
            }
        }

        total_calibration_result
    }

    fn solve_two(&self) -> BigInt {
        let mut total_calibration_result: BigInt = BigInt::default();
        for calibration in &self.calibrations {
            let target = &calibration.answer;
            let params = &calibration.params;
            let operators: [Operator; 3] = [Operator::Add, Operator::Mul, Operator::Concat];
            let operator_combinations = generate_combinations(params.len()-1, &operators);
            for opcom in &operator_combinations {
                if self.is_valid_equation(target.clone(), params, opcom) {
                    total_calibration_result = BigInt::add(&total_calibration_result, target);
                    break;
                }
            }
        }

        total_calibration_result
    }

    fn is_valid_equation(&self, target: BigInt, params: &[BigInt], opcom: &[Operator]) -> bool {
        let mut total = params[0].clone();
        for i in 1..params.len() {
            let operator = opcom[i-1];
            let param = params[i].clone();
            match operator {
                Operator::Mul => {
                    total = BigInt::mul(&total, &param);
                }
                Operator::Add => {
                    total = BigInt::add(&total, &param);
                }
                Operator::Concat => {
                    total = BigInt::concat(&total, &param);
                }
            };
            if BigInt::lhsgreater(&total, &target) {
                return false;
            }
        }

        total == target
    }
}

fn generate_combinations(num: usize, operators: &[Operator]) -> Vec<Vec<Operator>> {
    fn generate_recursive(current: Vec<Operator>, num: usize, combinations: &mut Vec<Vec<Operator>>, operators: &[Operator]) {
        if current.len() == num {
            combinations.push(current);
            return;
        }
        for op in operators {
            let mut new_curr = current.clone();
            new_curr.push(*op);
            generate_recursive(new_curr, num, combinations, operators)
        }
    }

    let mut combinations: Vec<Vec<Operator>> = Vec::new();
    let curr: Vec<Operator> = Vec::new();
    generate_recursive(curr, num, &mut combinations, operators);
    combinations
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Mul,
    Add,
    Concat,
}

/// digits are stored in reverse order!! rememeber this
#[derive(Debug, Clone, PartialEq, Eq)]
struct BigInt {
    digits: Vec<u8>,
}

impl BigInt {
    fn default() -> BigInt {
        BigInt { digits: vec![0] }
    }

    fn from_std_int(int: u128) -> BigInt {
        let mut digits = Vec::new();
        let mut num = int;

        if num == 0 {
            digits.push(0);
        } else {
            while num > 0 {
                digits.push((num % 10) as u8);
                num /= 10;
            }
        }

        BigInt { digits }
    }

    fn lhsgreater(lhs: &BigInt, rhs: &BigInt) -> bool {
        match lhs.digits.len().cmp(&rhs.digits.len()) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => {
                for i in (0..lhs.digits.len()).rev() {
                    match lhs.digits[i].cmp(&rhs.digits[i]) {
                        std::cmp::Ordering::Greater => return true,
                        std::cmp::Ordering::Less => return false,
                        std::cmp::Ordering::Equal => continue,
                    }
                }

                false
            }
        }
    }

    fn add(lhs: &BigInt, rhs: &BigInt) -> BigInt {
        let mut result = Vec::new();
        let mut carry = 0;
        let max_len = lhs.digits.len().max(rhs.digits.len());

        for i in 0..max_len {
            let lhs_digit = if i < lhs.digits.len() { lhs.digits[i] } else { 0 };
            let rhs_digit = if i < rhs.digits.len() { rhs.digits[i] } else { 0 };

            let sum = lhs_digit + rhs_digit + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }

        if carry > 0 {
            result.push(carry);
        }

        BigInt { digits: result }
    }

    fn mul(lhs: &BigInt, rhs: &BigInt) -> BigInt {
        let mut result = vec![0; lhs.digits.len() + rhs.digits.len()];

        for i in 0..lhs.digits.len() {
            for j in 0..rhs.digits.len() {
                let product = lhs.digits[i] * rhs.digits[j] + result[i + j];
                result[i + j] = product % 10;
                result[i + j + 1] += product / 10;
            }
        }

        while result.len() > 1 && result[result.len()-1] == 0 {
            result.pop();
        }

        BigInt { digits: result }
    }

    fn concat(lhs: &BigInt, rhs: &BigInt) -> BigInt {
        let mut result = lhs.digits.clone();
        let mut other = rhs.digits.clone();
        result.reverse();
        other.reverse();
        result.append(&mut other);
        result.reverse();

        BigInt { digits: result }
    }

    fn to_print(&self) -> String {
        self.digits.iter().rev().map(|&dig| (dig + b'0') as char).collect()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_test() {
        let first = BigInt::from_std_int(3213334343);
        let second = BigInt::from_std_int(98734343434);

        let concat = BigInt::concat(&first, &second);
        println!("{}", concat.to_print());
        assert_eq!(concat.to_print(), "321333434398734343434");
    }

    #[test]
    fn ordering_test() {
        let bigger = BigInt::from_std_int(999999);
        let smaller = BigInt::from_std_int(2934);

        assert!(BigInt::lhsgreater(&bigger, &smaller));
    }
}
