pub mod get_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// Returns information on a specific connect instance by alias
        #[builder(into, default)]
        pub instance_alias: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific connect instance by id
        #[builder(into, default)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assigned to the instance.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        /// ARN of the instance.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub auto_resolve_best_voices_enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether contact flow logs are enabled.
        pub contact_flow_logs_enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether contact lens is enabled.
        pub contact_lens_enabled: pulumi_wasm_rust::Output<bool>,
        /// When the instance was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Whether early media for outbound calls is enabled .
        pub early_media_enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Specifies The identity management type attached to the instance.
        pub identity_management_type: pulumi_wasm_rust::Output<String>,
        /// Whether inbound calls are enabled.
        pub inbound_calls_enabled: pulumi_wasm_rust::Output<bool>,
        pub instance_alias: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Whether multi-party calls/conference is enabled.
        pub multi_party_conference_enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether outbound calls are enabled.
        pub outbound_calls_enabled: pulumi_wasm_rust::Output<bool>,
        /// Service role of the instance.
        pub service_role: pulumi_wasm_rust::Output<String>,
        /// State of the instance.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assigned to the instance.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_alias_binding = args.instance_alias.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getInstance:getInstance".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceAlias".into(),
                    value: &instance_alias_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
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
                    name: "autoResolveBestVoicesEnabled".into(),
                },
                register_interface::ResultField {
                    name: "contactFlowLogsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "contactLensEnabled".into(),
                },
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "earlyMediaEnabled".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identityManagementType".into(),
                },
                register_interface::ResultField {
                    name: "inboundCallsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "instanceAlias".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "multiPartyConferenceEnabled".into(),
                },
                register_interface::ResultField {
                    name: "outboundCallsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "serviceRole".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInstanceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_resolve_best_voices_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoResolveBestVoicesEnabled").unwrap(),
            ),
            contact_flow_logs_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contactFlowLogsEnabled").unwrap(),
            ),
            contact_lens_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contactLensEnabled").unwrap(),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            early_media_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("earlyMediaEnabled").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identity_management_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityManagementType").unwrap(),
            ),
            inbound_calls_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inboundCallsEnabled").unwrap(),
            ),
            instance_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceAlias").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            multi_party_conference_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiPartyConferenceEnabled").unwrap(),
            ),
            outbound_calls_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundCallsEnabled").unwrap(),
            ),
            service_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceRole").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
