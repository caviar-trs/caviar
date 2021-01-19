mod trs;

fn main() {
    let start = "(% 0 x)";
    let end = "(1)";
    println!("Simplifying expression:\n {}\n", start);
    trs::prove_time(start, end);
}
