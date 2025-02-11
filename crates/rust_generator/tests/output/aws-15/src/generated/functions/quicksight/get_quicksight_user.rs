#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_quicksight_user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQuicksightUserArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// QuickSight namespace. Defaults to `default`.
        #[builder(into, default)]
        pub namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the user that you want to match.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetQuicksightUserResult {
        /// The active status of user. When you create an Amazon QuickSight user that’s not an IAM user or an Active Directory user, that user is inactive until they sign in and provide a password.
        pub active: pulumi_gestalt_rust::Output<bool>,
        /// The Amazon Resource Name (ARN) for the user.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The user's email address.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The type of identity authentication used by the user.
        pub identity_type: pulumi_gestalt_rust::Output<String>,
        pub namespace: pulumi_gestalt_rust::Output<Option<String>>,
        /// The principal ID of the user.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        pub user_name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon QuickSight role for the user. The user role can be one of the following:.
        /// - `READER`: A user who has read-only access to dashboards.
        /// - `AUTHOR`: A user who can create data sources, datasets, analyzes, and dashboards.
        /// - `ADMIN`: A user who is an author, who can also manage Amazon QuickSight settings.
        pub user_role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetQuicksightUserArgs,
    ) -> GetQuicksightUserResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aws_account_id_binding = args.aws_account_id.get_output(context);
        let namespace_binding = args.namespace.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:quicksight/getQuicksightUser:getQuicksightUser".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: &user_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetQuicksightUserResult {
            active: o.get_field("active"),
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            email: o.get_field("email"),
            id: o.get_field("id"),
            identity_type: o.get_field("identityType"),
            namespace: o.get_field("namespace"),
            principal_id: o.get_field("principalId"),
            user_name: o.get_field("userName"),
            user_role: o.get_field("userRole"),
        }
    }
}
