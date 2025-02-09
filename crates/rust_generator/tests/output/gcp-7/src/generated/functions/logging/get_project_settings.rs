#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_project_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProjectSettingsArgs {
        /// The ID of the project for which to retrieve settings.
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetProjectSettingsResult {
        /// If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed.
        pub disable_default_sink: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The resource name for the configured Cloud KMS key.
        /// KMS key name format:
        /// `'projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]'`
        /// To enable CMEK for the bucket, set this field to a valid kmsKeyName for which the associated service account has the required cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.
        /// The Cloud KMS key used by the bucket can be updated by changing the kmsKeyName to a new valid key name. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.
        /// See [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information.
        pub kms_key_name: pulumi_gestalt_rust::Output<String>,
        /// The service account associated with a project for which CMEK will apply.
        /// Before enabling CMEK for a logging bucket, you must first assign the cloudkms.cryptoKeyEncrypterDecrypter role to the service account associated with the project for which CMEK will apply. See [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information.
        pub kms_service_account_id: pulumi_gestalt_rust::Output<String>,
        /// The service account for the given container. Sinks use this service account as their writerIdentity if no custom service account is provided.
        pub logging_service_account_id: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the settings.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The storage location that Cloud Logging will use to create new resources when a location is needed but not explicitly provided.
        pub storage_location: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetProjectSettingsArgs,
    ) -> GetProjectSettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:logging/getProjectSettings:getProjectSettings".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetProjectSettingsResult {
            disable_default_sink: o.get_field("disableDefaultSink"),
            id: o.get_field("id"),
            kms_key_name: o.get_field("kmsKeyName"),
            kms_service_account_id: o.get_field("kmsServiceAccountId"),
            logging_service_account_id: o.get_field("loggingServiceAccountId"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            storage_location: o.get_field("storageLocation"),
        }
    }
}
