#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_parameter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetParameterArgs {
        /// Name of the parameter.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to return decrypted `SecureString` value. Defaults to `true`.
        ///
        /// In addition to all arguments above, the following attributes are exported:
        #[builder(into, default)]
        pub with_decryption: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetParameterResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub insecure_value: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub value: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<i32>,
        pub with_decryption: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetParameterArgs,
    ) -> GetParameterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let with_decryption_binding = args.with_decryption.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssm/getParameter:getParameter".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "withDecryption".into(),
                    value: &with_decryption_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetParameterResult {
            arn: o.get_field("arn"),
            id: o.get_field("id"),
            insecure_value: o.get_field("insecureValue"),
            name: o.get_field("name"),
            type_: o.get_field("type"),
            value: o.get_field("value"),
            version: o.get_field("version"),
            with_decryption: o.get_field("withDecryption"),
        }
    }
}
