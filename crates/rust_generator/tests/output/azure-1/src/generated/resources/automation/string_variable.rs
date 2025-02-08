/// Manages a string variable in Azure Automation
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("tfex-example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .location("${example.location}")
///             .name("tfex-example-account")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
///     let exampleStringVariable = string_variable::create(
///         "exampleStringVariable",
///         StringVariableArgs::builder()
///             .automation_account_name("${exampleAccount.name}")
///             .name("tfex-example-var")
///             .resource_group_name("${example.name}")
///             .value("Hello, Basic Test.")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Automation String Variable can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/stringVariable:StringVariable example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/tfex-example-rg/providers/Microsoft.Automation/automationAccounts/tfex-example-account/variables/tfex-example-var
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod string_variable {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StringVariableArgs {
        /// The name of the automation account in which the Variable is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the Automation Variable.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if the Automation Variable is encrypted. Defaults to `false`.
        #[builder(into, default)]
        pub encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Automation Variable. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Automation Variable. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The value of the Automation Variable as a `string`.
        #[builder(into, default)]
        pub value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StringVariableResult {
        /// The name of the automation account in which the Variable is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// The description of the Automation Variable.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if the Automation Variable is encrypted. Defaults to `false`.
        pub encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Automation Variable. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Automation Variable. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The value of the Automation Variable as a `string`.
        pub value: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: StringVariableArgs,
    ) -> StringVariableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let encrypted_binding = args.encrypted.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let value_binding = args.value.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/stringVariable:StringVariable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "encrypted".into(),
                    value: &encrypted_binding,
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
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StringVariableResult {
            automation_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automationAccountName"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encrypted"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
