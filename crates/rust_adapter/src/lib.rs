use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait GestaltContext {
    type Output<T>;
    type CompositeOutput;

    fn new_output<T: Serialize>(&self, value: &T) -> Self::Output<T>;
    fn new_secret<T: Serialize>(&self, value: &T) -> Self::Output<T>;
    fn register_resource(
        &self,
        request: RegisterResourceRequest<Self::Output<()>>,
    ) -> Self::CompositeOutput;
    fn invoke_resource(
        &self,
        request: InvokeResourceRequest<Self::Output<()>>,
    ) -> Self::CompositeOutput;
}

pub trait GestaltOutput<T>: Clone {
    type Me<A>;

    fn map<B, F>(&self, f: F) -> Self::Me<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: DeserializeOwned,
        B: Serialize;

    fn add_to_export(&self, key: &str);

    fn combine<RESULT>(&self, others: &[&Self::Me<()>]) -> Self::Me<RESULT>;

    /// Forcefully changes the visible type of underlying Output
    ///
    /// Can be used to work around Pulumi provider incorrect types
    ///
    /// MUST NOT change the underlying value - only the projected type
    ///
    /// # Safety
    ///
    /// The underlying output must be of type `F`.
    unsafe fn transmute<F>(self) -> Self::Me<F>;

    #[doc(hidden)]
    fn drop_type(self) -> Self::Me<()> {
        unsafe { self.transmute::<()>() }
    }
}

pub trait GestaltCompositeOutput {
    type Output<T>;

    fn get_field<T>(&self, key: &str) -> Self::Output<T>;
}

pub struct RegisterResourceRequest<'a, OUTPUT> {
    pub type_: String,
    pub name: String,
    pub version: String,
    pub object: &'a [ResourceRequestObjectField<'a, OUTPUT>],
}

pub struct InvokeResourceRequest<'a, OUTPUT> {
    pub token: String,
    pub version: String,
    pub object: &'a [ResourceRequestObjectField<'a, OUTPUT>],
}

pub struct ResourceRequestObjectField<'a, OUTPUT> {
    pub name: String,
    pub value: &'a OUTPUT,
}
