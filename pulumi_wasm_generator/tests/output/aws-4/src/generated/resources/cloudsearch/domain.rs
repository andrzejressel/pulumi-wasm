/// Provides an CloudSearch domain resource.
///
/// The provider waits for the domain to become `Active` when applying a configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain::create(
///         "example",
///         DomainArgs::builder()
///             .index_fields(
///                 vec![
///                     DomainIndexField::builder().analysisScheme("_en_default_")
///                     .highlight(false).name("headline"). return (true).search(true)
///                     .sort(true). type ("text").build_struct(),
///                     DomainIndexField::builder().facet(true).name("price"). return (true)
///                     .search(true).sort(true).sourceFields("headline"). type ("double")
///                     .build_struct(),
///                 ],
///             )
///             .name("example-domain")
///             .scaling_parameters(
///                 DomainScalingParameters::builder()
///                     .desiredInstanceType("search.medium")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudSearch Domains using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudsearch/domain:Domain example example-domain
/// ```
pub mod domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// Domain endpoint options. Documented below.
        #[builder(into, default)]
        pub endpoint_options: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudsearch::DomainEndpointOptions>,
        >,
        /// The index fields for documents added to the domain. Documented below.
        #[builder(into, default)]
        pub index_fields: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudsearch::DomainIndexField>>,
        >,
        /// Whether or not to maintain extra instances for the domain in a second Availability Zone to ensure high availability.
        #[builder(into, default)]
        pub multi_az: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the CloudSearch domain.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Domain scaling parameters. Documented below.
        #[builder(into, default)]
        pub scaling_parameters: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudsearch::DomainScalingParameters>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// The domain's ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The service endpoint for updating documents in a search domain.
        pub document_service_endpoint: pulumi_wasm_rust::Output<String>,
        /// An internally generated unique identifier for the domain.
        pub domain_id: pulumi_wasm_rust::Output<String>,
        /// Domain endpoint options. Documented below.
        pub endpoint_options: pulumi_wasm_rust::Output<
            super::super::types::cloudsearch::DomainEndpointOptions,
        >,
        /// The index fields for documents added to the domain. Documented below.
        pub index_fields: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudsearch::DomainIndexField>>,
        >,
        /// Whether or not to maintain extra instances for the domain in a second Availability Zone to ensure high availability.
        pub multi_az: pulumi_wasm_rust::Output<bool>,
        /// The name of the CloudSearch domain.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Domain scaling parameters. Documented below.
        pub scaling_parameters: pulumi_wasm_rust::Output<
            super::super::types::cloudsearch::DomainScalingParameters,
        >,
        /// The service endpoint for requesting search results from a search domain.
        pub search_service_endpoint: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainArgs) -> DomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let endpoint_options_binding = args.endpoint_options.get_inner();
        let index_fields_binding = args.index_fields.get_inner();
        let multi_az_binding = args.multi_az.get_inner();
        let name_binding = args.name.get_inner();
        let scaling_parameters_binding = args.scaling_parameters.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudsearch/domain:Domain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endpointOptions".into(),
                    value: &endpoint_options_binding,
                },
                register_interface::ObjectField {
                    name: "indexFields".into(),
                    value: &index_fields_binding,
                },
                register_interface::ObjectField {
                    name: "multiAz".into(),
                    value: &multi_az_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scalingParameters".into(),
                    value: &scaling_parameters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "documentServiceEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "domainId".into(),
                },
                register_interface::ResultField {
                    name: "endpointOptions".into(),
                },
                register_interface::ResultField {
                    name: "indexFields".into(),
                },
                register_interface::ResultField {
                    name: "multiAz".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "scalingParameters".into(),
                },
                register_interface::ResultField {
                    name: "searchServiceEndpoint".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            document_service_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentServiceEndpoint").unwrap(),
            ),
            domain_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainId").unwrap(),
            ),
            endpoint_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointOptions").unwrap(),
            ),
            index_fields: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexFields").unwrap(),
            ),
            multi_az: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiAz").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            scaling_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalingParameters").unwrap(),
            ),
            search_service_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("searchServiceEndpoint").unwrap(),
            ),
        }
    }
}
