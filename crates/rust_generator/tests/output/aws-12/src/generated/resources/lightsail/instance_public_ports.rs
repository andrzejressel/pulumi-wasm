/// Opens ports for a specific Amazon Lightsail instance, and specifies the IP addresses allowed to connect to the instance through the ports, and the protocol.
///
/// > See [What is Amazon Lightsail?](https://lightsail.aws.amazon.com/ls/docs/getting-started/article/what-is-amazon-lightsail) for more information.
///
/// > **Note:** Lightsail is currently only supported in a limited number of AWS Regions, please see ["Regions and Availability Zones in Amazon Lightsail"](https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail) for more details.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = instance::create(
///         "test",
///         InstanceArgs::builder()
///             .availability_zone("${available.names[0]}")
///             .blueprint_id("amazon_linux_2")
///             .bundle_id("nano_3_0")
///             .name("yak_sail")
///             .build_struct(),
///     );
///     let testInstancePublicPorts = instance_public_ports::create(
///         "testInstancePublicPorts",
///         InstancePublicPortsArgs::builder()
///             .instance_name("${test.name}")
///             .port_infos(
///                 vec![
///                     InstancePublicPortsPortInfo::builder().fromPort(80).protocol("tcp")
///                     .toPort(80).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
pub mod instance_public_ports {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstancePublicPortsArgs {
        /// Name of the Lightsail Instance.
        #[builder(into)]
        pub instance_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block with port information. AWS closes all currently open ports that are not included in the `port_info`. Detailed below.
        #[builder(into)]
        pub port_infos: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::lightsail::InstancePublicPortsPortInfo>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstancePublicPortsResult {
        /// Name of the Lightsail Instance.
        pub instance_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with port information. AWS closes all currently open ports that are not included in the `port_info`. Detailed below.
        pub port_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::types::lightsail::InstancePublicPortsPortInfo>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstancePublicPortsArgs,
    ) -> InstancePublicPortsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let instance_name_binding = args.instance_name.get_output(context).get_inner();
        let port_infos_binding = args.port_infos.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/instancePublicPorts:InstancePublicPorts".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceName".into(),
                    value: &instance_name_binding,
                },
                register_interface::ObjectField {
                    name: "portInfos".into(),
                    value: &port_infos_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstancePublicPortsResult {
            instance_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceName"),
            ),
            port_infos: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("portInfos"),
            ),
        }
    }
}
