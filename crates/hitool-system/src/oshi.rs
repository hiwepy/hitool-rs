//! OSHI-aligned hardware and operating-system snapshots backed by `sysinfo`.

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

/// CPU tick counters. Platforms that do not expose a counter leave it at zero.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CpuTicks {
    /// Idle ticks.
    pub idle: u64,
    /// Nice-priority ticks.
    pub nice: u64,
    /// Hardware interrupt ticks.
    pub irq: u64,
    /// Software interrupt ticks.
    pub soft_irq: u64,
    /// Hypervisor steal ticks.
    pub steal: u64,
    /// System ticks.
    pub system: u64,
    /// User ticks.
    pub user: u64,
    /// I/O wait ticks.
    pub io_wait: u64,
}

impl CpuTicks {
    /// Creates a complete tick snapshot.
    #[must_use]
    pub const fn new(values: [u64; 8]) -> Self {
        Self {
            idle: values[0],
            nice: values[1],
            irq: values[2],
            soft_irq: values[3],
            steal: values[4],
            system: values[5],
            user: values[6],
            io_wait: values[7],
        }
    }

    /// Returns the saturating total of every counter.
    #[must_use]
    pub fn total_cpu(self) -> u64 {
        [
            self.idle,
            self.nice,
            self.irq,
            self.soft_irq,
            self.steal,
            self.system,
            self.user,
            self.io_wait,
        ]
        .into_iter()
        .fold(0, u64::saturating_add)
    }
}

/// Portable CPU utilization view corresponding to Hutool's `CpuInfo`.
#[derive(Debug, Clone, PartialEq)]
pub struct CpuInfo {
    /// Logical CPU count.
    pub cpu_num: usize,
    /// Total busy percentage.
    pub total: f32,
    /// System percentage when available.
    pub system: f32,
    /// User percentage when available.
    pub user: f32,
    /// I/O wait percentage when available.
    pub wait: f32,
    /// Idle percentage.
    pub free: f32,
    /// Processor brand/model.
    pub cpu_model: String,
    /// Raw tick snapshot when available.
    pub ticks: CpuTicks,
}

impl CpuInfo {
    /// Creates a normalized CPU snapshot.
    #[must_use]
    pub fn new(cpu_num: usize, used: f32, cpu_model: impl Into<String>, ticks: CpuTicks) -> Self {
        let total = used.clamp(0.0, 100.0);
        Self {
            cpu_num,
            total,
            system: 0.0,
            user: total,
            wait: 0.0,
            free: 100.0 - total,
            cpu_model: cpu_model.into(),
            ticks,
        }
    }

    /// Returns total used percentage.
    #[must_use]
    pub const fn used(&self) -> f32 {
        self.total
    }
}

/// Process snapshot returned by [`OshiUtil::current_process`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessInfo {
    /// Process identifier.
    pub pid: u32,
    /// Executable/process name.
    pub name: String,
    /// Resident memory in bytes.
    pub memory: u64,
    /// Virtual memory in bytes.
    pub virtual_memory: u64,
    /// Process runtime in seconds.
    pub run_time: u64,
}

/// Physical memory snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryInfo {
    /// Total memory bytes.
    pub total: u64,
    /// Used memory bytes.
    pub used: u64,
    /// Available memory bytes.
    pub available: u64,
    /// Total swap bytes.
    pub swap_total: u64,
    /// Used swap bytes.
    pub swap_used: u64,
}

/// Disk-store snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskInfo {
    /// Device name.
    pub name: String,
    /// Mount point.
    pub mount_point: String,
    /// Total bytes.
    pub total_space: u64,
    /// Available bytes.
    pub available_space: u64,
}

/// Network-interface transfer snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkInfo {
    /// Interface name.
    pub name: String,
    /// Total received bytes since boot/counter reset.
    pub received: u64,
    /// Total transmitted bytes since boot/counter reset.
    pub transmitted: u64,
}

/// Hardware sensor snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct SensorInfo {
    /// Sensor label.
    pub label: String,
    /// Temperature in Celsius, if reported.
    pub temperature: Option<f32>,
}

/// Aggregate hardware snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareInfo {
    /// CPU information.
    pub cpu: CpuInfo,
    /// Memory information.
    pub memory: MemoryInfo,
    /// Disk stores.
    pub disks: Vec<DiskInfo>,
    /// Network interfaces.
    pub networks: Vec<NetworkInfo>,
    /// Hardware sensors.
    pub sensors: Vec<SensorInfo>,
}

/// Mature `sysinfo`-backed counterpart of Hutool's `OshiUtil`.
#[derive(Debug, Clone, Copy, Default)]
pub struct OshiUtil;

impl OshiUtil {
    /// Returns a fully refreshed `sysinfo` system.
    #[must_use]
    pub fn system() -> System {
        System::new_all()
    }

    /// Returns the current process when it remains visible during collection.
    #[must_use]
    pub fn current_process() -> Option<ProcessInfo> {
        let system = Self::system();
        let pid = Pid::from_u32(std::process::id());
        process_info(&system, pid)
    }

    /// Looks up a process in an existing system snapshot.
    #[must_use]
    pub fn process(system: &System, pid: u32) -> Option<ProcessInfo> {
        process_info(system, Pid::from_u32(pid))
    }

