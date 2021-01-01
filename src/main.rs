fn main() {
    println!("Hello, world!");
    let v = vec![
        vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 4],
        vec![6, 3], vec![7, 2], vec![8, 1], vec![9, 0], vec![0, 1]
    ];
    println!("{:?}", v);
    calc_bowling_score(v);
}

fn calc_bowling_score(pin_counts:Vec<Vec<i32>>) {
    println!("{:?}", pin_counts);
}

fn calc_bowling_frame_score(pin_counts:Vec<Vec<i32>>, frame_index:usize) -> i32 {
    let frame_score = &pin_counts[frame_index];
    if frame_score[0] == 10 {
        10 + calc_bowling_frame_score(pin_counts, frame_index + 1) + calc_bowling_frame_score(pin_counts, frame_index + 1)
    } else if frame_score[0] + frame_score[1] == 10 {
        10 + calc_bowling_frame_score(pin_counts, frame_index + 1)
    } else {
        frame_score[0]
    };
}
