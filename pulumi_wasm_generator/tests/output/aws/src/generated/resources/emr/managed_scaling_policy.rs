/// Provides a Managed Scaling policy for EMR Cluster. With Amazon EMR versions 5.30.0 and later (except for Amazon EMR 6.0.0), you can enable EMR managed scaling to automatically increase or decrease the number of instances or units in your cluster based on workload. See [Using EMR Managed Scaling in Amazon EMR](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-managed-scaling.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sample = cluster::create(
///         "sample",
///         ClusterArgs::builder()
///             .core_instance_group(
///                 ClusterCoreInstanceGroup::builder()
///                     .instanceType("c4.large")
///                     .build_struct(),
///             )
///             .master_instance_group(
///                 ClusterMasterInstanceGroup::builder()
///                     .instanceType("m4.large")
///                     .build_struct(),
///             )
///             .name("emr-sample-cluster")
///             .release_label("emr-5.30.0")
///             .build_struct(),
///     );
///     let samplepolicy = managed_scaling_policy::create(
///         "samplepolicy",
///         ManagedScalingPolicyArgs::builder()
///             .cluster_id("${sample.id}")
///             .compute_limits(
///                 vec![
///                     ManagedScalingPolicyComputeLimit::builder().maximumCapacityUnits(10)
///                     .maximumCoreCapacityUnits(10).maximumOndemandCapacityUnits(2)
///                     .minimumCapacityUnits(2).unitType("Instances").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EMR Managed Scaling Policies using the EMR Cluster identifier. For example:
///
/// ```sh
/// $ pulumi import aws:emr/managedScalingPolicy:ManagedScalingPolicy example j-123456ABCDEF
/// ```
pub mod managed_scaling_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedScalingPolicyArgs {
        /// ID of the EMR cluster
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// Configuration block with compute limit settings. Described below.
        #[builder(into)]
        pub compute_limits: pulumi_wasm_rust::Output<
            Vec<super::super::types::emr::ManagedScalingPolicyComputeLimit>,
        >,
    }
    #[allow(dead_code)]
    pub struct ManagedScalingPolicyResult {
        /// ID of the EMR cluster
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// Configuration block with compute limit settings. Described below.
        pub compute_limits: pulumi_wasm_rust::Output<
            Vec<super::super::types::emr::ManagedScalingPolicyComputeLimit>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedScalingPolicyArgs,
    ) -> ManagedScalingPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_inner();
        let compute_limits_binding = args.compute_limits.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:emr/managedScalingPolicy:ManagedScalingPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "computeLimits".into(),
                    value: &compute_limits_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "computeLimits".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedScalingPolicyResult {
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            compute_limits: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeLimits").unwrap(),
            ),
        }
    }
}