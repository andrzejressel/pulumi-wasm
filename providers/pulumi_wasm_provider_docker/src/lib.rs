use bindings::exports::pulumi::docker::container;
use bindings::exports::pulumi::docker::image;
use bindings::exports::pulumi::docker::network;
use bindings::exports::pulumi::docker::plugin;
use bindings::exports::pulumi::docker::registry_image;
use bindings::exports::pulumi::docker::remote_image;
use bindings::exports::pulumi::docker::secret;
use bindings::exports::pulumi::docker::service;
use bindings::exports::pulumi::docker::service_config;
use bindings::exports::pulumi::docker::tag;
use bindings::exports::pulumi::docker::volume;

use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl container::Guest for Component {
    fn invoke(name: String, args: container::Args) -> container::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/container:Container".into(),
            name,
            object: vec![
                ObjectField {
                    name: "attach".into(),
                    value: args.attach,
                },
                ObjectField {
                    name: "capabilities".into(),
                    value: args.capabilities,
                },
                ObjectField {
                    name: "cgroupnsMode".into(),
                    value: args.cgroupns_mode,
                },
                ObjectField {
                    name: "command".into(),
                    value: args.command,
                },
                ObjectField {
                    name: "containerReadRefreshTimeoutMilliseconds".into(),
                    value: args.container_read_refresh_timeout_milliseconds,
                },
                ObjectField {
                    name: "cpuSet".into(),
                    value: args.cpu_set,
                },
                ObjectField {
                    name: "cpuShares".into(),
                    value: args.cpu_shares,
                },
                ObjectField {
                    name: "destroyGraceSeconds".into(),
                    value: args.destroy_grace_seconds,
                },
                ObjectField {
                    name: "devices".into(),
                    value: args.devices,
                },
                ObjectField {
                    name: "dns".into(),
                    value: args.dns,
                },
                ObjectField {
                    name: "dnsOpts".into(),
                    value: args.dns_opts,
                },
                ObjectField {
                    name: "dnsSearches".into(),
                    value: args.dns_searches,
                },
                ObjectField {
                    name: "domainname".into(),
                    value: args.domainname,
                },
                ObjectField {
                    name: "entrypoints".into(),
                    value: args.entrypoints,
                },
                ObjectField {
                    name: "envs".into(),
                    value: args.envs,
                },
                ObjectField {
                    name: "gpus".into(),
                    value: args.gpus,
                },
                ObjectField {
                    name: "groupAdds".into(),
                    value: args.group_adds,
                },
                ObjectField {
                    name: "healthcheck".into(),
                    value: args.healthcheck,
                },
                ObjectField {
                    name: "hostname".into(),
                    value: args.hostname,
                },
                ObjectField {
                    name: "hosts".into(),
                    value: args.hosts,
                },
                ObjectField {
                    name: "image".into(),
                    value: args.image,
                },
                ObjectField {
                    name: "init".into(),
                    value: args.init,
                },
                ObjectField {
                    name: "ipcMode".into(),
                    value: args.ipc_mode,
                },
                ObjectField {
                    name: "labels".into(),
                    value: args.labels,
                },
                ObjectField {
                    name: "logDriver".into(),
                    value: args.log_driver,
                },
                ObjectField {
                    name: "logOpts".into(),
                    value: args.log_opts,
                },
                ObjectField {
                    name: "logs".into(),
                    value: args.logs,
                },
                ObjectField {
                    name: "maxRetryCount".into(),
                    value: args.max_retry_count,
                },
                ObjectField {
                    name: "memory".into(),
                    value: args.memory,
                },
                ObjectField {
                    name: "memorySwap".into(),
                    value: args.memory_swap,
                },
                ObjectField {
                    name: "mounts".into(),
                    value: args.mounts,
                },
                ObjectField {
                    name: "mustRun".into(),
                    value: args.must_run,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "networkMode".into(),
                    value: args.network_mode,
                },
                ObjectField {
                    name: "networksAdvanced".into(),
                    value: args.networks_advanced,
                },
                ObjectField {
                    name: "pidMode".into(),
                    value: args.pid_mode,
                },
                ObjectField {
                    name: "ports".into(),
                    value: args.ports,
                },
                ObjectField {
                    name: "privileged".into(),
                    value: args.privileged,
                },
                ObjectField {
                    name: "publishAllPorts".into(),
                    value: args.publish_all_ports,
                },
                ObjectField {
                    name: "readOnly".into(),
                    value: args.read_only,
                },
                ObjectField {
                    name: "removeVolumes".into(),
                    value: args.remove_volumes,
                },
                ObjectField {
                    name: "restart".into(),
                    value: args.restart,
                },
                ObjectField {
                    name: "rm".into(),
                    value: args.rm,
                },
                ObjectField {
                    name: "runtime".into(),
                    value: args.runtime,
                },
                ObjectField {
                    name: "securityOpts".into(),
                    value: args.security_opts,
                },
                ObjectField {
                    name: "shmSize".into(),
                    value: args.shm_size,
                },
                ObjectField {
                    name: "start".into(),
                    value: args.start,
                },
                ObjectField {
                    name: "stdinOpen".into(),
                    value: args.stdin_open,
                },
                ObjectField {
                    name: "stopSignal".into(),
                    value: args.stop_signal,
                },
                ObjectField {
                    name: "stopTimeout".into(),
                    value: args.stop_timeout,
                },
                ObjectField {
                    name: "storageOpts".into(),
                    value: args.storage_opts,
                },
                ObjectField {
                    name: "sysctls".into(),
                    value: args.sysctls,
                },
                ObjectField {
                    name: "tmpfs".into(),
                    value: args.tmpfs,
                },
                ObjectField {
                    name: "tty".into(),
                    value: args.tty,
                },
                ObjectField {
                    name: "ulimits".into(),
                    value: args.ulimits,
                },
                ObjectField {
                    name: "uploads".into(),
                    value: args.uploads,
                },
                ObjectField {
                    name: "user".into(),
                    value: args.user,
                },
                ObjectField {
                    name: "usernsMode".into(),
                    value: args.userns_mode,
                },
                ObjectField {
                    name: "volumes".into(),
                    value: args.volumes,
                },
                ObjectField {
                    name: "wait".into(),
                    value: args.wait,
                },
                ObjectField {
                    name: "waitTimeout".into(),
                    value: args.wait_timeout,
                },
                ObjectField {
                    name: "workingDir".into(),
                    value: args.working_dir,
                },
            ],
            results: vec![
                ResultField {
                    name: "attach".into(),
                },
                ResultField {
                    name: "bridge".into(),
                },
                ResultField {
                    name: "capabilities".into(),
                },
                ResultField {
                    name: "cgroupnsMode".into(),
                },
                ResultField {
                    name: "command".into(),
                },
                ResultField {
                    name: "containerLogs".into(),
                },
                ResultField {
                    name: "containerReadRefreshTimeoutMilliseconds".into(),
                },
                ResultField {
                    name: "cpuSet".into(),
                },
                ResultField {
                    name: "cpuShares".into(),
                },
                ResultField {
                    name: "destroyGraceSeconds".into(),
                },
                ResultField {
                    name: "devices".into(),
                },
                ResultField { name: "dns".into() },
                ResultField {
                    name: "dnsOpts".into(),
                },
                ResultField {
                    name: "dnsSearches".into(),
                },
                ResultField {
                    name: "domainname".into(),
                },
                ResultField {
                    name: "entrypoints".into(),
                },
                ResultField {
                    name: "envs".into(),
                },
                ResultField {
                    name: "exitCode".into(),
                },
                ResultField {
                    name: "gpus".into(),
                },
                ResultField {
                    name: "groupAdds".into(),
                },
                ResultField {
                    name: "healthcheck".into(),
                },
                ResultField {
                    name: "hostname".into(),
                },
                ResultField {
                    name: "hosts".into(),
                },
                ResultField {
                    name: "image".into(),
                },
                ResultField {
                    name: "init".into(),
                },
                ResultField {
                    name: "ipcMode".into(),
                },
                ResultField {
                    name: "labels".into(),
                },
                ResultField {
                    name: "logDriver".into(),
                },
                ResultField {
                    name: "logOpts".into(),
                },
                ResultField {
                    name: "logs".into(),
                },
                ResultField {
                    name: "maxRetryCount".into(),
                },
                ResultField {
                    name: "memory".into(),
                },
                ResultField {
                    name: "memorySwap".into(),
                },
                ResultField {
                    name: "mounts".into(),
                },
                ResultField {
                    name: "mustRun".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "networkDatas".into(),
                },
                ResultField {
                    name: "networkMode".into(),
                },
                ResultField {
                    name: "networksAdvanced".into(),
                },
                ResultField {
                    name: "pidMode".into(),
                },
                ResultField {
                    name: "ports".into(),
                },
                ResultField {
                    name: "privileged".into(),
                },
                ResultField {
                    name: "publishAllPorts".into(),
                },
                ResultField {
                    name: "readOnly".into(),
                },
                ResultField {
                    name: "removeVolumes".into(),
                },
                ResultField {
                    name: "restart".into(),
                },
                ResultField { name: "rm".into() },
                ResultField {
                    name: "runtime".into(),
                },
                ResultField {
                    name: "securityOpts".into(),
                },
                ResultField {
                    name: "shmSize".into(),
                },
                ResultField {
                    name: "start".into(),
                },
                ResultField {
                    name: "stdinOpen".into(),
                },
                ResultField {
                    name: "stopSignal".into(),
                },
                ResultField {
                    name: "stopTimeout".into(),
                },
                ResultField {
                    name: "storageOpts".into(),
                },
                ResultField {
                    name: "sysctls".into(),
                },
                ResultField {
                    name: "tmpfs".into(),
                },
                ResultField { name: "tty".into() },
                ResultField {
                    name: "ulimits".into(),
                },
                ResultField {
                    name: "uploads".into(),
                },
                ResultField {
                    name: "user".into(),
                },
                ResultField {
                    name: "usernsMode".into(),
                },
                ResultField {
                    name: "volumes".into(),
                },
                ResultField {
                    name: "wait".into(),
                },
                ResultField {
                    name: "waitTimeout".into(),
                },
                ResultField {
                    name: "workingDir".into(),
                },
            ],
        };

        let o = register(&request);

        container::Res {
            attach: o
                .fields
                .iter()
                .find(|o| o.name == "attach")
                .unwrap()
                .output
                .duplicate(),
            bridge: o
                .fields
                .iter()
                .find(|o| o.name == "bridge")
                .unwrap()
                .output
                .duplicate(),
            capabilities: o
                .fields
                .iter()
                .find(|o| o.name == "capabilities")
                .unwrap()
                .output
                .duplicate(),
            cgroupns_mode: o
                .fields
                .iter()
                .find(|o| o.name == "cgroupnsMode")
                .unwrap()
                .output
                .duplicate(),
            command: o
                .fields
                .iter()
                .find(|o| o.name == "command")
                .unwrap()
                .output
                .duplicate(),
            container_logs: o
                .fields
                .iter()
                .find(|o| o.name == "containerLogs")
                .unwrap()
                .output
                .duplicate(),
            container_read_refresh_timeout_milliseconds: o
                .fields
                .iter()
                .find(|o| o.name == "containerReadRefreshTimeoutMilliseconds")
                .unwrap()
                .output
                .duplicate(),
            cpu_set: o
                .fields
                .iter()
                .find(|o| o.name == "cpuSet")
                .unwrap()
                .output
                .duplicate(),
            cpu_shares: o
                .fields
                .iter()
                .find(|o| o.name == "cpuShares")
                .unwrap()
                .output
                .duplicate(),
            destroy_grace_seconds: o
                .fields
                .iter()
                .find(|o| o.name == "destroyGraceSeconds")
                .unwrap()
                .output
                .duplicate(),
            devices: o
                .fields
                .iter()
                .find(|o| o.name == "devices")
                .unwrap()
                .output
                .duplicate(),
            dns: o
                .fields
                .iter()
                .find(|o| o.name == "dns")
                .unwrap()
                .output
                .duplicate(),
            dns_opts: o
                .fields
                .iter()
                .find(|o| o.name == "dnsOpts")
                .unwrap()
                .output
                .duplicate(),
            dns_searches: o
                .fields
                .iter()
                .find(|o| o.name == "dnsSearches")
                .unwrap()
                .output
                .duplicate(),
            domainname: o
                .fields
                .iter()
                .find(|o| o.name == "domainname")
                .unwrap()
                .output
                .duplicate(),
            entrypoints: o
                .fields
                .iter()
                .find(|o| o.name == "entrypoints")
                .unwrap()
                .output
                .duplicate(),
            envs: o
                .fields
                .iter()
                .find(|o| o.name == "envs")
                .unwrap()
                .output
                .duplicate(),
            exit_code: o
                .fields
                .iter()
                .find(|o| o.name == "exitCode")
                .unwrap()
                .output
                .duplicate(),
            gpus: o
                .fields
                .iter()
                .find(|o| o.name == "gpus")
                .unwrap()
                .output
                .duplicate(),
            group_adds: o
                .fields
                .iter()
                .find(|o| o.name == "groupAdds")
                .unwrap()
                .output
                .duplicate(),
            healthcheck: o
                .fields
                .iter()
                .find(|o| o.name == "healthcheck")
                .unwrap()
                .output
                .duplicate(),
            hostname: o
                .fields
                .iter()
                .find(|o| o.name == "hostname")
                .unwrap()
                .output
                .duplicate(),
            hosts: o
                .fields
                .iter()
                .find(|o| o.name == "hosts")
                .unwrap()
                .output
                .duplicate(),
            image: o
                .fields
                .iter()
                .find(|o| o.name == "image")
                .unwrap()
                .output
                .duplicate(),
            init: o
                .fields
                .iter()
                .find(|o| o.name == "init")
                .unwrap()
                .output
                .duplicate(),
            ipc_mode: o
                .fields
                .iter()
                .find(|o| o.name == "ipcMode")
                .unwrap()
                .output
                .duplicate(),
            labels: o
                .fields
                .iter()
                .find(|o| o.name == "labels")
                .unwrap()
                .output
                .duplicate(),
            log_driver: o
                .fields
                .iter()
                .find(|o| o.name == "logDriver")
                .unwrap()
                .output
                .duplicate(),
            log_opts: o
                .fields
                .iter()
                .find(|o| o.name == "logOpts")
                .unwrap()
                .output
                .duplicate(),
            logs: o
                .fields
                .iter()
                .find(|o| o.name == "logs")
                .unwrap()
                .output
                .duplicate(),
            max_retry_count: o
                .fields
                .iter()
                .find(|o| o.name == "maxRetryCount")
                .unwrap()
                .output
                .duplicate(),
            memory: o
                .fields
                .iter()
                .find(|o| o.name == "memory")
                .unwrap()
                .output
                .duplicate(),
            memory_swap: o
                .fields
                .iter()
                .find(|o| o.name == "memorySwap")
                .unwrap()
                .output
                .duplicate(),
            mounts: o
                .fields
                .iter()
                .find(|o| o.name == "mounts")
                .unwrap()
                .output
                .duplicate(),
            must_run: o
                .fields
                .iter()
                .find(|o| o.name == "mustRun")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            network_datas: o
                .fields
                .iter()
                .find(|o| o.name == "networkDatas")
                .unwrap()
                .output
                .duplicate(),
            network_mode: o
                .fields
                .iter()
                .find(|o| o.name == "networkMode")
                .unwrap()
                .output
                .duplicate(),
            networks_advanced: o
                .fields
                .iter()
                .find(|o| o.name == "networksAdvanced")
                .unwrap()
                .output
                .duplicate(),
            pid_mode: o
                .fields
                .iter()
                .find(|o| o.name == "pidMode")
                .unwrap()
                .output
                .duplicate(),
            ports: o
                .fields
                .iter()
                .find(|o| o.name == "ports")
                .unwrap()
                .output
                .duplicate(),
            privileged: o
                .fields
                .iter()
                .find(|o| o.name == "privileged")
                .unwrap()
                .output
                .duplicate(),
            publish_all_ports: o
                .fields
                .iter()
                .find(|o| o.name == "publishAllPorts")
                .unwrap()
                .output
                .duplicate(),
            read_only: o
                .fields
                .iter()
                .find(|o| o.name == "readOnly")
                .unwrap()
                .output
                .duplicate(),
            remove_volumes: o
                .fields
                .iter()
                .find(|o| o.name == "removeVolumes")
                .unwrap()
                .output
                .duplicate(),
            restart: o
                .fields
                .iter()
                .find(|o| o.name == "restart")
                .unwrap()
                .output
                .duplicate(),
            rm: o
                .fields
                .iter()
                .find(|o| o.name == "rm")
                .unwrap()
                .output
                .duplicate(),
            runtime: o
                .fields
                .iter()
                .find(|o| o.name == "runtime")
                .unwrap()
                .output
                .duplicate(),
            security_opts: o
                .fields
                .iter()
                .find(|o| o.name == "securityOpts")
                .unwrap()
                .output
                .duplicate(),
            shm_size: o
                .fields
                .iter()
                .find(|o| o.name == "shmSize")
                .unwrap()
                .output
                .duplicate(),
            start: o
                .fields
                .iter()
                .find(|o| o.name == "start")
                .unwrap()
                .output
                .duplicate(),
            stdin_open: o
                .fields
                .iter()
                .find(|o| o.name == "stdinOpen")
                .unwrap()
                .output
                .duplicate(),
            stop_signal: o
                .fields
                .iter()
                .find(|o| o.name == "stopSignal")
                .unwrap()
                .output
                .duplicate(),
            stop_timeout: o
                .fields
                .iter()
                .find(|o| o.name == "stopTimeout")
                .unwrap()
                .output
                .duplicate(),
            storage_opts: o
                .fields
                .iter()
                .find(|o| o.name == "storageOpts")
                .unwrap()
                .output
                .duplicate(),
            sysctls: o
                .fields
                .iter()
                .find(|o| o.name == "sysctls")
                .unwrap()
                .output
                .duplicate(),
            tmpfs: o
                .fields
                .iter()
                .find(|o| o.name == "tmpfs")
                .unwrap()
                .output
                .duplicate(),
            tty: o
                .fields
                .iter()
                .find(|o| o.name == "tty")
                .unwrap()
                .output
                .duplicate(),
            ulimits: o
                .fields
                .iter()
                .find(|o| o.name == "ulimits")
                .unwrap()
                .output
                .duplicate(),
            uploads: o
                .fields
                .iter()
                .find(|o| o.name == "uploads")
                .unwrap()
                .output
                .duplicate(),
            user: o
                .fields
                .iter()
                .find(|o| o.name == "user")
                .unwrap()
                .output
                .duplicate(),
            userns_mode: o
                .fields
                .iter()
                .find(|o| o.name == "usernsMode")
                .unwrap()
                .output
                .duplicate(),
            volumes: o
                .fields
                .iter()
                .find(|o| o.name == "volumes")
                .unwrap()
                .output
                .duplicate(),
            wait: o
                .fields
                .iter()
                .find(|o| o.name == "wait")
                .unwrap()
                .output
                .duplicate(),
            wait_timeout: o
                .fields
                .iter()
                .find(|o| o.name == "waitTimeout")
                .unwrap()
                .output
                .duplicate(),
            working_dir: o
                .fields
                .iter()
                .find(|o| o.name == "workingDir")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
impl image::Guest for Component {
    fn invoke(name: String, args: image::Args) -> image::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/image:Image".into(),
            name,
            object: vec![
                ObjectField {
                    name: "build".into(),
                    value: args.build,
                },
                ObjectField {
                    name: "buildOnPreview".into(),
                    value: args.build_on_preview,
                },
                ObjectField {
                    name: "imageName".into(),
                    value: args.image_name,
                },
                ObjectField {
                    name: "registry".into(),
                    value: args.registry,
                },
                ObjectField {
                    name: "skipPush".into(),
                    value: args.skip_push,
                },
            ],
            results: vec![
                ResultField {
                    name: "baseImageName".into(),
                },
                ResultField {
                    name: "context".into(),
                },
                ResultField {
                    name: "dockerfile".into(),
                },
                ResultField {
                    name: "imageName".into(),
                },
                ResultField {
                    name: "platform".into(),
                },
                ResultField {
                    name: "registryServer".into(),
                },
                ResultField {
                    name: "repoDigest".into(),
                },
            ],
        };

        let o = register(&request);

        image::Res {
            base_image_name: o
                .fields
                .iter()
                .find(|o| o.name == "baseImageName")
                .unwrap()
                .output
                .duplicate(),
            context: o
                .fields
                .iter()
                .find(|o| o.name == "context")
                .unwrap()
                .output
                .duplicate(),
            dockerfile: o
                .fields
                .iter()
                .find(|o| o.name == "dockerfile")
                .unwrap()
                .output
                .duplicate(),
            image_name: o
                .fields
                .iter()
                .find(|o| o.name == "imageName")
                .unwrap()
                .output
                .duplicate(),
            platform: o
                .fields
                .iter()
                .find(|o| o.name == "platform")
                .unwrap()
                .output
                .duplicate(),
            registry_server: o
                .fields
                .iter()
                .find(|o| o.name == "registryServer")
                .unwrap()
                .output
                .duplicate(),
            repo_digest: o
                .fields
                .iter()
                .find(|o| o.name == "repoDigest")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
impl network::Guest for Component {
    fn invoke(name: String, args: network::Args) -> network::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/network:Network".into(),
            name,
            object: vec![
                ObjectField {
                    name: "attachable".into(),
                    value: args.attachable,
                },
                ObjectField {
                    name: "checkDuplicate".into(),
                    value: args.check_duplicate,
                },
                ObjectField {
                    name: "driver".into(),
                    value: args.driver,
                },
                ObjectField {
                    name: "ingress".into(),
                    value: args.ingress,
                },
                ObjectField {
                    name: "internal".into(),
                    value: args.internal,
                },
                ObjectField {
                    name: "ipamConfigs".into(),
                    value: args.ipam_configs,
                },
                ObjectField {
                    name: "ipamDriver".into(),
                    value: args.ipam_driver,
                },
                ObjectField {
                    name: "ipamOptions".into(),
                    value: args.ipam_options,
                },
                ObjectField {
                    name: "ipv6".into(),
                    value: args.ipv6,
                },
                ObjectField {
                    name: "labels".into(),
                    value: args.labels,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "options".into(),
                    value: args.options,
                },
            ],
            results: vec![
                ResultField {
                    name: "attachable".into(),
                },
                ResultField {
                    name: "checkDuplicate".into(),
                },
                ResultField {
                    name: "driver".into(),
                },
                ResultField {
                    name: "ingress".into(),
                },
                ResultField {
                    name: "internal".into(),
                },
                ResultField {
                    name: "ipamConfigs".into(),
                },
                ResultField {
                    name: "ipamDriver".into(),
                },
                ResultField {
                    name: "ipamOptions".into(),
                },
                ResultField {
                    name: "ipv6".into(),
                },
                ResultField {
                    name: "labels".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "options".into(),
                },
                ResultField {
                    name: "scope".into(),
                },
            ],
        };

        let o = register(&request);

        network::Res {
            attachable: o
                .fields
                .iter()
                .find(|o| o.name == "attachable")
                .unwrap()
                .output
                .duplicate(),
            check_duplicate: o
                .fields
                .iter()
                .find(|o| o.name == "checkDuplicate")
                .unwrap()
                .output
                .duplicate(),
            driver: o
                .fields
                .iter()
                .find(|o| o.name == "driver")
                .unwrap()
                .output
                .duplicate(),
            ingress: o
                .fields
                .iter()
                .find(|o| o.name == "ingress")
                .unwrap()
                .output
                .duplicate(),
            internal: o
                .fields
                .iter()
                .find(|o| o.name == "internal")
                .unwrap()
                .output
                .duplicate(),
            ipam_configs: o
                .fields
                .iter()
                .find(|o| o.name == "ipamConfigs")
                .unwrap()
                .output
                .duplicate(),
            ipam_driver: o
                .fields
                .iter()
                .find(|o| o.name == "ipamDriver")
                .unwrap()
                .output
                .duplicate(),
            ipam_options: o
                .fields
                .iter()
                .find(|o| o.name == "ipamOptions")
                .unwrap()
                .output
                .duplicate(),
            ipv6: o
                .fields
                .iter()
                .find(|o| o.name == "ipv6")
                .unwrap()
                .output
                .duplicate(),
            labels: o
                .fields
                .iter()
                .find(|o| o.name == "labels")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            options: o
                .fields
                .iter()
                .find(|o| o.name == "options")
                .unwrap()
                .output
                .duplicate(),
            scope: o
                .fields
                .iter()
                .find(|o| o.name == "scope")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
impl plugin::Guest for Component {
    fn invoke(name: String, args: plugin::Args) -> plugin::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/plugin:Plugin".into(),
            name,
            object: vec![
                ObjectField {
                    name: "alias".into(),
                    value: args.alias,
                },
                ObjectField {
                    name: "enableTimeout".into(),
                    value: args.enable_timeout,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "envs".into(),
                    value: args.envs,
                },
                ObjectField {
                    name: "forceDestroy".into(),
                    value: args.force_destroy,
                },
                ObjectField {
                    name: "forceDisable".into(),
                    value: args.force_disable,
                },
                ObjectField {
                    name: "grantAllPermissions".into(),
                    value: args.grant_all_permissions,
                },
                ObjectField {
                    name: "grantPermissions".into(),
                    value: args.grant_permissions,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "alias".into(),
                },
                ResultField {
                    name: "enableTimeout".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "envs".into(),
                },
                ResultField {
                    name: "forceDestroy".into(),
                },
                ResultField {
                    name: "forceDisable".into(),
                },
                ResultField {
                    name: "grantAllPermissions".into(),
                },
                ResultField {
                    name: "grantPermissions".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "pluginReference".into(),
                },
            ],
        };

        let o = register(&request);

        plugin::Res {
            alias: o
                .fields
                .iter()
                .find(|o| o.name == "alias")
                .unwrap()
                .output
                .duplicate(),
            enable_timeout: o
                .fields
                .iter()
                .find(|o| o.name == "enableTimeout")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            envs: o
                .fields
                .iter()
                .find(|o| o.name == "envs")
                .unwrap()
                .output
                .duplicate(),
            force_destroy: o
                .fields
                .iter()
                .find(|o| o.name == "forceDestroy")
                .unwrap()
                .output
                .duplicate(),
            force_disable: o
                .fields
                .iter()
                .find(|o| o.name == "forceDisable")
                .unwrap()
                .output
                .duplicate(),
            grant_all_permissions: o
                .fields
                .iter()
                .find(|o| o.name == "grantAllPermissions")
                .unwrap()
                .output
                .duplicate(),
            grant_permissions: o
                .fields
                .iter()
                .find(|o| o.name == "grantPermissions")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            plugin_reference: o
                .fields
                .iter()
                .find(|o| o.name == "pluginReference")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
impl registry_image::Guest for Component {
    fn invoke(name: String, args: registry_image::Args) -> registry_image::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/registryImage:RegistryImage".into(),
            name,
            object: vec![
                ObjectField {
                    name: "insecureSkipVerify".into(),
                    value: args.insecure_skip_verify,
                },
                ObjectField {
                    name: "keepRemotely".into(),
                    value: args.keep_remotely,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "triggers".into(),
                    value: args.triggers,
                },
            ],
            results: vec![
                ResultField {
                    name: "insecureSkipVerify".into(),
                },
                ResultField {
                    name: "keepRemotely".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "sha256Digest".into(),
                },
                ResultField {
                    name: "triggers".into(),
                },
            ],
        };

        let o = register(&request);

        registry_image::Res {
            insecure_skip_verify: o
                .fields
                .iter()
                .find(|o| o.name == "insecureSkipVerify")
                .unwrap()
                .output
                .duplicate(),
            keep_remotely: o
                .fields
                .iter()
                .find(|o| o.name == "keepRemotely")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            sha256_digest: o
                .fields
                .iter()
                .find(|o| o.name == "sha256Digest")
                .unwrap()
                .output
                .duplicate(),
            triggers: o
                .fields
                .iter()
                .find(|o| o.name == "triggers")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
impl remote_image::Guest for Component {
    fn invoke(name: String, args: remote_image::Args) -> remote_image::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/remoteImage:RemoteImage".into(),
            name,
            object: vec![
                ObjectField {
                    name: "build".into(),
                    value: args.build,
                },
                ObjectField {
                    name: "forceRemove".into(),
                    value: args.force_remove,
                },
                ObjectField {
                    name: "keepLocally".into(),
                    value: args.keep_locally,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "platform".into(),
                    value: args.platform,
                },
                ObjectField {
                    name: "pullTriggers".into(),
                    value: args.pull_triggers,
                },
                ObjectField {
                    name: "triggers".into(),
                    value: args.triggers,
                },
            ],
            results: vec![
                ResultField {
                    name: "build".into(),
                },
                ResultField {
                    name: "forceRemove".into(),
                },
                ResultField {
                    name: "imageId".into(),
                },
                ResultField {
                    name: "keepLocally".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "platform".into(),
                },
                ResultField {
                    name: "pullTriggers".into(),
                },
                ResultField {
                    name: "repoDigest".into(),
                },
                ResultField {
                    name: "triggers".into(),
                },
            ],
        };

        let o = register(&request);

        remote_image::Res {
            build: o
                .fields
                .iter()
                .find(|o| o.name == "build")
                .unwrap()
                .output
                .duplicate(),
            force_remove: o
                .fields
                .iter()
                .find(|o| o.name == "forceRemove")
                .unwrap()
                .output
                .duplicate(),
            image_id: o
                .fields
                .iter()
                .find(|o| o.name == "imageId")
                .unwrap()
                .output
                .duplicate(),
            keep_locally: o
                .fields
                .iter()
                .find(|o| o.name == "keepLocally")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            platform: o
                .fields
                .iter()
                .find(|o| o.name == "platform")
                .unwrap()
                .output
                .duplicate(),
            pull_triggers: o
                .fields
                .iter()
                .find(|o| o.name == "pullTriggers")
                .unwrap()
                .output
                .duplicate(),
            repo_digest: o
                .fields
                .iter()
                .find(|o| o.name == "repoDigest")
                .unwrap()
                .output
                .duplicate(),
            triggers: o
                .fields
                .iter()
                .find(|o| o.name == "triggers")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
impl secret::Guest for Component {
    fn invoke(name: String, args: secret::Args) -> secret::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/secret:Secret".into(),
            name,
            object: vec![
                ObjectField {
                    name: "data".into(),
                    value: args.data,
                },
                ObjectField {
                    name: "labels".into(),
                    value: args.labels,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "data".into(),
                },
                ResultField {
                    name: "labels".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        secret::Res {
            data: o
                .fields
                .iter()
                .find(|o| o.name == "data")
                .unwrap()
                .output
                .duplicate(),
            labels: o
                .fields
                .iter()
                .find(|o| o.name == "labels")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
impl service::Guest for Component {
    fn invoke(name: String, args: service::Args) -> service::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/service:Service".into(),
            name,
            object: vec![
                ObjectField {
                    name: "auth".into(),
                    value: args.auth,
                },
                ObjectField {
                    name: "convergeConfig".into(),
                    value: args.converge_config,
                },
                ObjectField {
                    name: "endpointSpec".into(),
                    value: args.endpoint_spec,
                },
                ObjectField {
                    name: "labels".into(),
                    value: args.labels,
                },
                ObjectField {
                    name: "mode".into(),
                    value: args.mode,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "rollbackConfig".into(),
                    value: args.rollback_config,
                },
                ObjectField {
                    name: "taskSpec".into(),
                    value: args.task_spec,
                },
                ObjectField {
                    name: "updateConfig".into(),
                    value: args.update_config,
                },
            ],
            results: vec![
                ResultField {
                    name: "auth".into(),
                },
                ResultField {
                    name: "convergeConfig".into(),
                },
                ResultField {
                    name: "endpointSpec".into(),
                },
                ResultField {
                    name: "labels".into(),
                },
                ResultField {
                    name: "mode".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "rollbackConfig".into(),
                },
                ResultField {
                    name: "taskSpec".into(),
                },
                ResultField {
                    name: "updateConfig".into(),
                },
            ],
        };

        let o = register(&request);

        service::Res {
            auth: o
                .fields
                .iter()
                .find(|o| o.name == "auth")
                .unwrap()
                .output
                .duplicate(),
            converge_config: o
                .fields
                .iter()
                .find(|o| o.name == "convergeConfig")
                .unwrap()
                .output
                .duplicate(),
            endpoint_spec: o
                .fields
                .iter()
                .find(|o| o.name == "endpointSpec")
                .unwrap()
                .output
                .duplicate(),
            labels: o
                .fields
                .iter()
                .find(|o| o.name == "labels")
                .unwrap()
                .output
                .duplicate(),
            mode: o
                .fields
                .iter()
                .find(|o| o.name == "mode")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            rollback_config: o
                .fields
                .iter()
                .find(|o| o.name == "rollbackConfig")
                .unwrap()
                .output
                .duplicate(),
            task_spec: o
                .fields
                .iter()
                .find(|o| o.name == "taskSpec")
                .unwrap()
                .output
                .duplicate(),
            update_config: o
                .fields
                .iter()
                .find(|o| o.name == "updateConfig")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
impl service_config::Guest for Component {
    fn invoke(name: String, args: service_config::Args) -> service_config::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/serviceConfig:ServiceConfig".into(),
            name,
            object: vec![
                ObjectField {
                    name: "data".into(),
                    value: args.data,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "data".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        service_config::Res {
            data: o
                .fields
                .iter()
                .find(|o| o.name == "data")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
impl tag::Guest for Component {
    fn invoke(name: String, args: tag::Args) -> tag::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/tag:Tag".into(),
            name,
            object: vec![
                ObjectField {
                    name: "sourceImage".into(),
                    value: args.source_image,
                },
                ObjectField {
                    name: "targetImage".into(),
                    value: args.target_image,
                },
            ],
            results: vec![
                ResultField {
                    name: "sourceImage".into(),
                },
                ResultField {
                    name: "sourceImageId".into(),
                },
                ResultField {
                    name: "targetImage".into(),
                },
            ],
        };

        let o = register(&request);

        tag::Res {
            source_image: o
                .fields
                .iter()
                .find(|o| o.name == "sourceImage")
                .unwrap()
                .output
                .duplicate(),
            source_image_id: o
                .fields
                .iter()
                .find(|o| o.name == "sourceImageId")
                .unwrap()
                .output
                .duplicate(),
            target_image: o
                .fields
                .iter()
                .find(|o| o.name == "targetImage")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
impl volume::Guest for Component {
    fn invoke(name: String, args: volume::Args) -> volume::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/volume:Volume".into(),
            name,
            object: vec![
                ObjectField {
                    name: "driver".into(),
                    value: args.driver,
                },
                ObjectField {
                    name: "driverOpts".into(),
                    value: args.driver_opts,
                },
                ObjectField {
                    name: "labels".into(),
                    value: args.labels,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "driver".into(),
                },
                ResultField {
                    name: "driverOpts".into(),
                },
                ResultField {
                    name: "labels".into(),
                },
                ResultField {
                    name: "mountpoint".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        volume::Res {
            driver: o
                .fields
                .iter()
                .find(|o| o.name == "driver")
                .unwrap()
                .output
                .duplicate(),
            driver_opts: o
                .fields
                .iter()
                .find(|o| o.name == "driverOpts")
                .unwrap()
                .output
                .duplicate(),
            labels: o
                .fields
                .iter()
                .find(|o| o.name == "labels")
                .unwrap()
                .output
                .duplicate(),
            mountpoint: o
                .fields
                .iter()
                .find(|o| o.name == "mountpoint")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
