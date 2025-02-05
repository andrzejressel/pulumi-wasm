pub mod get_vcenter_credentials {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVcenterCredentialsArgs {
        /// The resource name of the private cloud which contains the Vcenter.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVcenterCredentialsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The password of the Vcenter Credential.
        pub password: pulumi_wasm_rust::Output<String>,
        /// The username of the Vcenter Credential.
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVcenterCredentialsArgs,
    ) -> GetVcenterCredentialsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_binding = args.parent.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getVcenterCredentials:getVcenterCredentials".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVcenterCredentialsResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("username"),
            ),
        }
    }
}
