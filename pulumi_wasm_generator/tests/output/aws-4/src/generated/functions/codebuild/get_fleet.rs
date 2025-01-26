pub mod get_fleet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFleetArgs {
        /// Fleet name.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Mapping of Key-Value tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFleetResult {
        /// ARN of the Fleet.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Number of machines allocated to the ﬂeet.
        pub base_capacity: pulumi_wasm_rust::Output<i32>,
        /// Compute resources the compute fleet uses.
        pub compute_type: pulumi_wasm_rust::Output<String>,
        /// Creation time of the fleet.
        pub created: pulumi_wasm_rust::Output<String>,
        /// Environment type of the compute fleet.
        pub environment_type: pulumi_wasm_rust::Output<String>,
        /// The service role associated with the compute fleet.
        pub fleet_service_role: pulumi_wasm_rust::Output<String>,
        /// ARN of the Fleet.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Machine Image (AMI) of the compute fleet.
        pub image_id: pulumi_wasm_rust::Output<String>,
        /// Last modification time of the fleet.
        pub last_modified: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Overflow behavior for compute fleet.
        pub overflow_behavior: pulumi_wasm_rust::Output<String>,
        /// Nested attribute containing information about the scaling configuration.
        pub scaling_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::codebuild::GetFleetScalingConfiguration>,
        >,
        /// Nested attribute containing information about the current status of the fleet.
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::codebuild::GetFleetStatus>,
        >,
        /// Mapping of Key-Value tags for the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Nested attribute containing information about the VPC configuration.
        pub vpc_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::codebuild::GetFleetVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetFleetArgs,
    ) -> GetFleetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:codebuild/getFleet:getFleet".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFleetResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            base_capacity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("baseCapacity"),
            ),
            compute_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("computeType"),
            ),
            created: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("created"),
            ),
            environment_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("environmentType"),
            ),
            fleet_service_role: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fleetServiceRole"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            image_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageId"),
            ),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModified"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            overflow_behavior: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("overflowBehavior"),
            ),
            scaling_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scalingConfigurations"),
            ),
            statuses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("statuses"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            vpc_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcConfigs"),
            ),
        }
    }
}
