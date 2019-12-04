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
