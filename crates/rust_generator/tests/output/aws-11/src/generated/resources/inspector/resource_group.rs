/// Provides an Amazon Inspector Classic Resource Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:inspector:ResourceGroup
///     properties:
///       tags:
///         Name: foo
///         Env: bar
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceGroupArgs {
        /// Key-value map of tags that are used to select the EC2 instances to be included in an Amazon Inspector assessment target.
        #[builder(into)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceGroupResult {
        /// The resource group ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of tags that are used to select the EC2 instances to be included in an Amazon Inspector assessment target.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceGroupArgs,
    ) -> ResourceGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:inspector/resourceGroup:ResourceGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceGroupResult {
            arn: o.get_field("arn"),
            tags: o.get_field("tags"),
        }
    }
}
