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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod python_3_package {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Python3PackageArgs {
        /// The name of the automation account in which the Python3 Package is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The URL of the python package. Changing this forces a new Automation Python3 Package to be created.
        #[builder(into)]
        pub content_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specify the version of the python3 package. The value should meet the system.version class format like `1.1.1`. Changing this forces a new Automation Python3 Package to be created.
        #[builder(into, default)]
        pub content_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify the hash algorithm used to hash the content of the python3 package. Changing this forces a new Automation Python3 Package to be created.
        #[builder(into, default)]
        pub hash_algorithm: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specity the hash value of the content. Changing this forces a new Automation Python3 Package to be created.
        #[builder(into, default)]
        pub hash_value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Automation Python3 Package. Changing this forces a new Automation Python3 Package to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Python3 Package is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Automation Python3 Package.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct Python3PackageResult {
        /// The name of the automation account in which the Python3 Package is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// The URL of the python package. Changing this forces a new Automation Python3 Package to be created.
        pub content_uri: pulumi_gestalt_rust::Output<String>,
        /// Specify the version of the python3 package. The value should meet the system.version class format like `1.1.1`. Changing this forces a new Automation Python3 Package to be created.
        pub content_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specify the hash algorithm used to hash the content of the python3 package. Changing this forces a new Automation Python3 Package to be created.
        pub hash_algorithm: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specity the hash value of the content. Changing this forces a new Automation Python3 Package to be created.
        pub hash_value: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Automation Python3 Package. Changing this forces a new Automation Python3 Package to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Python3 Package is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Automation Python3 Package.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Python3PackageArgs,
    ) -> Python3PackageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context);
        let content_uri_binding = args.content_uri.get_output(context);
        let content_version_binding = args.content_version.get_output(context);
        let hash_algorithm_binding = args.hash_algorithm.get_output(context);
        let hash_value_binding = args.hash_value.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/python3Package:Python3Package".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentUri".into(),
                    value: &content_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentVersion".into(),
                    value: &content_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hashAlgorithm".into(),
                    value: &hash_algorithm_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hashValue".into(),
                    value: &hash_value_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        Python3PackageResult {
            automation_account_name: o.get_field("automationAccountName"),
            content_uri: o.get_field("contentUri"),
            content_version: o.get_field("contentVersion"),
            hash_algorithm: o.get_field("hashAlgorithm"),
            hash_value: o.get_field("hashValue"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
