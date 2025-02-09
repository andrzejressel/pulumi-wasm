/// Provides a [Route53 Delegation Set](https://docs.aws.amazon.com/Route53/latest/APIReference/API-actions-by-function.html#actions-by-function-reusable-delegation-sets) resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = delegation_set::create(
///         "main",
///         DelegationSetArgs::builder().reference_name("DynDNS").build_struct(),
///     );
///     let primary = zone::create(
///         "primary",
///         ZoneArgs::builder()
///             .delegation_set_id("${main.id}")
///             .name("mydomain.com")
///             .build_struct(),
///     );
///     let secondary = zone::create(
///         "secondary",
///         ZoneArgs::builder()
///             .delegation_set_id("${main.id}")
///             .name("coolcompany.io")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Delegation Sets using the delegation set `id`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/delegationSet:DelegationSet set1 N1PA6795SAMPLE
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod delegation_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DelegationSetArgs {
        /// This is a reference name used in Caller Reference
        /// (helpful for identifying single delegation set amongst others)
        #[builder(into, default)]
        pub reference_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DelegationSetResult {
        /// The Amazon Resource Name (ARN) of the Delegation Set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A list of authoritative name servers for the hosted zone
        /// (effectively a list of NS records).
        pub name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// This is a reference name used in Caller Reference
        /// (helpful for identifying single delegation set amongst others)
        pub reference_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DelegationSetArgs,
    ) -> DelegationSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let reference_name_binding = args.reference_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/delegationSet:DelegationSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "referenceName".into(),
                    value: reference_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DelegationSetResult {
            arn: o.get_field("arn"),
            name_servers: o.get_field("nameServers"),
            reference_name: o.get_field("referenceName"),
        }
    }
}
