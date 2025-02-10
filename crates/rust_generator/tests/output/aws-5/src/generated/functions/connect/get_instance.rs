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
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_alias_binding = args.instance_alias.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getInstance:getInstance".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceAlias".into(),
                    value: instance_alias_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceResult {
            arn: o.get_field("arn"),
            auto_resolve_best_voices_enabled: o
                .get_field("autoResolveBestVoicesEnabled"),
            contact_flow_logs_enabled: o.get_field("contactFlowLogsEnabled"),
            contact_lens_enabled: o.get_field("contactLensEnabled"),
            created_time: o.get_field("createdTime"),
            early_media_enabled: o.get_field("earlyMediaEnabled"),
            id: o.get_field("id"),
            identity_management_type: o.get_field("identityManagementType"),
            inbound_calls_enabled: o.get_field("inboundCallsEnabled"),
            instance_alias: o.get_field("instanceAlias"),
            instance_id: o.get_field("instanceId"),
            multi_party_conference_enabled: o.get_field("multiPartyConferenceEnabled"),
            outbound_calls_enabled: o.get_field("outboundCallsEnabled"),
            service_role: o.get_field("serviceRole"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
        }
    }
}
