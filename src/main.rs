mod trs;

fn main() {
    let start = "(<= (+ v0 -70) (+ (* (/ (- v0 v1)71)71) v1))";
    let end = "1";
    println!("Simplifying expression:\n {}\n", start);
    trs::prove_time(start, end);
}
