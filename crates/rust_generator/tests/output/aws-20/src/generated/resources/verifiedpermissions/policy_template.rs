/// Resource for managing an AWS Verified Permissions Policy Template.
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
///     let example = policy_template::create(
///         "example",
///         PolicyTemplateArgs::builder()
///             .policy_store_id("${exampleAwsVerifiedpermissionsPolicyStore.id}")
///             .statement(
///                 "permit (principal in ?principal, action in PhotoFlash::Action::\"FullPhotoAccess\", resource == ?resource) unless { resource.IsPrivate };",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Permissions Policy Store using the `policy_store_id:policy_template_id`. For example:
///
/// ```sh
/// $ pulumi import aws:verifiedpermissions/policyTemplate:PolicyTemplate example policyStoreId:policyTemplateId
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyTemplateArgs {
        /// Provides a description for the policy template.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Policy Store.
        #[builder(into)]
        pub policy_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Defines the content of the statement, written in Cedar policy language.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub statement: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyTemplateResult {
        /// The date the Policy Store was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Provides a description for the policy template.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Policy Store.
        pub policy_store_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Policy Store.
        pub policy_template_id: pulumi_gestalt_rust::Output<String>,
        /// Defines the content of the statement, written in Cedar policy language.
        ///
        /// The following arguments are optional:
        pub statement: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyTemplateArgs,
    ) -> PolicyTemplateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let policy_store_id_binding = args.policy_store_id.get_output(context);
        let statement_binding = args.statement.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:verifiedpermissions/policyTemplate:PolicyTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyStoreId".into(),
                    value: policy_store_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statement".into(),
                    value: statement_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyTemplateResult {
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            policy_store_id: o.get_field("policyStoreId"),
            policy_template_id: o.get_field("policyTemplateId"),
            statement: o.get_field("statement"),
        }
    }
}
