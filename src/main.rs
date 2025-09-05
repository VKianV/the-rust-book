use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let mutex1 = Arc::new(Mutex::new(0));
    let mutex2 = Arc::new(Mutex::new(0));

    let m1_clone = Arc::clone(&mutex1);
    let m2_clone = Arc::clone(&mutex2);

    let t1 = thread::spawn(move || {
        let _guard1 = m1_clone.lock().unwrap(); // Lock mutex1
        thread::sleep(std::time::Duration::from_millis(100)); // Simulate work
        let _guard2 = m2_clone.lock().unwrap(); // Wait for mutex2
        println!("Thread 1: Acquired both locks");
    });

    let t2 = thread::spawn(move || {
        let _guard2 = mutex2.lock().unwrap(); // Lock mutex2
        thread::sleep(std::time::Duration::from_millis(100)); // Simulate work
        let _guard1 = mutex1.lock().unwrap(); // Wait for mutex1
        println!("Thread 2: Acquired both locks");
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
