use std::marker::PhantomData;
use serde::Serialize;
use pulumi_gestalt_rust_adapter::{GestaltCompositeOutput, GestaltContext, GestaltOutput, InvokeResourceRequest, RegisterResourceRequest};
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

struct NativeCompositeOutput {
    inner: simple::CustomRegisterOutputId
}

impl GestaltCompositeOutput for NativeCompositeOutput {
    type Output<T> = NativeOutput<T>;

    fn get_field<T>(&self, key: &str) -> Self::Output<T> {
        let res = self.inner.get_output(key.to_string());
        NativeOutput {
            inner: res,
            tpe: PhantomData
        }
    }
}

impl NativeContext {
    fn new() -> NativeContext {
        NativeContext {
            inner: simple::PulumiEngine::create_engine()
        }
    }
}

impl GestaltContext for NativeContext {
    type Output<T> = NativeOutput<T>;
    type CompositeOutput = ();
    type OutputId = ();

    fn new_output<T: Serialize>(&self, value: &T) -> Self::Output<T> {
        let json = serde_json::to_string(value).unwrap();
        NativeOutput {
            inner: self.inner.create_output(json, false),
            tpe: PhantomData
        }
    }

    fn new_secret<T: Serialize>(&self, value: &T) -> Self::Output<T> {
        let json = serde_json::to_string(value).unwrap();
        NativeOutput {
            inner: self.inner.create_output(json, true),
            tpe: PhantomData
        }
    }

    fn register_resource(&self, request: RegisterResourceRequest<Self::OutputId>) -> Self::CompositeOutput {

        let result = self.inner.pulumi_register_resource(simple::RegisterResourceRequest {
            type_: request.type_,
            name: request.name,
            version: request.version,
            objects: request.object.iter().map(|k| {
                (k.name.clone(), k.value.inner)
            }).collect()
        });

    }

    fn invoke_resource(&self, request: InvokeResourceRequest<Self::OutputId>) -> Self::CompositeOutput {
        todo!()
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

        let function = move |v: String| {
            let v: T = serde_json::from_str(&v).unwrap();
            let v = f(v);
            serde_json::to_string(&v).unwrap()
        };

        let res = self.inner.map(function);

        // let res = &self.inner.map(|v| {
        //     let v: T = serde_json::from_str(&v).unwrap();
        //     let v = f(v);
        //     serde_json::to_string(&v).unwrap()
        // });

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