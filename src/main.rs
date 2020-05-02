fn main() {
    println!("Hello, world!");

    example1();
    example2();
    example3();
    example4();
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

fn example3() {
    println!("example 3");

    let g = Game { level: &3 };

    println!("The level is {}", g.level(&5))
}

struct Game<'a> {
    level: &'a i32,
}

impl Game<'_> {
    fn level(&self, x: &i32) -> &i32 {
        self.level
    }
}

fn example4() {
    println!("example 4");

    let x = 5;
    let r;

    {
        let y = 10;
        r = x_or_const(&x, &y);
    }

    println!("r is {}", r);
}

const DAN: i32 = 241;

struct Coordinate<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn x_or_const<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if x > y {
        x
    } else {
        &DAN
    }
}
