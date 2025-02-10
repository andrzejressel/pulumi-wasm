#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_outposts {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOutpostsArgs {
        /// Availability Zone name.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Availability Zone identifier.
        #[builder(into, default)]
        pub availability_zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AWS Account identifier of the Outpost owner.
        #[builder(into, default)]
        pub owner_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Site identifier.
        #[builder(into, default)]
        pub site_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetOutpostsResult {
        /// Set of Amazon Resource Names (ARNs).
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        pub availability_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of identifiers.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        pub site_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOutpostsArgs,
    ) -> GetOutpostsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let availability_zone_binding = args.availability_zone.get_output(context);
        let availability_zone_id_binding = args.availability_zone_id.get_output(context);
        let owner_id_binding = args.owner_id.get_output(context);
        let site_id_binding = args.site_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:outposts/getOutposts:getOutposts".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: availability_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZoneId".into(),
                    value: availability_zone_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownerId".into(),
                    value: owner_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteId".into(),
                    value: site_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOutpostsResult {
            arns: o.get_field("arns"),
            availability_zone: o.get_field("availabilityZone"),
            availability_zone_id: o.get_field("availabilityZoneId"),
            id: o.get_field("id"),
            ids: o.get_field("ids"),
            owner_id: o.get_field("ownerId"),
            site_id: o.get_field("siteId"),
        }
    }
}
