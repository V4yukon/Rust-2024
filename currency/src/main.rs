use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    // let handle = thread::spawn(move ||{
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         // thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // handle.join().unwrap();
    // for i in 1..5 {
    //     println!("hi number {i} from the main thread!");
    //     // thread::sleep(Duration::from_millis(1));
    // }


    /*move and thread::spawn */
    let a = vec![1, 3, 5, 7];
    let handle = thread::spawn( move ||{
        tx.send(a).unwrap();
    });

    let rev = rx.recv().unwrap();

    println!("Got: {:?}", rev);
}