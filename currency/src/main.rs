use std::thread;
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

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
    // let a = vec![1, 3, 5, 7];
    // let handle = thread::spawn( move ||{
    //     tx.send(a).unwrap();
    // });

    // let rev = rx.recv().unwrap();

    // println!("Got: {:?}", rev);
    thread::spawn(move || {
        let name_list = [
            String::from("Blueberry"),
            String::from("Cherry"),
            String::from("Apple"),
            String::from("Watermelon"),
            String::from("Strawberry"),
        ];

        // for val in name_list {
        //     tx.send(val).unwrap();
        //     thread::sleep(Duration::from_millis(1));
        // }
        name_list
            .into_iter()
            .for_each(|val| {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(1))
            });

    });

    // for rev in rx {
    //     println!("GOT: {:?}", rev);
    // }
    rx.into_iter()
        .for_each(|rev| {println!("Got: {:?}", rev)});
    let endup = start.elapsed();
    println!("total time: {:?}", endup);

}