pub mod get_orderable_db_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrderableDbInstanceArgs {
        /// DB engine. Default: `docdb`
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// Version of the DB engine.
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::Output<Option<String>>,
        /// DB instance class. Examples of classes are `db.r5.12xlarge`, `db.r5.24xlarge`, `db.r5.2xlarge`, `db.r5.4xlarge`, `db.r5.large`, `db.r5.xlarge`, and `db.t3.medium`. (Conflicts with `preferred_instance_classes`.)
        #[builder(into, default)]
        pub instance_class: pulumi_wasm_rust::Output<Option<String>>,
        /// License model. Default: `na`
        #[builder(into, default)]
        pub license_model: pulumi_wasm_rust::Output<Option<String>>,
        /// Ordered list of preferred DocumentDB DB instance classes. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned. (Conflicts with `instance_class`.)
        #[builder(into, default)]
        pub preferred_instance_classes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Enable to show only VPC.
        #[builder(into, default)]
        pub vpc: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetOrderableDbInstanceResult {
        /// Availability zones where the instance is available.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_class: pulumi_wasm_rust::Output<String>,
        pub license_model: pulumi_wasm_rust::Output<Option<String>>,
        pub preferred_instance_classes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub vpc: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetOrderableDbInstanceArgs) -> GetOrderableDbInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_binding = args.engine.get_inner();
        let engine_version_binding = args.engine_version.get_inner();
        let instance_class_binding = args.instance_class.get_inner();
        let license_model_binding = args.license_model.get_inner();
        let preferred_instance_classes_binding = args
            .preferred_instance_classes
            .get_inner();
        let vpc_binding = args.vpc.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:docdb/getOrderableDbInstance:getOrderableDbInstance".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "instanceClass".into(),
                    value: &instance_class_binding,
                },
                register_interface::ObjectField {
                    name: "licenseModel".into(),
                    value: &license_model_binding,
                },
                register_interface::ObjectField {
                    name: "preferredInstanceClasses".into(),
                    value: &preferred_instance_classes_binding,
                },
                register_interface::ObjectField {
                    name: "vpc".into(),
                    value: &vpc_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceClass".into(),
                },
                register_interface::ResultField {
                    name: "licenseModel".into(),
                },
                register_interface::ResultField {
                    name: "preferredInstanceClasses".into(),
                },
                register_interface::ResultField {
                    name: "vpc".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOrderableDbInstanceResult {
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceClass").unwrap(),
            ),
            license_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseModel").unwrap(),
            ),
            preferred_instance_classes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredInstanceClasses").unwrap(),
            ),
            vpc: pulumi_wasm_rust::__private::into_domain(hashmap.remove("vpc").unwrap()),
        }
    }
}
