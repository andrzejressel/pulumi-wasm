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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// The ID of the bundle for the WorkSpace.
        #[builder(into)]
        pub bundle_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the directory for the WorkSpace.
        #[builder(into)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether the data stored on the root volume is encrypted.
        #[builder(into, default)]
        pub root_volume_encryption_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The tags for the WorkSpace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The user name of the user for the WorkSpace. This user name must exist in the directory for the WorkSpace.
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether the data stored on the user volume is encrypted.
        #[builder(into, default)]
        pub user_volume_encryption_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ARN of a symmetric AWS KMS customer master key (CMK) used to encrypt data stored on your WorkSpace. Amazon WorkSpaces does not support asymmetric CMKs.
        #[builder(into, default)]
        pub volume_encryption_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The WorkSpace properties.
        #[builder(into, default)]
        pub workspace_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::workspaces::WorkspaceWorkspaceProperties>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// The ID of the bundle for the WorkSpace.
        pub bundle_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the WorkSpace, as seen by the operating system.
        pub computer_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the directory for the WorkSpace.
        pub directory_id: pulumi_gestalt_rust::Output<String>,
        /// The IP address of the WorkSpace.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the data stored on the root volume is encrypted.
        pub root_volume_encryption_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The operational state of the WorkSpace.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The tags for the WorkSpace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The user name of the user for the WorkSpace. This user name must exist in the directory for the WorkSpace.
        pub user_name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the data stored on the user volume is encrypted.
        pub user_volume_encryption_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ARN of a symmetric AWS KMS customer master key (CMK) used to encrypt data stored on your WorkSpace. Amazon WorkSpaces does not support asymmetric CMKs.
        pub volume_encryption_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The WorkSpace properties.
        pub workspace_properties: pulumi_gestalt_rust::Output<
            super::super::types::workspaces::WorkspaceWorkspaceProperties,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceArgs,
    ) -> WorkspaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bundle_id_binding = args.bundle_id.get_output(context);
        let directory_id_binding = args.directory_id.get_output(context);
        let root_volume_encryption_enabled_binding = args
            .root_volume_encryption_enabled
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let user_volume_encryption_enabled_binding = args
            .user_volume_encryption_enabled
            .get_output(context);
        let volume_encryption_key_binding = args
            .volume_encryption_key
            .get_output(context);
        let workspace_properties_binding = args.workspace_properties.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:workspaces/workspace:Workspace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bundleId".into(),
                    value: bundle_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryId".into(),
                    value: directory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rootVolumeEncryptionEnabled".into(),
                    value: root_volume_encryption_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: user_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userVolumeEncryptionEnabled".into(),
                    value: user_volume_encryption_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeEncryptionKey".into(),
                    value: volume_encryption_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceProperties".into(),
                    value: workspace_properties_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceResult {
            bundle_id: o.get_field("bundleId"),
            computer_name: o.get_field("computerName"),
            directory_id: o.get_field("directoryId"),
            ip_address: o.get_field("ipAddress"),
            root_volume_encryption_enabled: o.get_field("rootVolumeEncryptionEnabled"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_name: o.get_field("userName"),
            user_volume_encryption_enabled: o.get_field("userVolumeEncryptionEnabled"),
            volume_encryption_key: o.get_field("volumeEncryptionKey"),
            workspace_properties: o.get_field("workspaceProperties"),
        }
    }
}
