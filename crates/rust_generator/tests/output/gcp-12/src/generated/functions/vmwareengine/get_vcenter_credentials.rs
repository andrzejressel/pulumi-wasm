#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vcenter_credentials {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVcenterCredentialsArgs {
        /// The resource name of the private cloud which contains the Vcenter.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVcenterCredentialsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// The password of the Vcenter Credential.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// The username of the Vcenter Credential.
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVcenterCredentialsArgs,
    ) -> GetVcenterCredentialsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:vmwareengine/getVcenterCredentials:getVcenterCredentials".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVcenterCredentialsResult {
            id: o.get_field("id"),
            parent: o.get_field("parent"),
            password: o.get_field("password"),
            username: o.get_field("username"),
        }
    }
}
