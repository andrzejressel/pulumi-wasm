#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance_profiles {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceProfilesArgs {
        /// IAM role name.
        #[builder(into)]
        pub role_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceProfilesResult {
        /// Set of ARNs of instance profiles.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of IAM instance profile names.
        pub names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of IAM instance profile paths.
        pub paths: pulumi_gestalt_rust::Output<Vec<String>>,
        pub role_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceProfilesArgs,
    ) -> GetInstanceProfilesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let role_name_binding = args.role_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:iam/getInstanceProfiles:getInstanceProfiles".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleName".into(),
                    value: role_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceProfilesResult {
            arns: o.get_field("arns"),
            id: o.get_field("id"),
            names: o.get_field("names"),
            paths: o.get_field("paths"),
            role_name: o.get_field("roleName"),
        }
    }
}
