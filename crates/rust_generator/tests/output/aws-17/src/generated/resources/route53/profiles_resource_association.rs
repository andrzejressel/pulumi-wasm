/// Resource for managing an AWS Route 53 Profiles Resource Association.
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
///   exampleZone:
///     type: aws:route53:Zone
///     name: example
///     properties:
///       name: example.com
///       vpcs:
///         - vpcId: ${exampleVpc.id}
///   exampleProfilesResourceAssociation:
///     type: aws:route53:ProfilesResourceAssociation
///     name: example
///     properties:
///       name: example
///       profileId: ${example.id}
///       resourceArn: ${exampleZone.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route 53 Profiles Resource Association using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/profilesResourceAssociation:ProfilesResourceAssociation example rpa-id-12345678
/// ```
pub mod profiles_resource_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfilesResourceAssociationArgs {
        /// Name of the Profile Resource Association.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the profile associated with the VPC.
        #[builder(into)]
        pub profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource ID of the resource to be associated with the profile.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource properties for the resource to be associated with the profile.
        #[builder(into, default)]
        pub resource_properties: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::route53::ProfilesResourceAssociationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfilesResourceAssociationResult {
        /// Name of the Profile Resource Association.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the profile associated with the VPC.
        pub profile_id: pulumi_gestalt_rust::Output<String>,
        /// Resource ID of the resource to be associated with the profile.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// Resource properties for the resource to be associated with the profile.
        pub resource_properties: pulumi_gestalt_rust::Output<Option<String>>,
        /// Type of resource associated with the profile.
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// Status of the Profile Association. Valid values [AWS docs](https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53profiles_Profile.html)
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Status message of the Profile Resource Association.
        pub status_message: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::route53::ProfilesResourceAssociationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProfilesResourceAssociationArgs,
    ) -> ProfilesResourceAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let profile_id_binding = args.profile_id.get_output(context).get_inner();
        let resource_arn_binding = args.resource_arn.get_output(context).get_inner();
        let resource_properties_binding = args
            .resource_properties
            .get_output(context)
            .get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/profilesResourceAssociation:ProfilesResourceAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "profileId".into(),
                    value: &profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
                register_interface::ObjectField {
                    name: "resourceProperties".into(),
                    value: &resource_properties_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProfilesResourceAssociationResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            profile_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("profileId"),
            ),
            resource_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
            resource_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceProperties"),
            ),
            resource_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceType"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            status_message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusMessage"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
