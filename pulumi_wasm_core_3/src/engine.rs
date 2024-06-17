use std::collections::{HashMap, VecDeque};
use std::hint::unreachable_unchecked;

use log::error;
use rmpv::Value;
use uuid::Uuid;

use crate::model::{FunctionName, NodeValue, OutputId};
use crate::model::MaybeNodeValue::Set;
use crate::model::NodeValue::Exists;
use crate::nodes::{Callback, DoneNode, NativeFunctionNode};

enum EngineNode {
    Done(DoneNode),
    NativeFunction(NativeFunctionNode),
}

struct Engine {
    done_node_ids: VecDeque<OutputId>,
    unknown_foreign_function_ids: Vec<OutputId>,

    nodes: HashMap<OutputId, EngineNode>,
}

impl Engine {
    fn new() -> Self {
        Self {
            done_node_ids: VecDeque::new(),
            unknown_foreign_function_ids: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    fn run(&mut self, native_function_results: HashMap<OutputId, Value>) {

        // let v = EngineView {
        //     done_node_ids: &mut self.done_node_ids,
        //     unknown_foreign_function_ids: &mut self.unknown_foreign_function_ids,
        //     nodes: &mut self.nodes,
        // };

        loop {
            loop {
                let output_id = match self.done_node_ids.pop_back() {
                    None => break,
                    Some(output_id) => output_id
                };

                
                self.done_node_ids.push_back(output_id);
                
                let mut nodes = &mut self.nodes;

                let node = Engine::get_done_2(nodes, output_id);
                let value = Exists(node.get_value());
                let callbacks = node.get_callbacks();

                for callback in callbacks {
                    // match callback {
                    //     Callback::CreateResource(_, _) => {}
                    //     Callback::ExtractField(_) => {}
                    //     Callback::NativeFunction(output_id) => {
                    //         Engine::get_done_2(nodes, *output_id);
                    //     }
                    // }
                    Engine::handle_callback(&value, &callback, nodes);
                }

                // self.test();

                // for callback in node.get_callbacks() {
                //     self.done_node_ids.push_back(Uuid::now_v7().into());
                // self.test();
                // self.done_node_ids.push_back(OutputId::new(callback.get_output_id()));
                //     self.handle_callback(Exists(node.get_value().clone()), callback);
                // }
            }
        }
    }

    fn handle_callback(value: &NodeValue, callback: &Callback, nodes: &mut HashMap<OutputId, EngineNode>) {
        match callback {
            Callback::CreateResource(_, _) => {}
            Callback::ExtractField(_) => {}
            Callback::NativeFunction(output_id) => {}
        }
    }

    fn get_done(&self, output_id: OutputId) -> &DoneNode {
        match self.nodes.get(&output_id) {
            Some(EngineNode::Done(node)) => node,
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(EngineNode::NativeFunction(_)) => {
                error!("Node with id [{}] is native function, not done", output_id);
                panic!("Node with id [{}] is native function, not done", output_id)
            }
        }
    }

    fn get_done_2(nodes: &HashMap<OutputId, EngineNode>, output_id: OutputId) -> &DoneNode {
        match nodes.get(&output_id) {
            Some(EngineNode::Done(node)) => node,
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(EngineNode::NativeFunction(_)) => {
                error!("Node with id [{}] is native function, not done", output_id);
                panic!("Node with id [{}] is native function, not done", output_id)
            }
        }
    }

    fn get_done_mut(&mut self, output_id: OutputId) -> &mut DoneNode {
        match self.nodes.get_mut(&output_id) {
            Some(EngineNode::Done(node)) => node,
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(EngineNode::NativeFunction(_)) => {
                error!("Node with id [{}] is native function, not done", output_id);
                panic!("Node with id [{}] is native function, not done", output_id)
            }
        }
    }

    fn get_native_function(&mut self, output_id: OutputId) -> &mut NativeFunctionNode {
        match self.nodes.get_mut(&output_id) {
            Some(EngineNode::NativeFunction(node)) => node,
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id)
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
            Some(EngineNode::Done(_)) => {
                error!("Node with id [{}] is done, not native function", output_id);
                panic!("Node with id [{}] is done, not native function", output_id)
            }
        }
    }

    fn add_callback(&mut self, output_id: OutputId, callback: Callback) {
        match self.nodes.get_mut(&output_id) {
            Some(EngineNode::Done(node)) => node.add_callback(callback),
            Some(EngineNode::NativeFunction(node)) => node.add_callback(callback),
            None => {
                error!("Cannot find node with id {}", output_id);
                panic!("Cannot find node with id {}", output_id);
                // Maybe in the future?
                // unsafe { unreachable_unchecked() }
            }
        }
    }

    fn create_done_node(&mut self, value: Value) -> OutputId {
        let output_id = Uuid::now_v7().into();
        let node = DoneNode::new(value);
        self.done_node_ids.push_back(output_id);
        self.nodes.insert(output_id, EngineNode::Done(node));

        output_id
    }

    fn create_native_function_node(
        &mut self,
        function_name: FunctionName,
        source: OutputId,
    ) -> OutputId {
        let output_id = Uuid::now_v7().into();
        let node = NativeFunctionNode::new(function_name);
        let callback = Callback::native_function(output_id);
        self.unknown_foreign_function_ids.push(output_id);
        self.nodes
            .insert(output_id, EngineNode::NativeFunction(node));
        self.add_callback(source, callback);
        output_id
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rmpv::Value;
    use uuid::Uuid;

    use crate::engine::Engine;
    use crate::nodes::{Callback, DoneNode, NativeFunctionNode};

    #[test]
    fn create_done_node_create_node_in_map() {
        let mut engine = Engine::new();
        let value: Value = 1.into();
        let output_id = engine.create_done_node(value.clone());

        assert_eq!(engine.done_node_ids, vec![output_id]);
        assert_eq!(
            engine.get_done(output_id),
            &DoneNode::create(value, Vec::new())
        );
    }

    #[test]
    fn create_native_function_node_create_node_in_map() {
        let mut engine = Engine::new();
        let value: Value = 1.into();
        let done_node_output_id = engine.create_done_node(value.clone());
        let native_node_output_id =
            engine.create_native_function_node("func".into(), done_node_output_id);

        assert_eq!(engine.done_node_ids, vec![done_node_output_id]);
        assert_eq!(
            engine.unknown_foreign_function_ids,
            vec![native_node_output_id]
        );

        assert_eq!(
            engine.get_done(done_node_output_id),
            &DoneNode::create(
                value,
                Vec::from([Callback::NativeFunction(native_node_output_id)]),
            )
        );
        assert_eq!(
            engine.get_native_function(native_node_output_id),
            &NativeFunctionNode::new("func".into())
        );
    }

    #[test]
    fn run_return_native_functions() {
        let mut engine = Engine::new();
        let value: Value = 1.into();
        let done_node_output_id = engine.create_done_node(value.clone());
        let native_node_output_id =
            engine.create_native_function_node("func".into(), done_node_output_id);

        let result = engine.run(HashMap::new());
        // assert_eq!(
        //     result,
        //     vec![NativeFunctionNode::new("func".into())]
        // );
    }

    static UUID_1: Uuid = Uuid::from_bytes([1; 16]);
    static UUID_2: Uuid = Uuid::from_bytes([2; 16]);
    static UUID_3: Uuid = Uuid::from_bytes([3; 16]);
    static UUID_4: Uuid = Uuid::from_bytes([4; 16]);
    static UUID_5: Uuid = Uuid::from_bytes([5; 16]);
    static UUID_6: Uuid = Uuid::from_bytes([6; 16]);
}
