pub mod get_organization {
    #[allow(dead_code)]
    pub struct GetOrganizationResult {
        /// List of organization accounts including the master account. For a list excluding the master account, see the `non_master_accounts` attribute. All elements have these attributes:
        pub accounts: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::organizations::GetOrganizationAccount>,
        >,
        /// ARN of the root
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A list of AWS service principal names that have integration enabled with your organization. Organization must have `feature_set` set to `ALL`. For additional information, see the [AWS Organizations User Guide](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html).
        pub aws_service_access_principals: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of Organizations policy types that are enabled in the Organization Root. Organization must have `feature_set` set to `ALL`. For additional information about valid policy types (e.g., `SERVICE_CONTROL_POLICY`), see the [AWS Organizations API Reference](https://docs.aws.amazon.com/organizations/latest/APIReference/API_EnablePolicyType.html).
        pub enabled_policy_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// FeatureSet of the organization.
        pub feature_set: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN of the account that is designated as the master account for the organization.
        pub master_account_arn: pulumi_wasm_rust::Output<String>,
        /// The email address that is associated with the AWS account that is designated as the master account for the organization.
        pub master_account_email: pulumi_wasm_rust::Output<String>,
        /// Unique identifier (ID) of the master account of an organization.
        pub master_account_id: pulumi_wasm_rust::Output<String>,
        /// Name of the master account of an organization.
        pub master_account_name: pulumi_wasm_rust::Output<String>,
        /// List of organization accounts excluding the master account. For a list including the master account, see the `accounts` attribute. All elements have these attributes:
        pub non_master_accounts: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::organizations::GetOrganizationNonMasterAccount,
            >,
        >,
        /// List of organization roots. All elements have these attributes:
        pub roots: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::organizations::GetOrganizationRoot>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_wasm_rust::PulumiContext) -> GetOrganizationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:organizations/getOrganization:getOrganization".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accounts".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsServiceAccessPrincipals".into(),
                },
                register_interface::ResultField {
                    name: "enabledPolicyTypes".into(),
                },
                register_interface::ResultField {
                    name: "featureSet".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "masterAccountArn".into(),
                },
                register_interface::ResultField {
                    name: "masterAccountEmail".into(),
                },
                register_interface::ResultField {
                    name: "masterAccountId".into(),
                },
                register_interface::ResultField {
                    name: "masterAccountName".into(),
                },
                register_interface::ResultField {
                    name: "nonMasterAccounts".into(),
                },
                register_interface::ResultField {
                    name: "roots".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOrganizationResult {
            accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accounts").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_service_access_principals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsServiceAccessPrincipals").unwrap(),
            ),
            enabled_policy_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledPolicyTypes").unwrap(),
            ),
            feature_set: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("featureSet").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            master_account_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterAccountArn").unwrap(),
            ),
            master_account_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterAccountEmail").unwrap(),
            ),
            master_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterAccountId").unwrap(),
            ),
            master_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterAccountName").unwrap(),
            ),
            non_master_accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nonMasterAccounts").unwrap(),
            ),
            roots: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roots").unwrap(),
            ),
        }
    }
}
