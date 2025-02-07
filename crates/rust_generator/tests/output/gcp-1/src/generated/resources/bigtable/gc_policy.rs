/// Creates a Google Cloud Bigtable GC Policy inside a family. For more information see
/// [the official documentation](https://cloud.google.com/bigtable/) and
/// [API](https://cloud.google.com/bigtable/docs/go/reference).
///
/// > **Warning**: We don't recommend having multiple GC policies for the same column
/// family as it may result in unexpected behavior.
///
/// > **Note**: GC policies associated with a replicated table cannot be destroyed directly.
/// Destroying a GC policy is translated into never perform garbage collection, this is
/// considered relaxing from pure age-based or version-based GC policy, hence not allowed.
/// The workaround is unreplicating the instance first by updating the instance to have one
/// cluster.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .clusters(
///                 vec![
///                     InstanceCluster::builder().clusterId("tf-instance-cluster")
///                     .numNodes(3).storageType("HDD").build_struct(),
///                 ],
///             )
///             .name("tf-instance")
///             .build_struct(),
///     );
///     let policy = gc_policy::create(
///         "policy",
///         GcPolicyArgs::builder()
///             .column_family("name")
///             .deletion_policy("ABANDON")
///             .gc_rules(
///                 "  {\n    \"rules\": [\n      {\n        \"max_age\": \"168h\"\n      }\n    ]\n  }",
///             )
///             .instance_name("${instance.name}")
///             .table("${table.name}")
///             .build_struct(),
///     );
///     let table = table::create(
///         "table",
///         TableArgs::builder()
///             .column_families(
///                 vec![TableColumnFamily::builder().family("name").build_struct(),],
///             )
///             .instance_name("${instance.name}")
///             .name("tf-table")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Multiple conditions is also supported. `UNION` when any of its sub-policies apply (OR). `INTERSECTION` when all its sub-policies apply (AND)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let policy = gc_policy::create(
///         "policy",
///         GcPolicyArgs::builder()
///             .column_family("name")
///             .deletion_policy("ABANDON")
///             .gc_rules(
///                 "  {\n    \"mode\": \"union\",\n    \"rules\": [\n      {\n        \"max_age\": \"168h\"\n      },\n      {\n        \"max_version\": 10\n      }\n    ]\n  }",
///             )
///             .instance_name("${instance.name}")
///             .table("${table.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// An example of more complex GC policy:
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .clusters(
///                 vec![
///                     InstanceCluster::builder().clusterId("cid").zone("us-central1-b")
///                     .build_struct(),
///                 ],
///             )
///             .deletion_protection(false)
///             .instance_type("DEVELOPMENT")
///             .name("instance_name")
///             .build_struct(),
///     );
///     let policy = gc_policy::create(
///         "policy",
///         GcPolicyArgs::builder()
///             .column_family("cf1")
///             .deletion_policy("ABANDON")
///             .gc_rules(
///                 "  {\n    \"mode\": \"union\",\n    \"rules\": [\n      {\n        \"max_age\": \"10h\"\n      },\n      {\n        \"mode\": \"intersection\",\n        \"rules\": [\n          {\n            \"max_age\": \"2h\"\n          },\n          {\n            \"max_version\": 2\n          }\n        ]\n      }\n    ]\n  }",
///             )
///             .instance_name("${instance.id}")
///             .table("${table.name}")
///             .build_struct(),
///     );
///     let table = table::create(
///         "table",
///         TableArgs::builder()
///             .column_families(
///                 vec![TableColumnFamily::builder().family("cf1").build_struct(),],
///             )
///             .instance_name("${instance.id}")
///             .name("your-table")
///             .build_struct(),
///     );
/// }
/// ```
/// This is equivalent to running the following `cbt` command:
/// ```sh
/// cbt setgcpolicy your-table cf1 "(maxage=2d and maxversions=2) or maxage=10h"
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
pub mod gc_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GCPolicyArgs {
        /// The name of the column family.
        #[builder(into)]
        pub column_family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The deletion policy for the GC policy.
        /// Setting ABANDON allows the resource to be abandoned rather than deleted. This is useful for GC policy as it cannot be deleted in a replicated instance.
        ///
        /// Possible values are: `ABANDON`.
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Serialized JSON object to represent a more complex GC policy. Conflicts with `mode`, `max_age` and `max_version`. Conflicts with `mode`, `max_age` and `max_version`.
        #[builder(into, default)]
        pub gc_rules: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean for whether to allow ignoring warnings when updating the gc policy.
        /// Setting this to `true` allows relaxing the gc policy for replicated clusters by up to 90 days, but keep in mind this may increase how long clusters are inconsistent. Make sure
        /// you understand the risks listed at https://cloud.google.com/bigtable/docs/garbage-collection#increasing before setting this option.
        ///
        /// -----
        #[builder(into, default)]
        pub ignore_warnings: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Bigtable instance.
        #[builder(into)]
        pub instance_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// GC policy that applies to all cells older than the given age.
        #[builder(into, default)]
        pub max_age: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigtable::GcPolicyMaxAge>,
        >,
        /// GC policy that applies to all versions of a cell except for the most recent.
        #[builder(into, default)]
        pub max_versions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::bigtable::GcPolicyMaxVersion>>,
        >,
        /// If multiple policies are set, you should choose between `UNION` OR `INTERSECTION`.
        #[builder(into, default)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the table.
        #[builder(into)]
        pub table: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GCPolicyResult {
        /// The name of the column family.
        pub column_family: pulumi_gestalt_rust::Output<String>,
        /// The deletion policy for the GC policy.
        /// Setting ABANDON allows the resource to be abandoned rather than deleted. This is useful for GC policy as it cannot be deleted in a replicated instance.
        ///
        /// Possible values are: `ABANDON`.
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Serialized JSON object to represent a more complex GC policy. Conflicts with `mode`, `max_age` and `max_version`. Conflicts with `mode`, `max_age` and `max_version`.
        pub gc_rules: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean for whether to allow ignoring warnings when updating the gc policy.
        /// Setting this to `true` allows relaxing the gc policy for replicated clusters by up to 90 days, but keep in mind this may increase how long clusters are inconsistent. Make sure
        /// you understand the risks listed at https://cloud.google.com/bigtable/docs/garbage-collection#increasing before setting this option.
        ///
        /// -----
        pub ignore_warnings: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Bigtable instance.
        pub instance_name: pulumi_gestalt_rust::Output<String>,
        /// GC policy that applies to all cells older than the given age.
        pub max_age: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigtable::GcPolicyMaxAge>,
        >,
        /// GC policy that applies to all versions of a cell except for the most recent.
        pub max_versions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::bigtable::GcPolicyMaxVersion>>,
        >,
        /// If multiple policies are set, you should choose between `UNION` OR `INTERSECTION`.
        pub mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The name of the table.
        pub table: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GCPolicyArgs,
    ) -> GCPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let column_family_binding = args.column_family.get_output(context).get_inner();
        let deletion_policy_binding = args
            .deletion_policy
            .get_output(context)
            .get_inner();
        let gc_rules_binding = args.gc_rules.get_output(context).get_inner();
        let ignore_warnings_binding = args
            .ignore_warnings
            .get_output(context)
            .get_inner();
        let instance_name_binding = args.instance_name.get_output(context).get_inner();
        let max_age_binding = args.max_age.get_output(context).get_inner();
        let max_versions_binding = args.max_versions.get_output(context).get_inner();
        let mode_binding = args.mode.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let table_binding = args.table.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigtable/gCPolicy:GCPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "columnFamily".into(),
                    value: &column_family_binding,
                },
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "gcRules".into(),
                    value: &gc_rules_binding,
                },
                register_interface::ObjectField {
                    name: "ignoreWarnings".into(),
                    value: &ignore_warnings_binding,
                },
                register_interface::ObjectField {
                    name: "instanceName".into(),
                    value: &instance_name_binding,
                },
                register_interface::ObjectField {
                    name: "maxAge".into(),
                    value: &max_age_binding,
                },
                register_interface::ObjectField {
                    name: "maxVersions".into(),
                    value: &max_versions_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "table".into(),
                    value: &table_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GCPolicyResult {
            column_family: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("columnFamily"),
            ),
            deletion_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionPolicy"),
            ),
            gc_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gcRules"),
            ),
            ignore_warnings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ignoreWarnings"),
            ),
            instance_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceName"),
            ),
            max_age: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxAge"),
            ),
            max_versions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxVersions"),
            ),
            mode: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mode")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            table: pulumi_gestalt_rust::__private::into_domain(o.extract_field("table")),
        }
    }
}
