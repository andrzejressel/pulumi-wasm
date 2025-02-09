#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_fleet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFleetArgs {
        /// Fleet name.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Mapping of Key-Value tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFleetResult {
        /// ARN of the Fleet.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Number of machines allocated to the ﬂeet.
        pub base_capacity: pulumi_gestalt_rust::Output<i32>,
        /// Compute resources the compute fleet uses.
        pub compute_type: pulumi_gestalt_rust::Output<String>,
        /// Creation time of the fleet.
        pub created: pulumi_gestalt_rust::Output<String>,
        /// Environment type of the compute fleet.
        pub environment_type: pulumi_gestalt_rust::Output<String>,
        /// The service role associated with the compute fleet.
        pub fleet_service_role: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Fleet.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Machine Image (AMI) of the compute fleet.
        pub image_id: pulumi_gestalt_rust::Output<String>,
        /// Last modification time of the fleet.
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Overflow behavior for compute fleet.
        pub overflow_behavior: pulumi_gestalt_rust::Output<String>,
        /// Nested attribute containing information about the scaling configuration.
        pub scaling_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::codebuild::GetFleetScalingConfiguration>,
        >,
        /// Nested attribute containing information about the current status of the fleet.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::codebuild::GetFleetStatus>,
        >,
        /// Mapping of Key-Value tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Nested attribute containing information about the VPC configuration.
        pub vpc_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::codebuild::GetFleetVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFleetArgs,
    ) -> GetFleetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:codebuild/getFleet:getFleet".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFleetResult {
            arn: o.get_field("arn"),
            base_capacity: o.get_field("baseCapacity"),
            compute_type: o.get_field("computeType"),
            created: o.get_field("created"),
            environment_type: o.get_field("environmentType"),
            fleet_service_role: o.get_field("fleetServiceRole"),
            id: o.get_field("id"),
            image_id: o.get_field("imageId"),
            last_modified: o.get_field("lastModified"),
            name: o.get_field("name"),
            overflow_behavior: o.get_field("overflowBehavior"),
            scaling_configurations: o.get_field("scalingConfigurations"),
            statuses: o.get_field("statuses"),
            tags: o.get_field("tags"),
            vpc_configs: o.get_field("vpcConfigs"),
        }
    }
}
