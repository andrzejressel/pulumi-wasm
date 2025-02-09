/// ## Example Usage
///
/// ## Import
///
/// RepositoryGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/codeRepositoryIndexes/{{code_repository_index}}/repositoryGroups/{{repository_group_id}}`
///
/// * `{{project}}/{{location}}/{{code_repository_index}}/{{repository_group_id}}`
///
/// * `{{location}}/{{code_repository_index}}/{{repository_group_id}}`
///
/// When using the `pulumi import` command, RepositoryGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gemini/repositoryGroup:RepositoryGroup default projects/{{project}}/locations/{{location}}/codeRepositoryIndexes/{{code_repository_index}}/repositoryGroups/{{repository_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gemini/repositoryGroup:RepositoryGroup default {{project}}/{{location}}/{{code_repository_index}}/{{repository_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gemini/repositoryGroup:RepositoryGroup default {{location}}/{{code_repository_index}}/{{repository_group_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryGroupArgs {
        /// Required. Id of the Code Repository Index.
        #[builder(into)]
        pub code_repository_index: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Labels as key value pairs **Note**: This field is non-authoritative, and will only manage the labels present
        /// in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the Code Repository Index, for example `us-central1`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. List of repositories to group
        /// Structure is documented below.
        #[builder(into)]
        pub repositories: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::gemini::RepositoryGroupRepository>,
        >,
        /// Required. Id of the Repository Group.
        #[builder(into)]
        pub repository_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryGroupResult {
        /// Required. Id of the Code Repository Index.
        pub code_repository_index: pulumi_gestalt_rust::Output<String>,
        /// Output only. Create time stamp
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Labels as key value pairs **Note**: This field is non-authoritative, and will only manage the labels present
        /// in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the Code Repository Index, for example `us-central1`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Immutable. Identifier. name of resource
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. List of repositories to group
        /// Structure is documented below.
        pub repositories: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gemini::RepositoryGroupRepository>,
        >,
        /// Required. Id of the Repository Group.
        pub repository_group_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. Update time stamp
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryGroupArgs,
    ) -> RepositoryGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let code_repository_index_binding = args
            .code_repository_index
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let repositories_binding = args.repositories.get_output(context);
        let repository_group_id_binding = args.repository_group_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gemini/repositoryGroup:RepositoryGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "codeRepositoryIndex".into(),
                    value: code_repository_index_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositories".into(),
                    value: repositories_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryGroupId".into(),
                    value: repository_group_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RepositoryGroupResult {
            code_repository_index: o.get_field("codeRepositoryIndex"),
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            repositories: o.get_field("repositories"),
            repository_group_id: o.get_field("repositoryGroupId"),
            update_time: o.get_field("updateTime"),
        }
    }
}
