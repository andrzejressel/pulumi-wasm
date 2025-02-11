/// Represents an Autoscaler resource.
///
/// Autoscalers allow you to automatically scale virtual machine instances in
/// managed instance groups according to an autoscaling policy that you
/// define.
///
///
/// To get more information about Autoscaler, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/autoscalers)
/// * How-to Guides
///     * [Autoscaling Groups of Instances](https://cloud.google.com/compute/docs/autoscaler/)
///
/// ## Example Usage
///
/// ### Autoscaler Single Instance
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Autoscaler
///     properties:
///       name: my-autoscaler
///       zone: us-central1-f
///       target: ${defaultInstanceGroupManager.id}
///       autoscalingPolicy:
///         maxReplicas: 5
///         minReplicas: 1
///         cooldownPeriod: 60
///         metrics:
///           - name: pubsub.googleapis.com/subscription/num_undelivered_messages
///             filter: resource.type = pubsub_subscription AND resource.label.subscription_id = our-subscription
///             singleInstanceAssignment: 65535
///   defaultInstanceTemplate:
///     type: gcp:compute:InstanceTemplate
///     name: default
///     properties:
///       name: my-instance-template
///       machineType: e2-medium
///       canIpForward: false
///       tags:
///         - foo
///         - bar
///       disks:
///         - sourceImage: ${debian9.id}
///       networkInterfaces:
///         - network: default
///       metadata:
///         foo: bar
///       serviceAccount:
///         scopes:
///           - userinfo-email
///           - compute-ro
///           - storage-ro
///   defaultTargetPool:
///     type: gcp:compute:TargetPool
///     name: default
///     properties:
///       name: my-target-pool
///   defaultInstanceGroupManager:
///     type: gcp:compute:InstanceGroupManager
///     name: default
///     properties:
///       name: my-igm
///       zone: us-central1-f
///       versions:
///         - instanceTemplate: ${defaultInstanceTemplate.id}
///           name: primary
///       targetPools:
///         - ${defaultTargetPool.id}
///       baseInstanceName: autoscaler-sample
/// variables:
///   debian9:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
/// ### Autoscaler Basic
///
///
/// ```yaml
/// resources:
///   foobar:
///     type: gcp:compute:Autoscaler
///     properties:
///       name: my-autoscaler
///       zone: us-central1-f
///       target: ${foobarInstanceGroupManager.id}
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
///       machineType: e2-medium
///       canIpForward: false
///       tags:
///         - foo
///         - bar
///       disks:
///         - sourceImage: ${debian9.id}
///       networkInterfaces:
///         - network: default
///       metadata:
///         foo: bar
///       serviceAccount:
///         scopes:
///           - userinfo-email
///           - compute-ro
///           - storage-ro
///   foobarTargetPool:
///     type: gcp:compute:TargetPool
///     name: foobar
///     properties:
///       name: my-target-pool
///   foobarInstanceGroupManager:
///     type: gcp:compute:InstanceGroupManager
///     name: foobar
///     properties:
///       name: my-igm
///       zone: us-central1-f
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
/// Autoscaler can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/autoscalers/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Autoscaler can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/autoscaler:Autoscaler default projects/{{project}}/zones/{{zone}}/autoscalers/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/autoscaler:Autoscaler default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/autoscaler:Autoscaler default {{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/autoscaler:Autoscaler default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod autoscaler {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutoscalerArgs {
        /// The configuration parameters for the autoscaling algorithm. You can
        /// define one or more of the policies for an autoscaler: cpuUtilization,
        /// customMetricUtilizations, and loadBalancingUtilization.
        /// If none of these are specified, the default will be to autoscale based
        /// on cpuUtilization to 0.6 or 60%.
        /// Structure is documented below.
        #[builder(into)]
        pub autoscaling_policy: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::AutoscalerAutoscalingPolicy,
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
        /// URL of the managed instance group that this autoscaler will scale.
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<String>,
        /// URL of the zone where the instance group resides.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AutoscalerResult {
        /// The configuration parameters for the autoscaling algorithm. You can
        /// define one or more of the policies for an autoscaler: cpuUtilization,
        /// customMetricUtilizations, and loadBalancingUtilization.
        /// If none of these are specified, the default will be to autoscale based
        /// on cpuUtilization to 0.6 or 60%.
        /// Structure is documented below.
        pub autoscaling_policy: pulumi_gestalt_rust::Output<
            super::super::types::compute::AutoscalerAutoscalingPolicy,
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
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// URL of the managed instance group that this autoscaler will scale.
        pub target: pulumi_gestalt_rust::Output<String>,
        /// URL of the zone where the instance group resides.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AutoscalerArgs,
    ) -> AutoscalerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autoscaling_policy_binding = args.autoscaling_policy.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let target_binding = args.target.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/autoscaler:Autoscaler".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscalingPolicy".into(),
                    value: &autoscaling_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: &target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AutoscalerResult {
            autoscaling_policy: o.get_field("autoscalingPolicy"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            target: o.get_field("target"),
            zone: o.get_field("zone"),
        }
    }
}
