/// <!-- Note: This documentation is generated. Any manual changes will be overwritten -->
///
/// Manages a Load Test Service.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleLoadTest = load_test::create(
///         "exampleLoadTest",
///         LoadTestArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleUserAssignedIdentity = user_assigned_identity::create(
///         "exampleUserAssignedIdentity",
///         UserAssignedIdentityArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// An existing Load Test can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:loadtest/loadTest:LoadTest example /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.LoadTestService/loadTests/{loadTestName}
/// ```
///
/// * Where `{subscriptionId}` is the ID of the Azure Subscription where the Load Test exists. For example `12345678-1234-9876-4563-123456789012`.
///
/// * Where `{resourceGroupName}` is the name of Resource Group where this Load Test exists. For example `example-resource-group`.
///
/// * Where `{loadTestName}` is the name of the Load Test. For example `loadTestValue`.
///
pub mod load_test {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadTestArgs {
        /// Description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An `encryption` block as defined below. Changing this forces a new Load Test to be created.
        #[builder(into, default)]
        pub encryption: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::loadtest::LoadTestEncryption>,
        >,
        /// An `identity` block as defined below. Specifies the Managed Identity which should be assigned to this Load Test.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::loadtest::LoadTestIdentity>,
        >,
        /// The Azure Region where the Load Test should exist. Changing this forces a new Load Test to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Load Test. Changing this forces a new Load Test to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group within which this Load Test should exist. Changing this forces a new Load Test to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Load Test.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LoadTestResult {
        /// Resource data plane URI.
        pub data_plane_uri: pulumi_wasm_rust::Output<String>,
        /// Description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// An `encryption` block as defined below. Changing this forces a new Load Test to be created.
        pub encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::loadtest::LoadTestEncryption>,
        >,
        /// An `identity` block as defined below. Specifies the Managed Identity which should be assigned to this Load Test.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::loadtest::LoadTestIdentity>,
        >,
        /// The Azure Region where the Load Test should exist. Changing this forces a new Load Test to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Load Test. Changing this forces a new Load Test to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this Load Test should exist. Changing this forces a new Load Test to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Load Test.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LoadTestArgs,
    ) -> LoadTestResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let encryption_binding = args.encryption.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:loadtest/loadTest:LoadTest".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LoadTestResult {
            data_plane_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataPlaneUri"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            encryption: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryption"),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
