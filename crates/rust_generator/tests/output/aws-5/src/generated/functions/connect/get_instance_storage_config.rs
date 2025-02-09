#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance_storage_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceStorageConfigArgs {
        /// The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.
        #[builder(into)]
        pub association_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A valid resource type. Valid Values: `AGENT_EVENTS` | `ATTACHMENTS` | `CALL_RECORDINGS` | `CHAT_TRANSCRIPTS` | `CONTACT_EVALUATIONS` | `CONTACT_TRACE_RECORDS` | `MEDIA_STREAMS` | `REAL_TIME_CONTACT_ANALYSIS_SEGMENTS` | `SCHEDULED_REPORTS` |  `SCREEN_RECORDINGS`.
        #[builder(into)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceStorageConfigResult {
        pub association_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the storage configuration options for the Connect Instance. Documented below.
        pub storage_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::connect::GetInstanceStorageConfigStorageConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInstanceStorageConfigArgs,
    ) -> GetInstanceStorageConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let association_id_binding_1 = args.association_id.get_output(context);
        let association_id_binding = association_id_binding_1.get_inner();
        let instance_id_binding_1 = args.instance_id.get_output(context);
        let instance_id_binding = instance_id_binding_1.get_inner();
        let resource_type_binding_1 = args.resource_type.get_output(context);
        let resource_type_binding = resource_type_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getInstanceStorageConfig:getInstanceStorageConfig"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "associationId".into(),
                    value: &association_id_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInstanceStorageConfigResult {
            association_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("associationId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            resource_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceType"),
            ),
            storage_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageConfigs"),
            ),
        }
    }
}
