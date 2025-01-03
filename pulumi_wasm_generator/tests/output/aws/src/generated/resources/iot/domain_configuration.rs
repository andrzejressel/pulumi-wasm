/// Creates and manages an AWS IoT domain configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let iot = domain_configuration::create(
///         "iot",
///         DomainConfigurationArgs::builder()
///             .domain_name("iot.example.com")
///             .name("iot-")
///             .server_certificate_arns(vec!["${cert.arn}",])
///             .service_type("DATA")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import domain configurations using the name. For example:
///
/// ```sh
/// $ pulumi import aws:iot/domainConfiguration:DomainConfiguration example example
/// ```
pub mod domain_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainConfigurationArgs {
        /// An object that specifies the authorization service for a domain. See the `authorizer_config` Block below for details.
        #[builder(into, default)]
        pub authorizer_config: pulumi_wasm_rust::Output<
            Option<super::super::types::iot::DomainConfigurationAuthorizerConfig>,
        >,
        /// Fully-qualified domain name.
        #[builder(into, default)]
        pub domain_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the domain configuration. This value must be unique to a region.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARNs of the certificates that IoT passes to the device during the TLS handshake. Currently you can specify only one certificate ARN. This value is not required for Amazon Web Services-managed domains. When using a custom `domain_name`, the cert must include it.
        #[builder(into, default)]
        pub server_certificate_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The type of service delivered by the endpoint. Note: Amazon Web Services IoT Core currently supports only the `DATA` service type.
        #[builder(into, default)]
        pub service_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The status to which the domain configuration should be set. Valid values are `ENABLED` and `DISABLED`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An object that specifies the TLS configuration for a domain. See the `tls_config` Block below for details.
        #[builder(into, default)]
        pub tls_config: pulumi_wasm_rust::Output<
            Option<super::super::types::iot::DomainConfigurationTlsConfig>,
        >,
        /// The certificate used to validate the server certificate and prove domain name ownership. This certificate must be signed by a public certificate authority. This value is not required for Amazon Web Services-managed domains.
        #[builder(into, default)]
        pub validation_certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DomainConfigurationResult {
        /// The ARN of the domain configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// An object that specifies the authorization service for a domain. See the `authorizer_config` Block below for details.
        pub authorizer_config: pulumi_wasm_rust::Output<
            Option<super::super::types::iot::DomainConfigurationAuthorizerConfig>,
        >,
        /// Fully-qualified domain name.
        pub domain_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of the domain.
        pub domain_type: pulumi_wasm_rust::Output<String>,
        /// The name of the domain configuration. This value must be unique to a region.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ARNs of the certificates that IoT passes to the device during the TLS handshake. Currently you can specify only one certificate ARN. This value is not required for Amazon Web Services-managed domains. When using a custom `domain_name`, the cert must include it.
        pub server_certificate_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The type of service delivered by the endpoint. Note: Amazon Web Services IoT Core currently supports only the `DATA` service type.
        pub service_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The status to which the domain configuration should be set. Valid values are `ENABLED` and `DISABLED`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An object that specifies the TLS configuration for a domain. See the `tls_config` Block below for details.
        pub tls_config: pulumi_wasm_rust::Output<
            super::super::types::iot::DomainConfigurationTlsConfig,
        >,
        /// The certificate used to validate the server certificate and prove domain name ownership. This certificate must be signed by a public certificate authority. This value is not required for Amazon Web Services-managed domains.
        pub validation_certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DomainConfigurationArgs,
    ) -> DomainConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorizer_config_binding = args.authorizer_config.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let name_binding = args.name.get_inner();
        let server_certificate_arns_binding = args.server_certificate_arns.get_inner();
        let service_type_binding = args.service_type.get_inner();
        let status_binding = args.status.get_inner();
        let tags_binding = args.tags.get_inner();
        let tls_config_binding = args.tls_config.get_inner();
        let validation_certificate_arn_binding = args
            .validation_certificate_arn
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/domainConfiguration:DomainConfiguration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizerConfig".into(),
                    value: &authorizer_config_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serverCertificateArns".into(),
                    value: &server_certificate_arns_binding,
                },
                register_interface::ObjectField {
                    name: "serviceType".into(),
                    value: &service_type_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tlsConfig".into(),
                    value: &tls_config_binding,
                },
                register_interface::ObjectField {
                    name: "validationCertificateArn".into(),
                    value: &validation_certificate_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authorizerConfig".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "domainType".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "serverCertificateArns".into(),
                },
                register_interface::ResultField {
                    name: "serviceType".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "tlsConfig".into(),
                },
                register_interface::ResultField {
                    name: "validationCertificateArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authorizer_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerConfig").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            domain_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainType").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            server_certificate_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverCertificateArns").unwrap(),
            ),
            service_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceType").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            tls_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsConfig").unwrap(),
            ),
            validation_certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationCertificateArn").unwrap(),
            ),
        }
    }
}
