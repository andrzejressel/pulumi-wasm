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
pub mod repository_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryGroupArgs {
        /// Required. Id of the Code Repository Index.
        #[builder(into)]
        pub code_repository_index: pulumi_wasm_rust::InputOrOutput<String>,
        /// Optional. Labels as key value pairs **Note**: This field is non-authoritative, and will only manage the labels present
        /// in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the Code Repository Index, for example `us-central1`.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Required. List of repositories to group
        /// Structure is documented below.
        #[builder(into)]
        pub repositories: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::gemini::RepositoryGroupRepository>,
        >,
        /// Required. Id of the Repository Group.
        #[builder(into)]
        pub repository_group_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryGroupResult {
        /// Required. Id of the Code Repository Index.
        pub code_repository_index: pulumi_wasm_rust::Output<String>,
        /// Output only. Create time stamp
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Labels as key value pairs **Note**: This field is non-authoritative, and will only manage the labels present
        /// in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the Code Repository Index, for example `us-central1`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Immutable. Identifier. name of resource
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. List of repositories to group
        /// Structure is documented below.
        pub repositories: pulumi_wasm_rust::Output<
            Vec<super::super::types::gemini::RepositoryGroupRepository>,
        >,
        /// Required. Id of the Repository Group.
        pub repository_group_id: pulumi_wasm_rust::Output<String>,
        /// Output only. Update time stamp
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RepositoryGroupArgs,
    ) -> RepositoryGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let code_repository_index_binding = args
            .code_repository_index
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let repositories_binding = args.repositories.get_output(context).get_inner();
        let repository_group_id_binding = args
            .repository_group_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gemini/repositoryGroup:RepositoryGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "codeRepositoryIndex".into(),
                    value: &code_repository_index_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "repositories".into(),
                    value: &repositories_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryGroupId".into(),
                    value: &repository_group_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RepositoryGroupResult {
            code_repository_index: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("codeRepositoryIndex"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            repositories: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("repositories"),
            ),
            repository_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("repositoryGroupId"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
