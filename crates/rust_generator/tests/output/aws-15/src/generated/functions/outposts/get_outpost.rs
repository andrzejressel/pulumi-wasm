#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_outpost {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOutpostArgs {
        /// ARN.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the Outpost.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the Outpost.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AWS Account identifier of the Outpost owner.
        #[builder(into, default)]
        pub owner_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Outpost tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetOutpostResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Availability Zone name.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Availability Zone identifier.
        pub availability_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the Outpost.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The life cycle status.
        pub lifecycle_status: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub owner_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the site.
        pub site_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the site.
        pub site_id: pulumi_gestalt_rust::Output<String>,
        /// The hardware type.
        pub supported_hardware_type: pulumi_gestalt_rust::Output<String>,
        /// The Outpost tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetOutpostArgs,
    ) -> GetOutpostResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding_1 = args.arn.get_output(context);
        let arn_binding = arn_binding_1.get_inner();
        let id_binding_1 = args.id.get_output(context);
        let id_binding = id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let owner_id_binding_1 = args.owner_id.get_output(context);
        let owner_id_binding = owner_id_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:outposts/getOutpost:getOutpost".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ownerId".into(),
                    value: &owner_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOutpostResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            availability_zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZoneId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            lifecycle_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lifecycleStatus"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            site_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("siteArn"),
            ),
            site_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("siteId"),
            ),
            supported_hardware_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportedHardwareType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
