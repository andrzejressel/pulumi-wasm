use crate::Output;

#[derive(Clone, Debug)]
pub(crate) struct NativeOutput {

}

pub(crate) struct NativeEngine {

}

impl NativeEngine {
    fn new<T>(&self, value: String, secret: bool) -> Output<T> {
        native_panic()
    }
}

fn native_panic() -> ! {
    panic!("NativeEngine is stub for now")
}