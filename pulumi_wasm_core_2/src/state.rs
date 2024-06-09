use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use rmpv::Value;

use crate::node::{ActionableNode, MaybeNodeValue, NodeValue, OutputId};
use crate::pulumi::FieldName;

enum InitialNode {
    Done(Rc<Mutex<crate::node::DoneNode>>),
}

enum Mode {
    Initial,
    ForeignFunctions,
}

pub struct State {
    mode: Mode,
    initial_nodes: Vec<InitialNode>,
    foreign_functions: HashMap<OutputId, Rc<Mutex<crate::node::NativeFunctionNode>>>,
    node: HashMap<OutputId, Rc<Mutex<dyn crate::node::Node>>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: Mode::Initial,
            initial_nodes: Vec::new(),
            foreign_functions: HashMap::new(),
            node: HashMap::new(),
        }
    }
}

impl State {
    pub fn add_done_node(&mut self, value: Value) -> OutputId {
        let output_id = OutputId::generate_uuid();
        let node = Rc::new(Mutex::new(crate::node::DoneNode::new(
            output_id.clone(),
            value,
        )));
        self.initial_nodes.push(InitialNode::Done(node.clone()));
        self.node.insert(output_id.clone(), node);
        output_id
    }

    pub fn add_native_function_node(
        &mut self,
        function_name: String,
        argument: OutputId,
    ) -> anyhow::Result<OutputId> {
        let output_id = OutputId::generate_uuid();

        let node = Rc::new(Mutex::new(crate::node::NativeFunctionNode::new(
            output_id.clone(),
            function_name,
        )));
        let dep = self
            .node
            .get(&argument)
            .ok_or(anyhow::anyhow!("node with id {} not found", argument.0))?;
        let node1 = node.clone();
        dep.lock().unwrap().add_callback(Box::new(move |value| {
            node1.lock().unwrap().set_argument_value(value)
        }));

        self.foreign_functions
            .insert(output_id.clone(), node.clone());
        self.node.insert(output_id.clone(), node);
        Ok(output_id)
    }

    pub fn add_create_resource_node(
        &mut self,
        resource_name: String,
        resource_type: String,
        arguments: HashMap<FieldName, OutputId>,
        expected_results: HashMap<FieldName, msgpack_protobuf_converter::Type>,
    ) -> HashMap<FieldName, OutputId> {
        HashMap::new()
    }

    pub fn get_value(&self, output_id: &OutputId) -> Option<Value> {
        self.node
            .get(output_id)
            .map(|node| node.lock().unwrap().get_value().clone())
            .and_then(|mnv| match mnv {
                MaybeNodeValue::NotYetCalculated => None,
                MaybeNodeValue::Set(NodeValue::Nothing) => None,
                MaybeNodeValue::Set(NodeValue::Exists(v)) => Some(v.clone()),
            })
    }

    pub fn run(&mut self, values: HashMap<OutputId, Value>) -> Vec<ActionableNode> {
        // TODO: Refactor into mode?
        if values.is_empty() {
            self.initial_nodes
                .iter()
                .flat_map(|initial_node| match initial_node {
                    InitialNode::Done(done_node) => done_node.lock().unwrap().run(),
                })
                .collect()
        } else {
            values
                .iter()
                .flat_map(
                    |(output_id, value)| match self.foreign_functions.get(output_id) {
                        None => panic!("node with id {} not found", output_id.0),
                        Some(ff) => ff
                            .lock()
                            .unwrap()
                            .set_value(NodeValue::Exists(value.clone())),
                    },
                )
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn done_node_should_be_added_to_node_and_initial_nodes() {
        let mut state = State::default();
        let value = Value::Nil;
        state.add_done_node(value.clone());

        assert_eq!(state.initial_nodes.len(), 1);
        assert_eq!(state.node.len(), 1);
        assert_eq!(state.foreign_functions.len(), 0);
    }

    mod add_native_function_node {
        use super::*;

        #[test]
        fn should_fail_when_argument_not_found() {
            let mut state = State::default();
            let result = state.add_native_function_node(
                "native_function_name".into(),
                OutputId(UUID_1.to_string()),
            );

            let err = result.unwrap_err();
            let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
                .map(|e| e.to_string())
                .collect();

            assert_eq!(
                vec!["node with id 01010101-0101-0101-0101-010101010101 not found"],
                chain
            );
        }

        #[test]
        fn should_succeed_when_argument_found() {
            let mut state = State::default();
            let output_id = OutputId::generate_uuid();
            state.node.insert(
                output_id.clone(),
                Rc::new(Mutex::new(crate::node::DoneNode::new(
                    output_id.clone(),
                    Value::Nil,
                ))),
            );
            let result =
                state.add_native_function_node("native_function_name".into(), output_id.clone());

            assert!(result.is_ok());
            assert_eq!(state.node.len(), 2);
            assert_eq!(state.foreign_functions.len(), 1);
        }
    }

    mod run {
        use std::collections::HashMap;

        use crate::node::ActionableNode;
        use crate::State;

        #[test]
        fn run_should_pass_value_downwards_and_return_foreign_functions() {
            let mut state = State::default();

            let output_id_1 = state.add_done_node(2.into());
            let output_id_2 = state
                .add_native_function_node("add".into(), output_id_1.clone())
                .unwrap();

            let result = state.run(HashMap::new());
            assert_eq!(
                result,
                vec![ActionableNode::new_native_function(
                    output_id_2.clone(),
                    "add".into(),
                    2.into()
                )]
            );
        }

        #[test]
        fn run_should_distribute_values_and_return_foreign_functions() {
            let mut state = State::default();

            let output_id_1 = state.add_done_node(2.into());
            let output_id_2 = state
                .add_native_function_node("add".into(), output_id_1.clone())
                .unwrap();
            let output_id_3 = state
                .add_native_function_node("minus".into(), output_id_2.clone())
                .unwrap();

            let result = state.run(HashMap::new());
            assert_eq!(
                result,
                vec![ActionableNode::new_native_function(
                    output_id_2.clone(),
                    "add".into(),
                    2.into()
                )]
            );
            let result = state.run(HashMap::from([(output_id_2, 3.into())]));
            assert_eq!(
                result,
                vec![ActionableNode::new_native_function(
                    output_id_3.clone(),
                    "minus".into(),
                    3.into()
                )]
            );
        }
    }

    static UUID_1: Uuid = Uuid::from_bytes([1; 16]);
}
