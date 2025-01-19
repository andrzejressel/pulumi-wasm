/// A regional secret version resource.
///
///
///
///
///
/// ## Example Usage
///
/// ### Regional Secret Version Basic
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: secret-version
///       location: us-central1
///   regionalSecretVersionBasic:
///     type: gcp:secretmanager:RegionalSecretVersion
///     name: regional_secret_version_basic
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
/// ```
/// ### Regional Secret Version With Base64 Data
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: secret-version
///       location: us-central1
///   regionalSecretVersionBase64:
///     type: gcp:secretmanager:RegionalSecretVersion
///     name: regional_secret_version_base64
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: secret-data.pfx
///           return: result
///       isSecretDataBase64: true
/// ```
/// ### Regional Secret Version Disabled
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: secret-version
///       location: us-central1
///   regionalSecretVersionDisabled:
///     type: gcp:secretmanager:RegionalSecretVersion
///     name: regional_secret_version_disabled
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
///       enabled: false
/// ```
/// ### Regional Secret Version Deletion Policy Abandon
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: secret-version
///       location: us-central1
///   regionalSecretVersionDeletionPolicy:
///     type: gcp:secretmanager:RegionalSecretVersion
///     name: regional_secret_version_deletion_policy
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
///       deletionPolicy: ABANDON
/// ```
/// ### Regional Secret Version Deletion Policy Disable
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: secret-version
///       location: us-central1
///   regionalSecretVersionDeletionPolicy:
///     type: gcp:secretmanager:RegionalSecretVersion
///     name: regional_secret_version_deletion_policy
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
///       deletionPolicy: DISABLE
/// ```
///
/// ## Import
///
/// RegionalSecretVersion can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}/versions/{{version}}`
///
/// When using the `pulumi import` command, RegionalSecretVersion can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:secretmanager/regionalSecretVersion:RegionalSecretVersion default projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}/versions/{{version}}
/// ```
///
pub mod regional_secret_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionalSecretVersionArgs {
        /// The deletion policy for the regional secret version. Setting `ABANDON` allows the resource
        /// to be abandoned rather than deleted. Setting `DISABLE` allows the resource to be
        /// disabled rather than deleted. Default is `DELETE`. Possible values are:
        /// * DELETE
        /// * DISABLE
        /// * ABANDON
        #[builder(into, default)]
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The current state of the regional secret version.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// If set to 'true', the secret data is expected to be base64-encoded string and would be sent as is.
        #[builder(into, default)]
        pub is_secret_data_base64: pulumi_wasm_rust::Output<Option<bool>>,
        /// Secret Manager regional secret resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub secret: pulumi_wasm_rust::Output<String>,
        /// The secret data. Must be no larger than 64KiB.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into)]
        pub secret_data: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RegionalSecretVersionResult {
        /// The time at which the regional secret version was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The customer-managed encryption configuration of the regional secret.
        /// Structure is documented below.
        pub customer_managed_encryptions: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::secretmanager::RegionalSecretVersionCustomerManagedEncryption,
            >,
        >,
        /// The deletion policy for the regional secret version. Setting `ABANDON` allows the resource
        /// to be abandoned rather than deleted. Setting `DISABLE` allows the resource to be
        /// disabled rather than deleted. Default is `DELETE`. Possible values are:
        /// * DELETE
        /// * DISABLE
        /// * ABANDON
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The time at which the regional secret version was destroyed. Only present if state is DESTROYED.
        pub destroy_time: pulumi_wasm_rust::Output<String>,
        /// The current state of the regional secret version.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// If set to 'true', the secret data is expected to be base64-encoded string and would be sent as is.
        pub is_secret_data_base64: pulumi_wasm_rust::Output<Option<bool>>,
        /// Location of Secret Manager regional secret resource.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the regional secret version. Format:
        /// `projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}/versions/{{version}}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// Secret Manager regional secret resource.
        ///
        ///
        /// - - -
        pub secret: pulumi_wasm_rust::Output<String>,
        /// The secret data. Must be no larger than 64KiB.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub secret_data: pulumi_wasm_rust::Output<String>,
        /// The version of the Regional Secret.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RegionalSecretVersionArgs,
    ) -> RegionalSecretVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deletion_policy_binding = args.deletion_policy.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let is_secret_data_base64_binding = args.is_secret_data_base64.get_inner();
        let secret_binding = args.secret.get_inner();
        let secret_data_binding = args.secret_data.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:secretmanager/regionalSecretVersion:RegionalSecretVersion"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "isSecretDataBase64".into(),
                    value: &is_secret_data_base64_binding,
                },
                register_interface::ObjectField {
                    name: "secret".into(),
                    value: &secret_binding,
                },
                register_interface::ObjectField {
                    name: "secretData".into(),
                    value: &secret_data_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedEncryptions".into(),
                },
                register_interface::ResultField {
                    name: "deletionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "destroyTime".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "isSecretDataBase64".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "secret".into(),
                },
                register_interface::ResultField {
                    name: "secretData".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegionalSecretVersionResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            customer_managed_encryptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedEncryptions").unwrap(),
            ),
            deletion_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionPolicy").unwrap(),
            ),
            destroy_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destroyTime").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            is_secret_data_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isSecretDataBase64").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secret").unwrap(),
            ),
            secret_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretData").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
