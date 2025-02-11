/// Provides an CloudSearch domain resource.
///
/// The provider waits for the domain to become `Active` when applying a configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// Domain endpoint options. Documented below.
        #[builder(into, default)]
        pub endpoint_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudsearch::DomainEndpointOptions>,
        >,
        /// The index fields for documents added to the domain. Documented below.
        #[builder(into, default)]
        pub index_fields: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cloudsearch::DomainIndexField>>,
        >,
        /// Whether or not to maintain extra instances for the domain in a second Availability Zone to ensure high availability.
        #[builder(into, default)]
        pub multi_az: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the CloudSearch domain.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Domain scaling parameters. Documented below.
        #[builder(into, default)]
        pub scaling_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudsearch::DomainScalingParameters>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// The domain's ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The service endpoint for updating documents in a search domain.
        pub document_service_endpoint: pulumi_gestalt_rust::Output<String>,
        /// An internally generated unique identifier for the domain.
        pub domain_id: pulumi_gestalt_rust::Output<String>,
        /// Domain endpoint options. Documented below.
        pub endpoint_options: pulumi_gestalt_rust::Output<
            super::super::types::cloudsearch::DomainEndpointOptions,
        >,
        /// The index fields for documents added to the domain. Documented below.
        pub index_fields: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cloudsearch::DomainIndexField>>,
        >,
        /// Whether or not to maintain extra instances for the domain in a second Availability Zone to ensure high availability.
        pub multi_az: pulumi_gestalt_rust::Output<bool>,
        /// The name of the CloudSearch domain.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Domain scaling parameters. Documented below.
        pub scaling_parameters: pulumi_gestalt_rust::Output<
            super::super::types::cloudsearch::DomainScalingParameters,
        >,
        /// The service endpoint for requesting search results from a search domain.
        pub search_service_endpoint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainArgs,
    ) -> DomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let endpoint_options_binding = args.endpoint_options.get_output(context);
        let index_fields_binding = args.index_fields.get_output(context);
        let multi_az_binding = args.multi_az.get_output(context);
        let name_binding = args.name.get_output(context);
        let scaling_parameters_binding = args.scaling_parameters.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudsearch/domain:Domain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointOptions".into(),
                    value: &endpoint_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexFields".into(),
                    value: &index_fields_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiAz".into(),
                    value: &multi_az_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scalingParameters".into(),
                    value: &scaling_parameters_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainResult {
            arn: o.get_field("arn"),
            document_service_endpoint: o.get_field("documentServiceEndpoint"),
            domain_id: o.get_field("domainId"),
            endpoint_options: o.get_field("endpointOptions"),
            index_fields: o.get_field("indexFields"),
            multi_az: o.get_field("multiAz"),
            name: o.get_field("name"),
            scaling_parameters: o.get_field("scalingParameters"),
            search_service_endpoint: o.get_field("searchServiceEndpoint"),
        }
    }
}
