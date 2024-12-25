aoc::parts!(1, 2);

use std::collections::{HashMap, HashSet};

fn count_group_answers(group: &str) -> usize {
    let mut unique_answers = HashSet::new();
    for person in group.lines() {
        for answer in person.chars() {
            unique_answers.insert(answer);
        }
    }
    unique_answers.len()
}

fn count_everyone_answered(group: &str) -> usize {
    let mut answer_counts = HashMap::new();
    let mut num_people = 0;

    for person in group.lines() {
        num_people += 1;
        for answer in person.chars() {
            *answer_counts.entry(answer).or_insert(0) += 1;
        }
    }

    answer_counts
        .values()
        .filter(|&&count| count == num_people)
        .count()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let sum_of_counts: usize = input.raw().split("\n\n").map(count_group_answers).sum();

    sum_of_counts
}

fn part_2(input: aoc::Input) -> impl ToString {
    let sum_of_counts: usize = input.raw().split("\n\n").map(count_everyone_answered).sum();

    sum_of_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_group_answers_single_person() {
        assert_eq!(count_group_answers("abcx"), 4);
    }

    #[test]
    fn test_count_group_answers_multiple_persons() {
        assert_eq!(count_group_answers("abcx\nabcy\nabcz"), 6);
    }

    #[test]
    fn test_count_group_answers_empty_group() {
        assert_eq!(count_group_answers(""), 0);
    }

    #[test]
    fn test_part_1_example() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        let groups: Vec<&str> = input.split("\n\n").collect();
        let sum_of_counts: usize = groups.iter().map(|g| count_group_answers(g)).sum();
        assert_eq!(sum_of_counts, 11);
    }

    #[test]
    fn test_count_everyone_answered_group_1() {
        assert_eq!(count_everyone_answered("abc"), 3);
    }

    #[test]
    fn test_count_everyone_answered_group_2() {
        assert_eq!(count_everyone_answered("a\nb\nc"), 0);
    }

    #[test]
    fn test_count_everyone_answered_group_3() {
        assert_eq!(count_everyone_answered("ab\nac"), 1);
    }

    #[test]
    fn test_count_everyone_answered_group_4() {
        assert_eq!(count_everyone_answered("a\na\na\na"), 1);
    }

    #[test]
    fn test_count_everyone_answered_group_5() {
        assert_eq!(count_everyone_answered("b"), 1);
    }

    #[test]
    fn test_part_2_example() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        let groups: Vec<&str> = input.split("\n\n").collect();
        let sum_of_counts: usize = groups.iter().map(|g| count_everyone_answered(g)).sum();
        assert_eq!(sum_of_counts, 6);
    }
}
