/// Provides an Elastic MapReduce Studio Session Mapping.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = studio_session_mapping::create(
///         "example",
///         StudioSessionMappingArgs::builder()
///             .identity_id("example")
///             .identity_type("USER")
///             .session_policy_arn("${exampleAwsIamPolicy.arn}")
///             .studio_id("${exampleAwsEmrStudio.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EMR studio session mappings using `studio-id:identity-type:identity-id`. For example:
///
/// ```sh
/// $ pulumi import aws:emr/studioSessionMapping:StudioSessionMapping example es-xxxxx:USER:xxxxx-xxx-xxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod studio_session_mapping {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StudioSessionMappingArgs {
        /// The globally unique identifier (GUID) of the user or group from the Amazon Web Services SSO Identity Store.
        #[builder(into, default)]
        pub identity_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the user or group from the Amazon Web Services SSO Identity Store.
        #[builder(into, default)]
        pub identity_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the identity to map to the Amazon EMR Studio is a `USER` or a `GROUP`.
        #[builder(into)]
        pub identity_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) for the session policy that will be applied to the user or group. You should specify the ARN for the session policy that you want to apply, not the ARN of your user role.
        #[builder(into)]
        pub session_policy_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Amazon EMR Studio to which the user or group will be mapped.
        #[builder(into)]
        pub studio_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StudioSessionMappingResult {
        /// The globally unique identifier (GUID) of the user or group from the Amazon Web Services SSO Identity Store.
        pub identity_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the user or group from the Amazon Web Services SSO Identity Store.
        pub identity_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the identity to map to the Amazon EMR Studio is a `USER` or a `GROUP`.
        pub identity_type: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the session policy that will be applied to the user or group. You should specify the ARN for the session policy that you want to apply, not the ARN of your user role.
        pub session_policy_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Amazon EMR Studio to which the user or group will be mapped.
        pub studio_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StudioSessionMappingArgs,
    ) -> StudioSessionMappingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_id_binding = args.identity_id.get_output(context);
        let identity_name_binding = args.identity_name.get_output(context);
        let identity_type_binding = args.identity_type.get_output(context);
        let session_policy_arn_binding = args.session_policy_arn.get_output(context);
        let studio_id_binding = args.studio_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:emr/studioSessionMapping:StudioSessionMapping".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityId".into(),
                    value: &identity_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityName".into(),
                    value: &identity_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityType".into(),
                    value: &identity_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionPolicyArn".into(),
                    value: &session_policy_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "studioId".into(),
                    value: &studio_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        StudioSessionMappingResult {
            identity_id: o.get_field("identityId"),
            identity_name: o.get_field("identityName"),
            identity_type: o.get_field("identityType"),
            session_policy_arn: o.get_field("sessionPolicyArn"),
            studio_id: o.get_field("studioId"),
        }
    }
}
