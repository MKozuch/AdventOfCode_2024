use std::fmt::{self};

pub fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    let parse_args = |str: &str| -> Vec<i64> {
        str.split(" ")
            .map(|item| item.parse::<i64>().unwrap())
            .collect()
    };

    let parse_line = |line: &str| -> (i64, Vec<i64>) {
        let pair = line.split_once(": ").unwrap();
        (pair.0.parse::<i64>().unwrap(), parse_args(pair.1))
    };

    input
        .trim()
        .split("\n")
        .map(parse_line)
        .collect::<Vec<(i64, Vec<i64>)>>()
}

#[derive(Clone, Copy)]
enum Ops {
    Mul,
    Add,
    Concat,
}

impl Ops {
    pub fn calc(&self, a: i64, b: i64) -> i64 {
        match *self {
            Ops::Mul => a * b,
            Ops::Add => a + b,
            Ops::Concat => {
                let exp = (b as f32).log10().floor() as u32;
                let kek = a * 10_i64.pow(exp + 1);
                kek + b
            }
        }
    }
}

impl fmt::Display for Ops {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Ops::Mul => &"Mul",
                Ops::Add => &"Add",
                Ops::Concat => &"Concat",
            }
        )
    }
}

fn generate_ops_combinations(count: usize, ops: &Vec<Ops>) -> Vec<Vec<Ops>> {
    let mut out = Vec::<Vec<Ops>>::new();
    out.push(vec![]);

    for _ in 0..count {
        let mut new_out = Vec::<Vec<Ops>>::new();

        for op in ops {
            let mut kek = out.clone();
            for k in kek.iter_mut() {
                k.push(*op);
            }
            new_out.append(&mut kek);
        }

        out = new_out;
    }

    out
}

fn calculate_ops(ops: &Vec<Ops>, args: &Vec<i64>) -> i64 {
    let mut result = args[0];

    for i in 0..ops.len() {
        result = ops[i].calc(result, args[i + 1]);
    }

    return result;
}

fn validate_entry(test_value: &i64, args: &Vec<i64>, possible_ops: &Vec<Ops>) -> bool {
    let possible_ops = generate_ops_combinations(args.len() - 1, possible_ops);

    for ops in possible_ops {
        if calculate_ops(&ops, &args) == *test_value {
            return true;
        }
    }
    return false;
}

pub fn validate_entry_add_mul(test_value: &i64, args: &Vec<i64>) -> bool {
    return validate_entry(test_value, args, &vec![Ops::Add, Ops::Mul]);
}

pub fn validate_entry_add_mul_concat(test_value: &i64, args: &Vec<i64>) -> bool {
    return validate_entry(test_value, args, &vec![Ops::Add, Ops::Mul, Ops::Concat]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_ops_test() {
        let res = calculate_ops(&vec![Ops::Add], &vec![1, 2]);
        assert_eq!(res, 3);

        let res = calculate_ops(&vec![Ops::Mul], &vec![3, 2]);
        assert_eq!(res, 6);
    }

    #[test]
    fn generate_ops_combinations_test() {
        let combinations = generate_ops_combinations(6, &vec![Ops::Add, Ops::Mul]);
        assert_eq!(combinations.len(), 2_usize.pow(6));
    }

    #[test]
    fn generate_ops_combinations_with_concat_test() {
        let combinations = generate_ops_combinations(6, &vec![Ops::Add, Ops::Mul, Ops::Concat]);
        assert_eq!(combinations.len(), 3_usize.pow(6));
    }

    #[test]
    fn validate_entry_test() {
        let a = validate_entry_add_mul(&16, &vec![4, 4]);
        assert_eq!(a, true);

        let a = validate_entry_add_mul(&16, &vec![2, 2, 4]);
        assert_eq!(a, true);

        let a = validate_entry_add_mul(&16, &vec![1, 3]);
        assert_eq!(a, false);
    }

    #[test]
    fn validate_entry_2_test() {
        let a = validate_entry_add_mul_concat(&16, &vec![1, 6]);
        assert_eq!(a, true);

        let a = validate_entry_add_mul_concat(&16, &vec![1, 6, 1]);
        assert_eq!(a, true);

        let a = validate_entry_add_mul_concat(&16, &vec![1, 5, 1]);
        assert_eq!(a, true);
    }

    #[test]
    fn ops_calc_test() {
        let x = Ops::Concat.calc(100, 100);
        assert_eq!(x, 100100);
    }
}
