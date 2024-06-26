use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Mutex;

use rmpv::Value;

use pulumi_wasm_core_2::{NativeFunctionActionableNode, OutputId, Pulumi, State};

#[test]
fn abc() {
    let _rc = Rc::new(Mutex::new(1));

    // assert_eq!(rc, rc);
}

#[test]
fn foreign_functions_only() {
    struct PulumiImpl {}
    impl Pulumi for PulumiImpl {
        fn is_in_preview(&self) -> bool {
            todo!()
        }

        fn get_root_resource(&self) -> String {
            todo!()
        }

        fn register_outputs(&self, _outputs: HashMap<String, Value>) {
            todo!()
        }

        fn register_resource(
            &self,
            _request: pulumi_wasm_core_2::RegisterResourceRequest,
        ) -> pulumi_wasm_core_2::RegisterId {
            todo!()
        }

        fn register_resource_poll(
            &self,
            _register_ids: HashSet<pulumi_wasm_core_2::RegisterId>,
        ) -> HashMap<pulumi_wasm_core_2::RegisterId, pulumi_wasm_core_2::RegisterResourceResponse>
        {
            todo!()
        }
    }

    let mut state = State::new(Rc::new(PulumiImpl {}));

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

fn run_functions(functions: Vec<NativeFunctionActionableNode>) -> HashMap<OutputId, Value> {
    functions
        .iter()
        .map(|nfan| {
            (
                nfan.get_output_id().clone(),
                get_function_result(nfan.get_function_name(), nfan.get_argument()),
            )
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
