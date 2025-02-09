#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_access_keys {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccessKeysArgs {
        /// Name of the IAM user associated with the access keys.
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccessKeysResult {
        /// List of the IAM access keys associated with the specified user. See below.
        pub access_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::iam::GetAccessKeysAccessKey>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub user: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAccessKeysArgs,
    ) -> GetAccessKeysResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let user_binding_1 = args.user.get_output(context);
        let user_binding = user_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getAccessKeys:getAccessKeys".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "user".into(),
                    value: &user_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAccessKeysResult {
            access_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessKeys"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            user: pulumi_gestalt_rust::__private::into_domain(o.extract_field("user")),
        }
    }
}
