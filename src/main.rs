mod hello;
mod reverse_string;

fn main() {
    // 1. hello:
    let hello_works = hello::hello();
    println!("The hello module says: {}", hello_works);
    // 2. reversed string:
    let input = "cool";
    let reversal = reverse_string::reverse(input);
    println!("The reversed version of {} is {}.", input, reversal);
}
