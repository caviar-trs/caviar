mod trs;

fn main() {
    let start = "(== (+ x y) 0)";
    let end = "(== x -y)";
    println!("Simplifying expression:\n {}\n", start);
    trs::prove_time(start, end);
}
