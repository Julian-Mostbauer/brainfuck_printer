use std::io;

fn find_produkts(x: f32) -> [u32; 3] {
    let prod1: f32 = x.sqrt().floor();
    let prod2: f32 = (x / prod1).floor();

    let result: [u32; 3] = [prod1 as u32, prod2 as u32, (x - prod1 * prod2) as u32];
    return result;
}

// >(x*+)[<(y*+)>-]<(z*+).>

fn main() {
    println!("Enter your desired Output:");

    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();

    for c in input_text.trim().chars() {
        let prods = find_produkts((c as u32) as f32);

        print!(">");
        for _i in 0..prods[0] {
            print!("+");
        }

        print!("[<");
        for _i in 0..prods[1] {
            print!("+");
        }

        print!(">-]<");
        for _i in 0..prods[2] {
            print!("+");
        }
        println!(".>");
    }
    println!("Thanks for using JM-Software :) [Enter to close]");
    io::stdin().read_line(&mut input_text).unwrap();
}
