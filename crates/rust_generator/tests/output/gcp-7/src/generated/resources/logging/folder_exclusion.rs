/// Manages a folder-level logging exclusion. For more information see:
///
/// * [API documentation](https://cloud.google.com/logging/docs/reference/v2/rest/v2/folders.exclusions)
/// * How-to Guides
///     * [Excluding Logs](https://cloud.google.com/logging/docs/exclusions)
///
/// > You can specify exclusions for log sinks created by the provider by using the exclusions field of `gcp.logging.FolderSink`
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   my-exclusion:
///     type: gcp:logging:FolderExclusion
///     properties:
///       name: my-instance-debug-exclusion
///       folder: ${["my-folder"].name}
///       description: Exclude GCE instance debug logs
///       filter: resource.type = gce_instance AND severity <= DEBUG
///   my-folder:
///     type: gcp:organizations:Folder
///     properties:
///       displayName: My folder
///       parent: organizations/123456
/// ```
///
/// ## Import
///
/// Folder-level logging exclusions can be imported using their URI, e.g.
///
/// * `folders/{{folder}}/exclusions/{{name}}`
///
/// When using the `pulumi import` command, folder-level logging exclusions can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/folderExclusion:FolderExclusion default folders/{{folder}}/exclusions/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod folder_exclusion {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FolderExclusionArgs {
        /// A human-readable description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether this exclusion rule should be disabled or not. This defaults to
        /// false.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The filter to apply when excluding logs. Only log entries that match the filter are excluded.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced-filters) for information on how to
        /// write a filter.
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The folder to be exported to the sink. Note that either [FOLDER_ID] or "folders/[FOLDER_ID]" is
        /// accepted.
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the logging exclusion.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FolderExclusionResult {
        /// A human-readable description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether this exclusion rule should be disabled or not. This defaults to
        /// false.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The filter to apply when excluding logs. Only log entries that match the filter are excluded.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced-filters) for information on how to
        /// write a filter.
        pub filter: pulumi_gestalt_rust::Output<String>,
        /// The folder to be exported to the sink. Note that either [FOLDER_ID] or "folders/[FOLDER_ID]" is
        /// accepted.
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// The name of the logging exclusion.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FolderExclusionArgs,
    ) -> FolderExclusionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let folder_binding = args.folder.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:logging/folderExclusion:FolderExclusion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: disabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: folder_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FolderExclusionResult {
            description: o.get_field("description"),
            disabled: o.get_field("disabled"),
            filter: o.get_field("filter"),
            folder: o.get_field("folder"),
            name: o.get_field("name"),
        }
    }
}
