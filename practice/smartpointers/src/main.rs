struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some stuff"),
    };
    // let d = CustomSmartPointer {
    //     data: String::from("some other stuff"),
    // };

    println!("CustomSmartPointers created.");

    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");
}