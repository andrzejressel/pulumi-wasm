/// ## Example Usage
///
/// ### Gemini Code Repository Index Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = code_repository_index::create(
///         "example",
///         CodeRepositoryIndexArgs::builder()
///             .code_repository_index_id("")
///             .kms_key(
///                 "projects/projectExample/locations/locationExample/keyRings/keyRingExample/cryptoKeys/cryptoKeyExample",
///             )
///             .location("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// CodeRepositoryIndex can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/codeRepositoryIndexes/{{code_repository_index_id}}`
///
/// * `{{project}}/{{location}}/{{code_repository_index_id}}`
///
/// * `{{location}}/{{code_repository_index_id}}`
///
/// When using the `pulumi import` command, CodeRepositoryIndex can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gemini/codeRepositoryIndex:CodeRepositoryIndex default projects/{{project}}/locations/{{location}}/codeRepositoryIndexes/{{code_repository_index_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gemini/codeRepositoryIndex:CodeRepositoryIndex default {{project}}/{{location}}/{{code_repository_index_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gemini/codeRepositoryIndex:CodeRepositoryIndex default {{location}}/{{code_repository_index_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod code_repository_index {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CodeRepositoryIndexArgs {
        /// Required. Id of the Code Repository Index.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub code_repository_index_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Immutable. Customer-managed encryption key name, in the format
        /// projects/*/locations/*/keyRings/*/cryptoKeys/*.
        #[builder(into, default)]
        pub kms_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Labels as key value pairs.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the Code Repository Index, for example `us-central1`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CodeRepositoryIndexResult {
        /// Required. Id of the Code Repository Index.
        ///
        ///
        /// - - -
        pub code_repository_index_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. Create time stamp.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Immutable. Customer-managed encryption key name, in the format
        /// projects/*/locations/*/keyRings/*/cryptoKeys/*.
        pub kms_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. Labels as key value pairs.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the Code Repository Index, for example `us-central1`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Immutable. Identifier. Name of Code Repository Index.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Code Repository Index instance State.
        /// Possible values:
        /// STATE_UNSPECIFIED
        /// CREATING
        /// ACTIVE
        /// DELETING
        /// SUSPENDED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. Update time stamp.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CodeRepositoryIndexArgs,
    ) -> CodeRepositoryIndexResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let code_repository_index_id_binding_1 = args
            .code_repository_index_id
            .get_output(context);
        let code_repository_index_id_binding = code_repository_index_id_binding_1
            .get_inner();
        let kms_key_binding_1 = args.kms_key.get_output(context);
        let kms_key_binding = kms_key_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gemini/codeRepositoryIndex:CodeRepositoryIndex".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "codeRepositoryIndexId".into(),
                    value: &code_repository_index_id_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKey".into(),
                    value: &kms_key_binding,
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
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CodeRepositoryIndexResult {
            code_repository_index_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("codeRepositoryIndexId"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            kms_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKey"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
