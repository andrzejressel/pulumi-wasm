#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_outpost {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOutpostArgs {
        /// ARN.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the Outpost.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the Outpost.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AWS Account identifier of the Outpost owner.
        #[builder(into, default)]
        pub owner_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Outpost tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetOutpostResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Availability Zone name.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Availability Zone identifier.
        pub availability_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the Outpost.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The life cycle status.
        pub lifecycle_status: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub owner_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the site.
        pub site_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the site.
        pub site_id: pulumi_gestalt_rust::Output<String>,
        /// The hardware type.
        pub supported_hardware_type: pulumi_gestalt_rust::Output<String>,
        /// The Outpost tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOutpostArgs,
    ) -> GetOutpostResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let id_binding = args.id.get_output(context);
        let name_binding = args.name.get_output(context);
        let owner_id_binding = args.owner_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:outposts/getOutpost:getOutpost".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownerId".into(),
                    value: &owner_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOutpostResult {
            arn: o.get_field("arn"),
            availability_zone: o.get_field("availabilityZone"),
            availability_zone_id: o.get_field("availabilityZoneId"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            lifecycle_status: o.get_field("lifecycleStatus"),
            name: o.get_field("name"),
            owner_id: o.get_field("ownerId"),
            site_arn: o.get_field("siteArn"),
            site_id: o.get_field("siteId"),
            supported_hardware_type: o.get_field("supportedHardwareType"),
            tags: o.get_field("tags"),
        }
    }
}
