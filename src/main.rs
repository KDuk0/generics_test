use std::fmt::Display;

fn generic_display<T: Display>(item: T) {
    println!("{}", item);

}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T> (T, T);

enum Option<T> {
    Some(T),
    None
}



fn main() {
   let a: &str = "42";
   let b: i64 = 42;

   generic_display(a);
   generic_display(b);

   let (x, y) = (4i16, 2i64);

   let point: Point<i64> = Point {
    x: x.into(),
    y
   };
}