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
///               Sid:
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
pub mod license_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LicenseAssociationArgs {
        /// A token from Grafana Labs that ties your AWS account with a Grafana Labs account.
        #[builder(into, default)]
        pub grafana_token: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of license for the workspace license association. Valid values are `ENTERPRISE` and `ENTERPRISE_FREE_TRIAL`.
        #[builder(into)]
        pub license_type: pulumi_wasm_rust::Output<String>,
        /// The workspace id.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LicenseAssociationResult {
        /// If `license_type` is set to `ENTERPRISE_FREE_TRIAL`, this is the expiration date of the free trial.
        pub free_trial_expiration: pulumi_wasm_rust::Output<String>,
        /// A token from Grafana Labs that ties your AWS account with a Grafana Labs account.
        pub grafana_token: pulumi_wasm_rust::Output<Option<String>>,
        /// If `license_type` is set to `ENTERPRISE`, this is the expiration date of the enterprise license.
        pub license_expiration: pulumi_wasm_rust::Output<String>,
        /// The type of license for the workspace license association. Valid values are `ENTERPRISE` and `ENTERPRISE_FREE_TRIAL`.
        pub license_type: pulumi_wasm_rust::Output<String>,
        /// The workspace id.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LicenseAssociationArgs) -> LicenseAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let grafana_token_binding = args.grafana_token.get_inner();
        let license_type_binding = args.license_type.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:grafana/licenseAssociation:LicenseAssociation".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "freeTrialExpiration".into(),
                },
                register_interface::ResultField {
                    name: "grafanaToken".into(),
                },
                register_interface::ResultField {
                    name: "licenseExpiration".into(),
                },
                register_interface::ResultField {
                    name: "licenseType".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LicenseAssociationResult {
            free_trial_expiration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("freeTrialExpiration").unwrap(),
            ),
            grafana_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grafanaToken").unwrap(),
            ),
            license_expiration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseExpiration").unwrap(),
            ),
            license_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseType").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}