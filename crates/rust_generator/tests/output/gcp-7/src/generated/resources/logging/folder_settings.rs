/// Default resource settings control whether CMEK is required for new log buckets. These settings also determine the storage location for the _Default and _Required log buckets, and whether the _Default sink is enabled or disabled.
///
///
/// To get more information about FolderSettings, see:
///
/// * [API documentation](https://cloud.google.com/logging/docs/reference/v2/rest/v2/TopLevel/getSettings)
/// * How-to Guides
///     * [Configure default settings for organizations and folders](https://cloud.google.com/logging/docs/default-settings)
///
/// ## Example Usage
///
/// ### Logging Folder Settings All
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:logging:FolderSettings
///     properties:
///       disableDefaultSink: true
///       folder: ${myFolder.folderId}
///       kmsKeyName: kms-key
///       storageLocation: us-central1
///     options:
///       dependsOn:
///         - ${iam}
///   myFolder:
///     type: gcp:organizations:Folder
///     name: my_folder
///     properties:
///       displayName: folder-name
///       parent: organizations/123456789
///       deletionProtection: false
///   iam:
///     type: gcp:kms:CryptoKeyIAMMember
///     properties:
///       cryptoKeyId: kms-key
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:${settings.kmsServiceAccountId}
/// variables:
///   settings:
///     fn::invoke:
///       function: gcp:logging:getFolderSettings
///       arguments:
///         folder: ${myFolder.folderId}
/// ```
///
/// ## Import
///
/// FolderSettings can be imported using any of these accepted formats:
///
/// * `folders/{{folder}}/settings`
///
/// * `{{folder}}`
///
/// When using the `pulumi import` command, FolderSettings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/folderSettings:FolderSettings default folders/{{folder}}/settings
/// ```
///
/// ```sh
/// $ pulumi import gcp:logging/folderSettings:FolderSettings default {{folder}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod folder_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FolderSettingsArgs {
        /// If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed.
        #[builder(into, default)]
        pub disable_default_sink: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The folder for which to retrieve settings.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name for the configured Cloud KMS key.
        #[builder(into, default)]
        pub kms_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The storage location that Cloud Logging will use to create new resources when a location is needed but not explicitly provided.
        #[builder(into, default)]
        pub storage_location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FolderSettingsResult {
        /// If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed.
        pub disable_default_sink: pulumi_gestalt_rust::Output<bool>,
        /// The folder for which to retrieve settings.
        ///
        ///
        /// - - -
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// The resource name for the configured Cloud KMS key.
        pub kms_key_name: pulumi_gestalt_rust::Output<String>,
        /// The service account that will be used by the Log Router to access your Cloud KMS key.
        pub kms_service_account_id: pulumi_gestalt_rust::Output<String>,
        /// The service account for the given container. Sinks use this service account as their writerIdentity if no custom service account is provided.
        pub logging_service_account_id: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the settings.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The storage location that Cloud Logging will use to create new resources when a location is needed but not explicitly provided.
        pub storage_location: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FolderSettingsArgs,
    ) -> FolderSettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let disable_default_sink_binding = args
            .disable_default_sink
            .get_output(context)
            .get_inner();
        let folder_binding = args.folder.get_output(context).get_inner();
        let kms_key_name_binding = args.kms_key_name.get_output(context).get_inner();
        let storage_location_binding = args
            .storage_location
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/folderSettings:FolderSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "disableDefaultSink".into(),
                    value: &disable_default_sink_binding,
                },
                register_interface::ObjectField {
                    name: "folder".into(),
                    value: &folder_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyName".into(),
                    value: &kms_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageLocation".into(),
                    value: &storage_location_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FolderSettingsResult {
            disable_default_sink: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disableDefaultSink"),
            ),
            folder: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folder"),
            ),
            kms_key_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyName"),
            ),
            kms_service_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsServiceAccountId"),
            ),
            logging_service_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingServiceAccountId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            storage_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageLocation"),
            ),
        }
    }
}
