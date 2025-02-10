/// An `Environment Group attachment` in Apigee.
///
///
/// To get more information about EnvgroupAttachment, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.envgroups.attachments/create)
/// * How-to Guides
///     * [Creating an environment](https://cloud.google.com/apigee/docs/api-platform/get-started/create-environment)
///
/// ## Example Usage
///
/// ## Import
///
/// EnvgroupAttachment can be imported using any of these accepted formats:
///
/// * `{{envgroup_id}}/attachments/{{name}}`
///
/// * `{{envgroup_id}}/{{name}}`
///
/// When using the `pulumi import` command, EnvgroupAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/envGroupAttachment:EnvGroupAttachment default {{envgroup_id}}/attachments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/envGroupAttachment:EnvGroupAttachment default {{envgroup_id}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod env_group_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvGroupAttachmentArgs {
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/envgroups/{{envgroup_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub envgroup_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the environment.
        #[builder(into)]
        pub environment: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EnvGroupAttachmentResult {
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/envgroups/{{envgroup_name}}`.
        ///
        ///
        /// - - -
        pub envgroup_id: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the environment.
        pub environment: pulumi_gestalt_rust::Output<String>,
        /// The name of the newly created  attachment (output parameter).
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvGroupAttachmentArgs,
    ) -> EnvGroupAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let envgroup_id_binding = args.envgroup_id.get_output(context);
        let environment_binding = args.environment.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/envGroupAttachment:EnvGroupAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "envgroupId".into(),
                    value: envgroup_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environment".into(),
                    value: environment_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvGroupAttachmentResult {
            envgroup_id: o.get_field("envgroupId"),
            environment: o.get_field("environment"),
            name: o.get_field("name"),
        }
    }
}
