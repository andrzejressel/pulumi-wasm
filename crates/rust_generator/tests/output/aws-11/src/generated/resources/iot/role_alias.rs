/// Provides an IoT role alias.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   role:
///     type: aws:iam:Role
///     properties:
///       name: dynamodb-access-role
///       assumeRolePolicy: ${assumeRole.json}
///   alias:
///     type: aws:iot:RoleAlias
///     properties:
///       alias: Thermostat-dynamodb-access-role-alias
///       roleArn: ${role.arn}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         effect: Allow
///         principals:
///           - type: Service
///             identifiers:
///               - credentials.iot.amazonaws.com
///         actions:
///           - sts:AssumeRole
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IOT Role Alias using the alias. For example:
///
/// ```sh
/// $ pulumi import aws:iot/roleAlias:RoleAlias example myalias
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod role_alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoleAliasArgs {
        /// The name of the role alias.
        #[builder(into)]
        pub alias: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The duration of the credential, in seconds. If you do not specify a value for this setting, the default maximum of one hour is applied. This setting can have a value from 900 seconds (15 minutes) to 43200 seconds (12 hours).
        #[builder(into, default)]
        pub credential_duration: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The identity of the role to which the alias refers.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RoleAliasResult {
        /// The name of the role alias.
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// The ARN assigned by AWS to this role alias.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The duration of the credential, in seconds. If you do not specify a value for this setting, the default maximum of one hour is applied. This setting can have a value from 900 seconds (15 minutes) to 43200 seconds (12 hours).
        pub credential_duration: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The identity of the role to which the alias refers.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RoleAliasArgs,
    ) -> RoleAliasResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let alias_binding_1 = args.alias.get_output(context);
        let alias_binding = alias_binding_1.get_inner();
        let credential_duration_binding_1 = args.credential_duration.get_output(context);
        let credential_duration_binding = credential_duration_binding_1.get_inner();
        let role_arn_binding_1 = args.role_arn.get_output(context);
        let role_arn_binding = role_arn_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/roleAlias:RoleAlias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "credentialDuration".into(),
                    value: &credential_duration_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RoleAliasResult {
            alias: pulumi_gestalt_rust::__private::into_domain(o.extract_field("alias")),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            credential_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("credentialDuration"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
