fn main() {
    println!("Hello, world!");

    let str1 = "1234567890".to_string();
    let str2 = "123".to_string();

    println!("The longest string is {}", longest(&str1, &str2));
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
