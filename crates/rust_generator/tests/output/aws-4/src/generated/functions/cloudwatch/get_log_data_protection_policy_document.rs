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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLogDataProtectionPolicyDocumentArgs,
    ) -> GetLogDataProtectionPolicyDocumentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let statements_binding_1 = args.statements.get_output(context);
        let statements_binding = statements_binding_1.get_inner();
        let version_binding_1 = args.version.get_output(context);
        let version_binding = version_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudwatch/getLogDataProtectionPolicyDocument:getLogDataProtectionPolicyDocument"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "statements".into(),
                    value: &statements_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLogDataProtectionPolicyDocumentResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            json: pulumi_gestalt_rust::__private::into_domain(o.extract_field("json")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            statements: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statements"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
