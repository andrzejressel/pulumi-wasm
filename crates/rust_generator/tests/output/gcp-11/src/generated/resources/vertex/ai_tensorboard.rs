/// Tensorboard is a physical database that stores users' training metrics. A default Tensorboard is provided in each region of a GCP project. If needed users can also create extra Tensorboards in their projects.
///
///
/// To get more information about Tensorboard, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.tensorboards)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/vertex-ai/docs)
///
/// ## Example Usage
///
/// ### Vertex Ai Tensorboard
///
///
/// ```yaml
/// resources:
///   tensorboard:
///     type: gcp:vertex:AiTensorboard
///     properties:
///       displayName: terraform
///       description: sample description
///       labels:
///         key1: value1
///         key2: value2
///       region: us-central1
/// ```
/// ### Vertex Ai Tensorboard Full
///
///
/// ```yaml
/// resources:
///   tensorboard:
///     type: gcp:vertex:AiTensorboard
///     properties:
///       displayName: terraform
///       description: sample description
///       labels:
///         key1: value1
///         key2: value2
///       region: us-central1
///       encryptionSpec:
///         kmsKeyName: kms-name
///     options:
///       dependsOn:
///         - ${cryptoKey}
///   cryptoKey:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: crypto_key
///     properties:
///       cryptoKeyId: kms-name
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-aiplatform.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Tensorboard can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/tensorboards/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Tensorboard can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiTensorboard:AiTensorboard default projects/{{project}}/locations/{{region}}/tensorboards/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiTensorboard:AiTensorboard default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiTensorboard:AiTensorboard default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiTensorboard:AiTensorboard default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ai_tensorboard {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiTensorboardArgs {
        /// Description of this Tensorboard.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User provided name of this Tensorboard.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Customer-managed encryption key spec for a Tensorboard. If set, this Tensorboard and all sub-resources of this Tensorboard will be secured by this key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vertex::AiTensorboardEncryptionSpec>,
        >,
        /// The labels with user-defined metadata to organize your Tensorboards.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the tensorboard. eg us-central1
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiTensorboardResult {
        /// Consumer project Cloud Storage path prefix used to store blob data, which can either be a bucket or directory. Does not end with a '/'.
        pub blob_storage_path_prefix: pulumi_gestalt_rust::Output<String>,
        /// The timestamp of when the Tensorboard was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Description of this Tensorboard.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// User provided name of this Tensorboard.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Customer-managed encryption key spec for a Tensorboard. If set, this Tensorboard and all sub-resources of this Tensorboard will be secured by this key.
        /// Structure is documented below.
        pub encryption_spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::vertex::AiTensorboardEncryptionSpec>,
        >,
        /// The labels with user-defined metadata to organize your Tensorboards.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the Tensorboard.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the tensorboard. eg us-central1
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The number of Runs stored in this Tensorboard.
        pub run_count: pulumi_gestalt_rust::Output<String>,
        /// The timestamp of when the Tensorboard was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AiTensorboardArgs,
    ) -> AiTensorboardResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let encryption_spec_binding_1 = args.encryption_spec.get_output(context);
        let encryption_spec_binding = encryption_spec_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiTensorboard:AiTensorboard".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionSpec".into(),
                    value: &encryption_spec_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AiTensorboardResult {
            blob_storage_path_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("blobStoragePathPrefix"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            encryption_spec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionSpec"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            run_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runCount"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
