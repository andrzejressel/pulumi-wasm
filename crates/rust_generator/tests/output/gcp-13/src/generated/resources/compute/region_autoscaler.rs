/// Represents an Autoscaler resource.
///
/// Autoscalers allow you to automatically scale virtual machine instances in
/// managed instance groups according to an autoscaling policy that you
/// define.
///
///
/// To get more information about RegionAutoscaler, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/regionAutoscalers)
/// * How-to Guides
///     * [Autoscaling Groups of Instances](https://cloud.google.com/compute/docs/autoscaler/)
///
/// ## Example Usage
///
/// ### Region Autoscaler Basic
///
///
/// ```yaml
/// resources:
///   foobar:
///     type: gcp:compute:RegionAutoscaler
///     properties:
///       name: my-region-autoscaler
///       region: us-central1
///       target: ${foobarRegionInstanceGroupManager.id}
///       autoscalingPolicy:
///         maxReplicas: 5
///         minReplicas: 1
///         cooldownPeriod: 60
///         cpuUtilization:
///           target: 0.5
///   foobarInstanceTemplate:
///     type: gcp:compute:InstanceTemplate
///     name: foobar
///     properties:
///       name: my-instance-template
///       machineType: e2-standard-4
///       disks:
///         - sourceImage: debian-cloud/debian-11
///           diskSizeGb: 250
///       networkInterfaces:
///         - network: default
///           accessConfigs:
///             - networkTier: PREMIUM
///       serviceAccount:
///         scopes:
///           - https://www.googleapis.com/auth/devstorage.read_only
///           - https://www.googleapis.com/auth/logging.write
///           - https://www.googleapis.com/auth/monitoring.write
///           - https://www.googleapis.com/auth/pubsub
///           - https://www.googleapis.com/auth/service.management.readonly
///           - https://www.googleapis.com/auth/servicecontrol
///           - https://www.googleapis.com/auth/trace.append
///   foobarTargetPool:
///     type: gcp:compute:TargetPool
///     name: foobar
///     properties:
///       name: my-target-pool
///   foobarRegionInstanceGroupManager:
///     type: gcp:compute:RegionInstanceGroupManager
///     name: foobar
///     properties:
///       name: my-region-igm
///       region: us-central1
///       versions:
///         - instanceTemplate: ${foobarInstanceTemplate.id}
///           name: primary
///       targetPools:
///         - ${foobarTargetPool.id}
///       baseInstanceName: foobar
/// variables:
///   debian9:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
///
/// ## Import
///
/// RegionAutoscaler can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/autoscalers/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionAutoscaler can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionAutoscaler:RegionAutoscaler default projects/{{project}}/regions/{{region}}/autoscalers/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionAutoscaler:RegionAutoscaler default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionAutoscaler:RegionAutoscaler default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionAutoscaler:RegionAutoscaler default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_autoscaler {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionAutoscalerArgs {
        /// The configuration parameters for the autoscaling algorithm. You can
        /// define one or more of the policies for an autoscaler: cpuUtilization,
        /// customMetricUtilizations, and loadBalancingUtilization.
        /// If none of these are specified, the default will be to autoscale based
        /// on cpuUtilization to 0.6 or 60%.
        /// Structure is documented below.
        #[builder(into)]
        pub autoscaling_policy: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::RegionAutoscalerAutoscalingPolicy,
        >,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource. The name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL of the region where the instance group resides.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL of the managed instance group that this autoscaler will scale.
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegionAutoscalerResult {
        /// The configuration parameters for the autoscaling algorithm. You can
        /// define one or more of the policies for an autoscaler: cpuUtilization,
        /// customMetricUtilizations, and loadBalancingUtilization.
        /// If none of these are specified, the default will be to autoscale based
        /// on cpuUtilization to 0.6 or 60%.
        /// Structure is documented below.
        pub autoscaling_policy: pulumi_gestalt_rust::Output<
            super::super::types::compute::RegionAutoscalerAutoscalingPolicy,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource. The name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// URL of the region where the instance group resides.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// URL of the managed instance group that this autoscaler will scale.
        pub target: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RegionAutoscalerArgs,
    ) -> RegionAutoscalerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let autoscaling_policy_binding = args
            .autoscaling_policy
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let target_binding = args.target.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionAutoscaler:RegionAutoscaler".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoscalingPolicy".into(),
                    value: &autoscaling_policy_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegionAutoscalerResult {
            autoscaling_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoscalingPolicy"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            target: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("target"),
            ),
        }
    }
}
