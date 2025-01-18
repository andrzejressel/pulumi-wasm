pub mod get_fleet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFleetArgs {
        /// Fleet name.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Mapping of Key-Value tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
    pub fn invoke(args: GetFleetArgs) -> GetFleetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "baseCapacity".into(),
                },
                register_interface::ResultField {
                    name: "computeType".into(),
                },
                register_interface::ResultField {
                    name: "created".into(),
                },
                register_interface::ResultField {
                    name: "environmentType".into(),
                },
                register_interface::ResultField {
                    name: "fleetServiceRole".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageId".into(),
                },
                register_interface::ResultField {
                    name: "lastModified".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "overflowBehavior".into(),
                },
                register_interface::ResultField {
                    name: "scalingConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "statuses".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcConfigs".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFleetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            base_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseCapacity").unwrap(),
            ),
            compute_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeType").unwrap(),
            ),
            created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("created").unwrap(),
            ),
            environment_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentType").unwrap(),
            ),
            fleet_service_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleetServiceRole").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageId").unwrap(),
            ),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModified").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            overflow_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("overflowBehavior").unwrap(),
            ),
            scaling_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalingConfigurations").unwrap(),
            ),
            statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statuses").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfigs").unwrap(),
            ),
        }
    }
}
