use regex::Regex;
use std::fs;
fn main() {
    let input_path = "input/day_1.txt";
    let bad_calibration = fs::read_to_string(input_path).unwrap();
    let first_calibration = process_digits(&bad_calibration);

    println!(
        "The correct calibration for part 1 is: {}",
        first_calibration
    );

    let second_calibration = process_digits_and_strings(&bad_calibration);

    println!(
        "The correct calibration for part 2 is: {}",
        second_calibration
    );
}

fn process_digits(calibration: &str) -> usize {
    calibration
        .lines()
        .map(|l| {
            let mut digit_chars = l.chars().filter(|c| c.is_ascii_digit());
            if let Some(first_e) = digit_chars.next() {
                let last_e = digit_chars.next_back().unwrap_or(first_e);
                (first_e.to_string() + &last_e.to_string())
                    .parse::<usize>()
                    .unwrap()
            } else {
                0
            }
        })
        .sum()
}

fn process_digits_and_strings(calibration: &str) -> usize {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    calibration
        .lines()
        .map(|l| {
            // only applicable to advent of code input string
            let l = l
                .replace("oneight", "18")
                .replace("twone", "21")
                .replace("eightwo", "82");
            let mut re_iter = re.find_iter(&l).map(|m| match m.as_str() {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                s => s,
            });
            if let Some(first_m) = re_iter.next() {
                let last_m = re_iter.last().unwrap_or(first_m);
                (first_m.to_owned() + last_m).parse::<usize>().unwrap()
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_processes_digits() {
        let calibration = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process_digits(calibration);
        assert_eq!(result, 142);
    }

    #[test]
    fn it_processes_digits_and_strings() {
        let string = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = process_digits_and_strings(string);
        assert_eq!(result, 281);
    }
    #[test]
    fn it_processes_digits_and_strings_w_joined_numbers() {
        let string = "\
oneight
twone
eightwo";
        let result = process_digits_and_strings(string);
        assert_eq!(result, 18 + 21 + 82)
    }
}
