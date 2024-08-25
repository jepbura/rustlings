// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long1");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("The longest string is '{result}'");
    }

    let string1 = String::from("long string is long2");
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}
