/// Manages [DynamoDB Global Tables V1 (version 2017.11.29)](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V1.html). These are layered on top of existing DynamoDB Tables.
///
/// > **NOTE:** To instead manage [DynamoDB Global Tables V2 (version 2019.11.21)](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html), use the `aws.dynamodb.Table` resource `replica` configuration block.
///
/// > Note: There are many restrictions before you can properly create DynamoDB Global Tables in multiple regions. See the [AWS DynamoDB Global Table Requirements](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables_reqs_bestpractices.html) for more information.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   us-east-1:
///     type: aws:dynamodb:Table
///     properties:
///       hashKey: myAttribute
///       name: myTable
///       streamEnabled: true
///       streamViewType: NEW_AND_OLD_IMAGES
///       readCapacity: 1
///       writeCapacity: 1
///       attributes:
///         - name: myAttribute
///           type: S
///   us-west-2:
///     type: aws:dynamodb:Table
///     properties:
///       hashKey: myAttribute
///       name: myTable
///       streamEnabled: true
///       streamViewType: NEW_AND_OLD_IMAGES
///       readCapacity: 1
///       writeCapacity: 1
///       attributes:
///         - name: myAttribute
///           type: S
///   myTable:
///     type: aws:dynamodb:GlobalTable
///     properties:
///       name: myTable
///       replicas:
///         - regionName: us-east-1
///         - regionName: us-west-2
///     options:
///       dependsOn:
///         - ${["us-east-1"]}
///         - ${["us-west-2"]}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DynamoDB Global Tables using the global table name. For example:
///
/// ```sh
/// $ pulumi import aws:dynamodb/globalTable:GlobalTable MyTable MyTable
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod global_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalTableArgs {
        /// The name of the global table. Must match underlying DynamoDB Table names in all regions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Underlying DynamoDB Table. At least 1 replica must be defined. See below.
        #[builder(into)]
        pub replicas: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::dynamodb::GlobalTableReplica>,
        >,
    }
    #[allow(dead_code)]
    pub struct GlobalTableResult {
        /// The ARN of the DynamoDB Global Table
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the global table. Must match underlying DynamoDB Table names in all regions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Underlying DynamoDB Table. At least 1 replica must be defined. See below.
        pub replicas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dynamodb::GlobalTableReplica>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GlobalTableArgs,
    ) -> GlobalTableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let replicas_binding = args.replicas.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dynamodb/globalTable:GlobalTable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "replicas".into(),
                    value: &replicas_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GlobalTableResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            replicas: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicas"),
            ),
        }
    }
}
