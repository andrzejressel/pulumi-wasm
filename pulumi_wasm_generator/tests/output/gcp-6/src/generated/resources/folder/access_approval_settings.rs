/// Access Approval enables you to require your explicit approval whenever Google support and engineering need to access your customer content.
///
///
/// To get more information about FolderSettings, see:
///
/// * [API documentation](https://cloud.google.com/access-approval/docs/reference/rest/v1/folders)
///
/// ## Example Usage
///
/// ### Folder Access Approval Full
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let folderAccessApproval = access_approval_settings::create(
///         "folderAccessApproval",
///         AccessApprovalSettingsArgs::builder()
///             .enrolled_services(
///                 vec![
///                     AccessApprovalSettingsEnrolledService::builder().cloudProduct("all")
///                     .build_struct(),
///                 ],
///             )
///             .folder_id("${myFolder.folderId}")
///             .notification_emails(
///                 vec!["testuser@example.com", "example.user@example.com",],
///             )
///             .build_struct(),
///     );
///     let myFolder = folder::create(
///         "myFolder",
///         FolderArgs::builder()
///             .deletion_protection(false)
///             .display_name("my-folder")
///             .parent("organizations/123456789")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Folder Access Approval Active Key Version
///
///
/// ```yaml
/// resources:
///   myFolder:
///     type: gcp:organizations:Folder
///     name: my_folder
///     properties:
///       displayName: my-folder
///       parent: organizations/123456789
///       deletionProtection: false
///   myProject:
///     type: gcp:organizations:Project
///     name: my_project
///     properties:
///       name: My Project
///       projectId: your-project-id
///       folderId: ${myFolder.name}
///       deletionPolicy: DELETE
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: key-ring
///       location: global
///       project: ${myProject.projectId}
///   cryptoKey:
///     type: gcp:kms:CryptoKey
///     name: crypto_key
///     properties:
///       name: crypto-key
///       keyRing: ${keyRing.id}
///       purpose: ASYMMETRIC_SIGN
///       versionTemplate:
///         algorithm: EC_SIGN_P384_SHA384
///   iam:
///     type: gcp:kms:CryptoKeyIAMMember
///     properties:
///       cryptoKeyId: ${cryptoKey.id}
///       role: roles/cloudkms.signerVerifier
///       member: serviceAccount:${serviceAccount.accountEmail}
///   folderAccessApproval:
///     type: gcp:folder:AccessApprovalSettings
///     name: folder_access_approval
///     properties:
///       folderId: ${myFolder.folderId}
///       activeKeyVersion: ${cryptoKeyVersion.name}
///       enrolledServices:
///         - cloudProduct: all
///     options:
///       dependsOn:
///         - ${iam}
/// variables:
///   serviceAccount:
///     fn::invoke:
///       function: gcp:accessapproval:getFolderServiceAccount
///       arguments:
///         folderId: ${myFolder.folderId}
///   cryptoKeyVersion:
///     fn::invoke:
///       function: gcp:kms:getKMSCryptoKeyVersion
///       arguments:
///         cryptoKey: ${cryptoKey.id}
/// ```
///
/// ## Import
///
/// FolderSettings can be imported using any of these accepted formats:
///
/// * `folders/{{folder_id}}/accessApprovalSettings`
///
/// * `{{folder_id}}`
///
/// When using the `pulumi import` command, FolderSettings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:folder/accessApprovalSettings:AccessApprovalSettings default folders/{{folder_id}}/accessApprovalSettings
/// ```
///
/// ```sh
/// $ pulumi import gcp:folder/accessApprovalSettings:AccessApprovalSettings default {{folder_id}}
/// ```
///
pub mod access_approval_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessApprovalSettingsArgs {
        /// The asymmetric crypto key version to use for signing approval requests. Empty active_key_version indicates that a
        /// Google-managed key should be used for signing. This property will be ignored if set by an ancestor of the resource, and
        /// new non-empty values may not be set.
        #[builder(into, default)]
        pub active_key_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of Google Cloud Services for which the given resource has Access Approval enrolled.
        /// Access requests for the resource given by name against any of these services contained here will be required
        /// to have explicit approval. Enrollment can only be done on an all or nothing basis.
        /// A maximum of 10 enrolled services will be enforced, to be expanded as the set of supported services is expanded.
        /// Structure is documented below.
        #[builder(into)]
        pub enrolled_services: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::folder::AccessApprovalSettingsEnrolledService>,
        >,
        /// ID of the folder of the access approval settings.
        #[builder(into)]
        pub folder_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of email addresses to which notifications relating to approval requests should be sent. Notifications relating to
        /// a resource will be sent to all emails in the settings of ancestor resources of that resource. A maximum of 50 email
        /// addresses are allowed.
        #[builder(into, default)]
        pub notification_emails: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct AccessApprovalSettingsResult {
        /// The asymmetric crypto key version to use for signing approval requests. Empty active_key_version indicates that a
        /// Google-managed key should be used for signing. This property will be ignored if set by an ancestor of the resource, and
        /// new non-empty values may not be set.
        pub active_key_version: pulumi_wasm_rust::Output<Option<String>>,
        /// If the field is true, that indicates that an ancestor of this Folder has set active_key_version.
        pub ancestor_has_active_key_version: pulumi_wasm_rust::Output<bool>,
        /// If the field is true, that indicates that at least one service is enrolled for Access Approval in one or more ancestors of the Folder.
        pub enrolled_ancestor: pulumi_wasm_rust::Output<bool>,
        /// A list of Google Cloud Services for which the given resource has Access Approval enrolled.
        /// Access requests for the resource given by name against any of these services contained here will be required
        /// to have explicit approval. Enrollment can only be done on an all or nothing basis.
        /// A maximum of 10 enrolled services will be enforced, to be expanded as the set of supported services is expanded.
        /// Structure is documented below.
        pub enrolled_services: pulumi_wasm_rust::Output<
            Vec<super::super::types::folder::AccessApprovalSettingsEnrolledService>,
        >,
        /// ID of the folder of the access approval settings.
        pub folder_id: pulumi_wasm_rust::Output<String>,
        /// If the field is true, that indicates that there is some configuration issue with the active_key_version
        /// configured on this Folder (e.g. it doesn't exist or the Access Approval service account doesn't have the
        /// correct permissions on it, etc.) This key version is not necessarily the effective key version at this level,
        /// as key versions are inherited top-down.
        pub invalid_key_version: pulumi_wasm_rust::Output<bool>,
        /// The resource name of the settings. Format is "folders/{folder_id}/accessApprovalSettings"
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of email addresses to which notifications relating to approval requests should be sent. Notifications relating to
        /// a resource will be sent to all emails in the settings of ancestor resources of that resource. A maximum of 50 email
        /// addresses are allowed.
        pub notification_emails: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccessApprovalSettingsArgs,
    ) -> AccessApprovalSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let active_key_version_binding = args
            .active_key_version
            .get_output(context)
            .get_inner();
        let enrolled_services_binding = args
            .enrolled_services
            .get_output(context)
            .get_inner();
        let folder_id_binding = args.folder_id.get_output(context).get_inner();
        let notification_emails_binding = args
            .notification_emails
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:folder/accessApprovalSettings:AccessApprovalSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activeKeyVersion".into(),
                    value: &active_key_version_binding,
                },
                register_interface::ObjectField {
                    name: "enrolledServices".into(),
                    value: &enrolled_services_binding,
                },
                register_interface::ObjectField {
                    name: "folderId".into(),
                    value: &folder_id_binding,
                },
                register_interface::ObjectField {
                    name: "notificationEmails".into(),
                    value: &notification_emails_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccessApprovalSettingsResult {
            active_key_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("activeKeyVersion"),
            ),
            ancestor_has_active_key_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ancestorHasActiveKeyVersion"),
            ),
            enrolled_ancestor: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enrolledAncestor"),
            ),
            enrolled_services: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enrolledServices"),
            ),
            folder_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("folderId"),
            ),
            invalid_key_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("invalidKeyVersion"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            notification_emails: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notificationEmails"),
            ),
        }
    }
}
