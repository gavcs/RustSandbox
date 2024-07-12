fn main() {
    median();
}

fn median() {
    let mut vec = vec![23, 67, 38, 11, 94, 13];
    vec.sort();
    for elem in &vec {
        println!("{elem}");
    }
    let size = vec.len();
    match size%2{
        0 => {
            let l = size/2;
            let n1 = vec.get(l);
            let n2 = vec.get(l-1);
            if let Some(num1) = n1 {
                println!("num1 = {num1}");
                if let Some(num2) = n2 {
                    println!("num2 = {num2}");
                    let add = (num1 + num2);
                    let median = add as f32;
                    let median = median/2.0;
                    println!("The middle is {median}.");
                }
            }            
        },
        _ => {
            let l = (size + 1)/2;

            if let Some(num) = vec.get(l) {
                println!("The middle is {num}");
            };
        },
    };
}

fn stuff() {
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
