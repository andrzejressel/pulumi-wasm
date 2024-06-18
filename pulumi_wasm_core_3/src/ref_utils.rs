use std::cell::Ref;

pub(crate) trait Mappable<'b, T> {
    fn map<U, F>(self, f: F) -> Ref<'b, U>
    where
        F: FnOnce(&T) -> &U;
}

impl<'b, T: 'b> Mappable<'b, T> for Ref<'b, T> {
    fn map<U, F>(self, f: F) -> Ref<'b, U>
    where
        F: FnOnce(&T) -> &U,
    {
        Ref::map(self, f)
    }
}
