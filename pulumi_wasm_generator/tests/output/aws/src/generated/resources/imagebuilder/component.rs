/// Manages an Image Builder Component.
///
/// ## Example Usage
///
/// ### URI Document
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod component {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ComponentArgs {
        /// Change description of the component.
        #[builder(into, default)]
        pub change_description: pulumi_wasm_rust::Output<Option<String>>,
        /// Inline YAML string with data of the component. Exactly one of `data` and `uri` can be specified. the provider will only perform drift detection of its value when present in a configuration.
        #[builder(into, default)]
        pub data: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the component.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key used to encrypt the component.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the component.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Platform of the component.
        #[builder(into)]
        pub platform: pulumi_wasm_rust::Output<String>,
        /// Whether to retain the old version when the resource is destroyed or replacement is necessary. Defaults to `false`.
        #[builder(into, default)]
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Set of Operating Systems (OS) supported by the component.
        #[builder(into, default)]
        pub supported_os_versions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Key-value map of resource tags for the component. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// S3 URI with data of the component. Exactly one of `data` and `uri` can be specified.
        ///
        /// > **NOTE:** Updating `data` or `uri` requires specifying a new `version`. This causes replacement of the resource. The `skip_destroy` argument can be used to retain the old version.
        #[builder(into, default)]
        pub uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Version of the component.
        ///
        /// The following attributes are optional:
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ComponentResult {
        /// (Required) Amazon Resource Name (ARN) of the component.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Change description of the component.
        pub change_description: pulumi_wasm_rust::Output<Option<String>>,
        /// Inline YAML string with data of the component. Exactly one of `data` and `uri` can be specified. the provider will only perform drift detection of its value when present in a configuration.
        pub data: pulumi_wasm_rust::Output<String>,
        /// Date the component was created.
        pub date_created: pulumi_wasm_rust::Output<String>,
        /// Description of the component.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Encryption status of the component.
        pub encrypted: pulumi_wasm_rust::Output<bool>,
        /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key used to encrypt the component.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the component.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Owner of the component.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// Platform of the component.
        pub platform: pulumi_wasm_rust::Output<String>,
        /// Whether to retain the old version when the resource is destroyed or replacement is necessary. Defaults to `false`.
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Set of Operating Systems (OS) supported by the component.
        pub supported_os_versions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Key-value map of resource tags for the component. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of the component.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// S3 URI with data of the component. Exactly one of `data` and `uri` can be specified.
        ///
        /// > **NOTE:** Updating `data` or `uri` requires specifying a new `version`. This causes replacement of the resource. The `skip_destroy` argument can be used to retain the old version.
        pub uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Version of the component.
        ///
        /// The following attributes are optional:
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ComponentArgs) -> ComponentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let change_description_binding = args.change_description.get_inner();
        let data_binding = args.data.get_inner();
        let description_binding = args.description.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let name_binding = args.name.get_inner();
        let platform_binding = args.platform.get_inner();
        let skip_destroy_binding = args.skip_destroy.get_inner();
        let supported_os_versions_binding = args.supported_os_versions.get_inner();
        let tags_binding = args.tags.get_inner();
        let uri_binding = args.uri.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:imagebuilder/component:Component".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "changeDescription".into(),
                },
                register_interface::ResultField {
                    name: "data".into(),
                },
                register_interface::ResultField {
                    name: "dateCreated".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "encrypted".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "skipDestroy".into(),
                },
                register_interface::ResultField {
                    name: "supportedOsVersions".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "uri".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ComponentResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            change_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("changeDescription").unwrap(),
            ),
            data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("data").unwrap(),
            ),
            date_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateCreated").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encrypted").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            skip_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipDestroy").unwrap(),
            ),
            supported_os_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedOsVersions").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uri").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}