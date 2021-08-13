use std::thread;
use std::time::Duration;

fn never_gets_old() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn move_that_vec() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // This will obviously not compile
    // v.iter().for_each(|i| println!("Number: {}", i));

    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrency_is_black_magic() {
        never_gets_old();
    }

    #[test]
    fn vec_moves() {
        move_that_vec();
    }

}