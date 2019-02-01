pub trait Dirty {
    fn dirty() -> bool;
}

pub trait BuilderExt {
    fn hello_macro();
}

pub trait ReaderExt {
    fn hello_macro();
}