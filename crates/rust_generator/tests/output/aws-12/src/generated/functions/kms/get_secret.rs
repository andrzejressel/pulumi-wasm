#[allow(clippy::doc_lazy_continuation)]
pub mod get_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretArgs {
        #[builder(into)]
        pub secrets: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::super::types::kms::GetSecretSecret>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSecretResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub secrets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kms::GetSecretSecret>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSecretArgs,
    ) -> GetSecretResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let secrets_binding = args.secrets.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kms/getSecret:getSecret".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSecretResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            secrets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secrets"),
            ),
        }
    }
}
