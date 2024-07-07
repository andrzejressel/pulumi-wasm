use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};

use log::warn;
use prost::Message;
use prost_types::Struct;
use rmpv::Value;

use crate::model::{FieldName, OutputId};
use crate::pulumi::service::{PulumiService, RegisterResourceResponse};
use crate::{PulumiConnector, RegisterResourceRequest};

pub struct PulumiServiceImpl {
    connector: Box<dyn PulumiConnector>,
    expected_results:
        RefCell<HashMap<OutputId, HashMap<FieldName, msgpack_protobuf_converter::Type>>>,
    is_in_preview: bool,
}

mod grpc {
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]
    tonic::include_proto!("pulumirpc");
}

impl PulumiServiceImpl {
    pub fn new(connector: impl PulumiConnector + 'static) -> PulumiServiceImpl {
        let is_in_preview = connector.is_in_preview();
        Self {
            connector: Box::new(connector),
            is_in_preview,
            expected_results: RefCell::new(HashMap::new()),
        }
    }
}

impl PulumiService for PulumiServiceImpl {
    fn is_in_preview(&self) -> bool {
        self.is_in_preview
    }

    fn get_root_resource(&self) -> String {
        unimplemented!()
    }

    fn register_outputs(&self, outputs: HashMap<FieldName, Value>) {
        if !self.is_in_preview {
            let root = self.connector.get_root_resource();

            let object = Self::create_protobuf_struct(
                outputs.into_iter().map(|(k, v)| (k, Some(v))).collect(),
            );

            let request = grpc::RegisterResourceOutputsRequest {
                urn: root,
                outputs: Some(object),
            };

            self.connector.register_outputs(request.encode_to_vec());
        }
    }

    fn register_resource(&self, output_id: OutputId, request: RegisterResourceRequest) {
        {
            self.expected_results
                .borrow_mut()
                .insert(output_id, request.expected_results.clone());
        }

        let object = Self::create_protobuf_struct(request.object.clone());

        let req = grpc::RegisterResourceRequest {
            r#type: request.r#type.clone(),
            name: request.name.clone(),
            parent: "".to_string(),
            custom: true,
            object: Some(object),
            protect: false,
            dependencies: vec![],
            provider: "".to_string(),
            property_dependencies: Default::default(),
            // property_dependencies: HashMap::from(
            //     [("value".to_string(), register_resource_request::PropertyDependencies { urns: vec!["test".to_string()] })]
            // ),
            delete_before_replace: false,
            version: "".to_string(),
            ignore_changes: vec![],
            accept_secrets: true,
            additional_secret_outputs: vec![],
            alias_ur_ns: vec![],
            import_id: "".to_string(),
            custom_timeouts: None,
            delete_before_replace_defined: false,
            supports_partial_values: false,
            remote: false,
            accept_resources: false,
            providers: Default::default(),
            replace_on_changes: vec![],
            plugin_download_url: "".to_string(),
            plugin_checksums: Default::default(),
            retain_on_delete: false,
            aliases: vec![],
            deleted_with: "".to_string(),
            alias_specs: true,
            source_position: None,
        };

        self.connector
            .create_resource(output_id.to_string(), req.encode_to_vec());
    }

    fn register_resource_poll(
        &self,
        _register_ids: &HashSet<OutputId>,
    ) -> HashMap<OutputId, RegisterResourceResponse> {
        let results = { self.connector.get_created_resources() };

        let mut map = HashMap::new();

        for (output_id, response) in results {
            let output_id = output_id.into();
            let expected_results_ref = self.expected_results.borrow();
            let expected_results = expected_results_ref.get(&output_id).unwrap();

            let response = grpc::RegisterResourceResponse::decode(&*response).unwrap();

            let object = response.object.unwrap_or(Struct::default());

            let result = Self::protoc_object_to_messagepack_map(object, expected_results.clone());

            map.insert(output_id, RegisterResourceResponse { outputs: result });
        }

        map
    }
}

const UNKNOWN_VALUE: &str = "04da6b54-80e4-46f7-96ec-b56ff0331ba9";

impl PulumiServiceImpl {
    fn protoc_object_to_messagepack_map(
        o: Struct,
        schema: HashMap<FieldName, msgpack_protobuf_converter::Type>,
    ) -> HashMap<FieldName, Value> {
        o.fields
            .iter()
            .flat_map(|(k, v)| match schema.get(&k.into()) {
                None => {
                    warn!("Schema for field [{k}] not found");
                    None
                }
                Some(t) => {
                    let v = msgpack_protobuf_converter::protobuf_to_msgpack(v, t).unwrap();
                    Some((k.into(), v))
                }
            })
            .collect::<HashMap<_, _>>()
    }

    fn create_protobuf_struct(fields: HashMap<FieldName, Option<Value>>) -> Struct {
        let pairs = fields
            .iter()
            .map(|(name, value)| {
                let v = match value {
                    None => prost_types::Value {
                        kind: Some(prost_types::value::Kind::StringValue(UNKNOWN_VALUE.into())),
                    },
                    Some(value) => msgpack_protobuf_converter::msgpack_to_protobuf(value).unwrap(),
                };
                (name.as_string().clone(), v)
            })
            .collect::<Vec<_>>();

        Struct {
            fields: BTreeMap::from_iter(pairs),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use prost_types::Struct;

    use crate::pulumi::service_impl::PulumiServiceImpl;

    #[test]
    fn protoc_object_to_messagepack_map_ignored_fields_without_schema() {
        let s = Struct {
            fields: BTreeMap::from([
                (
                    "field1".into(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::NumberValue(1.0)),
                    },
                ),
                (
                    "field2".into(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::NumberValue(2.0)),
                    },
                ),
            ]),
        };

        let schema = HashMap::from([("field1".into(), msgpack_protobuf_converter::Type::Double)]);
        let result = PulumiServiceImpl::protoc_object_to_messagepack_map(s, schema);

        assert_eq!(result, HashMap::from([("field1".into(), 1.0.into())]))
    }

    mod register_resource {}

    mod register_resource_poll {}
}
