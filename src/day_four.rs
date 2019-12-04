fn passes(input: i32) -> bool {
    let input_string = input.to_string();
    if input_string.len() != 6 {
        return false;
    }
    let mut found_same = false;
    let mut last_letter = input_string.chars().next().unwrap();

    if !last_letter.is_digit(10) {
        return false;
    }
    for letter in input_string.chars().skip(1) {
        if !letter.is_digit(10) {
            return false;
        }
        if last_letter == letter {
            found_same = true;
        }
        if last_letter.to_digit(10) > letter.to_digit(10) {
            return false;
        }
        last_letter = letter;
    }
    found_same
}

fn second_task_pass(input: &str) -> bool {
    let mut sequence_counter = vec![(input.chars().next().unwrap(), 1)];
    for letter in input.chars().skip(1) {
        let (last_letter, letter_count) = sequence_counter.pop().unwrap();

        if last_letter == letter {
            sequence_counter.push((letter, letter_count+1));
        } else {
            sequence_counter.push((last_letter, letter_count));
            sequence_counter.push((letter, 1));
        }
    }
    sequence_counter.iter().any(|(_, count)| *count == 2)
}

pub fn one(){
    let input = "152085-670283";
    let mut inputs = input.split("-").filter_map(|s| s.parse::<i32>().ok());
    let start = inputs.next().unwrap();
    let end = inputs.next().unwrap();
    let mut counter = 0;
    for password in start..end {
        if passes(password) {
            counter+=1;
        }
    }
    println!("{}", counter);
}

pub fn two(){
    let input = "152085-670283";
    let mut inputs = input.split("-").filter_map(|s| s.parse::<i32>().ok());
    let start = inputs.next().unwrap();
    let end = inputs.next().unwrap();
    let mut counter = 0;
    for password in start..end {
        if passes(password) && second_task_pass(&password.to_string()) {
            counter+=1;
        }
    }
    println!("{}", counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_one() {
        assert!(passes(111111));
    }

    #[test]
    fn passes_two() {
        assert_eq!(passes(223450), false);
    }

    #[test]
    fn passes_three() {
        assert_eq!(passes(123789), false);
    }

    #[test]
    fn second_task_pass_one() {
        assert!(second_task_pass("112233"));
    }

    #[test]
    fn second_task_pass_two() {
        assert_eq!(second_task_pass("123444"), false);
    }

    #[test]
    fn second_task_pass_three() {
        assert!(second_task_pass("111122"));
    }
}