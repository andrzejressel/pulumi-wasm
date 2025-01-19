/// Provides a SageMaker Hub resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = hub::create(
///         "example",
///         HubArgs::builder().hub_description("example").hub_name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Hubs using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/hub:Hub test_hub my-code-repo
/// ```
pub mod hub {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HubArgs {
        /// A description of the hub.
        #[builder(into)]
        pub hub_description: pulumi_wasm_rust::Output<String>,
        /// The display name of the hub.
        #[builder(into, default)]
        pub hub_display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the hub.
        #[builder(into)]
        pub hub_name: pulumi_wasm_rust::Output<String>,
        /// The searchable keywords for the hub.
        #[builder(into, default)]
        pub hub_search_keywords: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Amazon S3 storage configuration for the hub. See S3 Storage Config details below.
        #[builder(into, default)]
        pub s3_storage_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::HubS3StorageConfig>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct HubResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Hub.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A description of the hub.
        pub hub_description: pulumi_wasm_rust::Output<String>,
        /// The display name of the hub.
        pub hub_display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the hub.
        pub hub_name: pulumi_wasm_rust::Output<String>,
        /// The searchable keywords for the hub.
        pub hub_search_keywords: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Amazon S3 storage configuration for the hub. See S3 Storage Config details below.
        pub s3_storage_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::HubS3StorageConfig>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HubArgs) -> HubResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hub_description_binding = args.hub_description.get_inner();
        let hub_display_name_binding = args.hub_display_name.get_inner();
        let hub_name_binding = args.hub_name.get_inner();
        let hub_search_keywords_binding = args.hub_search_keywords.get_inner();
        let s3_storage_config_binding = args.s3_storage_config.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/hub:Hub".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hubDescription".into(),
                    value: &hub_description_binding,
                },
                register_interface::ObjectField {
                    name: "hubDisplayName".into(),
                    value: &hub_display_name_binding,
                },
                register_interface::ObjectField {
                    name: "hubName".into(),
                    value: &hub_name_binding,
                },
                register_interface::ObjectField {
                    name: "hubSearchKeywords".into(),
                    value: &hub_search_keywords_binding,
                },
                register_interface::ObjectField {
                    name: "s3StorageConfig".into(),
                    value: &s3_storage_config_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "hubDescription".into(),
                },
                register_interface::ResultField {
                    name: "hubDisplayName".into(),
                },
                register_interface::ResultField {
                    name: "hubName".into(),
                },
                register_interface::ResultField {
                    name: "hubSearchKeywords".into(),
                },
                register_interface::ResultField {
                    name: "s3StorageConfig".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HubResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            hub_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hubDescription").unwrap(),
            ),
            hub_display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hubDisplayName").unwrap(),
            ),
            hub_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hubName").unwrap(),
            ),
            hub_search_keywords: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hubSearchKeywords").unwrap(),
            ),
            s3_storage_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3StorageConfig").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
