use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn check_the_water() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val.clone()).unwrap();
        println!("{}, this totally compiles.", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn out_to_sea() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn far_out_to_sea() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_waters() {
        check_the_water();
    }

    #[test]
    fn get_out_to_sea() {
        out_to_sea();
    }

    #[test]
    fn get_far_out() {
        far_out_to_sea();
    }

}