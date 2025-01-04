/// Default resource settings control whether CMEK is required for new log buckets. These settings also determine the storage location for the _Default and _Required log buckets, and whether the _Default sink is enabled or disabled.
///
///
/// To get more information about OrganizationSettings, see:
///
/// * [API documentation](https://cloud.google.com/logging/docs/reference/v2/rest/v2/TopLevel/getSettings)
/// * How-to Guides
///     * [Configure default settings for organizations and folders](https://cloud.google.com/logging/docs/default-settings)
///
/// ## Example Usage
///
/// ### Logging Organization Settings All
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:logging:OrganizationSettings
///     properties:
///       disableDefaultSink: true
///       kmsKeyName: kms-key
///       organization: '123456789'
///       storageLocation: us-central1
///     options:
///       dependsOn:
///         - ${iam}
///   iam:
///     type: gcp:kms:CryptoKeyIAMMember
///     properties:
///       cryptoKeyId: kms-key
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:${settings.kmsServiceAccountId}
/// variables:
///   settings:
///     fn::invoke:
///       function: gcp:logging:getOrganizationSettings
///       arguments:
///         organization: '123456789'
/// ```
///
/// ## Import
///
/// OrganizationSettings can be imported using any of these accepted formats:
///
/// * `organizations/{{organization}}/settings`
///
/// * `{{organization}}`
///
/// When using the `pulumi import` command, OrganizationSettings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/organizationSettings:OrganizationSettings default organizations/{{organization}}/settings
/// ```
///
/// ```sh
/// $ pulumi import gcp:logging/organizationSettings:OrganizationSettings default {{organization}}
/// ```
///
pub mod organization_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationSettingsArgs {
        /// If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed.
        #[builder(into, default)]
        pub disable_default_sink: pulumi_wasm_rust::Output<Option<bool>>,
        /// The resource name for the configured Cloud KMS key.
        #[builder(into, default)]
        pub kms_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The organization for which to retrieve or configure settings.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub organization: pulumi_wasm_rust::Output<String>,
        /// The storage location that Cloud Logging will use to create new resources when a location is needed but not explicitly provided.
        #[builder(into, default)]
        pub storage_location: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationSettingsResult {
        /// If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed.
        pub disable_default_sink: pulumi_wasm_rust::Output<bool>,
        /// The resource name for the configured Cloud KMS key.
        pub kms_key_name: pulumi_wasm_rust::Output<String>,
        /// The service account that will be used by the Log Router to access your Cloud KMS key.
        pub kms_service_account_id: pulumi_wasm_rust::Output<String>,
        /// The service account for the given container. Sinks use this service account as their writerIdentity if no custom service account is provided.
        pub logging_service_account_id: pulumi_wasm_rust::Output<String>,
        /// The resource name of the settings.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The organization for which to retrieve or configure settings.
        ///
        ///
        /// - - -
        pub organization: pulumi_wasm_rust::Output<String>,
        /// The storage location that Cloud Logging will use to create new resources when a location is needed but not explicitly provided.
        pub storage_location: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrganizationSettingsArgs,
    ) -> OrganizationSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disable_default_sink_binding = args.disable_default_sink.get_inner();
        let kms_key_name_binding = args.kms_key_name.get_inner();
        let organization_binding = args.organization.get_inner();
        let storage_location_binding = args.storage_location.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/organizationSettings:OrganizationSettings".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "disableDefaultSink".into(),
                    value: &disable_default_sink_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyName".into(),
                    value: &kms_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "organization".into(),
                    value: &organization_binding,
                },
                register_interface::ObjectField {
                    name: "storageLocation".into(),
                    value: &storage_location_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "disableDefaultSink".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyName".into(),
                },
                register_interface::ResultField {
                    name: "kmsServiceAccountId".into(),
                },
                register_interface::ResultField {
                    name: "loggingServiceAccountId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "organization".into(),
                },
                register_interface::ResultField {
                    name: "storageLocation".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationSettingsResult {
            disable_default_sink: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableDefaultSink").unwrap(),
            ),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyName").unwrap(),
            ),
            kms_service_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsServiceAccountId").unwrap(),
            ),
            logging_service_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingServiceAccountId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            organization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organization").unwrap(),
            ),
            storage_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageLocation").unwrap(),
            ),
        }
    }
}
