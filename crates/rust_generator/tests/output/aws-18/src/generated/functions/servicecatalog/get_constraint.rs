#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_constraint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConstraintArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the constraint.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Constraint identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetConstraintResult {
        pub accept_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// Description of the constraint.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Owner of the constraint.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// Constraint parameters in JSON format.
        pub parameters: pulumi_gestalt_rust::Output<String>,
        /// Portfolio identifier.
        pub portfolio_id: pulumi_gestalt_rust::Output<String>,
        /// Product identifier.
        pub product_id: pulumi_gestalt_rust::Output<String>,
        /// Constraint status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Type of constraint. Valid values are `LAUNCH`, `NOTIFICATION`, `RESOURCE_UPDATE`, `STACKSET`, and `TEMPLATE`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConstraintArgs,
    ) -> GetConstraintResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accept_language_binding = args.accept_language.get_output(context);
        let description_binding = args.description.get_output(context);
        let id_binding = args.id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:servicecatalog/getConstraint:getConstraint".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConstraintResult {
            accept_language: o.get_field("acceptLanguage"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            owner: o.get_field("owner"),
            parameters: o.get_field("parameters"),
            portfolio_id: o.get_field("portfolioId"),
            product_id: o.get_field("productId"),
            status: o.get_field("status"),
            type_: o.get_field("type"),
        }
    }
}
