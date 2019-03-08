/// questions:
/// 1.
/// can diferent thread  use Rc or Arc to reference to identical target?
/// consider: nerons in different threads Rc to the same connections.
/// if use mpsc::channel to build network, i.e. devices only own the ends of communication,
/// then how to implement imitation learning algorithm?
/// 2.
/// do thread.join pass back all the owned variables to the main thread?


use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::{Mutex, Arc};

fn main() {
    let (tx_report_0, rx_report_0) = mpsc::channel();
    let (tx_report_1, rx_report_1) = mpsc::channel();
    let (tx_confirm_0, rx_confirm_0) = mpsc::channel();
    let (tx_confirm_1, rx_confirm_1) = mpsc::channel();
    // let (tx_thread, rx_main) = mpsc::channel();
    
    let t0 = Arc::new(Mutex::new(1));    
    println!("t0 initial: {:?}.", t0);
    let t1 = Arc::clone(&t0);
    
    let agent_a = thread::spawn(move || {
        loop {
            println!("a");
            tx_report_0.send(0).unwrap();
            thread::sleep(Duration::from_millis(100));
            // println!("t0 in agent_a: {:?}.", t0);
            println!("t1 in agent_a: {:?}.", t1);
            *t1.lock().unwrap() += 1;
            if rx_confirm_0.recv().unwrap() == 1 {
                // tx_thread.send(t1).unwrap();
                // tx_thread.send(1).unwrap();
                break;
            }
        }});
    
    let agent_b = thread::spawn(move || {
        loop {
            println!("b");
            tx_report_1.send(0).unwrap();
            if rx_confirm_1.recv().unwrap() == 1 {
                break;
            }
        }});

    let synchronizer = thread::spawn(move || {
        let mut counter = 0;
        loop {
            rx_report_0.recv().unwrap();
            rx_report_1.recv().unwrap();
            println!("{}", counter);
            thread::sleep(Duration::from_millis(200));
            counter += 1;
            if counter > 4 {
                tx_confirm_0.send(1).unwrap();
                tx_confirm_1.send(1).unwrap();
                break;
            } else {
                tx_confirm_0.send(0).unwrap();
                tx_confirm_1.send(0).unwrap();
            }
        }});

    // println!("t1 before recv: {:?}.", t0);
    // let t1 = rx_main.recv().unwrap();
    // println!("t1 after recv: {:?}.", t0);
    agent_a.join().expect("The sender thread has panicked");
    agent_b.join().expect("The sender thread has panicked");
    synchronizer.join().expect("The sender thread has panicked");

    println!("t0 after join: {:?}.", t0);
}
