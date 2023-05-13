use std::fmt::Debug;

pub trait PostPrintable {
    fn print_post(&self, string: &str);
}

impl<T: Debug> PostPrintable for T {
    fn print_post(&self, string: &str) {
        println!("{}: {:#?}", string, &self);
    }
}

pub trait PrintableAnd {
    fn print_and(&self, string: &str) -> Self;
}

impl<T: PostPrintable + Clone> PrintableAnd for T {
    fn print_and(&self, string: &str) -> Self {
        self.print_post(string);
        self.clone()
    }
}
