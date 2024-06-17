enum Currency {
    Dollar(f64),
    Euro(f64),
}

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// impl Coin {
//     fn value_in_cents(&self) -> u8 {
//         match self {
//             Coin::Penny -> 1,
//             Coin::Nickel -> 5,
//             Coin::Dime -> 10,
//             Coin:: Quarter,
//         }
//     }
// }

impl Currency {
    fn convert(&self) -> Currency {
        match self {
            Currency::Dollar(amt) => {
                Currency::Euro(amt*0.93)
            }
            Currency::Euro(amt) => {
                Currency::Dollar(amt*1.07)
            }
        }
    }

    fn strfy(&self) -> String {
        match self {
            Currency::Dollar(amt) => format!("${:.2}", amt),
            Currency::Euro(amt) => format!("€{:.2}", amt),
        }
    }
}

fn main() {
    let d = Currency::Dollar(52.03);
    let e = d.convert();
    println!("d = {}", d.strfy());
    println!("e = {}", e.strfy());
}
