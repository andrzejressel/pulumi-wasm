/// Manages an Image Builder Component.
///
/// ## Example Usage
///
/// ### URI Document
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = component::create(
///         "example",
///         ComponentArgs::builder()
///             .name("example")
///             .platform("Linux")
///             .uri("s3://${exampleAwsS3Object.bucket}/${exampleAwsS3Object.key}")
///             .version("1.0.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_imagebuilder_components` resources using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/component:Component example arn:aws:imagebuilder:us-east-1:123456789012:component/example/1.0.0/1
/// ```
/// Certain resource arguments, such as `uri`, cannot be read via the API and imported into the provider. The provider will display a difference for these arguments the first run after import if declared in the the provider configuration for an imported resource.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod component {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ComponentArgs {
        /// Change description of the component.
        #[builder(into, default)]
        pub change_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Inline YAML string with data of the component. Exactly one of `data` and `uri` can be specified. the provider will only perform drift detection of its value when present in a configuration.
        #[builder(into, default)]
        pub data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the component.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key used to encrypt the component.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the component.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Platform of the component.
        #[builder(into)]
        pub platform: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to retain the old version when the resource is destroyed or replacement is necessary. Defaults to `false`.
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Set of Operating Systems (OS) supported by the component.
        #[builder(into, default)]
        pub supported_os_versions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Key-value map of resource tags for the component. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// S3 URI with data of the component. Exactly one of `data` and `uri` can be specified.
        ///
        /// > **NOTE:** Updating `data` or `uri` requires specifying a new `version`. This causes replacement of the resource. The `skip_destroy` argument can be used to retain the old version.
        #[builder(into, default)]
        pub uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Version of the component.
        ///
        /// The following attributes are optional:
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ComponentResult {
        /// (Required) Amazon Resource Name (ARN) of the component.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Change description of the component.
        pub change_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Inline YAML string with data of the component. Exactly one of `data` and `uri` can be specified. the provider will only perform drift detection of its value when present in a configuration.
        pub data: pulumi_gestalt_rust::Output<String>,
        /// Date the component was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Description of the component.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Encryption status of the component.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key used to encrypt the component.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the component.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Owner of the component.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// Platform of the component.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Whether to retain the old version when the resource is destroyed or replacement is necessary. Defaults to `false`.
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Set of Operating Systems (OS) supported by the component.
        pub supported_os_versions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Key-value map of resource tags for the component. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of the component.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// S3 URI with data of the component. Exactly one of `data` and `uri` can be specified.
        ///
        /// > **NOTE:** Updating `data` or `uri` requires specifying a new `version`. This causes replacement of the resource. The `skip_destroy` argument can be used to retain the old version.
        pub uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// Version of the component.
        ///
        /// The following attributes are optional:
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ComponentArgs,
    ) -> ComponentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let change_description_binding_1 = args.change_description.get_output(context);
        let change_description_binding = change_description_binding_1.get_inner();
        let data_binding_1 = args.data.get_output(context);
        let data_binding = data_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let kms_key_id_binding_1 = args.kms_key_id.get_output(context);
        let kms_key_id_binding = kms_key_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let platform_binding_1 = args.platform.get_output(context);
        let platform_binding = platform_binding_1.get_inner();
        let skip_destroy_binding_1 = args.skip_destroy.get_output(context);
        let skip_destroy_binding = skip_destroy_binding_1.get_inner();
        let supported_os_versions_binding_1 = args
            .supported_os_versions
            .get_output(context);
        let supported_os_versions_binding = supported_os_versions_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let uri_binding_1 = args.uri.get_output(context);
        let uri_binding = uri_binding_1.get_inner();
        let version_binding_1 = args.version.get_output(context);
        let version_binding = version_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:imagebuilder/component:Component".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "changeDescription".into(),
                    value: &change_description_binding,
                },
                register_interface::ObjectField {
                    name: "data".into(),
                    value: &data_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "platform".into(),
                    value: &platform_binding,
                },
                register_interface::ObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "supportedOsVersions".into(),
                    value: &supported_os_versions_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "uri".into(),
                    value: &uri_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ComponentResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            change_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("changeDescription"),
            ),
            data: pulumi_gestalt_rust::__private::into_domain(o.extract_field("data")),
            date_created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateCreated"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encrypted"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            skip_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skipDestroy"),
            ),
            supported_os_versions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportedOsVersions"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            uri: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uri")),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
