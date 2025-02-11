#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance_guest_attributes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceGuestAttributesArgs {
        /// The name or self_link of the instance.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If `self_link` is provided, this value is ignored.  If neither `self_link`
        /// nor `project` are provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Path to query for the guest attributes. Consists of
        /// `namespace` name for the attributes followed with a `/`.
        #[builder(into, default)]
        pub query_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key of a variable to get the value of. Consists of
        /// `namespace` name and `key` name for the variable separated by a `/`.
        #[builder(into, default)]
        pub variable_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone of the instance. If `self_link` is provided, this
        /// value is ignored.  If neither `self_link` nor `zone` are provided, the
        /// provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceGuestAttributesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub query_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// Structure is documented below.
        pub query_values: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGuestAttributesQueryValue,
            >,
        >,
        pub region: pulumi_gestalt_rust::Output<String>,
        pub variable_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Value of the queried guest_attribute.
        pub variable_value: pulumi_gestalt_rust::Output<String>,
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceGuestAttributesArgs,
    ) -> GetInstanceGuestAttributesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let query_path_binding = args.query_path.get_output(context);
        let region_binding = args.region.get_output(context);
        let variable_key_binding = args.variable_key.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getInstanceGuestAttributes:getInstanceGuestAttributes"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryPath".into(),
                    value: &query_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "variableKey".into(),
                    value: &variable_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceGuestAttributesResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            query_path: o.get_field("queryPath"),
            query_values: o.get_field("queryValues"),
            region: o.get_field("region"),
            variable_key: o.get_field("variableKey"),
            variable_value: o.get_field("variableValue"),
            zone: o.get_field("zone"),
        }
    }
}
