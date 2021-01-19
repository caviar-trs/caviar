mod trs;

fn main() {
    let start = "(|| (&& x y) (&& x z))";
    let end = "0";
    println!("Simplifying expression:\n {}\n", start);
    trs::prove_time(start, end);
}
