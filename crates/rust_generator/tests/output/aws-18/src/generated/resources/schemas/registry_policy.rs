/// Resource for managing an AWS EventBridge Schemas Registry Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   exampleRegistryPolicy:
///     type: aws:schemas:RegistryPolicy
///     name: example
///     properties:
///       registryName: example
///       policy: ${example.json}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: example
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '109876543210'
///             actions:
///               - schemas:*
///             resources:
///               - arn:aws:schemas:us-east-1:123456789012:registry/example
///               - arn:aws:schemas:us-east-1:123456789012:schema/example*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EventBridge Schema Registry Policy using the `registry_name`. For example:
///
/// ```sh
/// $ pulumi import aws:schemas/registryPolicy:RegistryPolicy example example
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod registry_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryPolicyArgs {
        /// Resource Policy for EventBridge Schema Registry
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of EventBridge Schema Registry
        #[builder(into)]
        pub registry_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegistryPolicyResult {
        /// Resource Policy for EventBridge Schema Registry
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Name of EventBridge Schema Registry
        pub registry_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RegistryPolicyArgs,
    ) -> RegistryPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_output(context).get_inner();
        let registry_name_binding = args.registry_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:schemas/registryPolicy:RegistryPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "registryName".into(),
                    value: &registry_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegistryPolicyResult {
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            registry_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registryName"),
            ),
        }
    }
}
