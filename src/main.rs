fn main() {
    println!("Hello, world!");

    example1();
    example2();
}

fn example1() {
    println!("example 1");

    let str1 = "1234567890".to_string();
    let str2 = "123".to_string();

    println!("The longest string is {}", longest1(&str1, &str2));
}

fn longest1<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn example2() {
    println!("example 2");

    let long = "123456".to_string();

    {
        let short = "123".to_string();

        let res = longest2(&long, &short);

        println!("The longest string is {}", res);
    }
}

fn longest2<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
