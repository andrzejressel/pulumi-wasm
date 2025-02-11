/// Resource for managing an AWS Service Quotas Template Association.
///
/// > Only the management account of an organization can associate Service Quota templates, and this must be done from the `us-east-1` region.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = template_association::create(
///         "example",
///         TemplateAssociationArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Service Quotas Template Association using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:servicequotas/templateAssociation:TemplateAssociation example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod template_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TemplateAssociationArgs {
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct TemplateAssociationResult {
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Association status. Creating this resource will result in an `ASSOCIATED` status, and quota increase requests in the template are automatically applied to new AWS accounts in the organization.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TemplateAssociationArgs,
    ) -> TemplateAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let skip_destroy_binding = args.skip_destroy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:servicequotas/templateAssociation:TemplateAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TemplateAssociationResult {
            skip_destroy: o.get_field("skipDestroy"),
            status: o.get_field("status"),
        }
    }
}
