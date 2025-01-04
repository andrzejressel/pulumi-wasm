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
pub mod autoscaler {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
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
        pub autoscaling_policy: pulumi_wasm_rust::Output<
            super::super::types::compute::AutoscalerAutoscalingPolicy,
        >,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource. The name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// URL of the managed instance group that this autoscaler will scale.
        #[builder(into)]
        pub target: pulumi_wasm_rust::Output<String>,
        /// URL of the zone where the instance group resides.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AutoscalerResult {
        /// The configuration parameters for the autoscaling algorithm. You can
        /// define one or more of the policies for an autoscaler: cpuUtilization,
        /// customMetricUtilizations, and loadBalancingUtilization.
        /// If none of these are specified, the default will be to autoscale based
        /// on cpuUtilization to 0.6 or 60%.
        /// Structure is documented below.
        pub autoscaling_policy: pulumi_wasm_rust::Output<
            super::super::types::compute::AutoscalerAutoscalingPolicy,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource. The name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// URL of the managed instance group that this autoscaler will scale.
        pub target: pulumi_wasm_rust::Output<String>,
        /// URL of the zone where the instance group resides.
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AutoscalerArgs) -> AutoscalerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let autoscaling_policy_binding = args.autoscaling_policy.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let target_binding = args.target.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/autoscaler:Autoscaler".into(),
            name: name.to_string(),
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
                    name: "target".into(),
                    value: &target_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoscalingPolicy".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "target".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AutoscalerResult {
            autoscaling_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscalingPolicy").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("target").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
