use std::fmt::Debug;

pub trait Printable {
    fn print(&self);
    fn print_pre(&self, string: &str);
}

impl<T: Debug> Printable for T {
    fn print(&self) {
        println!("{:?}", &self);
    }
    fn print_pre(&self, string: &str) {
        println!("{}{:?}", string, &self);
    }
}

pub trait PrintableAnd {
    fn print_and(&self) -> Self;
    fn print_pre_and(&self, string: &str) -> Self;
}

impl<T: Printable + Clone> PrintableAnd for T {
    fn print_and(&self) -> Self {
        self.print();
        self.clone()
    }
    fn print_pre_and(&self, string: &str) -> Self {
        self.print_pre(string);
        self.clone()
    }
}
