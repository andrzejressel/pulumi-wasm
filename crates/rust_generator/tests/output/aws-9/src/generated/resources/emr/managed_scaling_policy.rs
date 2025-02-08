/// Provides a Managed Scaling policy for EMR Cluster. With Amazon EMR versions 5.30.0 and later (except for Amazon EMR 6.0.0), you can enable EMR managed scaling to automatically increase or decrease the number of instances or units in your cluster based on workload. See [Using EMR Managed Scaling in Amazon EMR](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-managed-scaling.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation)]
pub mod managed_scaling_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedScalingPolicyArgs {
        /// ID of the EMR cluster
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block with compute limit settings. Described below.
        #[builder(into)]
        pub compute_limits: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::emr::ManagedScalingPolicyComputeLimit>,
        >,
    }
    #[allow(dead_code)]
    pub struct ManagedScalingPolicyResult {
        /// ID of the EMR cluster
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with compute limit settings. Described below.
        pub compute_limits: pulumi_gestalt_rust::Output<
            Vec<super::super::types::emr::ManagedScalingPolicyComputeLimit>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ManagedScalingPolicyArgs,
    ) -> ManagedScalingPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_output(context).get_inner();
        let compute_limits_binding = args.compute_limits.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:emr/managedScalingPolicy:ManagedScalingPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManagedScalingPolicyResult {
            cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterId"),
            ),
            compute_limits: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("computeLimits"),
            ),
        }
    }
}
