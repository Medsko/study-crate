use std::ops::Deref;
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

struct DiyBox<T> (T);

impl <T> DiyBox<T> {
    fn new(value: T) -> DiyBox<T> {
        DiyBox(value)
    }
}

impl<T> Deref for DiyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping pointer with data `{}`", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::List::{Cons, Nil};

    #[test]
    fn multiple_references_cons() {
        let oc_list = Rc::new(RcList::Cons(5, Rc::new(
            RcList::Cons(10, Rc::new(RcList::Nil)))));
        println!("Count after creating original cons list: {}", Rc::strong_count(&oc_list));
        let reference_a = RcList::Cons(3, Rc::clone(&oc_list));
        println!("Count after creating first reference: {}", Rc::strong_count(&oc_list));
        {
            let reference_b = RcList::Cons(4, Rc::clone(&oc_list));
            println!("Count after creating second reference: {}", Rc::strong_count(&oc_list));
        }
        println!("Count after second reference goes out of scope: {}", Rc::strong_count(&oc_list));
    }

    #[test]
    fn early_drop() {
        let pointer = CustomSmartPointer{
            data: String::from("Officer")
        };
        println!("Created a pointer.");
        drop(pointer);
        println!("Pointer dropped before going out of scope.");
        // println!("This is obviously invalid: {}", pointer.data);
    }

    #[test]
    fn drop_it_like_it_is_out_of_scope() {
        let boxy = CustomSmartPointer{
            data: String::from("Picard")
        };
        println!("Created a pointer.");
    }

    #[test]
    fn say_hello() {
        let name = DiyBox::new("Melle");

        hello(&name);
    }

    #[test]
    fn create_cons_list() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    #[test]
    fn diy_box() {
        let x = 5;
        let y = DiyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn first_box() {
        let b = Box::new(5);
        println!("b = {}", b);
    }

    #[test]
    fn box_vs_pointer() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);

        let y = Box::new(x);

        assert_eq!(5, *y);
    }
}
