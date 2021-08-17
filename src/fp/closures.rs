use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T> where T: Fn(u32) -> u32, {
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32, {

    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(val) => *val,
            None => {
                let calculated_value = (self.calculation)(arg);
                self.values.insert(arg, calculated_value);
                calculated_value
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn simulated_expensive_calculation_closure() -> fn(u32) -> u32 {
    |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }
}

// ...and what if the closure returning a closure returns a closure ok I'm done.
fn ridiculous_closure() -> fn(u32) -> fn(u32) -> u32 {
    |_| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        |_| {
            12
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|_| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;

    #[test]
    fn simulated_expensive_calculation_closure() {
        let closure = super::simulated_expensive_calculation_closure();
        assert_eq!(12, closure(12));
    }

    #[test]
    fn insanity_is_accepted() {
        let closure = super::ridiculous_closure();
        let inner_closure = closure(31);

        assert_eq!(12, inner_closure(7));
    }

    #[test]
    fn returns_cached_value() {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        let start = SystemTime::now();
        expensive_result.value(12);
        assert_eq!(12, expensive_result.value(12));

        let test_time = SystemTime::duration_since(&SystemTime::now(), start);
        assert!(test_time.is_ok());
        assert!(test_time.unwrap().as_secs() < 4);
    }

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn move_value_closure() {
        let x = vec![1, 2, 3];

        let equal_to_x = move |z| z == x;

        // println!("can't use x here: {:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }
}