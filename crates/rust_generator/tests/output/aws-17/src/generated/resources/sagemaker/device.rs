/// Provides a SageMaker Device resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = device::create(
///         "example",
///         DeviceArgs::builder()
///             .device(DeviceDevice::builder().deviceName("example").build_struct())
///             .device_fleet_name("${exampleAwsSagemakerDeviceFleet.deviceFleetName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Devices using the `device-fleet-name/device-name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/device:Device example my-fleet/my-device
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod device {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceArgs {
        /// The device to register with SageMaker Edge Manager. See Device details below.
        #[builder(into)]
        pub device: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::sagemaker::DeviceDevice,
        >,
        /// The name of the Device Fleet.
        #[builder(into)]
        pub device_fleet_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DeviceResult {
        pub agent_version: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) assigned by AWS to this Device.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The device to register with SageMaker Edge Manager. See Device details below.
        pub device: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::DeviceDevice,
        >,
        /// The name of the Device Fleet.
        pub device_fleet_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeviceArgs,
    ) -> DeviceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let device_binding = args.device.get_output(context);
        let device_fleet_name_binding = args.device_fleet_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/device:Device".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "device".into(),
                    value: &device_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceFleetName".into(),
                    value: &device_fleet_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeviceResult {
            agent_version: o.get_field("agentVersion"),
            arn: o.get_field("arn"),
            device: o.get_field("device"),
            device_fleet_name: o.get_field("deviceFleetName"),
        }
    }
}
