/// Provides a resource to manage AWS EMR Security Configurations
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = security_configuration::create(
///         "foo",
///         SecurityConfigurationArgs::builder()
///             .configuration(
///                 "{\n  \"EncryptionConfiguration\": {\n    \"AtRestEncryptionConfiguration\": {\n      \"S3EncryptionConfiguration\": {\n        \"EncryptionMode\": \"SSE-S3\"\n      },\n      \"LocalDiskEncryptionConfiguration\": {\n        \"EncryptionKeyProviderType\": \"AwsKms\",\n        \"AwsKmsKey\": \"arn:aws:kms:us-west-2:187416307283:alias/my_emr_test_key\"\n      }\n    },\n    \"EnableInTransitEncryption\": false,\n    \"EnableAtRestEncryption\": true\n  }\n}",
///             )
///             .name("emrsc_other")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EMR Security Configurations using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:emr/securityConfiguration:SecurityConfiguration sc example-sc-name
/// ```
pub mod security_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityConfigurationArgs {
        /// A JSON formatted Security Configuration
        #[builder(into)]
        pub configuration: pulumi_wasm_rust::Output<String>,
        /// The name of the EMR Security Configuration. By default generated by this provider.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecurityConfigurationResult {
        /// A JSON formatted Security Configuration
        pub configuration: pulumi_wasm_rust::Output<String>,
        /// Date the Security Configuration was created
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// The name of the EMR Security Configuration. By default generated by this provider.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SecurityConfigurationArgs,
    ) -> SecurityConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_binding = args.configuration.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:emr/securityConfiguration:SecurityConfiguration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecurityConfigurationResult {
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
        }
    }
}