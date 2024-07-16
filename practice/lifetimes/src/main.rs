/// This is kinda just a self note :P
/// I need to start leaving documentation as some form of "notes" because I feel like
/// it'll better help me understand what's happening


    // the 'a generic lifetime means that instances of this struct cannot outlive the text parameter that it holds
struct SomeText<'a> {
    text: &'a str,
}

fn main() {
    let x = String::from("abcd");
    let longest;
    {
        let y = String::from("hi");
        longest = longer(x.as_str(), y.as_str());
    }
    println!("The longest string is {longest}");
}

    // initially the function was declared as:
        // "fn longer(x: &str, y: &str) -> &str {"
    // this causes issues with lifetimes. The borrow checker isn't sure how long x and y should
    // exist for, so adding explicit lifetimes allows it to be either one because either one could
    // be returned. It means that each variable defined with the generic lifetime 'a, should live as
    // long as the generic lifetime 'a.
        
// simplified: the generic lifetime variable 'a basically says that any variables declared with it should
// only live as long as the shortest lived one. Let's say x is longer than y, meaning that once x is returned,
// y should no longer be living. It means that y is dropped if x is returned from what I understand. Unless ofc y is then needed?
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}