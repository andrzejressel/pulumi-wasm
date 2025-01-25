/// Manages a Service Catalog Product.
///
/// > **NOTE:** The user or role that uses this resources must have the `cloudformation:GetTemplate` IAM policy permission. This policy permission is required when using the `template_physical_id` argument.
///
/// > A "provisioning artifact" is also referred to as a "version." A "distributor" is also referred to as a "vendor."
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicecatalog:Product
///     properties:
///       name: example
///       owner: example-owner
///       type: CLOUD_FORMATION_TEMPLATE
///       provisioningArtifactParameters:
///         templateUrl: https://s3.amazonaws.com/cf-templates-ozkq9d3hgiq2-us-east-1/temp1.json
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_product` using the product ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/product:Product example prod-dnigbtea24ste
/// ```
pub mod product {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProductArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Description of the product.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Distributor (i.e., vendor) of the product.
        #[builder(into, default)]
        pub distributor: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the product.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Owner of the product.
        #[builder(into)]
        pub owner: pulumi_wasm_rust::InputOrOutput<String>,
        /// Configuration block for provisioning artifact (i.e., version) parameters. See `provisioning_artifact_parameters` Block for details.
        #[builder(into)]
        pub provisioning_artifact_parameters: pulumi_wasm_rust::InputOrOutput<
            super::super::types::servicecatalog::ProductProvisioningArtifactParameters,
        >,
        /// Support information about the product.
        #[builder(into, default)]
        pub support_description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Contact email for product support.
        #[builder(into, default)]
        pub support_email: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Contact URL for product support.
        #[builder(into, default)]
        pub support_url: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the product. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of product. See [AWS Docs](https://docs.aws.amazon.com/servicecatalog/latest/dg/API_CreateProduct.html#API_CreateProduct_RequestSyntax) for valid list of values.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProductResult {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the product.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Time when the product was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Description of the product.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Distributor (i.e., vendor) of the product.
        pub distributor: pulumi_wasm_rust::Output<String>,
        /// Whether the product has a default path. If the product does not have a default path, call `ListLaunchPaths` to disambiguate between paths.  Otherwise, `ListLaunchPaths` is not required, and the output of ProductViewSummary can be used directly with `DescribeProvisioningParameters`.
        pub has_default_path: pulumi_wasm_rust::Output<bool>,
        /// Name of the product.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Owner of the product.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// Configuration block for provisioning artifact (i.e., version) parameters. See `provisioning_artifact_parameters` Block for details.
        pub provisioning_artifact_parameters: pulumi_wasm_rust::Output<
            super::super::types::servicecatalog::ProductProvisioningArtifactParameters,
        >,
        /// Status of the product.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Support information about the product.
        pub support_description: pulumi_wasm_rust::Output<String>,
        /// Contact email for product support.
        pub support_email: pulumi_wasm_rust::Output<String>,
        /// Contact URL for product support.
        pub support_url: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the product. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of product. See [AWS Docs](https://docs.aws.amazon.com/servicecatalog/latest/dg/API_CreateProduct.html#API_CreateProduct_RequestSyntax) for valid list of values.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProductArgs,
    ) -> ProductResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args
            .accept_language
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let distributor_binding = args.distributor.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let owner_binding = args.owner.get_output(context).get_inner();
        let provisioning_artifact_parameters_binding = args
            .provisioning_artifact_parameters
            .get_output(context)
            .get_inner();
        let support_description_binding = args
            .support_description
            .get_output(context)
            .get_inner();
        let support_email_binding = args.support_email.get_output(context).get_inner();
        let support_url_binding = args.support_url.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/product:Product".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "distributor".into(),
                    value: &distributor_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "owner".into(),
                    value: &owner_binding,
                },
                register_interface::ObjectField {
                    name: "provisioningArtifactParameters".into(),
                    value: &provisioning_artifact_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "supportDescription".into(),
                    value: &support_description_binding,
                },
                register_interface::ObjectField {
                    name: "supportEmail".into(),
                    value: &support_email_binding,
                },
                register_interface::ObjectField {
                    name: "supportUrl".into(),
                    value: &support_url_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceptLanguage".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "distributor".into(),
                },
                register_interface::ResultField {
                    name: "hasDefaultPath".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "provisioningArtifactParameters".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "supportDescription".into(),
                },
                register_interface::ResultField {
                    name: "supportEmail".into(),
                },
                register_interface::ResultField {
                    name: "supportUrl".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProductResult {
            accept_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptLanguage").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            distributor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distributor").unwrap(),
            ),
            has_default_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasDefaultPath").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            provisioning_artifact_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisioningArtifactParameters").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            support_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportDescription").unwrap(),
            ),
            support_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportEmail").unwrap(),
            ),
            support_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportUrl").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
