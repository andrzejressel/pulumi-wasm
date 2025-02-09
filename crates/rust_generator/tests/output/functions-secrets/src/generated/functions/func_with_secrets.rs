#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod func_with_secrets {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithSecretsArgs {
        #[builder(into)]
        pub crypto_key: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub plaintext: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FuncWithSecretsResult {
        pub ciphertext: pulumi_gestalt_rust::Output<String>,
        pub crypto_key: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub plaintext: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: FuncWithSecretsArgs,
    ) -> FuncWithSecretsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crypto_key_binding = args.crypto_key.get_output(context);
        let plaintext_binding = args.plaintext.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "mypkg::funcWithSecrets".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKey".into(),
                    value: crypto_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plaintext".into(),
                    value: plaintext_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        FuncWithSecretsResult {
            ciphertext: o.get_field("ciphertext"),
            crypto_key: o.get_field("cryptoKey"),
            id: o.get_field("id"),
            plaintext: o.get_field("plaintext"),
        }
    }
}
