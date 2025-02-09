/// Manages an Azure Spring Cloud Custom Domain.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: rg-example
///       location: West Europe
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example-springcloud
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///   exampleSpringCloudApp:
///     type: azure:appplatform:SpringCloudApp
///     name: example
///     properties:
///       name: example-springcloudapp
///       resourceGroupName: ${exampleResourceGroup.name}
///       serviceName: ${exampleSpringCloudService.name}
///   exampleCNameRecord:
///     type: azure:dns:CNameRecord
///     name: example
///     properties:
///       name: record1
///       zoneName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       ttl: 300
///       record: ${exampleSpringCloudApp.fqdn}
///   exampleSpringCloudCustomDomain:
///     type: azure:appplatform:SpringCloudCustomDomain
///     name: example
///     properties:
///       name:
///         fn::invoke:
///           function: std:join
///           arguments:
///             separator: .
///             input:
///               - ${exampleCNameRecord.name}
///               - ${exampleCNameRecord.zoneName}
///           return: result
///       springCloudAppId: ${exampleSpringCloudApp.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:dns:getZone
///       arguments:
///         name: mydomain.com
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Spring Cloud Custom Domain can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudCustomDomain:SpringCloudCustomDomain example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.AppPlatform/spring/spring1/apps/app1/domains/domain.com
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_custom_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudCustomDomainArgs {
        /// Specifies the name of the Spring Cloud Certificate that binds to the Spring Cloud Custom Domain. Required when `thumbprint` is specified
        #[builder(into, default)]
        pub certificate_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Spring Cloud Custom Domain. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the resource ID of the Spring Cloud Application. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the thumbprint of the Spring Cloud Certificate that binds to the Spring Cloud Custom Domain. Required when `certificate_name` is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub thumbprint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudCustomDomainResult {
        /// Specifies the name of the Spring Cloud Certificate that binds to the Spring Cloud Custom Domain. Required when `thumbprint` is specified
        pub certificate_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Spring Cloud Custom Domain. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the resource ID of the Spring Cloud Application. Changing this forces a new resource to be created.
        pub spring_cloud_app_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the thumbprint of the Spring Cloud Certificate that binds to the Spring Cloud Custom Domain. Required when `certificate_name` is specified. Changing this forces a new resource to be created.
        pub thumbprint: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SpringCloudCustomDomainArgs,
    ) -> SpringCloudCustomDomainResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificate_name_binding_1 = args.certificate_name.get_output(context);
        let certificate_name_binding = certificate_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let spring_cloud_app_id_binding_1 = args.spring_cloud_app_id.get_output(context);
        let spring_cloud_app_id_binding = spring_cloud_app_id_binding_1.get_inner();
        let thumbprint_binding_1 = args.thumbprint.get_output(context);
        let thumbprint_binding = thumbprint_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudCustomDomain:SpringCloudCustomDomain"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateName".into(),
                    value: &certificate_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "thumbprint".into(),
                    value: &thumbprint_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SpringCloudCustomDomainResult {
            certificate_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            spring_cloud_app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("springCloudAppId"),
            ),
            thumbprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("thumbprint"),
            ),
        }
    }
}
