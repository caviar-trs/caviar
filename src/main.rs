mod trs;

fn main() {
    let start = "(&& (&& x (|| y z)) y)";
    let end = "(&& x y)";
    println!("Simplifying expression:\n {}\n", start);
    trs::prove_time(start, end);
}
