use std::{fs, str::FromStr};

fn main() {
    let input_path = "input/day_9.txt";
    let report_str = fs::read_to_string(input_path).unwrap();
    let report = report_str.parse::<OasisReport>().unwrap();

    let result: i32 = report.sum_of_next_values();
    println!("The sum of all next predictions is: {}", result);

    let result: i32 = report.sum_of_previous_values();
    println!("The sum of all previous predictions is: {}", result)
}

#[derive(Debug, PartialEq)]
struct Sequence(Vec<i32>);

impl Sequence {
    fn new(numbers: Vec<i32>) -> Self {
        Sequence(numbers)
    }

    fn to_difference_sequence(&self) -> Self {
        let numbers = self.0.windows(2).map(|nums| nums[1] - nums[0]).collect();
        Sequence::new(numbers)
    }

    fn is_null_sequence(&self) -> bool {
        self.0.iter().all(|&num| num == 0)
    }

    fn predict_next_value(&self) -> i32 {
        if self.is_null_sequence() {
            0
        } else {
            self.0.last().unwrap() + self.to_difference_sequence().predict_next_value()
        }
    }

    fn predict_previous_value(&self) -> i32 {
        if self.is_null_sequence() {
            0
        } else {
            self.0.first().unwrap() - self.to_difference_sequence().predict_previous_value()
        }
    }
}

#[derive(Debug)]
struct ParseSequenceError;

impl FromStr for Sequence {
    type Err = ParseSequenceError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let numbers = string
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(Sequence::new(numbers))
    }
}

#[derive(Debug)]
struct OasisReport(Vec<Sequence>);

impl OasisReport {
    fn new(sequences: Vec<Sequence>) -> Self {
        Self(sequences)
    }

    fn sum_of_next_values(&self) -> i32 {
        self.0.iter().map(|s| s.predict_next_value()).sum()
    }

    fn sum_of_previous_values(&self) -> i32 {
        self.0.iter().map(|s| s.predict_previous_value()).sum()
    }
}

#[derive(Debug)]
struct ParseOasisReportError;

impl FromStr for OasisReport {
    type Err = ParseOasisReportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sequences = s.lines().map(|l| l.parse::<Sequence>().unwrap()).collect();
        Ok(OasisReport::new(sequences))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{OasisReport, Sequence};

    #[test]
    fn it_predicts_the_next_value() {
        let sequence = Sequence::new(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(sequence.predict_next_value(), 18);

        let sequence = Sequence::new(vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(sequence.predict_next_value(), 28);

        let sequence = Sequence::new(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(sequence.predict_next_value(), 68);

        let sequence = Sequence::new(vec![-4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(sequence.predict_next_value(), 8);
    }

    #[test]
    fn it_predicts_the_previous_value() {
        let sequence = Sequence::new(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(sequence.predict_previous_value(), -3);

        let sequence = Sequence::new(vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(sequence.predict_previous_value(), 0);

        let sequence = Sequence::new(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(sequence.predict_previous_value(), 5);

        let sequence = Sequence::new(vec![-4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(sequence.predict_previous_value(), -5);
    }

    #[test]
    fn it_parses_a_sequence() {
        let sequence_str = "0 3 6 9 12 15";
        assert_eq!(
            Sequence::from_str(sequence_str).unwrap(),
            Sequence::new(vec![0, 3, 6, 9, 12, 15])
        );

        let sequence_str = "-6 -10 -14 -18 -22 -26";
        assert_eq!(
            Sequence::from_str(sequence_str).unwrap(),
            Sequence::new(vec![-6, -10, -14, -18, -22, -26]),
        )
    }
    #[test]
    fn it_solves_next_and_prev() {
        let report_str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let report = report_str.parse::<OasisReport>().unwrap();

        assert_eq!(report.sum_of_next_values(), 114);
        assert_eq!(report.sum_of_previous_values(), 2);
    }
}
