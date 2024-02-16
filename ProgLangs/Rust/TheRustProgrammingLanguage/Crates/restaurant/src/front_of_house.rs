// can define modules inside of a module
pub mod hosting;
mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}