/// Creates an HSM configuration that contains the information required by an Amazon Redshift cluster to store and use database encryption keys in a Hardware Security Module (HSM).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = hsm_configuration::create(
///         "example",
///         HsmConfigurationArgs::builder()
///             .description("example")
///             .hsm_configuration_identifier("example")
///             .hsm_ip_address("10.0.0.1")
///             .hsm_partition_name("aws")
///             .hsm_partition_password("example")
///             .hsm_server_public_certificate("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift HSM Client Certificates using `hsm_configuration_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/hsmConfiguration:HsmConfiguration example example
/// ```
pub mod hsm_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HsmConfigurationArgs {
        /// A text description of the HSM configuration to be created.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// The identifier to be assigned to the new Amazon Redshift HSM configuration.
        #[builder(into)]
        pub hsm_configuration_identifier: pulumi_wasm_rust::Output<String>,
        /// The IP address that the Amazon Redshift cluster must use to access the HSM.
        #[builder(into)]
        pub hsm_ip_address: pulumi_wasm_rust::Output<String>,
        /// The name of the partition in the HSM where the Amazon Redshift clusters will store their database encryption keys.
        #[builder(into)]
        pub hsm_partition_name: pulumi_wasm_rust::Output<String>,
        /// The password required to access the HSM partition.
        #[builder(into)]
        pub hsm_partition_password: pulumi_wasm_rust::Output<String>,
        /// The HSMs public certificate file. When using Cloud HSM, the file name is server.pem.
        #[builder(into)]
        pub hsm_server_public_certificate: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct HsmConfigurationResult {
        /// Amazon Resource Name (ARN) of the Hsm Client Certificate.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A text description of the HSM configuration to be created.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The identifier to be assigned to the new Amazon Redshift HSM configuration.
        pub hsm_configuration_identifier: pulumi_wasm_rust::Output<String>,
        /// The IP address that the Amazon Redshift cluster must use to access the HSM.
        pub hsm_ip_address: pulumi_wasm_rust::Output<String>,
        /// The name of the partition in the HSM where the Amazon Redshift clusters will store their database encryption keys.
        pub hsm_partition_name: pulumi_wasm_rust::Output<String>,
        /// The password required to access the HSM partition.
        pub hsm_partition_password: pulumi_wasm_rust::Output<String>,
        /// The HSMs public certificate file. When using Cloud HSM, the file name is server.pem.
        pub hsm_server_public_certificate: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: HsmConfigurationArgs) -> HsmConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let hsm_configuration_identifier_binding = args
            .hsm_configuration_identifier
            .get_inner();
        let hsm_ip_address_binding = args.hsm_ip_address.get_inner();
        let hsm_partition_name_binding = args.hsm_partition_name.get_inner();
        let hsm_partition_password_binding = args.hsm_partition_password.get_inner();
        let hsm_server_public_certificate_binding = args
            .hsm_server_public_certificate
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/hsmConfiguration:HsmConfiguration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "hsmConfigurationIdentifier".into(),
                    value: &hsm_configuration_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "hsmIpAddress".into(),
                    value: &hsm_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "hsmPartitionName".into(),
                    value: &hsm_partition_name_binding,
                },
                register_interface::ObjectField {
                    name: "hsmPartitionPassword".into(),
                    value: &hsm_partition_password_binding,
                },
                register_interface::ObjectField {
                    name: "hsmServerPublicCertificate".into(),
                    value: &hsm_server_public_certificate_binding,
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
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "hsmConfigurationIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "hsmIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "hsmPartitionName".into(),
                },
                register_interface::ResultField {
                    name: "hsmPartitionPassword".into(),
                },
                register_interface::ResultField {
                    name: "hsmServerPublicCertificate".into(),
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
        HsmConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            hsm_configuration_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hsmConfigurationIdentifier").unwrap(),
            ),
            hsm_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hsmIpAddress").unwrap(),
            ),
            hsm_partition_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hsmPartitionName").unwrap(),
            ),
            hsm_partition_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hsmPartitionPassword").unwrap(),
            ),
            hsm_server_public_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hsmServerPublicCertificate").unwrap(),
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
