/// Manages a Target Pool within GCE. This is a collection of instances used as
/// target of a network load balancer (Forwarding Rule). For more information see
/// [the official
/// documentation](https://cloud.google.com/compute/docs/load-balancing/network/target-pools)
/// and [API](https://cloud.google.com/compute/docs/reference/latest/targetPools).
///
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = target_pool::create(
///         "default",
///         TargetPoolArgs::builder()
///             .health_checks("${defaultHttpHealthCheck.name}")
///             .instances(vec!["us-central1-a/myinstance1", "us-central1-b/myinstance2",])
///             .name("instance-pool")
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("default")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Target pools can be imported using any of the following formats:
///
/// * `projects/{{project}}/regions/{{region}}/targetPools/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, target pools can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/targetPool:TargetPool default projects/{{project}}/regions/{{region}}/targetPools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetPool:TargetPool default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetPool:TargetPool default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetPool:TargetPool default {{name}}
/// ```
///
pub mod target_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetPoolArgs {
        /// URL to the backup target pool. Must also set
        /// failover_ratio.
        #[builder(into, default)]
        pub backup_pool: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Textual description field.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Ratio (0 to 1) of failed nodes before using the
        /// backup pool (which must also be set).
        #[builder(into, default)]
        pub failover_ratio: pulumi_wasm_rust::InputOrOutput<Option<f64>>,
        /// List of zero or one health check name or self_link. Only
        /// legacy `gcp.compute.HttpHealthCheck` is supported.
        #[builder(into, default)]
        pub health_checks: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of instances in the pool. They can be given as
        /// URLs, or in the form of "zone/name". Note that the instances need not exist
        /// at the time of target pool creation, so there is no need to use the
        /// interpolation to create a dependency on the instances from the
        /// target pool.
        #[builder(into, default)]
        pub instances: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A unique name for the resource, required by GCE. Changing
        /// this forces a new resource to be created.
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Where the target pool resides. Defaults to project
        /// region.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource URL for the security policy associated with this target pool.
        #[builder(into, default)]
        pub security_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// How to distribute load. Options are "NONE" (no
        /// affinity). "CLIENT_IP" (hash of the source/dest addresses / ports), and
        /// "CLIENT_IP_PROTO" also includes the protocol (default "NONE").
        #[builder(into, default)]
        pub session_affinity: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TargetPoolResult {
        /// URL to the backup target pool. Must also set
        /// failover_ratio.
        pub backup_pool: pulumi_wasm_rust::Output<Option<String>>,
        /// Textual description field.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Ratio (0 to 1) of failed nodes before using the
        /// backup pool (which must also be set).
        pub failover_ratio: pulumi_wasm_rust::Output<Option<f64>>,
        /// List of zero or one health check name or self_link. Only
        /// legacy `gcp.compute.HttpHealthCheck` is supported.
        pub health_checks: pulumi_wasm_rust::Output<Option<String>>,
        /// List of instances in the pool. They can be given as
        /// URLs, or in the form of "zone/name". Note that the instances need not exist
        /// at the time of target pool creation, so there is no need to use the
        /// interpolation to create a dependency on the instances from the
        /// target pool.
        pub instances: pulumi_wasm_rust::Output<Vec<String>>,
        /// A unique name for the resource, required by GCE. Changing
        /// this forces a new resource to be created.
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Where the target pool resides. Defaults to project
        /// region.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The resource URL for the security policy associated with this target pool.
        pub security_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// How to distribute load. Options are "NONE" (no
        /// affinity). "CLIENT_IP" (hash of the source/dest addresses / ports), and
        /// "CLIENT_IP_PROTO" also includes the protocol (default "NONE").
        pub session_affinity: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TargetPoolArgs,
    ) -> TargetPoolResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_pool_binding = args.backup_pool.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let failover_ratio_binding = args.failover_ratio.get_output(context).get_inner();
        let health_checks_binding = args.health_checks.get_output(context).get_inner();
        let instances_binding = args.instances.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let security_policy_binding = args
            .security_policy
            .get_output(context)
            .get_inner();
        let session_affinity_binding = args
            .session_affinity
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/targetPool:TargetPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupPool".into(),
                    value: &backup_pool_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "failoverRatio".into(),
                    value: &failover_ratio_binding,
                },
                register_interface::ObjectField {
                    name: "healthChecks".into(),
                    value: &health_checks_binding,
                },
                register_interface::ObjectField {
                    name: "instances".into(),
                    value: &instances_binding,
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
                    name: "securityPolicy".into(),
                    value: &security_policy_binding,
                },
                register_interface::ObjectField {
                    name: "sessionAffinity".into(),
                    value: &session_affinity_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TargetPoolResult {
            backup_pool: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupPool"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            failover_ratio: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("failoverRatio"),
            ),
            health_checks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthChecks"),
            ),
            instances: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instances"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            security_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityPolicy"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            session_affinity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sessionAffinity"),
            ),
        }
    }
}
