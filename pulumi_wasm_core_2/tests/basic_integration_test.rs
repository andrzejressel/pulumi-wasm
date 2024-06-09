use pulumi_wasm_core_2::{ActionableNode, OutputId, State};
use rmpv::Value;
use std::collections::HashMap;

#[test]
fn basic_test() {
    let mut state = State::default();

    let output_id_1 = state.add_done_node(2.into());
    let output_id_2 = state
        .add_native_function_node("double".into(), output_id_1.clone())
        .unwrap();
    let output_id_3 = state
        .add_native_function_node("quadruple".into(), output_id_2.clone())
        .unwrap();

    let value_1 = state.get_value(&output_id_1);
    let value_2 = state.get_value(&output_id_2);
    let value_3 = state.get_value(&output_id_3);

    assert_eq!(value_1, Some(2.into()));
    assert_eq!(value_2, None);
    assert_eq!(value_3, None);

    let functions = state.run(HashMap::new());
    assert_eq!(functions.len(), 1);

    let functions = state.run(run_functions(functions));
    assert_eq!(functions.len(), 1);

    let functions = state.run(run_functions(functions));
    assert_eq!(functions.len(), 0);

    assert_eq!(state.get_value(&output_id_1), Some(2.into()));
    assert_eq!(state.get_value(&output_id_2), Some(4.into()));
    assert_eq!(state.get_value(&output_id_3), Some(16.into()));
}

fn run_functions(functions: Vec<ActionableNode>) -> HashMap<OutputId, Value> {
    functions
        .iter()
        .map(|ac| match ac {
            ActionableNode::NativeFunction(nfan) => (
                nfan.get_output_id().clone(),
                get_function_result(nfan.get_function_name(), nfan.get_argument()),
            ),
        })
        .collect::<HashMap<_, _>>()
}

fn get_function_result(function_name: &String, argument: &Value) -> Value {
    match function_name.as_str() {
        "double" => (argument.as_i64().unwrap() * 2).into(),
        "quadruple" => (argument.as_i64().unwrap() * 4).into(),
        _ => panic!("function {} not found", function_name),
    }
}
