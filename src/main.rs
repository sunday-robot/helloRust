fn main() {
    println!("Hello, world!");
    let v = vec![vec![1, 2], vec![3, 4], vec![5,6], vec![7, 8, 9]];
    println!("{:?}", v);
    calc_bowling_score(v);
}

fn calc_bowling_score(pin_counts:Vec<Vec<i32>>) {
    println!("{:?}", pin_counts);
}
