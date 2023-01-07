// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);  //requires traits

    let pt1 = Point { x: 5, y: 10 };
    let pt2 = Point { x: 1.0, y: 4.0 };
    let pt3 = GenPoint { x: 5, y: 3.5 };
    println!("pt1.x={}", pt1.x());
    let pt4 = GenPoint{x:"Hello", y:'c'};
    let p5 = pt3.mixup(pt4);
    println!("p5.x={}, p5.y={}", p5.x, p5.y);
}

// Functions:-

/*
fn name<T>(...)->...{...}
*/

//structs
struct Point<T> {
    x: T,
    y: T,
}

struct GenPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> GenPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: GenPoint<X2, Y2>) -> GenPoint<X1, Y2> {
        GenPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// Enums
/*

enum Option<T>{
    Some(T),
    None,
}

enum Result<T, E>{
    Ok(T),
    Err(E),
}

*/
