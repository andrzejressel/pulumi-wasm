pub mod get_listener {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetListenerArgs {
        /// ID or Amazon Resource Name (ARN) of the listener
        #[builder(into)]
        pub listener_identifier: pulumi_wasm_rust::Output<String>,
        /// ID or Amazon Resource Name (ARN) of the service network
        #[builder(into)]
        pub service_identifier: pulumi_wasm_rust::Output<String>,
        /// List of tags associated with the listener.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetListenerResult {
        /// ARN of the listener.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date and time that the listener was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The actions for the default listener rule.
        pub default_actions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vpclattice::GetListenerDefaultAction>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The date and time the listener was last updated.
        pub last_updated_at: pulumi_wasm_rust::Output<String>,
        /// The ID of the listener.
        pub listener_id: pulumi_wasm_rust::Output<String>,
        pub listener_identifier: pulumi_wasm_rust::Output<String>,
        /// The name of the listener.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The listener port.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The listener protocol. Either `HTTPS` or `HTTP`.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// The ARN of the service.
        pub service_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the service.
        pub service_id: pulumi_wasm_rust::Output<String>,
        pub service_identifier: pulumi_wasm_rust::Output<String>,
        /// List of tags associated with the listener.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetListenerArgs) -> GetListenerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let listener_identifier_binding = args.listener_identifier.get_inner();
        let service_identifier_binding = args.service_identifier.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:vpclattice/getListener:getListener".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "listenerIdentifier".into(),
                    value: &listener_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "serviceIdentifier".into(),
                    value: &service_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "defaultActions".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedAt".into(),
                },
                register_interface::ResultField {
                    name: "listenerId".into(),
                },
                register_interface::ResultField {
                    name: "listenerIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "serviceArn".into(),
                },
                register_interface::ResultField {
                    name: "serviceId".into(),
                },
                register_interface::ResultField {
                    name: "serviceIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetListenerResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            default_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultActions").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedAt").unwrap(),
            ),
            listener_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listenerId").unwrap(),
            ),
            listener_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listenerIdentifier").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            service_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceArn").unwrap(),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceId").unwrap(),
            ),
            service_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceIdentifier").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
