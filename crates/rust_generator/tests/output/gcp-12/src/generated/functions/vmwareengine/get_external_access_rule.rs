#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_external_access_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExternalAccessRuleArgs {
        /// Name of the resource.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name of the network policy that this cluster belongs.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetExternalAccessRuleResult {
        pub action: pulumi_gestalt_rust::Output<String>,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub destination_ip_ranges: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::vmwareengine::GetExternalAccessRuleDestinationIpRange,
            >,
        >,
        pub destination_ports: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ip_protocol: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub parent: pulumi_gestalt_rust::Output<String>,
        pub priority: pulumi_gestalt_rust::Output<i32>,
        pub source_ip_ranges: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::vmwareengine::GetExternalAccessRuleSourceIpRange,
            >,
        >,
        pub source_ports: pulumi_gestalt_rust::Output<Vec<String>>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub uid: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetExternalAccessRuleArgs,
    ) -> GetExternalAccessRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:vmwareengine/getExternalAccessRule:getExternalAccessRule".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetExternalAccessRuleResult {
            action: o.get_field("action"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            destination_ip_ranges: o.get_field("destinationIpRanges"),
            destination_ports: o.get_field("destinationPorts"),
            id: o.get_field("id"),
            ip_protocol: o.get_field("ipProtocol"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            priority: o.get_field("priority"),
            source_ip_ranges: o.get_field("sourceIpRanges"),
            source_ports: o.get_field("sourcePorts"),
            state: o.get_field("state"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
