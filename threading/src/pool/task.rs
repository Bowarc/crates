pub trait FnBox {
    fn call_box(self: Box<Self>, _: u16);
}

impl<F: FnOnce(u16)> FnBox for F {
    fn call_box(self: Box<F>, worker_id: u16) {
        (*self)(worker_id)
    }
}

pub type Task = Box<dyn FnBox + Send + 'static>;
