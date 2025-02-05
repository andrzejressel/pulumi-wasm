pub mod get_caller_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCallerIdentityArgs {
        /// Account ID number of the account that owns or contains the calling entity.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCallerIdentityResult {
        /// AWS Account ID number of the account that owns or contains the calling entity.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// ARN associated with the calling entity.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Account ID number of the account that owns or contains the calling entity.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Unique identifier of the calling entity.
        pub user_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCallerIdentityArgs,
    ) -> GetCallerIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:index/getCallerIdentity:getCallerIdentity".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCallerIdentityResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            user_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("userId")),
        }
    }
}
