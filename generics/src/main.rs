fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest numnber is : {}", largest(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!("The largest numnber is : {}", largest(&number_list));
    let integer = Point { x: 5, y: 10 };
    println!("p.x = {}", integer.x());
//    let float = Point { x: 1.0, y: 4.0 };
    let p1 = NewPoint{x: 5, y: 10.4};
    let p2 = NewPoint {x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}
impl Point<f32>{
    fn distance_from_origin(&self)->f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct NewPoint<X1, Y1>{
    x: X1,
    y: Y1,
}

impl<X1, Y1> NewPoint<X1, Y1>{
    fn mixup<X2, Y2>(self, other: NewPoint<X2, Y2>)->NewPoint<X1, Y2>{
        NewPoint {
            x: self.x,
            y: other.y,
        }
    }
}

