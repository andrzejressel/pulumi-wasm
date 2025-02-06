/// Resource for managing an AWS Network Firewall TLS Inspection Configuration.
///
/// ## Example Usage
///
/// > **NOTE:** You must configure either inbound inspection, outbound inspection, or both.
///
/// ### Basic inbound/ingress inspection
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = tls_inspection_configuration::create(
///         "example",
///         TlsInspectionConfigurationArgs::builder()
///             .description("example")
///             .encryption_configurations(
///                 vec![
///                     TlsInspectionConfigurationEncryptionConfiguration::builder()
///                     .keyId("AWS_OWNED_KMS_KEY"). type ("AWS_OWNED_KMS_KEY")
///                     .build_struct(),
///                 ],
///             )
///             .name("example")
///             .tls_inspection_configuration(
///                 TlsInspectionConfigurationTlsInspectionConfiguration::builder()
///                     .serverCertificateConfiguration(
///                         TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfiguration::builder()
///                             .scopes(
///                                 vec![
///                                     TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScope::builder()
///                                     .destinationPorts(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestinationPort::builder()
///                                     .fromPort(443).toPort(443).build_struct(),])
///                                     .destinations(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestination::builder()
///                                     .addressDefinition("0.0.0.0/0").build_struct(),])
///                                     .protocols(vec![6,])
///                                     .sourcePorts(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSourcePort::builder()
///                                     .fromPort(0).toPort(65535).build_struct(),])
///                                     .sources(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSource::builder()
///                                     .addressDefinition("0.0.0.0/0").build_struct(),])
///                                     .build_struct(),
///                                 ],
///                             )
///                             .serverCertificates(
///                                 vec![
///                                     TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationServerCertificate::builder()
///                                     .resourceArn("${example1.arn}").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Basic outbound/engress inspection
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = tls_inspection_configuration::create(
///         "example",
///         TlsInspectionConfigurationArgs::builder()
///             .description("example")
///             .encryption_configurations(
///                 vec![
///                     TlsInspectionConfigurationEncryptionConfiguration::builder()
///                     .keyId("AWS_OWNED_KMS_KEY"). type ("AWS_OWNED_KMS_KEY")
///                     .build_struct(),
///                 ],
///             )
///             .name("example")
///             .tls_inspection_configuration(
///                 TlsInspectionConfigurationTlsInspectionConfiguration::builder()
///                     .serverCertificateConfiguration(
///                         TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfiguration::builder()
///                             .certificateAuthorityArn("${example1.arn}")
///                             .checkCertificateRevocationStatus(
///                                 TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationCheckCertificateRevocationStatus::builder()
///                                     .revokedStatusAction("REJECT")
///                                     .unknownStatusAction("PASS")
///                                     .build_struct(),
///                             )
///                             .scopes(
///                                 vec![
///                                     TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScope::builder()
///                                     .destinationPorts(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestinationPort::builder()
///                                     .fromPort(443).toPort(443).build_struct(),])
///                                     .destinations(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestination::builder()
///                                     .addressDefinition("0.0.0.0/0").build_struct(),])
///                                     .protocols(vec![6,])
///                                     .sourcePorts(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSourcePort::builder()
///                                     .fromPort(0).toPort(65535).build_struct(),])
///                                     .sources(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSource::builder()
///                                     .addressDefinition("0.0.0.0/0").build_struct(),])
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Inbound with encryption configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key::create(
///         "example",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("example")
///             .build_struct(),
///     );
///     let exampleTlsInspectionConfiguration = tls_inspection_configuration::create(
///         "exampleTlsInspectionConfiguration",
///         TlsInspectionConfigurationArgs::builder()
///             .description("example")
///             .encryption_configurations(
///                 vec![
///                     TlsInspectionConfigurationEncryptionConfiguration::builder()
///                     .keyId("${example.arn}"). type ("CUSTOMER_KMS").build_struct(),
///                 ],
///             )
///             .name("example")
///             .tls_inspection_configuration(
///                 TlsInspectionConfigurationTlsInspectionConfiguration::builder()
///                     .serverCertificateConfiguration(
///                         TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfiguration::builder()
///                             .scopes(
///                                 vec![
///                                     TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScope::builder()
///                                     .destinationPorts(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestinationPort::builder()
///                                     .fromPort(443).toPort(443).build_struct(),])
///                                     .destinations(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestination::builder()
///                                     .addressDefinition("0.0.0.0/0").build_struct(),])
///                                     .protocols(vec![6,])
///                                     .sourcePorts(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSourcePort::builder()
///                                     .fromPort(0).toPort(65535).build_struct(),])
///                                     .sources(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSource::builder()
///                                     .addressDefinition("0.0.0.0/0").build_struct(),])
///                                     .build_struct(),
///                                 ],
///                             )
///                             .serverCertificates(
///                                 vec![
///                                     TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationServerCertificate::builder()
///                                     .resourceArn("${example1.arn}").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Outbound with encryption configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: example
///       deletionWindowInDays: 7
///   exampleTlsInspectionConfiguration:
///     type: aws:networkfirewall:TlsInspectionConfiguration
///     name: example
///     properties:
///       name: example
///       description: example
///       encryptionConfigurations:
///         - keyId: ${example.arn}
///           type: CUSTOMER_KMS
///       tlsInspectionConfiguration:
///         serverCertificateConfigurations:
///           - certificateAuthorityArn: ${example1.arn}
///             checkCertificateRevocationStatus:
///               - revokedStatusAction: REJECT
///                 unknownStatusAction: PASS
///             scope:
///               - protocols:
///                   - 6
///                 destinationPorts:
///                   - fromPort: 443
///                     toPort: 443
///                 destination:
///                   - addressDefinition: 0.0.0.0/0
///                 sourcePorts:
///                   - fromPort: 0
///                     toPort: 65535
///                 source:
///                   - addressDefinition: 0.0.0.0/0
/// ```
///
/// ### Combined inbound and outbound
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = tls_inspection_configuration::create(
///         "example",
///         TlsInspectionConfigurationArgs::builder()
///             .description("example")
///             .encryption_configurations(
///                 vec![
///                     TlsInspectionConfigurationEncryptionConfiguration::builder()
///                     .keyId("AWS_OWNED_KMS_KEY"). type ("AWS_OWNED_KMS_KEY")
///                     .build_struct(),
///                 ],
///             )
///             .name("example")
///             .tls_inspection_configuration(
///                 TlsInspectionConfigurationTlsInspectionConfiguration::builder()
///                     .serverCertificateConfiguration(
///                         TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfiguration::builder()
///                             .certificateAuthorityArn("${example1.arn}")
///                             .checkCertificateRevocationStatus(
///                                 TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationCheckCertificateRevocationStatus::builder()
///                                     .revokedStatusAction("REJECT")
///                                     .unknownStatusAction("PASS")
///                                     .build_struct(),
///                             )
///                             .scopes(
///                                 vec![
///                                     TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScope::builder()
///                                     .destinationPorts(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestinationPort::builder()
///                                     .fromPort(443).toPort(443).build_struct(),])
///                                     .destinations(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestination::builder()
///                                     .addressDefinition("0.0.0.0/0").build_struct(),])
///                                     .protocols(vec![6,])
///                                     .sourcePorts(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSourcePort::builder()
///                                     .fromPort(0).toPort(65535).build_struct(),])
///                                     .sources(vec![TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSource::builder()
///                                     .addressDefinition("0.0.0.0/0").build_struct(),])
///                                     .build_struct(),
///                                 ],
///                             )
///                             .serverCertificates(
///                                 vec![
///                                     TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationServerCertificate::builder()
///                                     .resourceArn("${example2.arn}").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Firewall TLS Inspection Configuration using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:networkfirewall/tlsInspectionConfiguration:TlsInspectionConfiguration example arn:aws:network-firewall::<region>:<account_id>:tls-configuration/example
/// ```
pub mod tls_inspection_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TlsInspectionConfigurationArgs {
        /// Description of the TLS inspection configuration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Encryption configuration block. Detailed below.
        #[builder(into, default)]
        pub encryption_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::networkfirewall::TlsInspectionConfigurationEncryptionConfiguration,
                >,
            >,
        >,
        /// Descriptive name of the TLS inspection configuration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networkfirewall::TlsInspectionConfigurationTimeouts,
            >,
        >,
        /// TLS inspection configuration block. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub tls_inspection_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct TlsInspectionConfigurationResult {
        /// ARN of the TLS Inspection Configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Certificate Manager certificate block. See Certificate Authority below for details.
        pub certificate_authorities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::networkfirewall::TlsInspectionConfigurationCertificateAuthority,
            >,
        >,
        /// List of certificate blocks describing certificates associated with the TLS inspection configuration. See Certificates below for details.
        pub certificates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::networkfirewall::TlsInspectionConfigurationCertificate,
            >,
        >,
        /// Description of the TLS inspection configuration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Encryption configuration block. Detailed below.
        pub encryption_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::networkfirewall::TlsInspectionConfigurationEncryptionConfiguration,
            >,
        >,
        /// Descriptive name of the TLS inspection configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Number of firewall policies that use this TLS inspection configuration.
        pub number_of_associations: pulumi_gestalt_rust::Output<i32>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networkfirewall::TlsInspectionConfigurationTimeouts,
            >,
        >,
        /// TLS inspection configuration block. Detailed below.
        ///
        /// The following arguments are optional:
        pub tls_inspection_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfiguration,
            >,
        >,
        /// A unique identifier for the TLS inspection configuration.
        pub tls_inspection_configuration_id: pulumi_gestalt_rust::Output<String>,
        /// String token used when updating the rule group.
        pub update_token: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TlsInspectionConfigurationArgs,
    ) -> TlsInspectionConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let encryption_configurations_binding = args
            .encryption_configurations
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let tls_inspection_configuration_binding = args
            .tls_inspection_configuration
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkfirewall/tlsInspectionConfiguration:TlsInspectionConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfigurations".into(),
                    value: &encryption_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "tlsInspectionConfiguration".into(),
                    value: &tls_inspection_configuration_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TlsInspectionConfigurationResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            certificate_authorities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateAuthorities"),
            ),
            certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificates"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            encryption_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionConfigurations"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            number_of_associations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numberOfAssociations"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            tls_inspection_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsInspectionConfiguration"),
            ),
            tls_inspection_configuration_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsInspectionConfigurationId"),
            ),
            update_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateToken"),
            ),
        }
    }
}
