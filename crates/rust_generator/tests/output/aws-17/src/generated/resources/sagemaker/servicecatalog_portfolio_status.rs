/// Manages status of Service Catalog in SageMaker. Service Catalog is used to create SageMaker projects.
///
/// ## Example Usage
///
/// Usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = servicecatalog_portfolio_status::create(
///         "example",
///         ServicecatalogPortfolioStatusArgs::builder().status("Enabled").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import models using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/servicecatalogPortfolioStatus:ServicecatalogPortfolioStatus example us-east-1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod servicecatalog_portfolio_status {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicecatalogPortfolioStatusArgs {
        /// Whether Service Catalog is enabled or disabled in SageMaker. Valid values are `Enabled` and `Disabled`.
        #[builder(into)]
        pub status: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServicecatalogPortfolioStatusResult {
        /// Whether Service Catalog is enabled or disabled in SageMaker. Valid values are `Enabled` and `Disabled`.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServicecatalogPortfolioStatusArgs,
    ) -> ServicecatalogPortfolioStatusResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let status_binding = args.status.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/servicecatalogPortfolioStatus:ServicecatalogPortfolioStatus"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServicecatalogPortfolioStatusResult {
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
