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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetPoolArgs {
        /// URL to the backup target pool. Must also set
        /// failover_ratio.
        #[builder(into, default)]
        pub backup_pool: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Textual description field.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Ratio (0 to 1) of failed nodes before using the
        /// backup pool (which must also be set).
        #[builder(into, default)]
        pub failover_ratio: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// List of zero or one health check name or self_link. Only
        /// legacy `gcp.compute.HttpHealthCheck` is supported.
        #[builder(into, default)]
        pub health_checks: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of instances in the pool. They can be given as
        /// URLs, or in the form of "zone/name". Note that the instances need not exist
        /// at the time of target pool creation, so there is no need to use the
        /// interpolation to create a dependency on the instances from the
        /// target pool.
        #[builder(into, default)]
        pub instances: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A unique name for the resource, required by GCE. Changing
        /// this forces a new resource to be created.
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Where the target pool resides. Defaults to project
        /// region.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource URL for the security policy associated with this target pool.
        #[builder(into, default)]
        pub security_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// How to distribute load. Options are "NONE" (no
        /// affinity). "CLIENT_IP" (hash of the source/dest addresses / ports), and
        /// "CLIENT_IP_PROTO" also includes the protocol (default "NONE").
        #[builder(into, default)]
        pub session_affinity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TargetPoolResult {
        /// URL to the backup target pool. Must also set
        /// failover_ratio.
        pub backup_pool: pulumi_gestalt_rust::Output<Option<String>>,
        /// Textual description field.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Ratio (0 to 1) of failed nodes before using the
        /// backup pool (which must also be set).
        pub failover_ratio: pulumi_gestalt_rust::Output<Option<f64>>,
        /// List of zero or one health check name or self_link. Only
        /// legacy `gcp.compute.HttpHealthCheck` is supported.
        pub health_checks: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of instances in the pool. They can be given as
        /// URLs, or in the form of "zone/name". Note that the instances need not exist
        /// at the time of target pool creation, so there is no need to use the
        /// interpolation to create a dependency on the instances from the
        /// target pool.
        pub instances: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A unique name for the resource, required by GCE. Changing
        /// this forces a new resource to be created.
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Where the target pool resides. Defaults to project
        /// region.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The resource URL for the security policy associated with this target pool.
        pub security_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// How to distribute load. Options are "NONE" (no
        /// affinity). "CLIENT_IP" (hash of the source/dest addresses / ports), and
        /// "CLIENT_IP_PROTO" also includes the protocol (default "NONE").
        pub session_affinity: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetPoolArgs,
    ) -> TargetPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_pool_binding = args.backup_pool.get_output(context);
        let description_binding = args.description.get_output(context);
        let failover_ratio_binding = args.failover_ratio.get_output(context);
        let health_checks_binding = args.health_checks.get_output(context);
        let instances_binding = args.instances.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let security_policy_binding = args.security_policy.get_output(context);
        let session_affinity_binding = args.session_affinity.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/targetPool:TargetPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupPool".into(),
                    value: &backup_pool_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "failoverRatio".into(),
                    value: &failover_ratio_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthChecks".into(),
                    value: &health_checks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instances".into(),
                    value: &instances_binding.drop_type(),
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
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityPolicy".into(),
                    value: &security_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionAffinity".into(),
                    value: &session_affinity_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetPoolResult {
            backup_pool: o.get_field("backupPool"),
            description: o.get_field("description"),
            failover_ratio: o.get_field("failoverRatio"),
            health_checks: o.get_field("healthChecks"),
            instances: o.get_field("instances"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            security_policy: o.get_field("securityPolicy"),
            self_link: o.get_field("selfLink"),
            session_affinity: o.get_field("sessionAffinity"),
        }
    }
}
