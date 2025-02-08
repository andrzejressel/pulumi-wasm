/// Fleet contains information about a group of clusters.
///
///
/// To get more information about Fleet, see:
///
/// * [API documentation](https://cloud.google.com/anthos/multicluster-management/reference/rest/v1/projects.locations.fleets)
/// * How-to Guides
///     * [Registering a Cluster to a Fleet](https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster#register_cluster)
///
/// ## Example Usage
///
/// ### Gkehub Fleet Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = fleet::create(
///         "default",
///         FleetArgs::builder()
///             .default_cluster_config(
///                 FleetDefaultClusterConfig::builder()
///                     .securityPostureConfig(
///                         FleetDefaultClusterConfigSecurityPostureConfig::builder()
///                             .mode("DISABLED")
///                             .vulnerabilityMode("VULNERABILITY_DISABLED")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .display_name("my production fleet")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Fleet can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/fleets/default`
///
/// * `{{project}}`
///
/// When using the `pulumi import` command, Fleet can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkehub/fleet:Fleet default projects/{{project}}/locations/global/fleets/default
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/fleet:Fleet default {{project}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fleet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetArgs {
        /// The default cluster configurations to apply across the fleet.
        /// Structure is documented below.
        #[builder(into, default)]
        pub default_cluster_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkehub::FleetDefaultClusterConfig>,
        >,
        /// A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters.
        /// Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FleetResult {
        /// The time the fleet was created, in RFC3339 text format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The default cluster configurations to apply across the fleet.
        /// Structure is documented below.
        pub default_cluster_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkehub::FleetDefaultClusterConfig>,
        >,
        /// The time the fleet was deleted, in RFC3339 text format.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters.
        /// Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The state of the fleet resource.
        /// Structure is documented below.
        pub states: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkehub::FleetState>,
        >,
        /// Google-generated UUID for this resource. This is unique across all
        /// Fleet resources. If a Fleet resource is deleted and another
        /// resource with the same name is created, it gets a different uid.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The time the fleet was last updated, in RFC3339 text format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FleetArgs,
    ) -> FleetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let default_cluster_config_binding = args
            .default_cluster_config
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkehub/fleet:Fleet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultClusterConfig".into(),
                    value: &default_cluster_config_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FleetResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            default_cluster_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultClusterConfig"),
            ),
            delete_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteTime"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            states: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("states"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
