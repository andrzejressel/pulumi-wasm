/// Provides an SSM Document resource
///
/// > **NOTE on updating SSM documents:** Only documents with a schema version of 2.0
/// or greater can update their content once created, see [SSM Schema Features](http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-ssm-docs.html#document-schemas-features). To update a document with an older schema version you must recreate the resource. Not all document types support a schema version of 2.0 or greater. Refer to [SSM document schema features and examples](https://docs.aws.amazon.com/systems-manager/latest/userguide/document-schemas-features.html) for information about which schema versions are supported for the respective `document_type`.
///
/// ## Example Usage
///
/// ### Create an ssm document in JSON format
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = document::create(
///         "foo",
///         DocumentArgs::builder()
///             .content(
///                 "  {\n    \"schemaVersion\": \"1.2\",\n    \"description\": \"Check ip configuration of a Linux instance.\",\n    \"parameters\": {\n\n    },\n    \"runtimeConfig\": {\n      \"aws:runShellScript\": {\n        \"properties\": [\n          {\n            \"id\": \"0.aws:runShellScript\",\n            \"runCommand\": [\"ifconfig\"]\n          }\n        ]\n      }\n    }\n  }",
///             )
///             .document_type("Command")
///             .name("test_document")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create an ssm document in YAML format
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = document::create(
///         "foo",
///         DocumentArgs::builder()
///             .content(
///                 "schemaVersion: '1.2'\ndescription: Check ip configuration of a Linux instance.\nparameters: {}\nruntimeConfig:\n  'aws:runShellScript':\n    properties:\n      - id: '0.aws:runShellScript'\n        runCommand:\n          - ifconfig",
///             )
///             .document_format("YAML")
///             .document_type("Command")
///             .name("test_document")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSM Documents using the name. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/document:Document example example
/// ```
/// The `attachments_source` argument does not have an SSM API method for reading the attachment information detail after creation. If the argument is set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod document {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DocumentArgs {
        /// One or more configuration blocks describing attachments sources to a version of a document. See `attachments_source` block below for details.
        #[builder(into, default)]
        pub attachments_sources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ssm::DocumentAttachmentsSource>>,
        >,
        /// The content for the SSM document in JSON or YAML format. The content of the document must not exceed 64KB. This quota also includes the content specified for input parameters at runtime. We recommend storing the contents for your new document in an external JSON or YAML file and referencing the file in a command.
        #[builder(into)]
        pub content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The format of the document. Valid values: `JSON`, `TEXT`, `YAML`.
        #[builder(into, default)]
        pub document_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the document. For a list of valid values, see the [API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_CreateDocument.html#systemsmanager-CreateDocument-request-DocumentType).
        #[builder(into)]
        pub document_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the document.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Additional permissions to attach to the document. See Permissions below for details.
        #[builder(into, default)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags to assign to the object. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The target type which defines the kinds of resources the document can run on. For example, `/AWS::EC2::Instance`. For a list of valid resource types, see [AWS resource and property types reference](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html).
        #[builder(into, default)]
        pub target_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The version of the artifact associated with the document. For example, `12.6`. This value is unique across all versions of a document, and can't be changed.
        #[builder(into, default)]
        pub version_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DocumentResult {
        /// The Amazon Resource Name (ARN) of the document.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// One or more configuration blocks describing attachments sources to a version of a document. See `attachments_source` block below for details.
        pub attachments_sources: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ssm::DocumentAttachmentsSource>>,
        >,
        /// The content for the SSM document in JSON or YAML format. The content of the document must not exceed 64KB. This quota also includes the content specified for input parameters at runtime. We recommend storing the contents for your new document in an external JSON or YAML file and referencing the file in a command.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// The date the document was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// The default version of the document.
        pub default_version: pulumi_gestalt_rust::Output<String>,
        /// A description of what the parameter does, how to use it, the default value, and whether or not the parameter is optional.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The format of the document. Valid values: `JSON`, `TEXT`, `YAML`.
        pub document_format: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of the document. For a list of valid values, see the [API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_CreateDocument.html#systemsmanager-CreateDocument-request-DocumentType).
        pub document_type: pulumi_gestalt_rust::Output<String>,
        /// The document version.
        pub document_version: pulumi_gestalt_rust::Output<String>,
        /// The Sha256 or Sha1 hash created by the system when the document was created.
        pub hash: pulumi_gestalt_rust::Output<String>,
        /// The hash type of the document. Valid values: `Sha256`, `Sha1`.
        pub hash_type: pulumi_gestalt_rust::Output<String>,
        /// The latest version of the document.
        pub latest_version: pulumi_gestalt_rust::Output<String>,
        /// The name of the document.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Web Services user that created the document.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// One or more configuration blocks describing the parameters for the document. See `parameter` block below for details.
        pub parameters: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ssm::DocumentParameter>,
        >,
        /// Additional permissions to attach to the document. See Permissions below for details.
        pub permissions: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The list of operating system (OS) platforms compatible with this SSM document. Valid values: `Windows`, `Linux`, `MacOS`.
        pub platform_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The schema version of the document.
        pub schema_version: pulumi_gestalt_rust::Output<String>,
        /// The status of the SSM document. Valid values: `Creating`, `Active`, `Updating`, `Deleting`, `Failed`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the object. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The target type which defines the kinds of resources the document can run on. For example, `/AWS::EC2::Instance`. For a list of valid resource types, see [AWS resource and property types reference](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html).
        pub target_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The version of the artifact associated with the document. For example, `12.6`. This value is unique across all versions of a document, and can't be changed.
        pub version_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DocumentArgs,
    ) -> DocumentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let attachments_sources_binding = args
            .attachments_sources
            .get_output(context)
            .get_inner();
        let content_binding = args.content.get_output(context).get_inner();
        let document_format_binding = args
            .document_format
            .get_output(context)
            .get_inner();
        let document_type_binding = args.document_type.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let permissions_binding = args.permissions.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_type_binding = args.target_type.get_output(context).get_inner();
        let version_name_binding = args.version_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/document:Document".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attachmentsSources".into(),
                    value: &attachments_sources_binding,
                },
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "documentFormat".into(),
                    value: &document_format_binding,
                },
                register_interface::ObjectField {
                    name: "documentType".into(),
                    value: &document_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetType".into(),
                    value: &target_type_binding,
                },
                register_interface::ObjectField {
                    name: "versionName".into(),
                    value: &version_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DocumentResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            attachments_sources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachmentsSources"),
            ),
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            created_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            default_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultVersion"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            document_format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("documentFormat"),
            ),
            document_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("documentType"),
            ),
            document_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("documentVersion"),
            ),
            hash: pulumi_gestalt_rust::__private::into_domain(o.extract_field("hash")),
            hash_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hashType"),
            ),
            latest_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("latestVersion"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            platform_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformTypes"),
            ),
            schema_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schemaVersion"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            target_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetType"),
            ),
            version_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionName"),
            ),
        }
    }
}
