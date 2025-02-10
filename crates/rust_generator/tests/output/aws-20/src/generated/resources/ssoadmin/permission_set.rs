/// Provides a Single Sign-On (SSO) Permission Set resource
///
/// > **NOTE:** Updating this resource will automatically [Provision the Permission Set](https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_ProvisionPermissionSet.html) to apply the corresponding updates to all assigned accounts.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   examplePermissionSet:
///     type: aws:ssoadmin:PermissionSet
///     name: example
///     properties:
///       name: Example
///       description: An example
///       instanceArn: ${example.arns[0]}
///       relayState: https://s3.console.aws.amazon.com/s3/home?region=us-east-1#
///       sessionDuration: PT2H
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Permission Sets using the `arn` and `instance_arn` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/permissionSet:PermissionSet example arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod permission_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PermissionSetArgs {
        /// The description of the Permission Set.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        #[builder(into)]
        pub instance_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Permission Set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The relay state URL used to redirect users within the application during the federation authentication process.
        #[builder(into, default)]
        pub relay_state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The length of time that the application user sessions are valid in the ISO-8601 standard. Default: `PT1H`.
        #[builder(into, default)]
        pub session_duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PermissionSetResult {
        /// The Amazon Resource Name (ARN) of the Permission Set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date the Permission Set was created in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// The description of the Permission Set.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        pub instance_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Permission Set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The relay state URL used to redirect users within the application during the federation authentication process.
        pub relay_state: pulumi_gestalt_rust::Output<Option<String>>,
        /// The length of time that the application user sessions are valid in the ISO-8601 standard. Default: `PT1H`.
        pub session_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PermissionSetArgs,
    ) -> PermissionSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let instance_arn_binding = args.instance_arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let relay_state_binding = args.relay_state.get_output(context);
        let session_duration_binding = args.session_duration.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssoadmin/permissionSet:PermissionSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceArn".into(),
                    value: instance_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "relayState".into(),
                    value: relay_state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionDuration".into(),
                    value: session_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PermissionSetResult {
            arn: o.get_field("arn"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            instance_arn: o.get_field("instanceArn"),
            name: o.get_field("name"),
            relay_state: o.get_field("relayState"),
            session_duration: o.get_field("sessionDuration"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
