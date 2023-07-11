// Smart pointers implement Deref and Drop trait

// Box<T> : allocating values on the heap
// Rc<T> : a reference counting type enabling multiple ownership
// Ref<T>, RefMut<T> : accessed through RefCell<T>, enforcing borrowing at runtime.


// use crate::List::{Cons, Nil};

// fn main() {
//     /*
//     let b = Box::new(5);
//     println!("b = {}", b);
//     */
//     // recursive types can be implemented using Boxes
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }
// 
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }


// Deref

// 
// fn main() {
// //     let x =5;
// //     let y = MyBox::new(x);
// //     assert_eq!(5, x);
// //     assert_eq!(5, *y);
//     let m = MyBox::new(String::from("Rust"));
//     hello(&m);
// }
// 
// use std::ops::Deref;
// struct MyBox<T>(T);
// 
// impl<T> MyBox<T>{
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
// 
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// 
// fn hello(name: &str) {
//     println!("Hello, {name}");
// }

// Drop

// force drop can be done only using std::mem::drop
// 
// struct CustomSmartPointer {
//     data: String,
// }
// 
// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }
// 
// fn main() {
//     let c = CustomSmartPointer {
//         data: String::from("my stuff"),
//     };
//     let d = CustomSmartPointer {
//         data: String::from("other stuff"),
//     };
//     drop(c);
//     println!("c was droppped before end");
//     println!("CustomSmartPointers created.");
// }
// 



// Rc<T> -> Multiple owners for same data
// 
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
// use crate::List::{Cons, Nil};
// use std::rc::Rc;
// 
// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a={}", Rc::strong_count(&a));
//     let _b = Cons(3, Rc::clone(&a));
//     println!("count after creating b={}", Rc::strong_count(&a));
//     {
//         let _c = Cons(4, Rc::clone(&a));
//         println!("count after creating c={}", Rc::strong_count(&a));
//     }
//     println!("Count after c goes out of scope={}", Rc::strong_count(&a));
// }
//


// Interior mutability using RefCell<T>

fn main() {
    let x = 5;
//     let y = &mut x;
}

