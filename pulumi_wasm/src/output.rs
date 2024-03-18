use std::cell::RefCell;
use std::collections::HashMap;
use rmpv::Value;
use std::rc::Rc;
use crate::Output;

pub(crate) type OutputContentRefCell = Rc<RefCell<OutputContent>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FunctionId(String);

impl FunctionId {
    pub(crate) fn from_string(s: &String) -> FunctionId {
        FunctionId(s.to_string())
    }
    pub(crate) fn get(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FunctionSource(String);

impl From<FunctionSource> for String {
    fn from(val: FunctionSource) -> Self {
        val.0
    }
}

impl FunctionSource {
    pub(crate) fn from_string(s: &String) -> FunctionSource {
        FunctionSource(s.to_string())
    }
    pub(crate) fn from_str(s: &str) -> FunctionSource {
        FunctionSource(s.to_string())
    }
}

static mut GLOBAL_MAP: Option<Vec<OutputContentRefCell>> = None;
static mut OUTPUT_MAP: Option<HashMap<String, Output>> = None;

pub(crate) fn access_map() -> &'static mut Vec<OutputContentRefCell> {
    let maybe_map = unsafe { &mut GLOBAL_MAP };

    match maybe_map {
        None => {
            unsafe {
                GLOBAL_MAP = Some(Vec::new());
            };
            access_map()
        }
        Some(m) => m,
    }
}

pub(crate) fn output_map() -> &'static mut HashMap<String, Output> {
    let maybe_map = unsafe { &mut OUTPUT_MAP };

    match maybe_map {
        None => {
            unsafe {
                OUTPUT_MAP = Some(HashMap::new());
            };
            output_map()
        }
        Some(m) => m,
    }
}

pub(crate) enum OutputContent {
    Done(Value),
    Mapped(FunctionId, FunctionSource, OutputContentRefCell),
    Func(Vec<OutputContentRefCell>, Box<dyn Fn(Vec<Value>) -> Value>),
    Nothing,
}

impl OutputContent {
    pub(crate) fn tpe(&self) -> &'static str {
        match self {
            OutputContent::Done(_) => "Done",
            OutputContent::Mapped(_, _, _) => "Mapped",
            OutputContent::Func(_, _) => "Func",
            OutputContent::Nothing => "Nothing",
        }
    }
}

pub(crate) fn create_nothing() -> Rc<RefCell<OutputContent>> {
    create_wrapper(OutputContent::Nothing)
}

pub(crate) fn create_new(vec: Value) -> Rc<RefCell<OutputContent>> {
    create_wrapper(OutputContent::Done(vec))
}

pub(crate) fn map_external(
    function_id: FunctionId,
    function_source: FunctionSource,
    ref_cell: OutputContentRefCell,
) -> Rc<RefCell<OutputContent>> {
    create_wrapper(OutputContent::Mapped(
        function_id,
        function_source,
        ref_cell,
    ))
}

pub(crate) fn map_internal<F>(ref_cell: Vec<OutputContentRefCell>, f: F) -> Rc<RefCell<OutputContent>>
    where
        F: Fn(Vec<Value>) -> Value + 'static,
{
    create_wrapper(OutputContent::Func(ref_cell, Box::new(f)))
}

fn create_wrapper(o: OutputContent) -> Rc<RefCell<OutputContent>> {
    let rc = Rc::new(RefCell::new(o));
    access_map().push(rc.clone());
    rc
}
