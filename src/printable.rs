use std::fmt::Debug;

pub trait PostPrintable {
    fn print(&self, string: &str);
}

impl<T: Debug> PostPrintable for T {
    fn print(&self, string: &str) {
        println!("\n{}: {:#?}", string, &self);
    }
}

pub trait PrintableAnd {
    fn print_and(self, string: &str) -> Self;
}

impl<T: PostPrintable> PrintableAnd for T {
    fn print_and(self, string: &str) -> Self {
        self.print(string);
        self
    }
}
