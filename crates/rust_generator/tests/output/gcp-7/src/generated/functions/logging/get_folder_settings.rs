pub mod get_folder_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFolderSettingsArgs {
        /// The ID of the folder for which to retrieve settings.
        #[builder(into)]
        pub folder: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFolderSettingsResult {
        /// If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed.
        pub disable_default_sink: pulumi_wasm_rust::Output<bool>,
        pub folder: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The resource name for the configured Cloud KMS key.
        /// KMS key name format:
        /// `'projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]'`
        /// To enable CMEK for the bucket, set this field to a valid kmsKeyName for which the associated service account has the required cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.
        /// The Cloud KMS key used by the bucket can be updated by changing the kmsKeyName to a new valid key name. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.
        /// See [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information.
        pub kms_key_name: pulumi_wasm_rust::Output<String>,
        /// The service account associated with a project for which CMEK will apply.
        /// Before enabling CMEK for a logging bucket, you must first assign the cloudkms.cryptoKeyEncrypterDecrypter role to the service account associated with the project for which CMEK will apply. See [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information.
        pub kms_service_account_id: pulumi_wasm_rust::Output<String>,
        /// The service account for the given container. Sinks use this service account as their writerIdentity if no custom service account is provided.
        pub logging_service_account_id: pulumi_wasm_rust::Output<String>,
        /// The resource name of the settings.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The storage location that Cloud Logging will use to create new resources when a location is needed but not explicitly provided.
        pub storage_location: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetFolderSettingsArgs,
    ) -> GetFolderSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let folder_binding = args.folder.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:logging/getFolderSettings:getFolderSettings".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "folder".into(),
                    value: &folder_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFolderSettingsResult {
            disable_default_sink: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disableDefaultSink"),
            ),
            folder: pulumi_wasm_rust::__private::into_domain(o.extract_field("folder")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyName"),
            ),
            kms_service_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsServiceAccountId"),
            ),
            logging_service_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loggingServiceAccountId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            storage_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageLocation"),
            ),
        }
    }
}
