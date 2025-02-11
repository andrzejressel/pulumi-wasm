use std::marker::PhantomData;
use pulumi_gestalt_rust_adapter::GestaltOutput;
use pulumi_gestalt_rust_adapter_native_simple as simple;

struct NativeOutput<T> {
    inner: simple::CustomOutputId,
    tpe: PhantomData<T>
}

impl <T> Clone for NativeOutput<T> {
    fn clone(&self) -> Self {
        NativeOutput {
            inner: self.inner.clone(),
            tpe: PhantomData
        }
    }
}

struct NativeContext {
    inner: simple::PulumiEngine
}

impl NativeContext {
    fn new() -> NativeContext {
        NativeContext {
            inner: simple::PulumiEngine::create_engine()
        }
    }
}

impl <T> GestaltOutput<T> for NativeOutput<T> {
    type Me<A> = NativeOutput<A>;
    type OutputId = ();

    fn map<B, F>(&self, f: F) -> Self::Me<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: serde::de::DeserializeOwned,
        B: serde::ser::Serialize
    {
        todo!()
    }

    fn add_to_export(&self, key: &str) {
        self.inner.add_export(key.to_string());
    }

    fn combine<RESULT>(&self, others: &[&Self::OutputId]) -> Self::Me<RESULT> {
        todo!()
    }

    unsafe fn transmute<F>(self) -> Self::Me<F> {
        Self {
            inner: self.inner,
            tpe: PhantomData
        }
    }

    fn get_id(&self) -> &Self::OutputId {
        todo!()
    }
}