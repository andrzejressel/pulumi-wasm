/// Manages a Spring Cloud Build Pack Binding.
///
/// > **NOTE:** This resource is applicable only for Spring Cloud Service with enterprise tier.
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
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example-springcloud
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       skuName: E0
///   exampleSpringCloudBuilder:
///     type: azure:appplatform:SpringCloudBuilder
///     name: example
///     properties:
///       name: example
///       springCloudServiceId: ${exampleSpringCloudService.id}
///       buildPackGroups:
///         - name: mix
///           buildPackIds:
///             - tanzu-Build Packs/java-azure
///       stack:
///         id: io.Build Packs.stacks.bionic
///         version: base
///   exampleSpringCloudBuildPackBinding:
///     type: azure:appplatform:SpringCloudBuildPackBinding
///     name: example
///     properties:
///       name: example
///       springCloudBuilderId: ${exampleSpringCloudBuilder.id}
///       bindingType: ApplicationInsights
///       launch:
///         properties:
///           abc: def
///           any-string: any-string
///           sampling-rate: '12.0'
///         secrets:
///           connection-string: XXXXXXXXXXXXXXXXX=XXXXXXXXXXXXX-XXXXXXXXXXXXXXXXXXX;XXXXXXXXXXXXXXXXX=XXXXXXXXXXXXXXXXXXX
/// ```
///
/// ## Import
///
/// Spring Cloud Build Pack Bindings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudBuildPackBinding:SpringCloudBuildPackBinding example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.AppPlatform/spring/service1/buildServices/buildService1/builders/builder1/buildPackBindings/binding1
/// ```
///
pub mod spring_cloud_build_pack_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudBuildPackBindingArgs {
        /// Specifies the Build Pack Binding Type. Allowed values are `ApacheSkyWalking`, `AppDynamics`, `ApplicationInsights`, `Dynatrace`, `ElasticAPM` and `NewRelic`.
        #[builder(into, default)]
        pub binding_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A `launch` block as defined below.
        #[builder(into, default)]
        pub launch: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudBuildPackBindingLaunch>,
        >,
        /// The name which should be used for this Spring Cloud Build Pack Binding. Changing this forces a new Spring Cloud Build Pack Binding to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Spring Cloud Builder. Changing this forces a new Spring Cloud Build Pack Binding to be created.
        #[builder(into)]
        pub spring_cloud_builder_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudBuildPackBindingResult {
        /// Specifies the Build Pack Binding Type. Allowed values are `ApacheSkyWalking`, `AppDynamics`, `ApplicationInsights`, `Dynatrace`, `ElasticAPM` and `NewRelic`.
        pub binding_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A `launch` block as defined below.
        pub launch: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudBuildPackBindingLaunch>,
        >,
        /// The name which should be used for this Spring Cloud Build Pack Binding. Changing this forces a new Spring Cloud Build Pack Binding to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Spring Cloud Builder. Changing this forces a new Spring Cloud Build Pack Binding to be created.
        pub spring_cloud_builder_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudBuildPackBindingArgs,
    ) -> SpringCloudBuildPackBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let binding_type_binding = args.binding_type.get_inner();
        let launch_binding = args.launch.get_inner();
        let name_binding = args.name.get_inner();
        let spring_cloud_builder_id_binding = args.spring_cloud_builder_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudBuildPackBinding:SpringCloudBuildPackBinding"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bindingType".into(),
                    value: &binding_type_binding,
                },
                register_interface::ObjectField {
                    name: "launch".into(),
                    value: &launch_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudBuilderId".into(),
                    value: &spring_cloud_builder_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bindingType".into(),
                },
                register_interface::ResultField {
                    name: "launch".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "springCloudBuilderId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudBuildPackBindingResult {
            binding_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bindingType").unwrap(),
            ),
            launch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launch").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            spring_cloud_builder_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudBuilderId").unwrap(),
            ),
        }
    }
}