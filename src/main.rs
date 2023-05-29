mod hello;
mod reverse_string;

fn main() {
    hello::hello();
    let input = "cool";
    let reversed_string = reverse_string::reverse(input);
    println!("Reversed: {}", reversed_string);
}
