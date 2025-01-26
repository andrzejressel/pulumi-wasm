use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::rc::Rc;
use pulumi_wasm_core::{Engine, OutputId};

pub struct CustomOutputId {
    output_id: OutputId,
    engine: Rc<RefCell<Engine>>,
}
pub struct CustomRegisterOutputId{
    output_id: OutputId,
    engine: Rc<RefCell<Engine>>,
}

pub struct PulumiEngine {
    engine: Rc<RefCell<Engine>>,
    outputs: Vec<*mut Output>,
    in_preview: bool,
}

pub struct Output {
    native: pulumi_wasm_core::OutputId
}

pub struct RegisterOutput {
    native: pulumi_wasm_core::OutputId
}

#[repr(C)]
pub struct ObjectField {
    name: *const c_char,
    value: *const Output,
}

#[repr(C)]
pub struct ResultField {
    name: *const c_char,
}

#[repr(C)]
pub struct RegisterResourceResultField {
    name: *const c_char,
    output: *const Output,
}

#[repr(C)]
pub struct RegisterResourceRequest {
    type_: *const c_char,
    name: *const c_char,
    version: *const c_char,
    object: *const ObjectField,
    object_len: usize,
    results: *const ResultField,
    results_len: usize,
}

#[repr(C)]
pub struct RegisterResourceResult {
    fields: *const RegisterResourceResultField,
    fields_len: usize,
}

#[no_mangle]
pub extern "C" fn create_engine() -> *mut PulumiEngine {
    let engine = get_engine();
    let preview = false; //FIXME: get from env
    let t = PulumiEngine { engine, outputs: Vec::new() };
    Box::into_raw(Box::new(t))
}

#[no_mangle]
pub extern "C" fn free_engine(t: *mut PulumiEngine) {
    unsafe {
        let _ = Box::from_raw(t);
        for output in (*t).outputs.iter() {
            let _ = Box::from_raw(*output);
        }
    }
}

#[no_mangle]
pub extern "C" fn create_output(pulumi_engine: *mut PulumiEngine, value: *const c_char, secret: bool) -> *mut Output {
    let value = unsafe {
        CStr::from_ptr(value)
    }.to_str().unwrap().to_string();
    let pulumi_engine = unsafe {
        &mut *pulumi_engine
    };
    let value = serde_json::from_str(&value).unwrap();
    let output_id = pulumi_engine.engine.create_done_node(value, secret);
    let output = Output { native: output_id };
    let raw = Box::into_raw(Box::new(output));
    pulumi_engine.outputs.push(raw);
    raw
}

#[no_mangle]
pub extern "C" fn add_export(pulumi_engine: *mut PulumiEngine, name: *const c_char, value: *const Output) {
    // Implement the function logic here
}

#[no_mangle]
pub extern "C" fn finish(pulumi_engine: *mut PulumiEngine) {
}

fn get_engine() -> Engine {
    todo!()
}

#[no_mangle]
pub extern "C" fn register(pulumi_engine: *mut PulumiEngine, request: *const RegisterResourceRequest) -> RegisterResourceResult {
    let pulumi_engine = unsafe {
        &mut *pulumi_engine
    };

    let type_ = unsafe {
        CStr::from_ptr((*request).type_)
    }.to_str().unwrap().to_owned();

    let name = unsafe {
        CStr::from_ptr((*request).name)
    }.to_str().unwrap().to_owned();

    let version = unsafe {
        CStr::from_ptr((*request).version)
    }.to_str().unwrap().to_owned();

    let mut inputs: HashMap<FieldName, OutputId> = HashMap::new();

    unsafe {
        let request = &*request;
        std::slice::from_raw_parts(request.object, request.object_len).iter().for_each(|field| {
            let name = CStr::from_ptr(field.name).to_str().unwrap().to_owned();
            let output = (*field.value).native.clone();

            inputs.insert(name.into(), output);
        });
    }

    let output_id = pulumi_engine.engine.create_register_resource_node(
        type_,
        name,
        inputs,
        version,
    );

    RegisterResourceResult {
        fields: std::ptr::null(),
        fields_len: 0,
    }
}