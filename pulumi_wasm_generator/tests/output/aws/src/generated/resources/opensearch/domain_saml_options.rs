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
///               Function: std:file
///               Arguments:
///                 input: ./saml-metadata.xml
///               Return: result
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainSamlOptionsArgs {
        /// Name of the domain.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// SAML authentication options for an AWS OpenSearch Domain.
        #[builder(into, default)]
        pub saml_options: pulumi_wasm_rust::Output<
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
    pub fn create(name: &str, args: DomainSamlOptionsArgs) -> DomainSamlOptionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_inner();
        let saml_options_binding = args.saml_options.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/domainSamlOptions:DomainSamlOptions".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "samlOptions".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainSamlOptionsResult {
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            saml_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samlOptions").unwrap(),
            ),
        }
    }
}