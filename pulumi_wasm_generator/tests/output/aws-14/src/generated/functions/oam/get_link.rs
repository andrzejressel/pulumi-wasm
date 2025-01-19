pub mod get_link {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLinkArgs {
        /// ARN of the link.
        #[builder(into)]
        pub link_identifier: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLinkResult {
        /// ARN of the link.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Label that is assigned to this link.
        pub label: pulumi_wasm_rust::Output<String>,
        /// Human-readable name used to identify this source account when you are viewing data from it in the monitoring account.
        pub label_template: pulumi_wasm_rust::Output<String>,
        /// Configuration for creating filters that specify that only some metric namespaces or log groups are to be shared from the source account to the monitoring account. See `link_configuration` Block for details.
        pub link_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::oam::GetLinkLinkConfiguration>,
        >,
        /// ID string that AWS generated as part of the link ARN.
        pub link_id: pulumi_wasm_rust::Output<String>,
        pub link_identifier: pulumi_wasm_rust::Output<String>,
        /// Types of data that the source account shares with the monitoring account.
        pub resource_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// ARN of the sink that is used for this link.
        pub sink_arn: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLinkArgs) -> GetLinkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let link_identifier_binding = args.link_identifier.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:oam/getLink:getLink".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "linkIdentifier".into(),
                    value: &link_identifier_binding,
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "label".into(),
                },
                register_interface::ResultField {
                    name: "labelTemplate".into(),
                },
                register_interface::ResultField {
                    name: "linkConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "linkId".into(),
                },
                register_interface::ResultField {
                    name: "linkIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "resourceTypes".into(),
                },
                register_interface::ResultField {
                    name: "sinkArn".into(),
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
        GetLinkResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("label").unwrap(),
            ),
            label_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labelTemplate").unwrap(),
            ),
            link_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkConfigurations").unwrap(),
            ),
            link_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkId").unwrap(),
            ),
            link_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkIdentifier").unwrap(),
            ),
            resource_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTypes").unwrap(),
            ),
            sink_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sinkArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
