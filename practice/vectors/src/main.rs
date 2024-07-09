fn main() {
    let mut v: Vec<i32> = Vec::new();   // or let v = vec![1, 2, 3]; to initialize with values

    // add values to the vector
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let third: &i32 = &v[2];
    // gets the 3rd value in the vector

    println!("The third element in the vector is {third}");

    let third: Option<&i32> = v.get(2);
    // this also gets the 3rd value in the vector but as an option

    match third{
        Some(third) => println!("The third element in the vector is {third}"),
        None => println!("There is no third element"),
    }
}
