#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// Returns information on a specific connect instance by alias
        #[builder(into, default)]
        pub instance_alias: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific connect instance by id
        #[builder(into, default)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assigned to the instance.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        /// ARN of the instance.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub auto_resolve_best_voices_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether contact flow logs are enabled.
        pub contact_flow_logs_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether contact lens is enabled.
        pub contact_lens_enabled: pulumi_gestalt_rust::Output<bool>,
        /// When the instance was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Whether early media for outbound calls is enabled .
        pub early_media_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies The identity management type attached to the instance.
        pub identity_management_type: pulumi_gestalt_rust::Output<String>,
        /// Whether inbound calls are enabled.
        pub inbound_calls_enabled: pulumi_gestalt_rust::Output<bool>,
        pub instance_alias: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Whether multi-party calls/conference is enabled.
        pub multi_party_conference_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether outbound calls are enabled.
        pub outbound_calls_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Service role of the instance.
        pub service_role: pulumi_gestalt_rust::Output<String>,
        /// State of the instance.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assigned to the instance.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInstanceResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auto_resolve_best_voices_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoResolveBestVoicesEnabled"),
            ),
            contact_flow_logs_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contactFlowLogsEnabled"),
            ),
            contact_lens_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contactLensEnabled"),
            ),
            created_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            early_media_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("earlyMediaEnabled"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identity_management_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityManagementType"),
            ),
            inbound_calls_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inboundCallsEnabled"),
            ),
            instance_alias: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceAlias"),
            ),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            multi_party_conference_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiPartyConferenceEnabled"),
            ),
            outbound_calls_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outboundCallsEnabled"),
            ),
            service_role: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceRole"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
