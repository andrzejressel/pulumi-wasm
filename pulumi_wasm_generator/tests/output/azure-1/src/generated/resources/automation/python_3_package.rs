/// Manages a Automation Python3 Package.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: rg-example
///       location: '%[2]s'
///   exampleAccount:
///     type: azure:automation:Account
///     name: example
///     properties:
///       name: accexample
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///   examplePython3Package:
///     type: azure:automation:Python3Package
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       automationAccountName: ${exampleAccount.name}
///       contentUri: https://pypi.org/packages/source/r/requests/requests-2.31.0.tar.gz
///       contentVersion: 2.31.0
///       hashAlgorithm: sha256
///       hashValue: 942c5a758f98d790eaed1a29cb6eefc7ffb0d1cf7af05c3d2791656dbd6ad1e1
///       tags:
///         key: foo
/// ```
///
/// ## Import
///
/// Automation Python3 Packages can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/python3Package:Python3Package example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/python3Packages/pkg
/// ```
///
pub mod python_3_package {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Python3PackageArgs {
        /// The name of the automation account in which the Python3 Package is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The URL of the python package. Changing this forces a new Automation Python3 Package to be created.
        #[builder(into)]
        pub content_uri: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specify the version of the python3 package. The value should meet the system.version class format like `1.1.1`. Changing this forces a new Automation Python3 Package to be created.
        #[builder(into, default)]
        pub content_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specify the hash algorithm used to hash the content of the python3 package. Changing this forces a new Automation Python3 Package to be created.
        #[builder(into, default)]
        pub hash_algorithm: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specity the hash value of the content. Changing this forces a new Automation Python3 Package to be created.
        #[builder(into, default)]
        pub hash_value: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Automation Python3 Package. Changing this forces a new Automation Python3 Package to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Python3 Package is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Automation Python3 Package.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct Python3PackageResult {
        /// The name of the automation account in which the Python3 Package is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// The URL of the python package. Changing this forces a new Automation Python3 Package to be created.
        pub content_uri: pulumi_wasm_rust::Output<String>,
        /// Specify the version of the python3 package. The value should meet the system.version class format like `1.1.1`. Changing this forces a new Automation Python3 Package to be created.
        pub content_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the hash algorithm used to hash the content of the python3 package. Changing this forces a new Automation Python3 Package to be created.
        pub hash_algorithm: pulumi_wasm_rust::Output<Option<String>>,
        /// Specity the hash value of the content. Changing this forces a new Automation Python3 Package to be created.
        pub hash_value: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Automation Python3 Package. Changing this forces a new Automation Python3 Package to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Python3 Package is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Automation Python3 Package.
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
        args: Python3PackageArgs,
    ) -> Python3PackageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context)
            .get_inner();
        let content_uri_binding = args.content_uri.get_output(context).get_inner();
        let content_version_binding = args
            .content_version
            .get_output(context)
            .get_inner();
        let hash_algorithm_binding = args.hash_algorithm.get_output(context).get_inner();
        let hash_value_binding = args.hash_value.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/python3Package:Python3Package".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "contentUri".into(),
                    value: &content_uri_binding,
                },
                register_interface::ObjectField {
                    name: "contentVersion".into(),
                    value: &content_version_binding,
                },
                register_interface::ObjectField {
                    name: "hashAlgorithm".into(),
                    value: &hash_algorithm_binding,
                },
                register_interface::ObjectField {
                    name: "hashValue".into(),
                    value: &hash_value_binding,
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
        Python3PackageResult {
            automation_account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("automationAccountName"),
            ),
            content_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentUri"),
            ),
            content_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentVersion"),
            ),
            hash_algorithm: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hashAlgorithm"),
            ),
            hash_value: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hashValue"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
