
pub fn parse_input(input: &String) -> [Vec<String>; 4] {

    if input.is_empty() {
        return [Vec::<String>::new(), Vec::<String>::new(), Vec::<String>::new(), Vec::<String>::new()];
    }

    let lines: Vec<String> = input.trim().split("\n")
        .map(|line| line.to_string())
        .collect();

    let chars: Vec<Vec<char>> = lines.iter().map(|str| str.chars().collect::<Vec<char>>()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let columns: Vec<String> = (0..width).map(|i| chars.iter().map(|row| row[i]).collect::<String>()).collect();
    
    let mut diag1: Vec<String> = vec![];
    for i in 0..width {
        let mut tmp = Vec::<char>::new();

        for (x,y) in (i..width).zip(0..height) {
            tmp.push(chars[y][x]);
        }

        diag1.push(tmp.iter().collect::<String>());
    }
    for i in 1..height { 
        let mut tmp = Vec::<char>::new();

        for (x,y) in (0..width).zip(i..height) {
            tmp.push(chars[y][x]);
        }

        diag1.push(tmp.iter().collect::<String>());
    }

    let mut diag2: Vec<String> = vec![];
    for i in 0..width {
        let mut tmp = Vec::<char>::new();

        for (x,y) in (0..=i).rev().zip(0..height) {
            tmp.push(chars[y][x]);
        }

        diag2.push(tmp.iter().collect::<String>());
    }
    for i in 1..height {
        let mut tmp = Vec::<char>::new();

        for (x,y) in (0..width).rev().zip(i..height) {
            tmp.push(chars[y][x]);
        }

        diag2.push(tmp.iter().collect::<String>());
    }

    return [lines, columns, diag1, diag2];
}

pub fn count_xmas(str: &String) -> usize{
    let mut count: usize = 0;
    count += str.match_indices("XMAS").count();
    count += str.match_indices("SAMX").count();
    count
}

pub fn count_total_xmas(arrays: &[Vec<String>; 4]) -> usize {
    let mut count: usize = 0;
    for arr in arrays.iter() {
        for str in arr.iter() {
            count += count_xmas(str);
        }
    }
    count
}

pub fn count_x_mas(input: &String) -> usize {
    let lines: Vec<String> = input.trim().split("\n")
        .map(|line| line.to_string())
        .collect();

    let chars: Vec<Vec<char>> = lines.iter().map(|str| str.chars().collect::<Vec<char>>()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut count: usize = 0;

    for row in 1..height-1 {
        for col in 1..width-1 {
            if chars[row][col] == 'A' {
                let diag_1 = vec![chars[row-1][col-1], chars[row][col], chars[row+1][col+1]].iter().collect::<String>();
                let diag_2 = vec![chars[row-1][col+1], chars[row][col], chars[row+1][col-1]].iter().collect::<String>();

                if (diag_1 == "MAS" || diag_1 == "SAM") && (diag_2 == "MAS" || diag_2 == "SAM")
                {
                    count += 1;
                }
            }
        }

    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_empty() {
        let input = String::from("");
        let result = parse_input(&input);
        assert_eq!(result, [Vec::<String>::new(), Vec::<String>::new(), Vec::<String>::new(), Vec::<String>::new()]);
    }

    #[test]
    fn test_parse_input_single_line() {
        let input = String::from("abcd");
        let result = parse_input(&input);
        assert_eq!(result, [vec!["abcd"], vec!["a", "b", "c", "d"], vec![], vec![]]);
    }

    #[test]
    fn test_parse_input_multiple_lines() {
        let input = String::from("abcd\nefgh\nijkl\nmnop");
        let result = parse_input(&input);
        assert_eq!(result, [
            vec!["abcd", "efgh", "ijkl", "mnop"],
            vec!["aeim", "bfjn", "cgko", "dhlp"],
            vec!["afkp", "bgl", "ch", "d", "ejo", "in", "m"],
            vec!["a", "be", "cfi", "dgjm", "hkn", "lo", "p"]
        ]);
    }
}


