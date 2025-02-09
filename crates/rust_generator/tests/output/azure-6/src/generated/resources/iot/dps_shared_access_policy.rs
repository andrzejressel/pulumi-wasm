/// Manages an IotHub Device Provisioning Service Shared Access Policy
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleIotHubDps:
///     type: azure:iot:IotHubDps
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: S1
///         capacity: '1'
///   exampleDpsSharedAccessPolicy:
///     type: azure:iot:DpsSharedAccessPolicy
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       iothubDpsName: ${exampleIotHubDps.name}
///       enrollmentWrite: true
///       enrollmentRead: true
/// ```
///
/// ## Import
///
/// IoTHub Device Provisioning Service Shared Access Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/dpsSharedAccessPolicy:DpsSharedAccessPolicy shared_access_policy1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/provisioningServices/dps1/keys/shared_access_policy1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dps_shared_access_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DpsSharedAccessPolicyArgs {
        /// Adds `EnrollmentRead` permission to this Shared Access Account. It allows read access to enrollment data.
        ///
        /// > **NOTE** When `enrollment_read` is set to `true`, `registration_read` must also be set to true. This is a limitation of the Azure REST API
        #[builder(into, default)]
        pub enrollment_read: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Adds `EnrollmentWrite` permission to this Shared Access Account. It allows write access to enrollment data.
        ///
        /// > **NOTE** When `registration_write` is set to `true`, `enrollment_read`, `registration_read`, and `registration_write` must also be set to true. This is a requirement of the Azure API.
        #[builder(into, default)]
        pub enrollment_write: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the IoT Hub Device Provisioning service to which this Shared Access Policy belongs. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_dps_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the IotHub Shared Access Policy resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Adds `RegistrationStatusRead` permission to this Shared Access Account. It allows read access to device registrations.
        #[builder(into, default)]
        pub registration_read: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Adds `RegistrationStatusWrite` permission to this Shared Access Account. It allows write access to device registrations.
        ///
        /// > **NOTE** When `registration_write` is set to `true`, `registration_read` must also be set to true. This is a requirement of the Azure API.
        #[builder(into, default)]
        pub registration_write: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the resource group under which the IotHub Shared Access Policy resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Adds `ServiceConfig` permission to this Shared Access Account. It allows configuration of the Device Provisioning Service.
        ///
        /// > **NOTE** At least one of `registration_read`, `registration_write`, `service_config`, `enrollment_read`, `enrollment_write` permissions must be set to `true`.
        #[builder(into, default)]
        pub service_config: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct DpsSharedAccessPolicyResult {
        /// Adds `EnrollmentRead` permission to this Shared Access Account. It allows read access to enrollment data.
        ///
        /// > **NOTE** When `enrollment_read` is set to `true`, `registration_read` must also be set to true. This is a limitation of the Azure REST API
        pub enrollment_read: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Adds `EnrollmentWrite` permission to this Shared Access Account. It allows write access to enrollment data.
        ///
        /// > **NOTE** When `registration_write` is set to `true`, `enrollment_read`, `registration_read`, and `registration_write` must also be set to true. This is a requirement of the Azure API.
        pub enrollment_write: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the IoT Hub Device Provisioning service to which this Shared Access Policy belongs. Changing this forces a new resource to be created.
        pub iothub_dps_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the IotHub Shared Access Policy resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string of the Shared Access Policy.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The primary key used to create the authentication token.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// Adds `RegistrationStatusRead` permission to this Shared Access Account. It allows read access to device registrations.
        pub registration_read: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Adds `RegistrationStatusWrite` permission to this Shared Access Account. It allows write access to device registrations.
        ///
        /// > **NOTE** When `registration_write` is set to `true`, `registration_read` must also be set to true. This is a requirement of the Azure API.
        pub registration_write: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group under which the IotHub Shared Access Policy resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string of the Shared Access Policy.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The secondary key used to create the authentication token.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// Adds `ServiceConfig` permission to this Shared Access Account. It allows configuration of the Device Provisioning Service.
        ///
        /// > **NOTE** At least one of `registration_read`, `registration_write`, `service_config`, `enrollment_read`, `enrollment_write` permissions must be set to `true`.
        pub service_config: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DpsSharedAccessPolicyArgs,
    ) -> DpsSharedAccessPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let enrollment_read_binding_1 = args.enrollment_read.get_output(context);
        let enrollment_read_binding = enrollment_read_binding_1.get_inner();
        let enrollment_write_binding_1 = args.enrollment_write.get_output(context);
        let enrollment_write_binding = enrollment_write_binding_1.get_inner();
        let iothub_dps_name_binding_1 = args.iothub_dps_name.get_output(context);
        let iothub_dps_name_binding = iothub_dps_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let registration_read_binding_1 = args.registration_read.get_output(context);
        let registration_read_binding = registration_read_binding_1.get_inner();
        let registration_write_binding_1 = args.registration_write.get_output(context);
        let registration_write_binding = registration_write_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let service_config_binding_1 = args.service_config.get_output(context);
        let service_config_binding = service_config_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/dpsSharedAccessPolicy:DpsSharedAccessPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enrollmentRead".into(),
                    value: &enrollment_read_binding,
                },
                register_interface::ObjectField {
                    name: "enrollmentWrite".into(),
                    value: &enrollment_write_binding,
                },
                register_interface::ObjectField {
                    name: "iothubDpsName".into(),
                    value: &iothub_dps_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "registrationRead".into(),
                    value: &registration_read_binding,
                },
                register_interface::ObjectField {
                    name: "registrationWrite".into(),
                    value: &registration_write_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceConfig".into(),
                    value: &service_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DpsSharedAccessPolicyResult {
            enrollment_read: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enrollmentRead"),
            ),
            enrollment_write: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enrollmentWrite"),
            ),
            iothub_dps_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iothubDpsName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            primary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryConnectionString"),
            ),
            primary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryKey"),
            ),
            registration_read: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registrationRead"),
            ),
            registration_write: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registrationWrite"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryConnectionString"),
            ),
            secondary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryKey"),
            ),
            service_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceConfig"),
            ),
        }
    }
}
