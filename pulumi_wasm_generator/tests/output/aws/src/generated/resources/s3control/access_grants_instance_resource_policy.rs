/// Provides a resource to manage an S3 Access Grants instance resource policy.
/// Use a resource policy to manage cross-account access to your S3 Access Grants instance.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_grants_instance::create(
///         "example",
///         AccessGrantsInstanceArgs::builder().build_struct(),
///     );
///     let exampleAccessGrantsInstanceResourcePolicy = access_grants_instance_resource_policy::create(
///         "exampleAccessGrantsInstanceResourcePolicy",
///         AccessGrantsInstanceResourcePolicyArgs::builder()
///             .policy(
///                 "{\n  \"Version\": \"2012-10-17\",\n  \"Id\": \"S3AccessGrantsPolicy\",\n  \"Statement\": [{\n    \"Sid\": \"AllowAccessToS3AccessGrants\",\n    \"Effect\": \"Allow\",\n    \"Principal\": {\n      \"AWS\": \"123456789456\"\n    },\n    \"Action\": [\n      \"s3:ListAccessGrants\",\n      \"s3:ListAccessGrantsLocations\",\n      \"s3:GetDataAccess\"\n    ],\n    \"Resource\": \"${example.accessGrantsInstanceArn}\"\n  }]\n}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Access Grants instance resource policies using the `account_id`. For example:
///
/// ```sh
/// $ pulumi import aws:s3control/accessGrantsInstanceResourcePolicy:AccessGrantsInstanceResourcePolicy example 123456789012
/// ```
pub mod access_grants_instance_resource_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessGrantsInstanceResourcePolicyArgs {
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The policy document.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AccessGrantsInstanceResourcePolicyResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The policy document.
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AccessGrantsInstanceResourcePolicyArgs,
    ) -> AccessGrantsInstanceResourcePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let policy_binding = args.policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/accessGrantsInstanceResourcePolicy:AccessGrantsInstanceResourcePolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessGrantsInstanceResourcePolicyResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
        }
    }
}