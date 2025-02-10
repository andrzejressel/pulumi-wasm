/// Provides an Amazon Managed Grafana workspace SAML configuration resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:grafana:WorkspaceSamlConfiguration
///     properties:
///       editorRoleValues:
///         - editor
///       idpMetadataUrl: https://my_idp_metadata.url
///       workspaceId: ${exampleWorkspace.id}
///   exampleWorkspace:
///     type: aws:grafana:Workspace
///     name: example
///     properties:
///       accountAccessType: CURRENT_ACCOUNT
///       authenticationProviders:
///         - SAML
///       permissionType: SERVICE_MANAGED
///       roleArn: ${assume.arn}
///   assume:
///     type: aws:iam:Role
///     properties:
///       name: grafana-assume
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid: ""
///               Principal:
///                 Service: grafana.amazonaws.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Grafana Workspace SAML configuration using the workspace's `id`. For example:
///
/// ```sh
/// $ pulumi import aws:grafana/workspaceSamlConfiguration:WorkspaceSamlConfiguration example g-2054c75a02
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace_saml_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceSamlConfigurationArgs {
        /// The admin role values.
        #[builder(into, default)]
        pub admin_role_values: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The allowed organizations.
        #[builder(into, default)]
        pub allowed_organizations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The editor role values.
        #[builder(into)]
        pub editor_role_values: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The email assertion.
        #[builder(into, default)]
        pub email_assertion: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The groups assertion.
        #[builder(into, default)]
        pub groups_assertion: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IDP Metadata URL. Note that either `idp_metadata_url` or `idp_metadata_xml` (but not both) must be specified.
        #[builder(into, default)]
        pub idp_metadata_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IDP Metadata XML. Note that either `idp_metadata_url` or `idp_metadata_xml` (but not both) must be specified.
        #[builder(into, default)]
        pub idp_metadata_xml: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The login assertion.
        #[builder(into, default)]
        pub login_assertion: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The login validity duration.
        #[builder(into, default)]
        pub login_validity_duration: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name assertion.
        #[builder(into, default)]
        pub name_assertion: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The org assertion.
        #[builder(into, default)]
        pub org_assertion: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role assertion.
        #[builder(into, default)]
        pub role_assertion: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The workspace id.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceSamlConfigurationResult {
        /// The admin role values.
        pub admin_role_values: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The allowed organizations.
        pub allowed_organizations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The editor role values.
        pub editor_role_values: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The email assertion.
        pub email_assertion: pulumi_gestalt_rust::Output<String>,
        /// The groups assertion.
        pub groups_assertion: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IDP Metadata URL. Note that either `idp_metadata_url` or `idp_metadata_xml` (but not both) must be specified.
        pub idp_metadata_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IDP Metadata XML. Note that either `idp_metadata_url` or `idp_metadata_xml` (but not both) must be specified.
        pub idp_metadata_xml: pulumi_gestalt_rust::Output<Option<String>>,
        /// The login assertion.
        pub login_assertion: pulumi_gestalt_rust::Output<String>,
        /// The login validity duration.
        pub login_validity_duration: pulumi_gestalt_rust::Output<i32>,
        /// The name assertion.
        pub name_assertion: pulumi_gestalt_rust::Output<String>,
        /// The org assertion.
        pub org_assertion: pulumi_gestalt_rust::Output<Option<String>>,
        /// The role assertion.
        pub role_assertion: pulumi_gestalt_rust::Output<Option<String>>,
        /// The status of the SAML configuration.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The workspace id.
        ///
        /// The following arguments are optional:
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceSamlConfigurationArgs,
    ) -> WorkspaceSamlConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let admin_role_values_binding = args.admin_role_values.get_output(context);
        let allowed_organizations_binding = args
            .allowed_organizations
            .get_output(context);
        let editor_role_values_binding = args.editor_role_values.get_output(context);
        let email_assertion_binding = args.email_assertion.get_output(context);
        let groups_assertion_binding = args.groups_assertion.get_output(context);
        let idp_metadata_url_binding = args.idp_metadata_url.get_output(context);
        let idp_metadata_xml_binding = args.idp_metadata_xml.get_output(context);
        let login_assertion_binding = args.login_assertion.get_output(context);
        let login_validity_duration_binding = args
            .login_validity_duration
            .get_output(context);
        let name_assertion_binding = args.name_assertion.get_output(context);
        let org_assertion_binding = args.org_assertion.get_output(context);
        let role_assertion_binding = args.role_assertion.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:grafana/workspaceSamlConfiguration:WorkspaceSamlConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminRoleValues".into(),
                    value: admin_role_values_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedOrganizations".into(),
                    value: allowed_organizations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "editorRoleValues".into(),
                    value: editor_role_values_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailAssertion".into(),
                    value: email_assertion_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupsAssertion".into(),
                    value: groups_assertion_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idpMetadataUrl".into(),
                    value: idp_metadata_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idpMetadataXml".into(),
                    value: idp_metadata_xml_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loginAssertion".into(),
                    value: login_assertion_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loginValidityDuration".into(),
                    value: login_validity_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nameAssertion".into(),
                    value: name_assertion_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgAssertion".into(),
                    value: org_assertion_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleAssertion".into(),
                    value: role_assertion_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceSamlConfigurationResult {
            admin_role_values: o.get_field("adminRoleValues"),
            allowed_organizations: o.get_field("allowedOrganizations"),
            editor_role_values: o.get_field("editorRoleValues"),
            email_assertion: o.get_field("emailAssertion"),
            groups_assertion: o.get_field("groupsAssertion"),
            idp_metadata_url: o.get_field("idpMetadataUrl"),
            idp_metadata_xml: o.get_field("idpMetadataXml"),
            login_assertion: o.get_field("loginAssertion"),
            login_validity_duration: o.get_field("loginValidityDuration"),
            name_assertion: o.get_field("nameAssertion"),
            org_assertion: o.get_field("orgAssertion"),
            role_assertion: o.get_field("roleAssertion"),
            status: o.get_field("status"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
