// enum List{
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
fn main() {
    // Smart pointer are structs which implement Deref and Drop traits.

    // * Box<T>
    // Store data on a heap rather than stack.
    // Used when :-
    // 1. We don't know the size.
    // 2. Large amount of data to transfer ownership but not copy.
    // 3. Own a value which implements a particular trait.

    // let b = Box::new(5);
    // println!("b = {}", b); //prints b=5
    // // Example of cons list (Lisp version of Linked List):-
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // * Studying about Deref trait.
    // let x = 5;
    // let y = &x;  // or let y = Box::new(x);
    // // Box<T> points to a copied value of x.

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let x=5;
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // hello("rust");

    // let m = MyBox::new(String::from("rust"));
    // hello(&m); // Coercion happens because fo deref trait.
    // same as hello(&(*m)[..]);

    // DerefMut can be used to dereference mutable reference.

    // * Drop trait
    // let c =CustomSmartPointer{
    //     data:String::from("my stuff"),
    // };

    // let d = CustomSmartPointer{
    //     data:String::from("other stuff"),
    // };
    // println!("Custom pointers created.");

    // Disabling drop and using custom drop: Not suggested, but might be needed.

    // use std::mem::drop;
    // drop(c);

    // * Rc<T>, the reference counted smart pointer.
    // meant only for single threaded scenarios
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("Count after creating a:{}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("Count after creating b:{}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("Count after creating c:{}", Rc::strong_count(&a));
    // }
    // println!("Count after c goes out of scope:{}", Rc::strong_count(&a));

    // * RefCell<T> & interior mutability.
    // for single threaded.

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}",a);
    println!("b after = {:?}",b);
    println!("c after = {:?}",c);
}

#[derive(Debug)]
enum List{
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// struct MyBox<T>(T);

// use std::ops::Deref;

// impl <T> Deref for MyBox<T>{
//     type Target = T;

//     fn deref(&self) -> &Self::Target{
//         &self.0
//     }
//     // *y means *(y.deref())
// }

// impl<T> MyBox<T>{
//     fn new(x: T) -> MyBox<T>{
//         MyBox(x)
//     }
// }

// fn hello(name:&str){
//     println!("Hello, {name}!");
// }

// struct CustomSmartPointer{
//     data:String,
// }

// impl Drop for CustomSmartPointer{
//     fn drop(&mut self){
//         println!("Dropping CustomSmartPointer with data {}", self.data);
//     }
// }
