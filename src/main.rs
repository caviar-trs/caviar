mod trs;

fn main() {
    let start = "(|| (&& x y) x)";
    let end = "(&& ?x (|| ?x ?y))";
    println!("Simplifying expression:\n {}\n", start);
    trs::prove_time(start, end);
}
