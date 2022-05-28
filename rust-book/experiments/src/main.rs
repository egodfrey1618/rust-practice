fn _invalid_array_index() {
    // Do something complicated so Rust can't infer this will panic
    let mut y = 0;
    for i in 1..30 {
        if i * i == 19 * 19 {
            y = i
        }
    }

    // Experiment: Does accessing an invalid array index panic if you have --release set?
    let a = [3; 3];
    println!("{}", a[y]);

    // Answer: Yes it does! So array indices must be kept track of
}

fn is_for_a_expression() {
    // Yes, for loops are expressions!
    // But they can't be non-trivial values - you can't break with something that isn't a unit
    let x = for _ in 1..4 {};

    println!("{:?}", x)
}

fn references() {
    let mut y = 2;
    let x = &mut y;
    // this wouldn't compile
    // let z = &y;

    println!("{}", x);
}

fn main() {
    // invalid_array_index();
    is_for_a_expression();
    references();
}
