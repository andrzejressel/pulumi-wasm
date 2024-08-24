use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::docker::container;
use crate::Component;
use std::collections::HashMap;

impl container::Guest for Component {
    fn invoke(name: String, args: container::Args) -> container::Res {
        pulumi_wasm_common::setup_logger();
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

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        container::Res {
            attach: hashmap.remove("attach").unwrap(),
            bridge: hashmap.remove("bridge").unwrap(),
            capabilities: hashmap.remove("capabilities").unwrap(),
            cgroupns_mode: hashmap.remove("cgroupnsMode").unwrap(),
            command: hashmap.remove("command").unwrap(),
            container_logs: hashmap.remove("containerLogs").unwrap(),
            container_read_refresh_timeout_milliseconds: hashmap
                .remove("containerReadRefreshTimeoutMilliseconds")
                .unwrap(),
            cpu_set: hashmap.remove("cpuSet").unwrap(),
            cpu_shares: hashmap.remove("cpuShares").unwrap(),
            destroy_grace_seconds: hashmap.remove("destroyGraceSeconds").unwrap(),
            devices: hashmap.remove("devices").unwrap(),
            dns: hashmap.remove("dns").unwrap(),
            dns_opts: hashmap.remove("dnsOpts").unwrap(),
            dns_searches: hashmap.remove("dnsSearches").unwrap(),
            domainname: hashmap.remove("domainname").unwrap(),
            entrypoints: hashmap.remove("entrypoints").unwrap(),
            envs: hashmap.remove("envs").unwrap(),
            exit_code: hashmap.remove("exitCode").unwrap(),
            gpus: hashmap.remove("gpus").unwrap(),
            group_adds: hashmap.remove("groupAdds").unwrap(),
            healthcheck: hashmap.remove("healthcheck").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            hosts: hashmap.remove("hosts").unwrap(),
            image: hashmap.remove("image").unwrap(),
            init: hashmap.remove("init").unwrap(),
            ipc_mode: hashmap.remove("ipcMode").unwrap(),
            labels: hashmap.remove("labels").unwrap(),
            log_driver: hashmap.remove("logDriver").unwrap(),
            log_opts: hashmap.remove("logOpts").unwrap(),
            logs: hashmap.remove("logs").unwrap(),
            max_retry_count: hashmap.remove("maxRetryCount").unwrap(),
            memory: hashmap.remove("memory").unwrap(),
            memory_swap: hashmap.remove("memorySwap").unwrap(),
            mounts: hashmap.remove("mounts").unwrap(),
            must_run: hashmap.remove("mustRun").unwrap(),
            name: hashmap.remove("name").unwrap(),
            network_datas: hashmap.remove("networkDatas").unwrap(),
            network_mode: hashmap.remove("networkMode").unwrap(),
            networks_advanced: hashmap.remove("networksAdvanced").unwrap(),
            pid_mode: hashmap.remove("pidMode").unwrap(),
            ports: hashmap.remove("ports").unwrap(),
            privileged: hashmap.remove("privileged").unwrap(),
            publish_all_ports: hashmap.remove("publishAllPorts").unwrap(),
            read_only: hashmap.remove("readOnly").unwrap(),
            remove_volumes: hashmap.remove("removeVolumes").unwrap(),
            restart: hashmap.remove("restart").unwrap(),
            rm: hashmap.remove("rm").unwrap(),
            runtime: hashmap.remove("runtime").unwrap(),
            security_opts: hashmap.remove("securityOpts").unwrap(),
            shm_size: hashmap.remove("shmSize").unwrap(),
            start: hashmap.remove("start").unwrap(),
            stdin_open: hashmap.remove("stdinOpen").unwrap(),
            stop_signal: hashmap.remove("stopSignal").unwrap(),
            stop_timeout: hashmap.remove("stopTimeout").unwrap(),
            storage_opts: hashmap.remove("storageOpts").unwrap(),
            sysctls: hashmap.remove("sysctls").unwrap(),
            tmpfs: hashmap.remove("tmpfs").unwrap(),
            tty: hashmap.remove("tty").unwrap(),
            ulimits: hashmap.remove("ulimits").unwrap(),
            uploads: hashmap.remove("uploads").unwrap(),
            user: hashmap.remove("user").unwrap(),
            userns_mode: hashmap.remove("usernsMode").unwrap(),
            volumes: hashmap.remove("volumes").unwrap(),
            wait: hashmap.remove("wait").unwrap(),
            wait_timeout: hashmap.remove("waitTimeout").unwrap(),
            working_dir: hashmap.remove("workingDir").unwrap(),
        }
    }
}
