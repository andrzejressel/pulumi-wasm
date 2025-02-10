#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_orderable_db_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrderableDbInstanceArgs {
        /// DB engine. Default: `docdb`
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Version of the DB engine.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// DB instance class. Examples of classes are `db.r5.12xlarge`, `db.r5.24xlarge`, `db.r5.2xlarge`, `db.r5.4xlarge`, `db.r5.large`, `db.r5.xlarge`, and `db.t3.medium`. (Conflicts with `preferred_instance_classes`.)
        #[builder(into, default)]
        pub instance_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// License model. Default: `na`
        #[builder(into, default)]
        pub license_model: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Ordered list of preferred DocumentDB DB instance classes. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned. (Conflicts with `instance_class`.)
        #[builder(into, default)]
        pub preferred_instance_classes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Enable to show only VPC.
        #[builder(into, default)]
        pub vpc: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetOrderableDbInstanceResult {
        /// Availability zones where the instance is available.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        pub engine: pulumi_gestalt_rust::Output<Option<String>>,
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_class: pulumi_gestalt_rust::Output<String>,
        pub license_model: pulumi_gestalt_rust::Output<Option<String>>,
        pub preferred_instance_classes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub vpc: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOrderableDbInstanceArgs,
    ) -> GetOrderableDbInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let engine_binding = args.engine.get_output(context);
        let engine_version_binding = args.engine_version.get_output(context);
        let instance_class_binding = args.instance_class.get_output(context);
        let license_model_binding = args.license_model.get_output(context);
        let preferred_instance_classes_binding = args
            .preferred_instance_classes
            .get_output(context);
        let vpc_binding = args.vpc.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:docdb/getOrderableDbInstance:getOrderableDbInstance".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: engine_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineVersion".into(),
                    value: engine_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceClass".into(),
                    value: instance_class_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseModel".into(),
                    value: license_model_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredInstanceClasses".into(),
                    value: preferred_instance_classes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpc".into(),
                    value: vpc_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOrderableDbInstanceResult {
            availability_zones: o.get_field("availabilityZones"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            id: o.get_field("id"),
            instance_class: o.get_field("instanceClass"),
            license_model: o.get_field("licenseModel"),
            preferred_instance_classes: o.get_field("preferredInstanceClasses"),
            vpc: o.get_field("vpc"),
        }
    }
}
