#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_log_data_protection_policy_document {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLogDataProtectionPolicyDocumentArgs {
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the data protection policy document.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configures the data protection policy.
        ///
        /// > There must be exactly two statements: the first with an `audit` operation, and the second with a `deidentify` operation.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub statements: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatement,
            >,
        >,
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetLogDataProtectionPolicyDocumentResult {
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Standard JSON policy document rendered based on the arguments above.
        pub json: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub statements: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatement,
            >,
        >,
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLogDataProtectionPolicyDocumentArgs,
    ) -> GetLogDataProtectionPolicyDocumentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let statements_binding = args.statements.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudwatch/getLogDataProtectionPolicyDocument:getLogDataProtectionPolicyDocument"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statements".into(),
                    value: statements_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLogDataProtectionPolicyDocumentResult {
            description: o.get_field("description"),
            id: o.get_field("id"),
            json: o.get_field("json"),
            name: o.get_field("name"),
            statements: o.get_field("statements"),
            version: o.get_field("version"),
        }
    }
}
