#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceProfileArgs {
        /// Friendly IAM instance profile name to match.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceProfileResult {
        /// ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// String representation of the date the instance profile was created.
        pub create_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Path to the instance profile.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// Role ARN associated with this instance profile.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Role ID associated with this instance profile.
        pub role_id: pulumi_gestalt_rust::Output<String>,
        /// Role name associated with this instance profile.
        pub role_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceProfileArgs,
    ) -> GetInstanceProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:iam/getInstanceProfile:getInstanceProfile".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceProfileResult {
            arn: o.get_field("arn"),
            create_date: o.get_field("createDate"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            path: o.get_field("path"),
            role_arn: o.get_field("roleArn"),
            role_id: o.get_field("roleId"),
            role_name: o.get_field("roleName"),
        }
    }
}
