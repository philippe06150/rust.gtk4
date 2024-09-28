use graphique::Courbe;

fn main() {
    let x = Courbe::new();
    for i in 1..10 {
        toto(&x, i as u8);
    }
    println!("{}", x);
}

fn toto(c: &Courbe, i: u8) {
    c.add(i);
}

mod graphique {
    use std::cell::RefCell;
    use std::fmt;
    use std::rc::Rc;

    pub struct Courbe(Rc<RefCell<Vec<u8>>>);

    impl Courbe {
        pub fn new() -> Courbe {
            Courbe(Rc::new(RefCell::new(Vec::<u8>::new())))
        }
        pub fn add(&self, x: u8) {
            self.0.borrow_mut().push(x);
        }
    }

    impl std::fmt::Display for Courbe {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = self.0.borrow();
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }
            write!(f, "]")
        }
    }
}
