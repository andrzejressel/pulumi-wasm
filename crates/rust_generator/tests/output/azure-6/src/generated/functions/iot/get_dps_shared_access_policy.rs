#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dps_shared_access_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDpsSharedAccessPolicyArgs {
        /// Specifies the name of the IoT Hub Device Provisioning service to which the Shared Access Policy belongs.
        #[builder(into)]
        pub iothub_dps_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the IotHub Shared Access Policy.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group under which the IotHub Shared Access Policy resource exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDpsSharedAccessPolicyResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub iothub_dps_name: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string of the Shared Access Policy.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The primary key used to create the authentication token.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string of the Shared Access Policy.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The secondary key used to create the authentication token.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDpsSharedAccessPolicyArgs,
    ) -> GetDpsSharedAccessPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let iothub_dps_name_binding = args.iothub_dps_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:iot/getDpsSharedAccessPolicy:getDpsSharedAccessPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iothubDpsName".into(),
                    value: iothub_dps_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDpsSharedAccessPolicyResult {
            id: o.get_field("id"),
            iothub_dps_name: o.get_field("iothubDpsName"),
            name: o.get_field("name"),
            primary_connection_string: o.get_field("primaryConnectionString"),
            primary_key: o.get_field("primaryKey"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_connection_string: o.get_field("secondaryConnectionString"),
            secondary_key: o.get_field("secondaryKey"),
        }
    }
}
