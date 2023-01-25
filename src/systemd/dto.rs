use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

use serde::Serialize;

use super::dbus::{service::OrgFreedesktopSystemd1Service, unit::OrgFreedesktopSystemd1Unit};

type ExecDataTuple = (String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecData {
  pub path_to_binary: String,
  
  /// Arguments passed to binary
  pub args: Vec<String>,
  
  /// a boolean whether it should be considered a failure if the process exits uncleanly
  pub should_unclean_exit_mean_failure: bool,

  /// clock realtime timestamp in microseconds when process began running
  pub clock_realtime_start: u64,

  /// clock monotonic timestamp in microseconds when process began running
  pub clock_monotonic_start: u64,

  /// clock realtime timestamp in microseconds when process finished running
  pub clock_realtime_finish: u64,

  /// clock monotonic timestamp in microseconds when process began running
  pub clock_monotonic_finish: u64,

  pub pid: u32,

  /// This is SIGCHLD
  /// Defined in linux kernel as
  /// 
  /// ```c
  /// #define __SI_CHLD	(4 << 16)
  /// // ...
  /// #define CLD_EXITED	(__SI_CHLD|1)	/* child has exited */
  /// #define CLD_KILLED	(__SI_CHLD|2)	/* child was killed */
  /// #define CLD_DUMPED	(__SI_CHLD|3)	/* child terminated abnormally */
  /// #define CLD_TRAPPED	(__SI_CHLD|4)	/* traced child has trapped */
  /// #define CLD_STOPPED	(__SI_CHLD|5)	/* child has stopped */
  /// #define CLD_CONTINUED	(__SI_CHLD|6)	/* stopped child has continued */ 
  /// 
  /// But from what I saw systemd just returns `1, 2, 3, 4, 5, 6`
  /// ``` 
  pub last_exit_code: i32,

  /// Last **process** return/exit code. 0 - success
  pub last_status: i32,
}

impl From<ExecDataTuple> for ExecData {
  fn from(value: ExecDataTuple) -> Self {
    ExecData {
      path_to_binary: value.0,
      args: value.1,
      should_unclean_exit_mean_failure: value.2,
      clock_realtime_start: value.3,
      clock_monotonic_start: value.4,
      clock_realtime_finish: value.5,
      clock_monotonic_finish: value.6,
      pid: value.7,
      last_exit_code: value.8,
      last_status: value.9,
    }
  }
}

fn exec_data_tuple_vec_to_struct(data: Vec<ExecDataTuple>) -> Vec<ExecData> {
  data.into_iter().map(ExecData::from).collect()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitDto {
  pub id: String,
  pub names: Vec<String>,
  pub description: String,
  pub documentation: Vec<String>,
  pub triggered_by: Vec<String>,
  pub load_state: String,
  pub active_state: String,
  pub load_error: (String, String),
  pub fragment_path: String,
  pub unit_file_state: String,
  pub unit_file_preset: String,
  pub state_change_timestamp: u64,

  #[serde(flatten)]
  service: Option<ServiceDto>,
}

impl UnitDto {
  pub fn add_service(&mut self, service: ServiceDto) {
    self.service = Some(service);
  }

  pub fn create_from_proxy(
    proxy: &impl OrgFreedesktopSystemd1Unit,
  ) -> Result<UnitDto, dbus::Error> {
    Ok(UnitDto {
      id: proxy.id()?,
      names: proxy.names()?,
      description: proxy.description()?,
      documentation: proxy.documentation()?,
      triggered_by: proxy.triggered_by()?,
      load_state: proxy.load_state()?,
      active_state: proxy.active_state()?,
      load_error: proxy.load_error()?,
      fragment_path: proxy.fragment_path()?,
      unit_file_state: proxy.unit_file_state()?,
      unit_file_preset: proxy.unit_file_preset()?,
      state_change_timestamp: proxy.state_change_timestamp()?,
      service: None,
    })
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceDto {
  pub exec_main_pid: u32,
  pub exec_main_code: i32,
  pub exec_main_status: i32,
  pub memory_current: u64,
  pub cpu_usage_nsec: u64,
  pub tasks_current: u64,
  pub result: String,
  pub status_text: String,
  pub status_errno: i32,
  pub exec_start: Vec<ExecData>,

  pub extra_main_name: Option<String>,
}

impl ServiceDto {
  pub fn create_from_proxy(
    proxy: &impl OrgFreedesktopSystemd1Service,
  ) -> Result<ServiceDto, dbus::Error> {
    let exec_main_pid = proxy.exec_main_pid()?;
    let mut sys = System::new();
    let mut extra_main_name: Option<String> = None;

    let process_exists = sys.refresh_process(Pid::from_u32(exec_main_pid));

    if process_exists {
      let process = sys.process(Pid::from_u32(exec_main_pid));
      extra_main_name = process.map(|p| p.name().to_owned());
    }

    Ok(ServiceDto {
      exec_main_pid,
      exec_main_code: proxy.exec_main_code()?,
      exec_main_status: proxy.exec_main_status()?,
      memory_current: proxy.memory_current()?,
      cpu_usage_nsec: proxy.cpuusage_nsec()?,
      tasks_current: proxy.tasks_current()?,
      result: proxy.result()?,
      status_text: proxy.status_text()?,
      status_errno: proxy.status_errno()?,
      exec_start: exec_data_tuple_vec_to_struct(proxy.exec_start()?),

      // Extra properties are provided by me, they do not come from DBus
      extra_main_name,
    })
  }
}
