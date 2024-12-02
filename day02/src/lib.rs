type Level = i64;
type Report = Vec<Level>;


pub fn parse_input(input: &String) -> Vec<Vec<i64>> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(char::is_whitespace)
                .map(|str| str.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn is_safe(report: &Report) -> bool {

    let diff_min= 1;
    let diff_max = 3;

    assert!(report.len() > 1);

    let init_slope = (report[1]-report[0]).signum();

    for i in 0..(report.len()-1){
        let diff = report[i+1] - report[i];

        if diff == 0 || diff.signum() != init_slope || diff.abs() < diff_min || diff.abs() > diff_max { return false };
    }

    true
}

pub fn is_safe_with_dampener(report: &Report) -> bool {

    if is_safe(report) { return true };
    
    for i in 0..(report.len()){
        let mut fixed_report = report.clone();
        fixed_report.remove(i);
        if is_safe(&fixed_report) {return true};
    }

    return false;
}




#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn is_safe_test(){
        assert!(is_safe(&vec![7, 6, 4, 2, 1]) == true);
        assert!(is_safe(&vec![1, 2, 7, 8, 9]) == false);
        assert!(is_safe(&vec![9, 7, 6, 2, 1]) == false);
        assert!(is_safe(&vec![1, 3, 2, 4, 5]) == false);
        assert!(is_safe(&vec![8, 6, 4, 4, 1]) == false);
        assert!(is_safe(&vec![1, 3, 6, 7, 9]) == true);

        assert!(is_safe(&vec![81, 85, 88, 89, 91, 93]) == false);
        assert!(is_safe(&vec![56, 61, 63, 65, 68, 71, 73]) == false);
    }

    #[test]
    fn is_safe_with_dampener_test(){
        assert!(is_safe_with_dampener(&vec![7, 6, 4, 2, 1]) == true);
        assert!(is_safe_with_dampener(&vec![1, 2, 7, 8, 9]) == false);
        assert!(is_safe_with_dampener(&vec![9, 7, 6, 2, 1]) == false);
        assert!(is_safe_with_dampener(&vec![1, 3, 2, 4, 5]) == true);
        assert!(is_safe_with_dampener(&vec![8, 6, 4, 4, 1]) == true);
        assert!(is_safe_with_dampener(&vec![1, 3, 6, 7, 9]) == true);

        // assert!(is_safe(&vec![81, 85, 88, 89, 91, 93]) == false);
        // assert!(is_safe(&vec![56, 61, 63, 65, 68, 71, 73]) == false);
    }
}