use log::{error, log};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use rmpv::Value;

use crate::node::{ActionableNode, MaybeNodeValue, NativeFunctionNode, Node, NodeValue, OutputId};
use crate::pulumi::FieldName;
use crate::register_resource::node::RegisterResourceNode;
use crate::{NativeFunctionActionableNode, RegisterId};

enum InitialNode {
    Done(Rc<Mutex<crate::node::DoneNode>>),
}

pub struct State {
    pulumi: Rc<dyn crate::pulumi::Pulumi>,
    initial_nodes: Vec<InitialNode>,
    foreign_functions: HashMap<OutputId, Rc<Mutex<NativeFunctionNode>>>,
    in_progress_registers: HashMap<OutputId, RegisterId>,
    register_resource_nodes: HashMap<OutputId, Rc<Mutex<RegisterResourceNode>>>,
    node: HashMap<OutputId, Rc<Mutex<dyn Node>>>,
}

impl State {
    pub fn new(pulumi: Rc<dyn crate::pulumi::Pulumi>) -> Self {
        Self {
            pulumi,
            in_progress_registers: HashMap::new(),
            initial_nodes: Vec::new(),
            register_resource_nodes: HashMap::new(),
            foreign_functions: HashMap::new(),
            node: HashMap::new(),
        }
    }
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

