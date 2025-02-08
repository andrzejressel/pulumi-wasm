/// Manages a Hostname Binding within an App Service (or Function App).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   server:
///     type: random:RandomId
///     properties:
///       keepers:
///         azi_id: 1
///       byteLength: 8
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: some-resource-group
///       location: West Europe
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///     properties:
///       name: some-app-service-plan
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         tier: Standard
///         size: S1
///   exampleAppService:
///     type: azure:appservice:AppService
///     name: example
///     properties:
///       name: ${server.hex}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       appServicePlanId: ${examplePlan.id}
///   exampleCustomHostnameBinding:
///     type: azure:appservice:CustomHostnameBinding
///     name: example
///     properties:
///       hostname: www.mywebsite.com
///       appServiceName: ${exampleAppService.name}
///       resourceGroupName: ${example.name}
/// ```
///
/// ## Import
///
/// App Service Custom Hostname Bindings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/customHostnameBinding:CustomHostnameBinding mywebsite /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/sites/instance1/hostNameBindings/mywebsite.com
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod custom_hostname_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomHostnameBindingArgs {
        /// The name of the App Service in which to add the Custom Hostname Binding. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Custom Hostname to use for the App Service, example `www.example.com`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A CNAME needs to be configured from this Hostname to the Azure Website - otherwise Azure will reject the Hostname Binding.
        #[builder(into)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the App Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SSL type. Possible values are `IpBasedEnabled` and `SniEnabled`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub ssl_state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The SSL certificate thumbprint. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `thumbprint` must be specified when `ssl_state` is set.
        #[builder(into, default)]
        pub thumbprint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CustomHostnameBindingResult {
        /// The name of the App Service in which to add the Custom Hostname Binding. Changing this forces a new resource to be created.
        pub app_service_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Custom Hostname to use for the App Service, example `www.example.com`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A CNAME needs to be configured from this Hostname to the Azure Website - otherwise Azure will reject the Hostname Binding.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the App Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SSL type. Possible values are `IpBasedEnabled` and `SniEnabled`. Changing this forces a new resource to be created.
        pub ssl_state: pulumi_gestalt_rust::Output<String>,
        /// The SSL certificate thumbprint. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `thumbprint` must be specified when `ssl_state` is set.
        pub thumbprint: pulumi_gestalt_rust::Output<String>,
        /// The virtual IP address assigned to the hostname if IP based SSL is enabled.
        pub virtual_ip: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CustomHostnameBindingArgs,
    ) -> CustomHostnameBindingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_service_name_binding = args
            .app_service_name
            .get_output(context)
            .get_inner();
        let hostname_binding = args.hostname.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let ssl_state_binding = args.ssl_state.get_output(context).get_inner();
        let thumbprint_binding = args.thumbprint.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/customHostnameBinding:CustomHostnameBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServiceName".into(),
                    value: &app_service_name_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sslState".into(),
                    value: &ssl_state_binding,
                },
                register_interface::ObjectField {
                    name: "thumbprint".into(),
                    value: &thumbprint_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomHostnameBindingResult {
            app_service_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appServiceName"),
            ),
            hostname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            ssl_state: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sslState"),
            ),
            thumbprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("thumbprint"),
            ),
            virtual_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualIp"),
            ),
        }
    }
}
