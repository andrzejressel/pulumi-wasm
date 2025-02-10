/// ## Example Usage
///
/// ### Kms Autokey Config All
///
///
/// ```yaml
/// resources:
///   # Create Folder in GCP Organization
///   autokmsFolder:
///     type: gcp:organizations:Folder
///     name: autokms_folder
///     properties:
///       displayName: my-folder
///       parent: organizations/123456789
///       deletionProtection: false
///   # Create the key project
///   keyProject:
///     type: gcp:organizations:Project
///     name: key_project
///     properties:
///       projectId: key-proj
///       name: key-proj
///       folderId: ${autokmsFolder.folderId}
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///     options:
///       dependsOn:
///         - ${autokmsFolder}
///   # Enable the Cloud KMS API
///   kmsApiService:
///     type: gcp:projects:Service
///     name: kms_api_service
///     properties:
///       service: cloudkms.googleapis.com
///       project: ${keyProject.projectId}
///       disableOnDestroy: false
///       disableDependentServices: true
///     options:
///       dependsOn:
///         - ${keyProject}
///   # Wait delay after enabling APIs
///   waitEnableServiceApi:
///     type: time:sleep
///     name: wait_enable_service_api
///     properties:
///       createDuration: 30s
///     options:
///       dependsOn:
///         - ${kmsApiService}
///   #Create KMS Service Agent
///   kmsServiceAgent:
///     type: gcp:projects:ServiceIdentity
///     name: kms_service_agent
///     properties:
///       service: cloudkms.googleapis.com
///       project: ${keyProject.number}
///     options:
///       dependsOn:
///         - ${waitEnableServiceApi}
///   # Wait delay after creating service agent.
///   waitServiceAgent:
///     type: time:sleep
///     name: wait_service_agent
///     properties:
///       createDuration: 10s
///     options:
///       dependsOn:
///         - ${kmsServiceAgent}
///   #Grant the KMS Service Agent the Cloud KMS Admin role
///   autokeyProjectAdmin:
///     type: gcp:projects:IAMMember
///     name: autokey_project_admin
///     properties:
///       project: ${keyProject.projectId}
///       role: roles/cloudkms.admin
///       member: serviceAccount:service-${keyProject.number}@gcp-sa-cloudkms.iam.gserviceaccount.com
///     options:
///       dependsOn:
///         - ${waitServiceAgent}
///   # Wait delay after granting IAM permissions
///   waitSrvAccPermissions:
///     type: time:sleep
///     name: wait_srv_acc_permissions
///     properties:
///       createDuration: 10s
///     options:
///       dependsOn:
///         - ${autokeyProjectAdmin}
///   example-autokeyconfig:
///     type: gcp:kms:AutokeyConfig
///     properties:
///       folder: ${autokmsFolder.id}
///       keyProject: projects/${keyProject.projectId}
///     options:
///       dependsOn:
///         - ${waitSrvAccPermissions}
///   # Wait delay after setting AutokeyConfig, to prevent diffs on reapply,
///   # because setting the config takes a little to fully propagate.
///   waitAutokeyPropagation:
///     type: time:sleep
///     name: wait_autokey_propagation
///     properties:
///       createDuration: 30s
///     options:
///       dependsOn:
///         - ${["example-autokeyconfig"]}
/// ```
///
/// ## Import
///
/// AutokeyConfig can be imported using any of these accepted formats:
///
/// * `folders/{{folder}}/autokeyConfig`
///
/// * `{{folder}}`
///
/// When using the `pulumi import` command, AutokeyConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:kms/autokeyConfig:AutokeyConfig default folders/{{folder}}/autokeyConfig
/// ```
///
/// ```sh
/// $ pulumi import gcp:kms/autokeyConfig:AutokeyConfig default {{folder}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod autokey_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutokeyConfigArgs {
        /// The folder for which to retrieve config.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The target key project for a given folder where KMS Autokey will provision a
        /// CryptoKey for any new KeyHandle the Developer creates. Should have the form
        /// `projects/<project_id_or_number>`.
        #[builder(into, default)]
        pub key_project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AutokeyConfigResult {
        /// The folder for which to retrieve config.
        ///
        ///
        /// - - -
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// The target key project for a given folder where KMS Autokey will provision a
        /// CryptoKey for any new KeyHandle the Developer creates. Should have the form
        /// `projects/<project_id_or_number>`.
        pub key_project: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AutokeyConfigArgs,
    ) -> AutokeyConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let folder_binding = args.folder.get_output(context);
        let key_project_binding = args.key_project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:kms/autokeyConfig:AutokeyConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: folder_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyProject".into(),
                    value: key_project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AutokeyConfigResult {
            folder: o.get_field("folder"),
            key_project: o.get_field("keyProject"),
        }
    }
}
