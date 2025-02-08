/// Resource for managing an AWS CloudFront KeyValueStore Key.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key_value_store::create(
///         "example",
///         KeyValueStoreArgs::builder()
///             .comment("This is an example key value store")
///             .name("ExampleKeyValueStore")
///             .build_struct(),
///     );
///     let exampleKeyvaluestoreKey = keyvaluestore_key::create(
///         "exampleKeyvaluestoreKey",
///         KeyvaluestoreKeyArgs::builder()
///             .key("Test Key")
///             .key_value_store_arn("${example.arn}")
///             .value("Test Value")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront KeyValueStore Key using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/keyvaluestoreKey:KeyvaluestoreKey example arn:aws:cloudfront::111111111111:key-value-store/8562g61f-caba-2845-9d99-b97diwae5d3c,someKey
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod keyvaluestore_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyvaluestoreKeyArgs {
        /// Key to put.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Amazon Resource Name (ARN) of the Key Value Store.
        #[builder(into)]
        pub key_value_store_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Value to put.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct KeyvaluestoreKeyResult {
        /// Key to put.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Key Value Store.
        pub key_value_store_arn: pulumi_gestalt_rust::Output<String>,
        /// Total size of the Key Value Store in bytes.
        pub total_size_in_bytes: pulumi_gestalt_rust::Output<i32>,
        /// Value to put.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: KeyvaluestoreKeyArgs,
    ) -> KeyvaluestoreKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_binding = args.key.get_output(context).get_inner();
        let key_value_store_arn_binding = args
            .key_value_store_arn
            .get_output(context)
            .get_inner();
        let value_binding = args.value.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/keyvaluestoreKey:KeyvaluestoreKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "keyValueStoreArn".into(),
                    value: &key_value_store_arn_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        KeyvaluestoreKeyResult {
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            key_value_store_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyValueStoreArn"),
            ),
            total_size_in_bytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("totalSizeInBytes"),
            ),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
