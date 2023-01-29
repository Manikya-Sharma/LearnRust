use std::rc::Rc;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // * Intro
    // let handle = thread::spawn(|| {
    //     for i in 1..10{
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // // handle.join().unwrap(); //spawn runs first then the main
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1)); // Once main thread ends, other thread also stop.
    // }
    // // handle.join().unwrap(); // both run concurrently and completely

    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(move || {
    // // move causes a change in ownership so that v cannot be changed in main thread.
    //     println!("Here's a vector {:?}", v);
    // });
    // handle.join().unwrap();

    // * Message passing

    // // using channels implementation.

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //     // tx.send(val).unwrap(); // send returns Result<T, E>
    //     for val in vals{
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // // let received = rx.recv().unwrap();
    // // println!("Got: {}", received);

    // for received in rx{
    //     println!("Got:{}", received);
    // }

    // // Creating multiple producers

    // let (tx, rx) = mpsc::channel();

    // let tx1 = tx.clone();
    // thread::spawn(move ||{
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //     for val in vals{
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // thread::spawn(move|| {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //     for val in vals{
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx{
    //     println!("Got {}", received);
    // }

    // * Shared State Concurrency

    // multiple threads access same shared data. //not recommended in other languages.

    // mutex (Mutual Exclusion) is used.
    // let m =Mutex::new(5); // Mutex<T> is a smart pointer.
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}", m);

    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    // for _ in 0..10{
    //     let handle = thread::spawn(move|| {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles{
    //     handle.join().unwrap();
    // }
    // println!("Result : {}", *counter.lock().unwrap());
    // // !ERROR because multiple threads cannot take ownership of same
    // // ! variable, so we will have to use Rc.
    // But Rc is not concurrency safe, so Arc<T> is used.

    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 1..10{
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles{
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", counter.lock().unwrap()); // Wont work

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // * Mutex<T> comes with the risk of deadlock situation.
}
