/// Provides an Elastic MapReduce Cluster Instance Group configuration.
/// See [Amazon Elastic MapReduce Documentation](https://aws.amazon.com/documentation/emr/) for more information.
///
/// > **NOTE:** At this time, Instance Groups cannot be destroyed through the API nor
/// web interface. Instance Groups are destroyed when the EMR Cluster is destroyed.
/// this provider will resize any Instance Group to zero when destroying the resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let task = instance_group::create(
///         "task",
///         InstanceGroupArgs::builder()
///             .cluster_id("${[\"tf-test-cluster\"].id}")
///             .instance_count(1)
///             .instance_type("m5.xlarge")
///             .name("my little instance group")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EMR task instance group using their EMR Cluster id and Instance Group id separated by a forward-slash `/`. For example:
///
/// ```sh
/// $ pulumi import aws:emr/instanceGroup:InstanceGroup task_group j-123456ABCDEF/ig-15EK4O09RZLNR
/// ```
pub mod instance_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceGroupArgs {
        /// The autoscaling policy document. This is a JSON formatted string. See [EMR Auto Scaling](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-automatic-scaling.html)
        #[builder(into, default)]
        pub autoscaling_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// If set, the bid price for each EC2 instance in the instance group, expressed in USD. By setting this attribute, the instance group is being declared as a Spot Instance, and will implicitly create a Spot request. Leave this blank to use On-Demand Instances.
        #[builder(into, default)]
        pub bid_price: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the EMR Cluster to attach to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// A JSON string for supplying list of configurations specific to the EMR instance group. Note that this can only be changed when using EMR release 5.21 or later.
        ///
        /// ```ignore
        /// use pulumi_wasm_rust::Output;
        /// use pulumi_wasm_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let task = instance_group::create(
        ///         "task",
        ///         InstanceGroupArgs::builder()
        ///             .configurations_json(
        ///                 "[\n{\n\"Classification\": \"hadoop-env\",\n\"Configurations\": [\n{\n\"Classification\": \"export\",\n\"Properties\": {\n\"JAVA_HOME\": \"/usr/lib/jvm/java-1.8.0\"\n}\n}\n],\n\"Properties\": {}\n}\n]",
        ///             )
        ///             .build_struct(),
        ///     );
        /// }
        /// ```
        #[builder(into, default)]
        pub configurations_json: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `ebs_config` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub ebs_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::emr::InstanceGroupEbsConfig>>,
        >,
        /// Indicates whether an Amazon EBS volume is EBS-optimized. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub ebs_optimized: pulumi_wasm_rust::Output<Option<bool>>,
        /// target number of instances for the instance group. defaults to 0.
        #[builder(into, default)]
        pub instance_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The EC2 instance type for all instances in the instance group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// Human friendly name given to the instance group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceGroupResult {
        /// The autoscaling policy document. This is a JSON formatted string. See [EMR Auto Scaling](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-automatic-scaling.html)
        pub autoscaling_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// If set, the bid price for each EC2 instance in the instance group, expressed in USD. By setting this attribute, the instance group is being declared as a Spot Instance, and will implicitly create a Spot request. Leave this blank to use On-Demand Instances.
        pub bid_price: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the EMR Cluster to attach to. Changing this forces a new resource to be created.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// A JSON string for supplying list of configurations specific to the EMR instance group. Note that this can only be changed when using EMR release 5.21 or later.
        ///
        /// ```ignore
        /// use pulumi_wasm_rust::Output;
        /// use pulumi_wasm_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let task = instance_group::create(
        ///         "task",
        ///         InstanceGroupArgs::builder()
        ///             .configurations_json(
        ///                 "[\n{\n\"Classification\": \"hadoop-env\",\n\"Configurations\": [\n{\n\"Classification\": \"export\",\n\"Properties\": {\n\"JAVA_HOME\": \"/usr/lib/jvm/java-1.8.0\"\n}\n}\n],\n\"Properties\": {}\n}\n]",
        ///             )
        ///             .build_struct(),
        ///     );
        /// }
        /// ```
        pub configurations_json: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `ebs_config` blocks as defined below. Changing this forces a new resource to be created.
        pub ebs_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::emr::InstanceGroupEbsConfig>,
        >,
        /// Indicates whether an Amazon EBS volume is EBS-optimized. Changing this forces a new resource to be created.
        pub ebs_optimized: pulumi_wasm_rust::Output<Option<bool>>,
        /// target number of instances for the instance group. defaults to 0.
        pub instance_count: pulumi_wasm_rust::Output<i32>,
        /// The EC2 instance type for all instances in the instance group. Changing this forces a new resource to be created.
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// Human friendly name given to the instance group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of instances currently running in this instance group.
        pub running_instance_count: pulumi_wasm_rust::Output<i32>,
        /// The current status of the instance group.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InstanceGroupArgs) -> InstanceGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let autoscaling_policy_binding = args.autoscaling_policy.get_inner();
        let bid_price_binding = args.bid_price.get_inner();
        let cluster_id_binding = args.cluster_id.get_inner();
        let configurations_json_binding = args.configurations_json.get_inner();
        let ebs_configs_binding = args.ebs_configs.get_inner();
        let ebs_optimized_binding = args.ebs_optimized.get_inner();
        let instance_count_binding = args.instance_count.get_inner();
        let instance_type_binding = args.instance_type.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:emr/instanceGroup:InstanceGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoscalingPolicy".into(),
                    value: &autoscaling_policy_binding,
                },
                register_interface::ObjectField {
                    name: "bidPrice".into(),
                    value: &bid_price_binding,
                },
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "configurationsJson".into(),
                    value: &configurations_json_binding,
                },
                register_interface::ObjectField {
                    name: "ebsConfigs".into(),
                    value: &ebs_configs_binding,
                },
                register_interface::ObjectField {
                    name: "ebsOptimized".into(),
                    value: &ebs_optimized_binding,
                },
                register_interface::ObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoscalingPolicy".into(),
                },
                register_interface::ResultField {
                    name: "bidPrice".into(),
                },
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "configurationsJson".into(),
                },
                register_interface::ResultField {
                    name: "ebsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "ebsOptimized".into(),
                },
                register_interface::ResultField {
                    name: "instanceCount".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "runningInstanceCount".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceGroupResult {
            autoscaling_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscalingPolicy").unwrap(),
            ),
            bid_price: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bidPrice").unwrap(),
            ),
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            configurations_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationsJson").unwrap(),
            ),
            ebs_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsConfigs").unwrap(),
            ),
            ebs_optimized: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsOptimized").unwrap(),
            ),
            instance_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceCount").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            running_instance_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runningInstanceCount").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
