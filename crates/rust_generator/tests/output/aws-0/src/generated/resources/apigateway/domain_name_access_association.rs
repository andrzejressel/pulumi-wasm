/// Creates a domain name access association resource between an access association source and a private custom domain name.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain_name_access_association::create(
///         "example",
///         DomainNameAccessAssociationArgs::builder()
///             .access_association_source("${exampleAwsVpcEndpoint.id}")
///             .access_association_source_type("VPCE")
///             .domain_name_arn("${exampleAwsApiGatewayDomainName.domainNameArn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import API Gateway domain name acces associations as using their `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/domainNameAccessAssociation:DomainNameAccessAssociation example arn:aws:apigateway:us-west-2:123456789012:/domainnameaccessassociations/domainname/12qmzgp2.9m7ilski.test+hykg7a12e7/vpcesource/vpce-05de3f8f82740a748
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_name_access_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainNameAccessAssociationArgs {
        /// The identifier of the domain name access association source. For a `VPCE`, the value is the VPC endpoint ID.
        #[builder(into)]
        pub access_association_source: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the domain name access association source. Valid values are `VPCE`.
        #[builder(into)]
        pub access_association_source_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the domain name.
        #[builder(into)]
        pub domain_name_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainNameAccessAssociationResult {
        /// The identifier of the domain name access association source. For a `VPCE`, the value is the VPC endpoint ID.
        pub access_association_source: pulumi_gestalt_rust::Output<String>,
        /// The type of the domain name access association source. Valid values are `VPCE`.
        pub access_association_source_type: pulumi_gestalt_rust::Output<String>,
        /// ARN of the domain name access association.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the domain name.
        pub domain_name_arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainNameAccessAssociationArgs,
    ) -> DomainNameAccessAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_association_source_binding = args
            .access_association_source
            .get_output(context);
        let access_association_source_type_binding = args
            .access_association_source_type
            .get_output(context);
        let domain_name_arn_binding = args.domain_name_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/domainNameAccessAssociation:DomainNameAccessAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessAssociationSource".into(),
                    value: access_association_source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessAssociationSourceType".into(),
                    value: access_association_source_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainNameArn".into(),
                    value: domain_name_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainNameAccessAssociationResult {
            access_association_source: o.get_field("accessAssociationSource"),
            access_association_source_type: o.get_field("accessAssociationSourceType"),
            arn: o.get_field("arn"),
            domain_name_arn: o.get_field("domainNameArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
