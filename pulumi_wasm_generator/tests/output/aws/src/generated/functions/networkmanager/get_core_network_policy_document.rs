pub mod get_core_network_policy_document {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCoreNetworkPolicyDocumentArgs {
        /// In a core network, all attachments use the block argument `attachment_policies` section to map an attachment to a segment. Instead of manually associating a segment to each attachment, attachments use tags, and then the tags are used to associate the attachment to the specified segment. Detailed below.
        #[builder(into, default)]
        pub attachment_policies: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentAttachmentPolicy,
                >,
            >,
        >,
        /// The core network configuration section defines the Regions where a core network should operate. For AWS Regions that are defined in the policy, the core network creates a Core Network Edge where you can connect attachments. After it's created, each Core Network Edge is peered with every other defined Region and is configured with consistent segment and routing across all Regions. Regions cannot be removed until the associated attachments are deleted. Detailed below.
        #[builder(into)]
        pub core_network_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentCoreNetworkConfiguration,
            >,
        >,
        /// Block argument that defines the service insertion actions you want to include. Detailed below.
        #[builder(into, default)]
        pub network_function_groups: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentNetworkFunctionGroup,
                >,
            >,
        >,
        /// A block argument, `segment_actions` define how routing works between segments. By default, attachments can only communicate with other attachments in the same segment. Detailed below.
        #[builder(into, default)]
        pub segment_actions: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegmentAction,
                >,
            >,
        >,
        /// Block argument that defines the different segments in the network. Here you can provide descriptions, change defaults, and provide explicit Regional operational and route filters. The names defined for each segment are used in the `segment_actions` and `attachment_policies` section. Each segment is created, and operates, as a completely separated routing domain. By default, attachments can only communicate with other attachments in the same segment. Detailed below.
        #[builder(into)]
        pub segments: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegment,
            >,
        >,
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCoreNetworkPolicyDocumentResult {
        pub attachment_policies: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentAttachmentPolicy,
                >,
            >,
        >,
        pub core_network_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentCoreNetworkConfiguration,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Standard JSON policy document rendered based on the arguments above.
        pub json: pulumi_wasm_rust::Output<String>,
        pub network_function_groups: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentNetworkFunctionGroup,
                >,
            >,
        >,
        pub segment_actions: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegmentAction,
                >,
            >,
        >,
        pub segments: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegment,
            >,
        >,
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetCoreNetworkPolicyDocumentArgs,
    ) -> GetCoreNetworkPolicyDocumentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attachment_policies_binding = args.attachment_policies.get_inner();
        let core_network_configurations_binding = args
            .core_network_configurations
            .get_inner();
        let network_function_groups_binding = args.network_function_groups.get_inner();
        let segment_actions_binding = args.segment_actions.get_inner();
        let segments_binding = args.segments.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:networkmanager/getCoreNetworkPolicyDocument:getCoreNetworkPolicyDocument"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attachmentPolicies".into(),
                    value: &attachment_policies_binding,
                },
                register_interface::ObjectField {
                    name: "coreNetworkConfigurations".into(),
                    value: &core_network_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "networkFunctionGroups".into(),
                    value: &network_function_groups_binding,
                },
                register_interface::ObjectField {
                    name: "segmentActions".into(),
                    value: &segment_actions_binding,
                },
                register_interface::ObjectField {
                    name: "segments".into(),
                    value: &segments_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attachmentPolicies".into(),
                },
                register_interface::ResultField {
                    name: "coreNetworkConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "json".into(),
                },
                register_interface::ResultField {
                    name: "networkFunctionGroups".into(),
                },
                register_interface::ResultField {
                    name: "segmentActions".into(),
                },
                register_interface::ResultField {
                    name: "segments".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCoreNetworkPolicyDocumentResult {
            attachment_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachmentPolicies").unwrap(),
            ),
            core_network_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreNetworkConfigurations").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("json").unwrap(),
            ),
            network_function_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkFunctionGroups").unwrap(),
            ),
            segment_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("segmentActions").unwrap(),
            ),
            segments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("segments").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}