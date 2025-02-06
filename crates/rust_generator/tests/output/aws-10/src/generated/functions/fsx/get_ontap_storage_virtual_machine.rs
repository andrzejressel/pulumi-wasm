pub mod get_ontap_storage_virtual_machine {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOntapStorageVirtualMachineArgs {
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::fsx::GetOntapStorageVirtualMachineFilter>,
            >,
        >,
        /// Identifier of the storage virtual machine (e.g. `svm-12345678`).
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetOntapStorageVirtualMachineResult {
        /// The Microsoft Active Directory configuration to which the SVM is joined, if applicable. See Active Directory Configuration below.
        pub active_directory_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::fsx::GetOntapStorageVirtualMachineActiveDirectoryConfiguration,
            >,
        >,
        /// Amazon Resource Name of the SVM.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The time that the SVM was created.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// The endpoints that are used to access data or to manage the SVM using the NetApp ONTAP CLI, REST API, or NetApp CloudManager. They are the Iscsi, Management, Nfs, and Smb endpoints. See SVM Endpoints below.
        pub endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::fsx::GetOntapStorageVirtualMachineEndpoint>,
        >,
        /// Identifier of the file system (e.g. `fs-12345678`).
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::fsx::GetOntapStorageVirtualMachineFilter>,
            >,
        >,
        /// The SVM's system generated unique ID.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The SVM's lifecycle status.
        pub lifecycle_status: pulumi_wasm_rust::Output<String>,
        /// Describes why the SVM lifecycle state changed. See Lifecycle Transition Reason below.
        pub lifecycle_transition_reasons: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::fsx::GetOntapStorageVirtualMachineLifecycleTransitionReason,
            >,
        >,
        /// The name of the SVM, if provisioned.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The SVM's subtype.
        pub subtype: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The SVM's UUID.
        pub uuid: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOntapStorageVirtualMachineArgs,
    ) -> GetOntapStorageVirtualMachineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:fsx/getOntapStorageVirtualMachine:getOntapStorageVirtualMachine"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOntapStorageVirtualMachineResult {
            active_directory_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("activeDirectoryConfigurations"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTime"),
            ),
            endpoints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoints"),
            ),
            file_system_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fileSystemId"),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            lifecycle_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lifecycleStatus"),
            ),
            lifecycle_transition_reasons: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lifecycleTransitionReasons"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            subtype: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subtype"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            uuid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uuid")),
        }
    }
}
