use pulumi_gestalt_core::{FieldName, OutputId};
use pulumi_gestalt_rust_integration as simple;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_char, c_void, CStr, CString};
use std::rc::{Rc, Weak};

pub struct CustomOutputId {
    native: simple::CustomOutputId,
    engine: Weak<RefCell<InnerPulumiEngine>>,
}

pub struct CustomRegisterOutputId {
    native: simple::CustomRegisterOutputId,
    engine: Weak<RefCell<InnerPulumiEngine>>,
}

pub struct InnerPulumiEngine {
    engine: simple::PulumiEngine,
    outputs: Vec<*mut CustomOutputId>,
    context: *const c_void,
}

pub struct PulumiEngine {
    inner: Rc<RefCell<InnerPulumiEngine>>,
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

/// Arguments: Engine context, Function context, Serialized JSON value
/// Returned string must represent a JSON value;
/// Library will free the returned string
type MappingFunction = extern "C" fn(*const c_void, *const c_void, *const c_char) -> *mut c_char;

#[no_mangle]
extern "C" fn create_engine(context: *const c_void) -> *mut PulumiEngine {
    let engine = simple::PulumiEngine::create_engine();
    let t = InnerPulumiEngine {
        engine,
        outputs: Vec::new(),
        context,
    };
    let t = PulumiEngine {
        inner: Rc::new(RefCell::new(t)),
    };
    Box::into_raw(Box::new(t))
}

#[no_mangle]
extern "C" fn free_engine(t: *mut PulumiEngine) {
    unsafe {
        let b = Box::from_raw(t);
        for output in b.inner.borrow_mut().outputs.iter() {
            let _ = Box::from_raw(*output);
        }
    }
}

#[no_mangle]
extern "C" fn create_output(
    pulumi_engine: *mut PulumiEngine,
    value: *const c_char,
    secret: bool,
) -> *mut CustomOutputId {
    let value = unsafe { CStr::from_ptr(value) }
        .to_str()
        .unwrap()
        .to_string();
    let pulumi_engine = unsafe { &mut *pulumi_engine };
    let inner = &pulumi_engine.inner;
    let mut inner_engine = pulumi_engine.inner.borrow_mut();
    let output_id = inner_engine.engine.create_output(value, secret);
    let output = CustomOutputId {
        native: output_id,
        engine: Rc::downgrade(inner),
    };
    let raw = Box::into_raw(Box::new(output));
    inner_engine.outputs.push(raw);
    raw
}

#[no_mangle]
extern "C" fn add_export(value: *const CustomOutputId, name: *const c_char) {
    let name = unsafe { CStr::from_ptr(name) }
        .to_str()
        .unwrap()
        .to_string();
    let value = unsafe { &*value };
    value.native.add_export(name);
}

#[no_mangle]
extern "C" fn finish(pulumi_engine: *mut PulumiEngine) {
    let pulumi_engine = unsafe { &mut *pulumi_engine };
    pulumi_engine.inner.borrow_mut().engine.finish();
}

#[no_mangle]
extern "C" fn pulumi_map(
    pulumi_engine: *mut PulumiEngine,
    output: *const CustomOutputId,
    function_context: *const c_void,
    function: MappingFunction,
) -> *mut CustomOutputId {
    let output = unsafe { &*output };
    let engine = unsafe { &mut *pulumi_engine };
    let mut inner_engine = engine.inner.borrow_mut();
    let context = inner_engine.context;

    let f = move |value: String| {
        let c_string = CString::new(value).unwrap();
        let str = function(context, function_context, c_string.as_ptr());
        let result = unsafe { CStr::from_ptr(str) }.to_str().unwrap();
        let v = result.to_owned();
        unsafe {
            libc::free(str as *mut c_void);
        }
        v
    };

    let second_output = output.native.map(Box::new(f));

    let output = CustomOutputId {
        native: second_output,
        engine: Rc::downgrade(&engine.inner),
    };
    let raw = Box::into_raw(Box::new(output));
    inner_engine.outputs.push(raw);
    raw
}

#[no_mangle]
extern "C" fn pulumi_get_output(
    custom_register_output_id: *mut CustomRegisterOutputId,
    field_name: *const c_char,
) -> *mut CustomOutputId {
    let field_name = unsafe { CStr::from_ptr(field_name) }
        .to_str()
        .unwrap()
        .to_string();
    let custom_register_output_id = unsafe { &*custom_register_output_id };
    let output = custom_register_output_id.native.get_output(field_name);

    let binding = custom_register_output_id.engine.upgrade().unwrap();
    let mut engine = binding.borrow_mut();

    let output = CustomOutputId {
        native: output,
        engine: custom_register_output_id.engine.clone(),
    };
    let raw = Box::into_raw(Box::new(output));
    engine.outputs.push(raw);
    raw
}

#[no_mangle]
extern "C" fn pulumi_register_resource(
    pulumi_engine: *mut PulumiEngine,
    request: *const RegisterResourceRequest,
) -> *mut CustomRegisterOutputId {
    let pulumi_engine = unsafe { &mut *pulumi_engine };
    let request = unsafe { &*request };

    let type_ = unsafe { CStr::from_ptr(request.type_) }
        .to_str()
        .unwrap()
        .to_owned();

    let name = unsafe { CStr::from_ptr(request.name) }
        .to_str()
        .unwrap()
        .to_owned();

    let version = unsafe { CStr::from_ptr(request.version) }
        .to_str()
        .unwrap()
        .to_owned();

    let mut objects: HashMap<FieldName, OutputId> = HashMap::new();

    unsafe {
        std::slice::from_raw_parts(request.object, request.object_len)
            .iter()
            .for_each(|field| {
                let name = CStr::from_ptr(field.name).to_str().unwrap().to_owned();
                let output = &(*field.value).native.get_id().clone();

                objects.insert(name.into(), *output);
            });
    }

    let inner = &pulumi_engine.inner;
    let inner_engine = pulumi_engine.inner.borrow_mut();
    let request = simple::RegisterResourceRequest {
        type_,
        name,
        objects,
        version,
    };
    let output_id = inner_engine.engine.pulumi_register_resource(request);

    let output = CustomRegisterOutputId {
        native: output_id,
        engine: Rc::downgrade(inner),
    };

    // inner_engine.outputs.push(raw); //FIXME
    Box::into_raw(Box::new(output))
}
