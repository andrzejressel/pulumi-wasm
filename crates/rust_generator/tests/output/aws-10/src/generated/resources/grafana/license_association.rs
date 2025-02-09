/// Provides an Amazon Managed Grafana workspace license association resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:grafana:LicenseAssociation
///     properties:
///       licenseType: ENTERPRISE_FREE_TRIAL
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
/// Using `pulumi import`, import Grafana workspace license association using the workspace's `id`. For example:
///
/// ```sh
/// $ pulumi import aws:grafana/licenseAssociation:LicenseAssociation example g-2054c75a02
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod license_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LicenseAssociationArgs {
        /// A token from Grafana Labs that ties your AWS account with a Grafana Labs account.
        #[builder(into, default)]
        pub grafana_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of license for the workspace license association. Valid values are `ENTERPRISE` and `ENTERPRISE_FREE_TRIAL`.
        #[builder(into)]
        pub license_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The workspace id.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LicenseAssociationResult {
        /// If `license_type` is set to `ENTERPRISE_FREE_TRIAL`, this is the expiration date of the free trial.
        pub free_trial_expiration: pulumi_gestalt_rust::Output<String>,
        /// A token from Grafana Labs that ties your AWS account with a Grafana Labs account.
        pub grafana_token: pulumi_gestalt_rust::Output<Option<String>>,
        /// If `license_type` is set to `ENTERPRISE`, this is the expiration date of the enterprise license.
        pub license_expiration: pulumi_gestalt_rust::Output<String>,
        /// The type of license for the workspace license association. Valid values are `ENTERPRISE` and `ENTERPRISE_FREE_TRIAL`.
        pub license_type: pulumi_gestalt_rust::Output<String>,
        /// The workspace id.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LicenseAssociationArgs,
    ) -> LicenseAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let grafana_token_binding_1 = args.grafana_token.get_output(context);
        let grafana_token_binding = grafana_token_binding_1.get_inner();
        let license_type_binding_1 = args.license_type.get_output(context);
        let license_type_binding = license_type_binding_1.get_inner();
        let workspace_id_binding_1 = args.workspace_id.get_output(context);
        let workspace_id_binding = workspace_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:grafana/licenseAssociation:LicenseAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "grafanaToken".into(),
                    value: &grafana_token_binding,
                },
                register_interface::ObjectField {
                    name: "licenseType".into(),
                    value: &license_type_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LicenseAssociationResult {
            free_trial_expiration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("freeTrialExpiration"),
            ),
            grafana_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grafanaToken"),
            ),
            license_expiration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseExpiration"),
            ),
            license_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseType"),
            ),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}
