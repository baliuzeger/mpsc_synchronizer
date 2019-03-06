use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx_report_0, rx_report_0) = mpsc::channel();
    let (tx_report_1, rx_report_1) = mpsc::channel();
    let (tx_confirm_0, rx_confirm_0) = mpsc::channel();
    let (tx_confirm_1, rx_confirm_1) = mpsc::channel();
    let agent_a = thread::spawn(move || {
        loop {
            println!("a");
            tx_report_0.send(0).unwrap();
            thread::sleep(Duration::from_millis(100));
            if rx_confirm_0.recv().unwrap() == 1 {
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
    agent_a.join().expect("The sender thread has panicked");
    agent_b.join().expect("The sender thread has panicked");
    synchronizer.join().expect("The sender thread has panicked");
}
