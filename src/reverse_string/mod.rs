pub fn reverse(input: &str) -> String {
    // create a string and reverse it.
    let reversed_string: String = input.chars().rev().collect();
    // publish the string.
    reversed_string
}