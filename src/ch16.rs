use mpsc::channel;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn use_thread() {
    let (tx,rx) = channel();

    let handle = thread::spawn(move || {
        println!("starting thread {}", thread::current().name().get_or_insert("thread_001"));
        thread::sleep(Duration::from_secs(2));
        let m = String::from("aa");
        tx.send(m).unwrap();
    });
    let s = rx.recv().unwrap();
    println!("finish main thread, got:{}", s);
    handle.join().unwrap();
}
mod tests {
    use crate::ch16::use_thread;

    #[test]
    fn test_use_thread() {
        use_thread();
    }
}