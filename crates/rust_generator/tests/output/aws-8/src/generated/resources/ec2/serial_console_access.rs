/// Provides a resource to manage whether serial console access is enabled for your AWS account in the current AWS region.
///
/// > **NOTE:** Removing this resource disables serial console access.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = serial_console_access::create(
///         "example",
///         SerialConsoleAccessArgs::builder().enabled(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import serial console access state. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/serialConsoleAccess:SerialConsoleAccess example default
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod serial_console_access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SerialConsoleAccessArgs {
        /// Whether or not serial console access is enabled. Valid values are `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SerialConsoleAccessResult {
        /// Whether or not serial console access is enabled. Valid values are `true` or `false`. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SerialConsoleAccessArgs,
    ) -> SerialConsoleAccessResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_binding = args.enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/serialConsoleAccess:SerialConsoleAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SerialConsoleAccessResult {
            enabled: o.get_field("enabled"),
        }
    }
}
