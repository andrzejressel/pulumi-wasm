/// Manages SAML authentication options for an AWS Elasticsearch Domain.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:elasticsearch:Domain
///     properties:
///       domainName: example
///       elasticsearchVersion: '1.5'
///       clusterConfig:
///         instanceType: r4.large.elasticsearch
///       snapshotOptions:
///         automatedSnapshotStartHour: 23
///       tags:
///         Domain: TestDomain
///   exampleDomainSamlOptions:
///     type: aws:elasticsearch:DomainSamlOptions
///     name: example
///     properties:
///       domainName: ${example.domainName}
///       samlOptions:
///         enabled: true
///         idp:
///           entityId: https://example.com
///           metadataContent:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: ./saml-metadata.xml
///               return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Elasticsearch domains using the `domain_name`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticsearch/domainSamlOptions:DomainSamlOptions example domain_name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_saml_options {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainSamlOptionsArgs {
        /// Name of the domain.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SAML authentication options for an AWS Elasticsearch Domain.
        #[builder(into, default)]
        pub saml_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elasticsearch::DomainSamlOptionsSamlOptions>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainSamlOptionsResult {
        /// Name of the domain.
        ///
        /// The following arguments are optional:
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// The SAML authentication options for an AWS Elasticsearch Domain.
        pub saml_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::elasticsearch::DomainSamlOptionsSamlOptions>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainSamlOptionsArgs,
    ) -> DomainSamlOptionsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let saml_options_binding = args.saml_options.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elasticsearch/domainSamlOptions:DomainSamlOptions".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "samlOptions".into(),
                    value: &saml_options_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainSamlOptionsResult {
            domain_name: o.get_field("domainName"),
            saml_options: o.get_field("samlOptions"),
        }
    }
}
