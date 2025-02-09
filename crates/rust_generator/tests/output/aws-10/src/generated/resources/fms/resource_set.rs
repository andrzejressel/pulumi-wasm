/// Resource for managing an AWS FMS (Firewall Manager) Resource Set.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_set::create(
///         "example",
///         ResourceSetArgs::builder()
///             .resource_sets(
///                 vec![
///                     ResourceSetResourceSet::builder().name("testing")
///                     .resourceTypeLists(vec!["AWS::NetworkFirewall::Firewall",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import FMS (Firewall Manager) Resource Set using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:fms/resourceSet:ResourceSet example resource_set-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceSetArgs {
        /// Details about the resource set to be created or updated. See `resource_set` Attribute Reference below.
        #[builder(into, default)]
        pub resource_sets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::fms::ResourceSetResourceSet>>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fms::ResourceSetTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceSetResult {
        /// ARN of the Resource Set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Details about the resource set to be created or updated. See `resource_set` Attribute Reference below.
        pub resource_sets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::fms::ResourceSetResourceSet>>,
        >,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::fms::ResourceSetTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceSetArgs,
    ) -> ResourceSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resource_sets_binding = args.resource_sets.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:fms/resourceSet:ResourceSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceSets".into(),
                    value: resource_sets_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceSetResult {
            arn: o.get_field("arn"),
            resource_sets: o.get_field("resourceSets"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
