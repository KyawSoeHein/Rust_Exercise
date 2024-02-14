use super::common::ask_user_input;

pub fn find_median() {
    let user_input = ask_user_input();
    let nums = covert_string_to_array(user_input);
    find_and_print_median(nums);
}

fn find_and_print_median(nums: Vec<i32>) -> Vec<i32> {
    let count = nums.len();

    if count % 2 == 0 {
        println!(
            "Median is {} and {}",
            nums[count / 2],
            nums[(count - 1) / 2]
        );
        return vec![nums[count / 2], nums[(count - 1) / 2]];
    } else {
        println!("Median is {}", nums[count / 2]);
        return vec![nums[count / 2]];
    }
}

fn covert_string_to_array(input: String) -> Vec<i32> {
    if input.is_empty() {
        panic!("String is empty");
    }

    let num_array: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    return num_array;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "String is empty")]
    fn test_convert_string_to_array_empty_string() {
        covert_string_to_array(String::from(""));
    }

    #[test]
    fn test_convert_string_to_array() {
        assert_eq!(
            covert_string_to_array(String::from("12 11 10 4")),
            [12, 11, 10, 4]
        );
    }

    #[test]
    fn test_median_odd() {
        assert_eq!(find_and_print_median(vec![1, 2, 3, 4, 5]), vec![3]);
    }

    #[test]
    fn test_median_even() {
        assert_eq!(find_and_print_median(vec![1, 2, 3, 4]), vec![3, 2]);
    }
}
