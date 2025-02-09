/// The resource `random.RandomUuid` generates a random uuid string that is intended to be used as a unique identifier for other resources.
///
/// This resource uses [hashicorp/go-uuid](https://github.com/hashicorp/go-uuid) to generate a UUID-formatted string for use with services needing a unique string identifier.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testRandomUuid:
///     type: random:RandomUuid
///   testResourceGroup:
///     type: azure:core:ResourceGroup
///     properties:
///       location: Central US
/// ```
///
/// ## Import
///
/// Random UUID's can be imported. This can be used to replace a config value with a value interpolated from the random provider without experiencing diffs.
///
/// ```sh
///  $ pulumi import random:index/randomUuid:RandomUuid main aabbccdd-eeff-0011-2233-445566778899
/// ```
///
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod random_uuid {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomUuidArgs {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        #[builder(into, default)]
        pub keepers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RandomUuidResult {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        pub keepers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The generated uuid presented in string format.
        pub result: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RandomUuidArgs,
    ) -> RandomUuidResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let keepers_binding_1 = args.keepers.get_output(context);
        let keepers_binding = keepers_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "random:index/randomUuid:RandomUuid".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keepers".into(),
                    value: &keepers_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RandomUuidResult {
            keepers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keepers"),
            ),
            result: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("result"),
            ),
        }
    }
}
