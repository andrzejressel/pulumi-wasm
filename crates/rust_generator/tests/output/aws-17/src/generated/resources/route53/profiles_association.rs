/// Resource for managing an AWS Route 53 Profiles Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:route53:ProfilesProfile
///     properties:
///       name: example
///   exampleVpc:
///     type: aws:ec2:Vpc
///     name: example
///     properties:
///       cidr: 10.0.0.0/16
///   exampleProfilesAssociation:
///     type: aws:route53:ProfilesAssociation
///     name: example
///     properties:
///       name: example
///       profileId: ${example.id}
///       resourceId: ${exampleVpc.id}
///       tags:
///         Environment: dev
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route 53 Profiles Association using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/profilesAssociation:ProfilesAssociation example rpa-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod profiles_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfilesAssociationArgs {
        /// Name of the Profile Association. Must match a regex of `(?!^[0-9]+$)([a-zA-Z0-9\\-_' ']+)`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the profile associated with the VPC.
        #[builder(into)]
        pub profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource ID of the VPC the profile to be associated with.
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::route53::ProfilesAssociationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfilesAssociationResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the Profile Association. Must match a regex of `(?!^[0-9]+$)([a-zA-Z0-9\\-_' ']+)`.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the profile associated with the VPC.
        pub profile_id: pulumi_gestalt_rust::Output<String>,
        /// Resource ID of the VPC the profile to be associated with.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// Status of the Profile Association.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Status message of the Profile Association.
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
            Option<super::super::types::route53::ProfilesAssociationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProfilesAssociationArgs,
    ) -> ProfilesAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let profile_id_binding = args.profile_id.get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/profilesAssociation:ProfilesAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profileId".into(),
                    value: profile_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: resource_id_binding.get_id(),
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
        ProfilesAssociationResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            owner_id: o.get_field("ownerId"),
            profile_id: o.get_field("profileId"),
            resource_id: o.get_field("resourceId"),
            status: o.get_field("status"),
            status_message: o.get_field("statusMessage"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
