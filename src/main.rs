use std::thread;
use std::time::Duration;
extern crate crossbeam_channel;

fn main() {
    let (tx0, rx0) = crossbeam_channel::bounded(0);
    let (tx1, rx1) = crossbeam_channel::bounded(0);
    let agent_a = thread::spawn(move || {
        let mut c_a = 0;
        loop {
            println!("a{}", c_a);
            c_a += 1;
            thread::sleep(Duration::from_millis(30));
            if rx0.recv().unwrap() == 1 {
                break;
            }
        }});
    let agent_b = thread::spawn(move || {
        let mut c_b = 0;
        loop {
            println!("b{}", c_b);
            c_b += 1;
            thread::sleep(Duration::from_millis(20));
            if rx1.recv().unwrap() == 1 {
                break;
            }
        }});
    let synchronizer = thread::spawn(move || {
        let mut counter = 0;
        loop {
            thread::sleep(Duration::from_millis(100));
            if counter > 4 {
                tx0.send(1).unwrap();
                tx1.send(1).unwrap();
                break;
            } else {
                tx0.send(0).unwrap();
                tx1.send(0).unwrap();
            }
            println!("{}", counter);
            counter += 1;
        }});
    agent_a.join().expect("The sender thread has panicked");
    agent_b.join().expect("The sender thread has panicked");
    synchronizer.join().expect("The sender thread has panicked");
}
