use pulumi_gestalt_rust_adapter::{
    GestaltCompositeOutput, GestaltContext, GestaltOutput, InvokeResourceRequest,
    RegisterResourceRequest,
};
use pulumi_gestalt_rust_integration as integration;
use serde::Serialize;
use std::marker::PhantomData;

pub struct NativeOutput<T> {
    inner: integration::CustomOutputId,
    tpe: PhantomData<T>,
}

impl<T> Clone for NativeOutput<T> {
    fn clone(&self) -> Self {
        NativeOutput {
            inner: self.inner.clone(),
            tpe: PhantomData,
        }
    }
}

pub struct NativeContext {
    inner: integration::PulumiEngine,
}

pub struct NativeCompositeOutput {
    inner: integration::CustomRegisterOutputId,
}

impl GestaltCompositeOutput for NativeCompositeOutput {
    type Output<T> = NativeOutput<T>;

    fn get_field<T>(&self, key: &str) -> Self::Output<T> {
        let res = self.inner.get_output(key.to_string());
        NativeOutput {
            inner: res,
            tpe: PhantomData,
        }
    }
}

impl Default for NativeContext {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeContext {
    pub fn new() -> NativeContext {
        NativeContext {
            inner: integration::PulumiEngine::create_engine(),
        }
    }

    pub fn finish(&self) {
        self.inner.finish();
    }
}

impl GestaltContext for NativeContext {
    type Output<T> = NativeOutput<T>;
    type CompositeOutput = NativeCompositeOutput;

    fn new_output<T: Serialize>(&self, value: &T) -> Self::Output<T> {
        let json = serde_json::to_string(value).unwrap();
        NativeOutput {
            inner: self.inner.create_output(json, false),
            tpe: PhantomData,
        }
    }

    fn new_secret<T: Serialize>(&self, value: &T) -> Self::Output<T> {
        let json = serde_json::to_string(value).unwrap();
        NativeOutput {
            inner: self.inner.create_output(json, true),
            tpe: PhantomData,
        }
    }

    fn register_resource(
        &self,
        request: RegisterResourceRequest<Self::Output<()>>,
    ) -> Self::CompositeOutput {
        let result = self
            .inner
            .pulumi_register_resource(integration::RegisterResourceRequest {
                type_: request.type_,
                name: request.name,
                version: request.version,
                objects: request
                    .object
                    .iter()
                    .map(|k| (k.name.clone().into(), *k.value.inner.get_id()))
                    .collect(),
            });

        NativeCompositeOutput { inner: result }
    }

    fn invoke_resource(
        &self,
        request: InvokeResourceRequest<Self::Output<()>>,
    ) -> Self::CompositeOutput {
        let result = self
            .inner
            .pulumi_invoke_resource(integration::InvokeResourceRequest {
                token: request.token,
                version: request.version,
                objects: request
                    .object
                    .iter()
                    .map(|k| (k.name.clone().into(), *k.value.inner.get_id()))
                    .collect(),
            });

        NativeCompositeOutput { inner: result }
    }
}

impl<T> GestaltOutput<T> for NativeOutput<T> {
    type Me<A> = NativeOutput<A>;

    fn map<B, F>(&self, f: F) -> Self::Me<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: serde::de::DeserializeOwned,
        B: serde::ser::Serialize,
    {
        let function = move |v: String| {
            let v: T = serde_json::from_str(&v).unwrap();
            let v = f(v);
            serde_json::to_string(&v).unwrap()
        };

        let res = self.inner.map(Box::new(function));

        NativeOutput {
            inner: res,
            tpe: PhantomData,
        }
    }

    fn add_to_export(&self, key: &str) {
        self.inner.add_export(key.to_string());
    }

    fn combine<RESULT>(&self, others: &[&Self::Me<()>]) -> Self::Me<RESULT> {
        let all_outputs = others.iter().map(|other| &other.inner).collect::<Vec<_>>();

        let combined = self.inner.combine(&all_outputs);

        NativeOutput {
            inner: combined,
            tpe: PhantomData,
        }
    }

    unsafe fn transmute<F>(self) -> Self::Me<F> {
        NativeOutput {
            inner: self.inner,
            tpe: PhantomData,
        }
    }
}
