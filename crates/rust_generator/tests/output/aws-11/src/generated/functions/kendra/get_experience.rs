#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_experience {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExperienceArgs {
        /// Identifier of the Experience.
        #[builder(into)]
        pub experience_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of the index that contains the Experience.
        #[builder(into)]
        pub index_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetExperienceResult {
        /// ARN of the Experience.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Block that specifies the configuration information for your Amazon Kendra Experience. This includes `content_source_configuration`, which specifies the data source IDs and/or FAQ IDs, and `user_identity_configuration`, which specifies the user or group information to grant access to your Amazon Kendra Experience. Documented below.
        pub configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kendra::GetExperienceConfiguration>,
        >,
        /// Unix datetime that the Experience was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Description of the Experience.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Shows the endpoint URLs for your Amazon Kendra Experiences. The URLs are unique and fully hosted by AWS. Documented below.
        pub endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kendra::GetExperienceEndpoint>,
        >,
        /// Reason your Amazon Kendra Experience could not properly process.
        pub error_message: pulumi_gestalt_rust::Output<String>,
        pub experience_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub index_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the Experience.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Shows the ARN of a role with permission to access `Query` API, `QuerySuggestions` API, `SubmitFeedback` API, and AWS SSO that stores your user and group information.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Current processing status of your Amazon Kendra Experience. When the status is `ACTIVE`, your Amazon Kendra Experience is ready to use. When the status is `FAILED`, the `error_message` field contains the reason that this failed.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Date and time that the Experience was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetExperienceArgs,
    ) -> GetExperienceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let experience_id_binding = args.experience_id.get_output(context);
        let index_id_binding = args.index_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:kendra/getExperience:getExperience".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "experienceId".into(),
                    value: experience_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexId".into(),
                    value: index_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetExperienceResult {
            arn: o.get_field("arn"),
            configurations: o.get_field("configurations"),
            created_at: o.get_field("createdAt"),
            description: o.get_field("description"),
            endpoints: o.get_field("endpoints"),
            error_message: o.get_field("errorMessage"),
            experience_id: o.get_field("experienceId"),
            id: o.get_field("id"),
            index_id: o.get_field("indexId"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
            status: o.get_field("status"),
            updated_at: o.get_field("updatedAt"),
        }
    }
}
