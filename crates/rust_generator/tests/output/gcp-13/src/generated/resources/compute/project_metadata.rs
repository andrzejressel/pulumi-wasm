/// Authoritatively manages metadata common to all instances for a project in GCE. For more information see
/// [the official documentation](https://cloud.google.com/compute/docs/storing-retrieving-metadata)
/// and
/// [API](https://cloud.google.com/compute/docs/reference/latest/projects/setCommonInstanceMetadata).
///
/// > **Note:**  This resource manages all project-level metadata including project-level ssh keys.
/// Keys unset in config but set on the server will be removed. If you want to manage only single
/// key/value pairs within the project metadata rather than the entire set, then use
/// google_compute_project_metadata_item.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:ProjectMetadata
///     properties:
///       metadata:
///         foo: bar
///         fizz: buzz
///         '13': '42'
/// ```
///
/// ### Adding An SSH Key
///
/// ```yaml
/// resources:
///   # /*
///   # A key set in project metadata is propagated to every instance in the project.
///   # This resource configuration is prone to causing frequent diffs as Google adds SSH Keys when the SSH Button is pressed in the console.
///   # It is better to use OS Login instead.
///   # */
///   mySshKey:
///     type: gcp:compute:ProjectMetadata
///     name: my_ssh_key
///     properties:
///       metadata:
///         ssh-keys: |2
///                 dev:ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILg6UtHDNyMNAh0GjaytsJdrUxjtLy3APXqZfNZhvCeT dev
///                 foo:ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILg6UtHDNyMNAh0GjaytsJdrUxjtLy3APXqZfNZhvCeT bar
/// ```
///
/// ## Import
///
/// Project metadata can be imported using the project ID:
///
/// * `{{project_id}}`
///
/// When using the `pulumi import` command, project metadata can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/projectMetadata:ProjectMetadata default {{project_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project_metadata {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectMetadataArgs {
        /// A series of key value pairs.
        ///
        /// - - -
        #[builder(into)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProjectMetadataResult {
        /// A series of key value pairs.
        ///
        /// - - -
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectMetadataArgs,
    ) -> ProjectMetadataResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let metadata_binding = args.metadata.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/projectMetadata:ProjectMetadata".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProjectMetadataResult {
            metadata: o.get_field("metadata"),
            project: o.get_field("project"),
        }
    }
}
