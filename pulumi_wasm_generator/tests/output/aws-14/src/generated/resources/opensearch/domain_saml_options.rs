/// Manages SAML authentication options for an AWS OpenSearch Domain.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:Domain
///     properties:
///       domainName: example
///       engineVersion: OpenSearch_1.1
///       clusterConfig:
///         instanceType: r4.large.search
///       snapshotOptions:
///         automatedSnapshotStartHour: 23
///       tags:
///         Domain: TestDomain
///   exampleDomainSamlOptions:
///     type: aws:opensearch:DomainSamlOptions
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
/// Using `pulumi import`, import OpenSearch domains using the `domain_name`. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/domainSamlOptions:DomainSamlOptions example domain_name
/// ```
pub mod domain_saml_options {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainSamlOptionsArgs {
        /// Name of the domain.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// SAML authentication options for an AWS OpenSearch Domain.
        #[builder(into, default)]
        pub saml_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::opensearch::DomainSamlOptionsSamlOptions>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainSamlOptionsResult {
        /// Name of the domain.
        ///
        /// The following arguments are optional:
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// SAML authentication options for an AWS OpenSearch Domain.
        pub saml_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainSamlOptionsSamlOptions>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DomainSamlOptionsArgs,
    ) -> DomainSamlOptionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let saml_options_binding = args.saml_options.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/domainSamlOptions:DomainSamlOptions".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "samlOptions".into(),
                    value: &saml_options_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DomainSamlOptionsResult {
            domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            saml_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("samlOptions"),
            ),
        }
    }
}
