#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_permission_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPermissionSetArgs {
        /// ARN of the permission set.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the SSO Instance associated with the permission set.
        #[builder(into)]
        pub instance_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the SSO Permission Set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPermissionSetResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the Permission Set.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_arn: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Relay state URL used to redirect users within the application during the federation authentication process.
        pub relay_state: pulumi_gestalt_rust::Output<String>,
        /// Length of time that the application user sessions are valid in the ISO-8601 standard.
        pub session_duration: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPermissionSetArgs,
    ) -> GetPermissionSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let instance_arn_binding = args.instance_arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssoadmin/getPermissionSet:getPermissionSet".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPermissionSetResult {
            arn: o.get_field("arn"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            instance_arn: o.get_field("instanceArn"),
            name: o.get_field("name"),
            relay_state: o.get_field("relayState"),
            session_duration: o.get_field("sessionDuration"),
            tags: o.get_field("tags"),
        }
    }
}
