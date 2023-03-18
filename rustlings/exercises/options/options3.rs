// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

I AM NOT DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    if let Some(ref y) = y {
        println!("Co-ordinates are {},{} ", y.x, y.y);
    } else{
        println!("No match");
    }
    y; // Fix without deleting this line.
}
