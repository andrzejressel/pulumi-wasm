/// Accepts a License Manager grant. This allows for sharing licenses with other aws accounts.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = license_grant_accepter::create(
///         "test",
///         LicenseGrantAccepterArgs::builder()
///             .grant_arn(
///                 "arn:aws:license-manager::123456789012:grant:g-1cf9fba4ba2f42dcab11c686c4b4d329",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_licensemanager_grant_accepter` using the grant arn. For example:
///
/// ```sh
/// $ pulumi import aws:licensemanager/licenseGrantAccepter:LicenseGrantAccepter test arn:aws:license-manager::123456789012:grant:g-1cf9fba4ba2f42dcab11c686c4b4d329
/// ```
pub mod license_grant_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LicenseGrantAccepterArgs {
        /// The ARN of the grant to accept.
        #[builder(into)]
        pub grant_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LicenseGrantAccepterResult {
        /// A list of the allowed operations for the grant.
        pub allowed_operations: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ARN of the grant to accept.
        pub grant_arn: pulumi_wasm_rust::Output<String>,
        /// The home region for the license.
        pub home_region: pulumi_wasm_rust::Output<String>,
        /// The ARN of the license for the grant.
        pub license_arn: pulumi_wasm_rust::Output<String>,
        /// The Name of the grant.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent ARN.
        pub parent_arn: pulumi_wasm_rust::Output<String>,
        /// The target account for the grant.
        pub principal: pulumi_wasm_rust::Output<String>,
        /// The grant status.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The grant version.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LicenseGrantAccepterArgs,
    ) -> LicenseGrantAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let grant_arn_binding = args.grant_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:licensemanager/licenseGrantAccepter:LicenseGrantAccepter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "grantArn".into(),
                    value: &grant_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowedOperations".into(),
                },
                register_interface::ResultField {
                    name: "grantArn".into(),
                },
                register_interface::ResultField {
                    name: "homeRegion".into(),
                },
                register_interface::ResultField {
                    name: "licenseArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parentArn".into(),
                },
                register_interface::ResultField {
                    name: "principal".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LicenseGrantAccepterResult {
            allowed_operations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedOperations").unwrap(),
            ),
            grant_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grantArn").unwrap(),
            ),
            home_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("homeRegion").unwrap(),
            ),
            license_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentArn").unwrap(),
            ),
            principal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principal").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