        let node = Rc::new(Mutex::new(NativeFunctionNode::new(
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
    ) -> anyhow::Result<(OutputId, HashMap<FieldName, OutputId>)> {
        // FIXME
        if arguments.is_empty() {
            error!("arguments is empty");
            return Err(anyhow::anyhow!("arguments is empty"));
        }
        if expected_results.is_empty() {
            error!("expected_results is empty");
            return Err(anyhow::anyhow!("expected_results is empty"));
        }

        let output_id = OutputId::generate_uuid();

        let node = Rc::new(Mutex::new(RegisterResourceNode::new(
            output_id.clone(),
            self.pulumi.clone(),
            resource_type,
            resource_name,
            arguments.keys().cloned().collect(),
            expected_results,
        )));

        self.register_resource_nodes
            .insert(output_id.clone(), node.clone());

        for (field_name, output_id) in arguments {
            let dep = self
                .node
                .get(&output_id)
                .ok_or(anyhow::anyhow!("node with id {} not found", output_id.0))?;
            let node1 = node.clone();
            dep.lock().unwrap().add_callback(Box::new(move |value| {
                node1.lock().unwrap().set_input(field_name.clone(), value)
            }));
        }

        Ok((output_id, HashMap::new()))
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

    pub fn run(&mut self, values: HashMap<OutputId, Value>) -> Vec<NativeFunctionActionableNode> {
        // TODO: Refactor into mode?
        let nodes: Vec<_> = if values.is_empty() {
            self.initial_nodes
                .iter()
                .flat_map(|initial_node| match initial_node {
                    InitialNode::Done(done_node) => done_node.lock().unwrap().run(),
                })
                // .map(|actionable_node: ActionableNode| {
                //     if let ActionableNode::NativeFunction(nf) = actionable_node {
                //         nf
                //     } else {
                //         todo!("RegisterResource")
                //     }
                // })
                // .map(|actionable_node: ActionableNode| match actionable_node {
                //     nf@ ActionableNode::NativeFunction(_) => nf.clone(),
                //     ActionableNode::RegisterResource(_) => todo!("RegisterResource")
                // })
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
                // .map(|actionable_node: ActionableNode| {
                //     if let ActionableNode::NativeFunction(nf) = actionable_node {
                //         nf
                //     } else {
                //         todo!("RegisterResource")
                //     }
                // })
                .collect()
        };

        let mut native_function_actionable_nodes = Vec::new();
        let mut register_resource_actionable_nodes = Vec::new();

        for actionable_node in nodes {
            match actionable_node {
                ActionableNode::NativeFunction(nf) => native_function_actionable_nodes.push(nf),
                ActionableNode::RegisterResource(rr) => register_resource_actionable_nodes.push(rr),
            }
        }

        self.in_progress_registers.extend(
            register_resource_actionable_nodes
                .iter()
                .map(|rr| (rr.output_id.clone(), rr.register_id.clone())),
        );

        if native_function_actionable_nodes.is_empty() {
            self.handle_register_resources()
        } else {
            native_function_actionable_nodes
        }
    }

    pub fn handle_register_resources(&mut self) -> Vec<NativeFunctionActionableNode> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::pulumi::MockPulumi;

    use super::*;

    #[test]
    fn done_node_should_be_added_to_node_and_initial_nodes() {
        let mut state = State::new(Rc::new(MockPulumi::new()));
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
            let mut state = State::new(Rc::new(MockPulumi::new()));
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
            let mut state = State::new(Rc::new(MockPulumi::new()));
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
        use std::rc::Rc;

        use crate::pulumi::{FieldName, MockPulumi};
        use crate::{NativeFunctionActionableNode, RegisterId, State};

        #[test]
        fn run_should_pass_value_downwards_and_return_foreign_functions() {
            let mut state = State::new(Rc::new(MockPulumi::new()));

            let output_id_1 = state.add_done_node(2.into());
            let output_id_2 = state
                .add_native_function_node("add".into(), output_id_1.clone())
                .unwrap();

            let result = state.run(HashMap::new());
            assert_eq!(
                result,
                vec![NativeFunctionActionableNode::new(
                    output_id_2.clone(),
                    "add".into(),
                    2.into(),
                )]
            );
        }

        #[test]
        fn run_should_pass_value_downwards_and_save_register_resources() {
            let mut state = State::new(Rc::new(MockPulumi::new()));

            let output_id_1 = state.add_done_node(2.into());
            let output_id_2 = state
                .add_native_function_node("add".into(), output_id_1.clone())
                .unwrap();

            let result = state.run(HashMap::new());
            assert_eq!(
                result,
                vec![NativeFunctionActionableNode::new(
                    output_id_2.clone(),
                    "add".into(),
                    2.into(),
                )]
            );
        }

        #[test]
        fn run_should_distribute_values_and_return_foreign_functions() {
            let mut state = State::new(Rc::new(MockPulumi::new()));

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
                vec![NativeFunctionActionableNode::new(
                    output_id_2.clone(),
                    "add".into(),
                    2.into(),
                )]
            );
            let result = state.run(HashMap::from([(output_id_2, 3.into())]));
            assert_eq!(
                result,
                vec![NativeFunctionActionableNode::new(
                    output_id_3.clone(),
                    "minus".into(),
                    3.into(),
                )]
            );
        }

        #[test]
        fn run_should_distribute_values_save_register_resources_and_return_foreign_functions_if_exists() {
            let mut pulumi = MockPulumi::new();
            pulumi
                .expect_register_resource()
                .once()
                // .times(1)
                .returning(|_| RegisterId::new("register_id".into()));

            let mut state = State::new(Rc::new(pulumi));

            let output_id_1 = state.add_done_node(2.into());
            let output_id_2 = state
                .add_native_function_node("add".into(), output_id_1.clone())
                .unwrap();
            let (output_ids_3, fields) = state
                .add_create_resource_node(
                    "minus".into(),
                    "type".into(),
                    HashMap::from([(FieldName::new("field1".into()), output_id_1)]),
                    HashMap::from([(
                        FieldName::new("field2".to_string()),
                        msgpack_protobuf_converter::Type::String,
                    )]),
                )
                .unwrap();

            let result = state.run(HashMap::new());
            assert_eq!(
                result,
                vec![NativeFunctionActionableNode::new(
                    output_id_2.clone(),
                    "add".into(),
                    2.into(),
                )]
            );
            assert_eq!(
                state.in_progress_registers,
                HashMap::from([(output_ids_3, RegisterId::new("register_id".into()))])
            )
        }

        fn run_should_distribute_values_save_and_wait_for_register_resources_if_no_foreign_functions() {}
    }

    static UUID_1: Uuid = Uuid::from_bytes([1; 16]);
}
