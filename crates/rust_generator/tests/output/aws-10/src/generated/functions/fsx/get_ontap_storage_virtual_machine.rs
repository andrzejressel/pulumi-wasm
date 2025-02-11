#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_ontap_storage_virtual_machine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOntapStorageVirtualMachineArgs {
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::fsx::GetOntapStorageVirtualMachineFilter>,
            >,
        >,
        /// Identifier of the storage virtual machine (e.g. `svm-12345678`).
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetOntapStorageVirtualMachineResult {
        /// The Microsoft Active Directory configuration to which the SVM is joined, if applicable. See Active Directory Configuration below.
        pub active_directory_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::fsx::GetOntapStorageVirtualMachineActiveDirectoryConfiguration,
            >,
        >,
        /// Amazon Resource Name of the SVM.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The time that the SVM was created.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// The endpoints that are used to access data or to manage the SVM using the NetApp ONTAP CLI, REST API, or NetApp CloudManager. They are the Iscsi, Management, Nfs, and Smb endpoints. See SVM Endpoints below.
        pub endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::fsx::GetOntapStorageVirtualMachineEndpoint>,
        >,
        /// Identifier of the file system (e.g. `fs-12345678`).
        pub file_system_id: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::fsx::GetOntapStorageVirtualMachineFilter>,
            >,
        >,
        /// The SVM's system generated unique ID.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The SVM's lifecycle status.
        pub lifecycle_status: pulumi_gestalt_rust::Output<String>,
        /// Describes why the SVM lifecycle state changed. See Lifecycle Transition Reason below.
        pub lifecycle_transition_reasons: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::fsx::GetOntapStorageVirtualMachineLifecycleTransitionReason,
            >,
        >,
        /// The name of the SVM, if provisioned.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The SVM's subtype.
        pub subtype: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The SVM's UUID.
        pub uuid: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOntapStorageVirtualMachineArgs,
    ) -> GetOntapStorageVirtualMachineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:fsx/getOntapStorageVirtualMachine:getOntapStorageVirtualMachine"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOntapStorageVirtualMachineResult {
            active_directory_configurations: o
                .get_field("activeDirectoryConfigurations"),
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            endpoints: o.get_field("endpoints"),
            file_system_id: o.get_field("fileSystemId"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            lifecycle_status: o.get_field("lifecycleStatus"),
            lifecycle_transition_reasons: o.get_field("lifecycleTransitionReasons"),
            name: o.get_field("name"),
            subtype: o.get_field("subtype"),
            tags: o.get_field("tags"),
            uuid: o.get_field("uuid"),
        }
    }
}
