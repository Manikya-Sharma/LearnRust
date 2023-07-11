// Using threads to run code simultaneously

// use std::thread;
// use std::time::Duration;
//
// fn main() {
//   let handle = thread::spawn(|| {
//       for i in 1..10 {
//           println!("Hi number {} from the spawned thread!", i);
//           thread::sleep(Duration::from_millis(1));
//       }
//   });
//      handle.join().unwrap();
//   for i in 1..5 {
//       println!("Hi number {} from the main thread", i);
//       thread::sleep(Duration::from_millis(1));
//   }
//   handle.join().unwrap();
//
//     let v = vec![1, 2, 3];
//
//     let handle = thread::spawn(move || {
//         println!("Here'a a vector: {:?}", v);
//     });
//
//     handle.join().unwrap();
// }

// Using message passing to transfer data between threads

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;
// 
// fn main() {
    // Single Value
    //     let (tx, rx) = mpsc::channel();
    //     thread::spawn(move || {
    //         let val = String::from("hello");
    //         tx.send(val).unwrap();
    //     });
    //     let recieved = rx.recv().unwrap();
    //     println!("Got: {}", recieved);
    // Multiple values
//     let (tx, rx) = mpsc::channel();
//     let tx1 = tx.clone();
// 
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
// 
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
// 
//     for recieved in rx {
//         println!("Got: {}", recieved);
//     }
//
// }
//


// Mutex
// use std::sync::{Mutex, Arc};
// use std::rc::Rc;
// use std::thread;
// 
// fn main() {
// //     let m = Mutex::new(5);
// // 
// //     {
// //         let mut num = m.lock().unwrap();
// //         *num = 6;
// //     }
// // 
// //     println!("m = {:?}", m);
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];
//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }
// 
//     for handle in handles {
//         handle.join().unwrap();
//     }
// 
//     println!("Result: {}", counter.lock().unwrap());
// }



// Sync and Send traits

// Send: means value can be sent across multiple threads -> usually it is implemented by most of the types.
// Sync: Means it can be referenced from multiple threads

