use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn sleep_sort(vls: Vec<u64>) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for val in vls {
            let txc = tx.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(val));
                txc.send(val).unwrap();
            });
        }
    });

    let mut vls1 = vec![];

    for received in rx {
        println!("val:{}", received);
        vls1.push(received);
    }

    println!("Got: {:?}", vls1);
}

fn main() {
    let vls = vec![2, 4, 5, 6, 1, 8, 9];
    sleep_sort(vls);
}
