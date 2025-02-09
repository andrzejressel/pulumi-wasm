/// Manages an Amazon API Gateway Version 2 domain name.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html).
///
/// > **Note:** This resource establishes ownership of and the TLS settings for
/// a particular domain name. An API stage can be associated with the domain name using the `aws.apigatewayv2.ApiMapping` resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain_name::create(
///         "example",
///         DomainNameArgs::builder()
///             .domain_name("ws-api.example.com")
///             .domain_name_configuration(
///                 DomainNameDomainNameConfiguration::builder()
///                     .certificateArn("${exampleAwsAcmCertificate.arn}")
///                     .endpointType("REGIONAL")
///                     .securityPolicy("TLS_1_2")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Associated Route 53 Resource Record
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain_name::create(
///         "example",
///         DomainNameArgs::builder()
///             .domain_name("http-api.example.com")
///             .domain_name_configuration(
///                 DomainNameDomainNameConfiguration::builder()
///                     .certificateArn("${exampleAwsAcmCertificate.arn}")
///                     .endpointType("REGIONAL")
///                     .securityPolicy("TLS_1_2")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleRecord = record::create(
///         "exampleRecord",
///         RecordArgs::builder()
///             .aliases(
///                 vec![
///                     RecordAlias::builder().evaluateTargetHealth(false)
///                     .name("${example.domainNameConfiguration.targetDomainName}")
///                     .zoneId("${example.domainNameConfiguration.hostedZoneId}")
///                     .build_struct(),
///                 ],
///             )
///             .name("${example.domainName}")
///             .type_("A")
///             .zone_id("${exampleAwsRoute53Zone.zoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_domain_name` using the domain name. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/domainName:DomainName example ws-api.example.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_name {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainNameArgs {
        /// Domain name. Must be between 1 and 512 characters in length.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Domain name configuration. See below.
        #[builder(into)]
        pub domain_name_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::apigatewayv2::DomainNameDomainNameConfiguration,
        >,
        /// Mutual TLS authentication configuration for the domain name.
        #[builder(into, default)]
        pub mutual_tls_authentication: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigatewayv2::DomainNameMutualTlsAuthentication>,
        >,
        /// Map of tags to assign to the domain name. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainNameResult {
        /// [API mapping selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-mapping-selection-expressions) for the domain name.
        pub api_mapping_selection_expression: pulumi_gestalt_rust::Output<String>,
        /// ARN of the domain name.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Domain name. Must be between 1 and 512 characters in length.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Domain name configuration. See below.
        pub domain_name_configuration: pulumi_gestalt_rust::Output<
            super::super::types::apigatewayv2::DomainNameDomainNameConfiguration,
        >,
        /// Mutual TLS authentication configuration for the domain name.
        pub mutual_tls_authentication: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigatewayv2::DomainNameMutualTlsAuthentication>,
        >,
        /// Map of tags to assign to the domain name. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DomainNameArgs,
    ) -> DomainNameResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let domain_name_binding_1 = args.domain_name.get_output(context);
        let domain_name_binding = domain_name_binding_1.get_inner();
        let domain_name_configuration_binding_1 = args
            .domain_name_configuration
            .get_output(context);
        let domain_name_configuration_binding = domain_name_configuration_binding_1
            .get_inner();
        let mutual_tls_authentication_binding_1 = args
            .mutual_tls_authentication
            .get_output(context);
        let mutual_tls_authentication_binding = mutual_tls_authentication_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigatewayv2/domainName:DomainName".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "domainNameConfiguration".into(),
                    value: &domain_name_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "mutualTlsAuthentication".into(),
                    value: &mutual_tls_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DomainNameResult {
            api_mapping_selection_expression: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiMappingSelectionExpression"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            domain_name_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainNameConfiguration"),
            ),
            mutual_tls_authentication: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mutualTlsAuthentication"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