    /// Returns current physical and swap memory counters.
    #[must_use]
    pub fn memory() -> MemoryInfo {
        let system = Self::system();
        MemoryInfo {
            total: system.total_memory(),
            used: system.used_memory(),
            available: system.available_memory(),
            swap_total: system.total_swap(),
            swap_used: system.used_swap(),
        }
    }

    /// Samples aggregate CPU utilization after `interval`.
    #[must_use]
    pub fn cpu_info(interval: Duration) -> CpuInfo {
        let mut system = System::new_all();
        if !interval.is_zero() {
            thread::sleep(interval);
            system.refresh_cpu_usage();
        }
        let cpus = system.cpus();
        let model = cpus
            .first()
            .map_or_else(String::new, |cpu| cpu.brand().to_owned());
        CpuInfo::new(
            cpus.len(),
            system.global_cpu_usage(),
            model,
            CpuTicks::default(),
        )
    }

    /// Collects disk-store summaries.
    #[must_use]
    pub fn disk_stores() -> Vec<DiskInfo> {
        Disks::new_with_refreshed_list()
            .list()
            .iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().into_owned(),
                mount_point: disk.mount_point().to_string_lossy().into_owned(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
            })
            .collect()
    }

    /// Collects network-interface counters.
    #[must_use]
    pub fn network_interfaces() -> Vec<NetworkInfo> {
        Networks::new_with_refreshed_list()
            .iter()
            .map(|(name, data)| NetworkInfo {
                name: name.clone(),
                received: data.total_received(),
                transmitted: data.total_transmitted(),
            })
            .collect()
    }

    /// Collects temperature sensors supported by the host.
    #[must_use]
    pub fn sensors() -> Vec<SensorInfo> {
        Components::new_with_refreshed_list()
            .iter()
            .map(|component| SensorInfo {
                label: component.label().to_owned(),
                temperature: component.temperature(),
            })
            .collect()
    }

    /// Collects the complete hardware view.
    #[must_use]
    pub fn hardware() -> HardwareInfo {
        HardwareInfo {
            cpu: Self::cpu_info(Duration::ZERO),
            memory: Self::memory(),
            disks: Self::disk_stores(),
            networks: Self::network_interfaces(),
            sensors: Self::sensors(),
        }
    }
}

fn process_info(system: &System, pid: Pid) -> Option<ProcessInfo> {
    let process = system.process(pid)?;
    Some(ProcessInfo {
        pid: pid.as_u32(),
        name: process.name().to_string_lossy().into_owned(),
        memory: process.memory(),
        virtual_memory: process.virtual_memory(),
        run_time: process.run_time(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticks_and_cpu_models_are_normalized_and_mutable_as_rust_data() {
        let ticks = CpuTicks::new([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(ticks.total_cpu(), 36);
        let saturated = CpuTicks::new([u64::MAX; 8]);
        assert_eq!(saturated.total_cpu(), u64::MAX);

        let mut cpu = CpuInfo::new(8, 120.0, "test", ticks);
        assert!((cpu.used() - 100.0).abs() < f32::EPSILON);
        assert!(cpu.free.abs() < f32::EPSILON);
        cpu.system = 10.0;
        cpu.user = 80.0;
        cpu.wait = 10.0;
        cpu.cpu_num = 4;
        cpu.cpu_model = "changed".into();
        cpu.ticks = CpuTicks::default();
        assert!(format!("{cpu:?}").contains("changed"));
        assert!(CpuInfo::new(1, -1.0, "", ticks).used().abs() < f32::EPSILON);
    }

    #[test]
    fn live_collectors_return_consistent_process_memory_and_hardware_views() {
        let system = OshiUtil::system();
        assert!(!system.cpus().is_empty());
        let process = OshiUtil::current_process().unwrap();
        assert_eq!(process.pid, std::process::id());
        assert!(!process.name.is_empty());
        assert!(format!("{process:?}").contains("ProcessInfo"));
        assert!(OshiUtil::process(&system, u32::MAX).is_none());
        assert_eq!(
            OshiUtil::process(&system, std::process::id()).unwrap().pid,
            std::process::id()
        );

        let memory = OshiUtil::memory();
        assert!(memory.total > 0);
        assert!(memory.used <= memory.total);
        assert!(memory.available <= memory.total);
        assert!(memory.swap_used <= memory.swap_total);

        let immediate = OshiUtil::cpu_info(Duration::ZERO);
        assert!(!immediate.cpu_model.is_empty());
        let sampled = OshiUtil::cpu_info(Duration::from_millis(1));
        assert_eq!(sampled.cpu_num, immediate.cpu_num);

        let disks = OshiUtil::disk_stores();
        assert!(
            disks
                .iter()
                .all(|disk| disk.available_space <= disk.total_space)
        );
        let networks = OshiUtil::network_interfaces();
        assert!(networks.iter().all(|network| !network.name.is_empty()));
        let sensors = OshiUtil::sensors();
        assert!(sensors.iter().all(|sensor| !sensor.label.is_empty()));

        let hardware = OshiUtil::hardware();
        assert!(hardware.memory.total > 0);
        assert!(format!("{hardware:?}").contains("HardwareInfo"));
        assert!(format!("{OshiUtil:?}").contains("OshiUtil"));
    }
}
