fn main() {
    println!("Hello, world!");
    let v = [[1, 2], [3, 4], [5,6]];
    println!("{:?}", v);
    calc_bowling_score(v);
}

fn calc_bowling_score(pin_counts:&[&[i32]]) {
    println!("{:?}", pin_counts);
}
