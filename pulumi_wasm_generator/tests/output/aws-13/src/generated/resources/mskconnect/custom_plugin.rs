/// Provides an Amazon MSK Connect Custom Plugin Resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
///   exampleBucketObjectv2:
///     type: aws:s3:BucketObjectv2
///     name: example
///     properties:
///       bucket: ${example.id}
///       key: debezium.zip
///       source:
///         fn::FileAsset: debezium.zip
///   exampleCustomPlugin:
///     type: aws:mskconnect:CustomPlugin
///     name: example
///     properties:
///       name: debezium-example
///       contentType: ZIP
///       location:
///         s3:
///           bucketArn: ${example.arn}
///           fileKey: ${exampleBucketObjectv2.key}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MSK Connect Custom Plugin using the plugin's `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:mskconnect/customPlugin:CustomPlugin example 'arn:aws:kafkaconnect:eu-central-1:123456789012:custom-plugin/debezium-example/abcdefgh-1234-5678-9abc-defghijklmno-4'
/// ```
pub mod custom_plugin {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomPluginArgs {
        /// The type of the plugin file. Allowed values are `ZIP` and `JAR`.
        #[builder(into)]
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// A summary description of the custom plugin.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Information about the location of a custom plugin. See `location` Block for details.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<
            super::super::types::mskconnect::CustomPluginLocation,
        >,
        /// The name of the custom plugin..
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomPluginResult {
        /// the Amazon Resource Name (ARN) of the custom plugin.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The type of the plugin file. Allowed values are `ZIP` and `JAR`.
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// A summary description of the custom plugin.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// an ID of the latest successfully created revision of the custom plugin.
        pub latest_revision: pulumi_wasm_rust::Output<i32>,
        /// Information about the location of a custom plugin. See `location` Block for details.
        pub location: pulumi_wasm_rust::Output<
            super::super::types::mskconnect::CustomPluginLocation,
        >,
        /// The name of the custom plugin..
        pub name: pulumi_wasm_rust::Output<String>,
        /// the state of the custom plugin.
        pub state: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// The following arguments are optional:
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
    pub fn create(name: &str, args: CustomPluginArgs) -> CustomPluginResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_type_binding = args.content_type.get_inner();
        let description_binding = args.description.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:mskconnect/customPlugin:CustomPlugin".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "contentType".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "latestRevision".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
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
        CustomPluginResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            latest_revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestRevision").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
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
