use std::io;

fn run_test_case(mut names: Vec<String>) -> usize {
    let mut count = 0;

    for i in 1..names.len() {
        if names[i] < names[i - 1] {
            count += 1;
            names.swap(i, i - 1);
        }
    }
    count
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("failed to read T");
    let t: u32 = buf.trim().parse().unwrap();
    buf.clear();

    for case_number in 0..t {
        io::stdin().read_line(&mut buf).expect("failed to read N");
        let n: usize = buf.trim().parse().unwrap();
        buf.clear();

        let mut names = Vec::with_capacity(n);
        for _ in 0..n {
            io::stdin()
                .read_line(&mut buf)
                .expect("failed to read name");
            // This stumped me for a bit.
            // .trim() returns a &str, so you need to use a to_string() to turn it into a String
            // I could do buf.clone().trim(), but it's then not obvious how to get the string from that
            let s = buf.trim().to_string();
            buf.clear();
            names.push(s);
        }

        println!("Case #{}: {}", case_number + 1, run_test_case(names));
    }
}
