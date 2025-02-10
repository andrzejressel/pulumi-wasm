/// A collection of DataItems and Annotations on them.
///
///
/// To get more information about Dataset, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.datasets)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/vertex-ai/docs)
///
/// ## Example Usage
///
/// ### Vertex Ai Dataset
///
///
/// ```yaml
/// resources:
///   dataset:
///     type: gcp:vertex:AiDataset
///     properties:
///       displayName: terraform
///       metadataSchemaUri: gs://google-cloud-aiplatform/schema/dataset/metadata/image_1.0.0.yaml
///       region: us-central1
///       labels:
///         env: test
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ai_dataset {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiDatasetArgs {
        /// The user-defined name of the Dataset. The name can be up to 128 characters long and can be consist of any UTF-8 characters.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Customer-managed encryption key spec for a Dataset. If set, this Dataset and all sub-resources of this Dataset will be secured by this key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vertex::AiDatasetEncryptionSpec>,
        >,
        /// A set of key/value label pairs to assign to this Workflow.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Points to a YAML file stored on Google Cloud Storage describing additional information about the Dataset. The schema is defined as an OpenAPI 3.0.2 Schema Object. The schema files that can be used here are found in gs://google-cloud-aiplatform/schema/dataset/metadata/.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub metadata_schema_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the dataset. eg us-central1
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiDatasetResult {
        /// The timestamp of when the dataset was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The user-defined name of the Dataset. The name can be up to 128 characters long and can be consist of any UTF-8 characters.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Customer-managed encryption key spec for a Dataset. If set, this Dataset and all sub-resources of this Dataset will be secured by this key.
        /// Structure is documented below.
        pub encryption_spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::vertex::AiDatasetEncryptionSpec>,
        >,
        /// A set of key/value label pairs to assign to this Workflow.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Points to a YAML file stored on Google Cloud Storage describing additional information about the Dataset. The schema is defined as an OpenAPI 3.0.2 Schema Object. The schema files that can be used here are found in gs://google-cloud-aiplatform/schema/dataset/metadata/.
        ///
        ///
        /// - - -
        pub metadata_schema_uri: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the Dataset. This value is set by Google.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the dataset. eg us-central1
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The timestamp of when the dataset was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AiDatasetArgs,
    ) -> AiDatasetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let encryption_spec_binding = args.encryption_spec.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let metadata_schema_uri_binding = args.metadata_schema_uri.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:vertex/aiDataset:AiDataset".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionSpec".into(),
                    value: encryption_spec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadataSchemaUri".into(),
                    value: metadata_schema_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AiDatasetResult {
            create_time: o.get_field("createTime"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            encryption_spec: o.get_field("encryptionSpec"),
            labels: o.get_field("labels"),
            metadata_schema_uri: o.get_field("metadataSchemaUri"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            update_time: o.get_field("updateTime"),
        }
    }
}
