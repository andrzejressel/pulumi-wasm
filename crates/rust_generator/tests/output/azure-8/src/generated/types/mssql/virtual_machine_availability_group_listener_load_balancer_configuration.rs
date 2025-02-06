#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineAvailabilityGroupListenerLoadBalancerConfiguration {
    /// The ID of the Load Balancer. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "loadBalancerId")]
    pub r#load_balancer_id: Box<String>,
    /// The private IP Address of the listener. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// The probe port of the listener. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "probePort")]
    pub r#probe_port: Box<i32>,
    /// Specifies a list of SQL Virtual Machine IDs. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sqlVirtualMachineIds")]
    pub r#sql_virtual_machine_ids: Box<Vec<String>>,
    /// The ID of the Subnet to create the listener. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** `sql_virtual_machine_ids` should match with the SQL Virtual Machines specified in `replica`.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
