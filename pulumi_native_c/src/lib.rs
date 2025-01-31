mod native_pulumi_connector;

use pulumi_wasm_core::{Engine, FieldName, OutputId, PulumiServiceImpl};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::ops::Deref;
use std::rc::Rc;

pub struct CustomOutputId {
    output_id: OutputId,
    engine: Rc<RefCell<Engine>>,
}

pub struct CustomRegisterOutputId {
    output_id: OutputId,
    engine: Rc<RefCell<Engine>>,
}

pub struct PulumiEngine {
    engine: Rc<RefCell<Engine>>,
    outputs: Vec<*mut CustomOutputId>,
    in_preview: bool,
}

#[repr(C)]
pub struct ObjectField {
    name: *const c_char,
    value: *const CustomOutputId,
}

#[repr(C)]
pub struct ResultField {
    name: *const c_char,
}

#[repr(C)]
pub struct RegisterResourceResultField {
    name: *const c_char,
    output: *const CustomOutputId,
}

#[repr(C)]
pub struct RegisterResourceRequest {
    type_: *const c_char,
    name: *const c_char,
    version: *const c_char,
    object: *const ObjectField,
    object_len: usize,
}

#[repr(C)]
pub struct RegisterResourceResult {
    fields: *const RegisterResourceResultField,
    fields_len: usize,
}

#[no_mangle]
pub extern "C" fn create_engine() -> *mut PulumiEngine {
    let engine = Rc::new(RefCell::new(get_engine()));
    let in_preview = match std::env::var("PULUMI_DRY_RUN") {
        Ok(preview) if preview == "true" => true,
        Ok(preview) if preview == "false" => false,
        _ => false,
    };
    let t = PulumiEngine {
        engine,
        outputs: Vec::new(),
        in_preview,
    };
    Box::into_raw(Box::new(t))
}

#[no_mangle]
pub extern "C" fn free_engine(t: *mut PulumiEngine) {
    unsafe {
        let b = Box::from_raw(t);
        for output in b.outputs.iter() {
            let _ = Box::from_raw(*output);
        }
    }
}

#[no_mangle]
pub extern "C" fn create_output(
    pulumi_engine: *mut PulumiEngine,
    value: *const c_char,
    secret: bool,
) -> *mut CustomOutputId {
    let value = unsafe { CStr::from_ptr(value) }
        .to_str()
        .unwrap()
        .to_string();
    let pulumi_engine = unsafe { &mut *pulumi_engine };
    let value = serde_json::from_str(&value).unwrap();
    let output_id = pulumi_engine
        .engine
        .deref()
        .borrow_mut()
        .create_done_node(value, secret);
    let output = CustomOutputId {
        output_id,
        engine: Rc::clone(&pulumi_engine.engine),
    };
    let raw = Box::into_raw(Box::new(output));
    pulumi_engine.outputs.push(raw);
    raw
}

#[no_mangle]
pub extern "C" fn add_export(value: *const CustomOutputId, name: *const c_char) {
    let name = unsafe { CStr::from_ptr(name) }
        .to_str()
        .unwrap()
        .to_string();
    let value = unsafe { &*value };
    let pulumi_engine = &value.engine;
    let output_id = value.output_id;
    pulumi_engine
        .deref()
        .borrow_mut()
        .add_output(name.into(), output_id);
}

#[no_mangle]
pub extern "C" fn finish(pulumi_engine: *mut PulumiEngine) {
    let pulumi_engine = unsafe { &mut *pulumi_engine };
    let result = pulumi_engine
        .engine
        .deref()
        .borrow_mut()
        .run(HashMap::new());
    if result.is_some() {
        panic!("Result: {:?}", result);
    }
}

fn get_engine() -> Engine {
    let pulumi_engine_url = std::env::var("PULUMI_ENGINE").unwrap();
    let pulumi_monitor_url = std::env::var("PULUMI_MONITOR").unwrap();
    let pulumi_stack = std::env::var("PULUMI_STACK").unwrap();
    let pulumi_project = std::env::var("PULUMI_PROJECT").unwrap();
    let in_preview = match std::env::var("PULUMI_DRY_RUN") {
        Ok(preview) if preview == "true" => true,
        Ok(preview) if preview == "false" => false,
        _ => false,
    };

    let native_pulumi_connector = native_pulumi_connector::NativePulumiConnector::new(
        pulumi_monitor_url,
        pulumi_engine_url,
        pulumi_project,
        pulumi_stack,
    );

    Engine::new(PulumiServiceImpl::new(native_pulumi_connector, in_preview))
}

#[no_mangle]
pub extern "C" fn pulumi_get_output(
    custom_register_output_id: *mut CustomRegisterOutputId,
    field_name: *const c_char,
) -> *mut CustomOutputId {
    let pulumi_engine = unsafe { &(*custom_register_output_id).engine.clone() };

    let output_id = unsafe { &(*custom_register_output_id).output_id.clone() };

    let field_name = unsafe { CStr::from_ptr(field_name) }
        .to_str()
        .unwrap()
        .to_string();

    let output = pulumi_engine
        .borrow_mut()
        .create_extract_field(field_name.into(), *output_id);

    let output = CustomOutputId {
        output_id: output,
        engine: Rc::clone(pulumi_engine),
    };
    Box::into_raw(Box::new(output))
}

#[no_mangle]
pub extern "C" fn pulumi_register_resource(
    pulumi_engine: *mut PulumiEngine,
    request: *const RegisterResourceRequest,
) -> *mut CustomRegisterOutputId {
    let pulumi_engine = unsafe { &mut *pulumi_engine };

    let type_ = unsafe { CStr::from_ptr((*request).type_) }
        .to_str()
        .unwrap()
        .to_owned();

    let name = unsafe { CStr::from_ptr((*request).name) }
        .to_str()
        .unwrap()
        .to_owned();

    let version = unsafe { CStr::from_ptr((*request).version) }
        .to_str()
        .unwrap()
        .to_owned();

    let mut inputs: HashMap<FieldName, OutputId> = HashMap::new();

    unsafe {
        let request = &*request;
        std::slice::from_raw_parts(request.object, request.object_len)
            .iter()
            .for_each(|field| {
                let name = CStr::from_ptr(field.name).to_str().unwrap().to_owned();
                let output = (*field.value).output_id;

                inputs.insert(name.into(), output);
            });
    }

    let output_id = pulumi_engine
        .engine
        .deref()
        .borrow_mut()
        .create_register_resource_node(type_, name, inputs, version);

    let register_output_id = CustomRegisterOutputId {
        output_id,
        engine: Rc::clone(&pulumi_engine.engine),
    };
    Box::into_raw(Box::new(register_output_id))
}
