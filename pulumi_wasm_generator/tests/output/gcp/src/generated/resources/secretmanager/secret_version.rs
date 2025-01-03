/// A secret version resource.
///
///
///
///
///
/// ## Example Usage
///
/// ### Secret Version Basic
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret-version
///       labels:
///         label: my-label
///       replication:
///         auto: {}
///   secret-version-basic:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
/// ```
/// ### Secret Version Deletion Policy Abandon
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret-version
///       replication:
///         userManaged:
///           replicas:
///             - location: us-central1
///   secret-version-deletion-policy:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
///       deletionPolicy: ABANDON
/// ```
/// ### Secret Version Deletion Policy Disable
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret-version
///       replication:
///         userManaged:
///           replicas:
///             - location: us-central1
///   secret-version-deletion-policy:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
///       deletionPolicy: DISABLE
/// ```
/// ### Secret Version With Base64 String Secret Data
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret-version
///       replication:
///         userManaged:
///           replicas:
///             - location: us-central1
///   secret-version-base64:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["secret-basic"].id}
///       isSecretDataBase64: true
///       secretData:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: secret-data.pfx
///           return: result
/// ```
///
/// ## Import
///
/// SecretVersion can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/secrets/{{secret_id}}/versions/{{version}}`
///
/// When using the `pulumi import` command, SecretVersion can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:secretmanager/secretVersion:SecretVersion default projects/{{project}}/secrets/{{secret_id}}/versions/{{version}}
/// ```
///
pub mod secret_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretVersionArgs {
        /// The deletion policy for the secret version. Setting `ABANDON` allows the resource
        /// to be abandoned rather than deleted. Setting `DISABLE` allows the resource to be
        /// disabled rather than deleted. Default is `DELETE`. Possible values are:
        /// * DELETE
        /// * DISABLE
        /// * ABANDON
        #[builder(into, default)]
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The current state of the SecretVersion.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// If set to 'true', the secret data is expected to be base64-encoded string and would be sent as is.
        #[builder(into, default)]
        pub is_secret_data_base64: pulumi_wasm_rust::Output<Option<bool>>,
        /// Secret Manager secret resource
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
    pub struct SecretVersionResult {
        /// The time at which the Secret was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The deletion policy for the secret version. Setting `ABANDON` allows the resource
        /// to be abandoned rather than deleted. Setting `DISABLE` allows the resource to be
        /// disabled rather than deleted. Default is `DELETE`. Possible values are:
        /// * DELETE
        /// * DISABLE
        /// * ABANDON
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The time at which the Secret was destroyed. Only present if state is DESTROYED.
        pub destroy_time: pulumi_wasm_rust::Output<String>,
        /// The current state of the SecretVersion.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// If set to 'true', the secret data is expected to be base64-encoded string and would be sent as is.
        pub is_secret_data_base64: pulumi_wasm_rust::Output<Option<bool>>,
        /// The resource name of the SecretVersion. Format:
        /// `projects/{{project}}/secrets/{{secret_id}}/versions/{{version}}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// Secret Manager secret resource
        ///
        ///
        /// - - -
        pub secret: pulumi_wasm_rust::Output<String>,
        /// The secret data. Must be no larger than 64KiB.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub secret_data: pulumi_wasm_rust::Output<String>,
        /// The version of the Secret.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SecretVersionArgs) -> SecretVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deletion_policy_binding = args.deletion_policy.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let is_secret_data_base64_binding = args.is_secret_data_base64.get_inner();
        let secret_binding = args.secret.get_inner();
        let secret_data_binding = args.secret_data.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:secretmanager/secretVersion:SecretVersion".into(),
            name: name.to_string(),
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
        SecretVersionResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
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
