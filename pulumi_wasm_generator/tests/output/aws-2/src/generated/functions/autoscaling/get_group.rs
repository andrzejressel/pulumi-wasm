pub mod get_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupArgs {
        /// Specify the exact name of the desired autoscaling group.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupResult {
        /// ARN of the Auto Scaling group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// One or more Availability Zones for the group.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        pub default_cooldown: pulumi_wasm_rust::Output<i32>,
        /// Desired size of the group.
        pub desired_capacity: pulumi_wasm_rust::Output<i32>,
        /// The unit of measurement for the value returned for `desired_capacity`.
        pub desired_capacity_type: pulumi_wasm_rust::Output<String>,
        /// List of metrics enabled for collection.
        pub enabled_metrics: pulumi_wasm_rust::Output<Vec<String>>,
        /// The amount of time, in seconds, that Amazon EC2 Auto Scaling waits before checking the health status of an EC2 instance that has come into service.
        pub health_check_grace_period: pulumi_wasm_rust::Output<i32>,
        /// Service to use for the health checks. The valid values are EC2 and ELB.
        pub health_check_type: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Instance maintenance policy for the group.
        pub instance_maintenance_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::autoscaling::GetGroupInstanceMaintenancePolicy,
            >,
        >,
        /// The name of the associated launch configuration.
        pub launch_configuration: pulumi_wasm_rust::Output<String>,
        /// List of launch templates along with the overrides.
        pub launch_templates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::autoscaling::GetGroupLaunchTemplate>,
        >,
        /// One or more load balancers associated with the group.
        pub load_balancers: pulumi_wasm_rust::Output<Vec<String>>,
        /// Maximum amount of time, in seconds, that an instance can be in service.
        pub max_instance_lifetime: pulumi_wasm_rust::Output<i32>,
        /// Maximum size of the group.
        pub max_size: pulumi_wasm_rust::Output<i32>,
        /// Minimum number of instances to maintain in the warm pool.
        pub min_size: pulumi_wasm_rust::Output<i32>,
        /// List of mixed instances policy objects for the group.
        pub mixed_instances_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::autoscaling::GetGroupMixedInstancesPolicy>,
        >,
        /// Name of the Auto Scaling Group.
        pub name: pulumi_wasm_rust::Output<String>,
        pub new_instances_protected_from_scale_in: pulumi_wasm_rust::Output<bool>,
        /// Name of the placement group into which to launch your instances, if any. For more information, see Placement Groups (http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html) in the Amazon Elastic Compute Cloud User Guide.
        pub placement_group: pulumi_wasm_rust::Output<String>,
        /// Predicted capacity of the group.
        pub predicted_capacity: pulumi_wasm_rust::Output<i32>,
        /// ARN of the service-linked role that the Auto Scaling group uses to call other AWS services on your behalf.
        pub service_linked_role_arn: pulumi_wasm_rust::Output<String>,
        /// Current state of the group when DeleteAutoScalingGroup is in progress.
        pub status: pulumi_wasm_rust::Output<String>,
        /// List of processes suspended processes for the Auto Scaling Group.
        pub suspended_processes: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of tags for the group.
        pub tags: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::autoscaling::GetGroupTag>,
        >,
        /// ARNs of the target groups for your load balancer.
        pub target_group_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The termination policies for the group.
        pub termination_policies: pulumi_wasm_rust::Output<Vec<String>>,
        /// Traffic sources.
        pub traffic_sources: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::autoscaling::GetGroupTrafficSource>,
        >,
        /// VPC ID for the group.
        pub vpc_zone_identifier: pulumi_wasm_rust::Output<String>,
        /// Current size of the warm pool.
        pub warm_pool_size: pulumi_wasm_rust::Output<i32>,
        /// List of warm pool configuration objects.
        pub warm_pools: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::autoscaling::GetGroupWarmPool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetGroupArgs,
    ) -> GetGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:autoscaling/getGroup:getGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "defaultCooldown".into(),
                },
                register_interface::ResultField {
                    name: "desiredCapacity".into(),
                },
                register_interface::ResultField {
                    name: "desiredCapacityType".into(),
                },
                register_interface::ResultField {
                    name: "enabledMetrics".into(),
                },
                register_interface::ResultField {
                    name: "healthCheckGracePeriod".into(),
                },
                register_interface::ResultField {
                    name: "healthCheckType".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceMaintenancePolicies".into(),
                },
                register_interface::ResultField {
                    name: "launchConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "launchTemplates".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancers".into(),
                },
                register_interface::ResultField {
                    name: "maxInstanceLifetime".into(),
                },
                register_interface::ResultField {
                    name: "maxSize".into(),
                },
                register_interface::ResultField {
                    name: "minSize".into(),
                },
                register_interface::ResultField {
                    name: "mixedInstancesPolicies".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "newInstancesProtectedFromScaleIn".into(),
                },
                register_interface::ResultField {
                    name: "placementGroup".into(),
                },
                register_interface::ResultField {
                    name: "predictedCapacity".into(),
                },
                register_interface::ResultField {
                    name: "serviceLinkedRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "suspendedProcesses".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "targetGroupArns".into(),
                },
                register_interface::ResultField {
                    name: "terminationPolicies".into(),
                },
                register_interface::ResultField {
                    name: "trafficSources".into(),
                },
                register_interface::ResultField {
                    name: "vpcZoneIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "warmPoolSize".into(),
                },
                register_interface::ResultField {
                    name: "warmPools".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            default_cooldown: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultCooldown").unwrap(),
            ),
            desired_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredCapacity").unwrap(),
            ),
            desired_capacity_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredCapacityType").unwrap(),
            ),
            enabled_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledMetrics").unwrap(),
            ),
            health_check_grace_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheckGracePeriod").unwrap(),
            ),
            health_check_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheckType").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_maintenance_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceMaintenancePolicies").unwrap(),
            ),
            launch_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchConfiguration").unwrap(),
            ),
            launch_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchTemplates").unwrap(),
            ),
            load_balancers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancers").unwrap(),
            ),
            max_instance_lifetime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxInstanceLifetime").unwrap(),
            ),
            max_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxSize").unwrap(),
            ),
            min_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minSize").unwrap(),
            ),
            mixed_instances_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mixedInstancesPolicies").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            new_instances_protected_from_scale_in: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("newInstancesProtectedFromScaleIn").unwrap(),
            ),
            placement_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placementGroup").unwrap(),
            ),
            predicted_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("predictedCapacity").unwrap(),
            ),
            service_linked_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceLinkedRoleArn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            suspended_processes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suspendedProcesses").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            target_group_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetGroupArns").unwrap(),
            ),
            termination_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminationPolicies").unwrap(),
            ),
            traffic_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficSources").unwrap(),
            ),
            vpc_zone_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcZoneIdentifier").unwrap(),
            ),
            warm_pool_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("warmPoolSize").unwrap(),
            ),
            warm_pools: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("warmPools").unwrap(),
            ),
        }
    }
}
