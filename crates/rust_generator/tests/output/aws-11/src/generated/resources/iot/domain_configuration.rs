/// Creates and manages an AWS IoT domain configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainConfigurationArgs {
        /// An object that specifies the authorization service for a domain. See the `authorizer_config` Block below for details.
        #[builder(into, default)]
        pub authorizer_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iot::DomainConfigurationAuthorizerConfig>,
        >,
        /// Fully-qualified domain name.
        #[builder(into, default)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the domain configuration. This value must be unique to a region.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARNs of the certificates that IoT passes to the device during the TLS handshake. Currently you can specify only one certificate ARN. This value is not required for Amazon Web Services-managed domains. When using a custom `domain_name`, the cert must include it.
        #[builder(into, default)]
        pub server_certificate_arns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The type of service delivered by the endpoint. Note: Amazon Web Services IoT Core currently supports only the `DATA` service type.
        #[builder(into, default)]
        pub service_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The status to which the domain configuration should be set. Valid values are `ENABLED` and `DISABLED`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An object that specifies the TLS configuration for a domain. See the `tls_config` Block below for details.
        #[builder(into, default)]
        pub tls_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iot::DomainConfigurationTlsConfig>,
        >,
        /// The certificate used to validate the server certificate and prove domain name ownership. This certificate must be signed by a public certificate authority. This value is not required for Amazon Web Services-managed domains.
        #[builder(into, default)]
        pub validation_certificate_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainConfigurationResult {
        /// The ARN of the domain configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// An object that specifies the authorization service for a domain. See the `authorizer_config` Block below for details.
        pub authorizer_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::iot::DomainConfigurationAuthorizerConfig>,
        >,
        /// Fully-qualified domain name.
        pub domain_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of the domain.
        pub domain_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the domain configuration. This value must be unique to a region.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARNs of the certificates that IoT passes to the device during the TLS handshake. Currently you can specify only one certificate ARN. This value is not required for Amazon Web Services-managed domains. When using a custom `domain_name`, the cert must include it.
        pub server_certificate_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The type of service delivered by the endpoint. Note: Amazon Web Services IoT Core currently supports only the `DATA` service type.
        pub service_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The status to which the domain configuration should be set. Valid values are `ENABLED` and `DISABLED`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An object that specifies the TLS configuration for a domain. See the `tls_config` Block below for details.
        pub tls_config: pulumi_gestalt_rust::Output<
            super::super::types::iot::DomainConfigurationTlsConfig,
        >,
        /// The certificate used to validate the server certificate and prove domain name ownership. This certificate must be signed by a public certificate authority. This value is not required for Amazon Web Services-managed domains.
        pub validation_certificate_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainConfigurationArgs,
    ) -> DomainConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorizer_config_binding = args.authorizer_config.get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let server_certificate_arns_binding = args
            .server_certificate_arns
            .get_output(context);
        let service_type_binding = args.service_type.get_output(context);
        let status_binding = args.status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tls_config_binding = args.tls_config.get_output(context);
        let validation_certificate_arn_binding = args
            .validation_certificate_arn
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iot/domainConfiguration:DomainConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizerConfig".into(),
                    value: authorizer_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverCertificateArns".into(),
                    value: server_certificate_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceType".into(),
                    value: service_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsConfig".into(),
                    value: tls_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validationCertificateArn".into(),
                    value: validation_certificate_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainConfigurationResult {
            arn: o.get_field("arn"),
            authorizer_config: o.get_field("authorizerConfig"),
            domain_name: o.get_field("domainName"),
            domain_type: o.get_field("domainType"),
            name: o.get_field("name"),
            server_certificate_arns: o.get_field("serverCertificateArns"),
            service_type: o.get_field("serviceType"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tls_config: o.get_field("tlsConfig"),
            validation_certificate_arn: o.get_field("validationCertificateArn"),
        }
    }
}
