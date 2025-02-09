/// Resource for managing an AWS Route 53 Profile.
///
/// ## Example Usage
///
/// ### Empty Profile
///
/// ```yaml
/// resources:
///   example:
///     type: aws:route53:ProfilesProfile
///     properties:
///       name: example
///       tags:
///         Environment: dev
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route 53 Profiles Profile using the `example`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/profilesProfile:ProfilesProfile example rp-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod profiles_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfilesProfileArgs {
        /// Name of the Profile.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::route53::ProfilesProfileTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfilesProfileResult {
        /// ARN of the Profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the Profile.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Share status of the Profile.
        pub share_status: pulumi_gestalt_rust::Output<String>,
        /// Status of the Profile.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Status message of the Profile.
        pub status_message: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::route53::ProfilesProfileTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProfilesProfileArgs,
    ) -> ProfilesProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/profilesProfile:ProfilesProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
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
        ProfilesProfileResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            owner_id: o.get_field("ownerId"),
            share_status: o.get_field("shareStatus"),
            status: o.get_field("status"),
            status_message: o.get_field("statusMessage"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
