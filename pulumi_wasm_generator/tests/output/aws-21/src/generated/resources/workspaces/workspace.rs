/// Provides a workspace in [AWS Workspaces](https://docs.aws.amazon.com/workspaces/latest/adminguide/amazon-workspaces.html) Service
///
/// > **NOTE:** AWS WorkSpaces service requires [`workspaces_DefaultRole`](https://docs.aws.amazon.com/workspaces/latest/adminguide/workspaces-access-control.html#create-default-role) IAM role to operate normally.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:workspaces:Workspace
///     properties:
///       directoryId: ${exampleAwsWorkspacesDirectory.id}
///       bundleId: ${valueWindows10.id}
///       userName: john.doe
///       rootVolumeEncryptionEnabled: true
///       userVolumeEncryptionEnabled: true
///       volumeEncryptionKey: ${workspaces.arn}
///       workspaceProperties:
///         computeTypeName: VALUE
///         userVolumeSizeGib: 10
///         rootVolumeSizeGib: 80
///         runningMode: AUTO_STOP
///         runningModeAutoStopTimeoutInMinutes: 60
///       tags:
///         Department: IT
/// variables:
///   valueWindows10:
///     fn::invoke:
///       function: aws:workspaces:getBundle
///       arguments:
///         bundleId: wsb-bh8rsxt14
///   workspaces:
///     fn::invoke:
///       function: aws:kms:getKey
///       arguments:
///         keyId: alias/aws/workspaces
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Workspaces using their ID. For example:
///
/// ```sh
/// $ pulumi import aws:workspaces/workspace:Workspace example ws-9z9zmbkhv
/// ```
pub mod workspace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// The ID of the bundle for the WorkSpace.
        #[builder(into)]
        pub bundle_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the directory for the WorkSpace.
        #[builder(into)]
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the data stored on the root volume is encrypted.
        #[builder(into, default)]
        pub root_volume_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The tags for the WorkSpace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The user name of the user for the WorkSpace. This user name must exist in the directory for the WorkSpace.
        #[builder(into)]
        pub user_name: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the data stored on the user volume is encrypted.
        #[builder(into, default)]
        pub user_volume_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ARN of a symmetric AWS KMS customer master key (CMK) used to encrypt data stored on your WorkSpace. Amazon WorkSpaces does not support asymmetric CMKs.
        #[builder(into, default)]
        pub volume_encryption_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The WorkSpace properties.
        #[builder(into, default)]
        pub workspace_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::workspaces::WorkspaceWorkspaceProperties>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// The ID of the bundle for the WorkSpace.
        pub bundle_id: pulumi_wasm_rust::Output<String>,
        /// The name of the WorkSpace, as seen by the operating system.
        pub computer_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the directory for the WorkSpace.
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// The IP address of the WorkSpace.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the data stored on the root volume is encrypted.
        pub root_volume_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The operational state of the WorkSpace.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The tags for the WorkSpace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The user name of the user for the WorkSpace. This user name must exist in the directory for the WorkSpace.
        pub user_name: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the data stored on the user volume is encrypted.
        pub user_volume_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ARN of a symmetric AWS KMS customer master key (CMK) used to encrypt data stored on your WorkSpace. Amazon WorkSpaces does not support asymmetric CMKs.
        pub volume_encryption_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The WorkSpace properties.
        pub workspace_properties: pulumi_wasm_rust::Output<
            super::super::types::workspaces::WorkspaceWorkspaceProperties,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkspaceArgs) -> WorkspaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bundle_id_binding = args.bundle_id.get_inner();
        let directory_id_binding = args.directory_id.get_inner();
        let root_volume_encryption_enabled_binding = args
            .root_volume_encryption_enabled
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let user_name_binding = args.user_name.get_inner();
        let user_volume_encryption_enabled_binding = args
            .user_volume_encryption_enabled
            .get_inner();
        let volume_encryption_key_binding = args.volume_encryption_key.get_inner();
        let workspace_properties_binding = args.workspace_properties.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:workspaces/workspace:Workspace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bundleId".into(),
                    value: &bundle_id_binding,
                },
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "rootVolumeEncryptionEnabled".into(),
                    value: &root_volume_encryption_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
                register_interface::ObjectField {
                    name: "userVolumeEncryptionEnabled".into(),
                    value: &user_volume_encryption_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "volumeEncryptionKey".into(),
                    value: &volume_encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceProperties".into(),
                    value: &workspace_properties_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bundleId".into(),
                },
                register_interface::ResultField {
                    name: "computerName".into(),
                },
                register_interface::ResultField {
                    name: "directoryId".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "rootVolumeEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
                },
                register_interface::ResultField {
                    name: "userVolumeEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "volumeEncryptionKey".into(),
                },
                register_interface::ResultField {
                    name: "workspaceProperties".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkspaceResult {
            bundle_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bundleId").unwrap(),
            ),
            computer_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computerName").unwrap(),
            ),
            directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryId").unwrap(),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            root_volume_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootVolumeEncryptionEnabled").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
            user_volume_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userVolumeEncryptionEnabled").unwrap(),
            ),
            volume_encryption_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeEncryptionKey").unwrap(),
            ),
            workspace_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceProperties").unwrap(),
            ),
        }
    }
}
