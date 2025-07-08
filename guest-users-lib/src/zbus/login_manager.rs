/// generated via https://github.com/dbus2/zbus/blob/main/zbus_xmlgen
#[zbus::proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
pub trait LoginManager {
    /// ActivateSession method
    fn activate_session(&self, session_id: &str) -> zbus::Result<()>;

    /// ActivateSessionOnSeat method
    fn activate_session_on_seat(&self, session_id: &str, seat_id: &str) -> zbus::Result<()>;

    /// AttachDevice method
    fn attach_device(&self, seat_id: &str, sysfs_path: &str, interactive: bool)
        -> zbus::Result<()>;

    /// CanHalt method
    fn can_halt(&self) -> zbus::Result<String>;

    /// CanHibernate method
    fn can_hibernate(&self) -> zbus::Result<String>;

    /// CanHybridSleep method
    fn can_hybrid_sleep(&self) -> zbus::Result<String>;

    /// CanPowerOff method
    fn can_power_off(&self) -> zbus::Result<String>;

    /// CanReboot method
    fn can_reboot(&self) -> zbus::Result<String>;

    /// CanRebootParameter method
    fn can_reboot_parameter(&self) -> zbus::Result<String>;

    /// CanRebootToBootLoaderEntry method
    fn can_reboot_to_boot_loader_entry(&self) -> zbus::Result<String>;

    /// CanRebootToBootLoaderMenu method
    fn can_reboot_to_boot_loader_menu(&self) -> zbus::Result<String>;

    /// CanRebootToFirmwareSetup method
    fn can_reboot_to_firmware_setup(&self) -> zbus::Result<String>;

    /// CanSuspend method
    fn can_suspend(&self) -> zbus::Result<String>;

    /// CanSuspendThenHibernate method
    fn can_suspend_then_hibernate(&self) -> zbus::Result<String>;

    /// CancelScheduledShutdown method
    fn cancel_scheduled_shutdown(&self) -> zbus::Result<bool>;

    /// CreateSession method
    #[allow(clippy::too_many_arguments)]
    fn create_session(
        &self,
        uid: u32,
        pid: u32,
        service: &str,
        type_: &str,
        class: &str,
        desktop: &str,
        seat_id: &str,
        vtnr: u32,
        tty: &str,
        display: &str,
        remote: bool,
        remote_user: &str,
        remote_host: &str,
        properties: &[&(&str, &zbus::zvariant::Value<'_>)],
    ) -> zbus::Result<(
        String,
        zbus::zvariant::OwnedObjectPath,
        String,
        zbus::zvariant::OwnedFd,
        u32,
        String,
        u32,
        bool,
    )>;

    /// CreateSessionWithPIDFD method
    #[zbus(name = "CreateSessionWithPIDFD")]
    #[allow(clippy::too_many_arguments)]
    fn create_session_with_pidfd(
        &self,
        uid: u32,
        pidfd: zbus::zvariant::Fd<'_>,
        service: &str,
        type_: &str,
        class: &str,
        desktop: &str,
        seat_id: &str,
        vtnr: u32,
        tty: &str,
        display: &str,
        remote: bool,
        remote_user: &str,
        remote_host: &str,
        flags: u64,
        properties: &[&(&str, &zbus::zvariant::Value<'_>)],
    ) -> zbus::Result<(
        String,
        zbus::zvariant::OwnedObjectPath,
        String,
        zbus::zvariant::OwnedFd,
        u32,
        String,
        u32,
        bool,
    )>;

    /// FlushDevices method
    fn flush_devices(&self, interactive: bool) -> zbus::Result<()>;

    /// GetSeat method
    fn get_seat(&self, seat_id: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetSession method
    fn get_session(&self, session_id: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetSessionByPID method
    #[zbus(name = "GetSessionByPID")]
    fn get_session_by_pid(&self, pid: u32) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetUser method
    fn get_user(&self, uid: u32) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetUserByPID method
    #[zbus(name = "GetUserByPID")]
    fn get_user_by_pid(&self, pid: u32) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Halt method
    fn halt(&self, interactive: bool) -> zbus::Result<()>;

    /// HaltWithFlags method
    fn halt_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// Hibernate method
    fn hibernate(&self, interactive: bool) -> zbus::Result<()>;

    /// HibernateWithFlags method
    fn hibernate_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// HybridSleep method
    fn hybrid_sleep(&self, interactive: bool) -> zbus::Result<()>;

    /// HybridSleepWithFlags method
    fn hybrid_sleep_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// Inhibit method
    fn inhibit(
        &self,
        what: &str,
        who: &str,
        why: &str,
        mode: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedFd>;

    /// KillSession method
    fn kill_session(&self, session_id: &str, who: &str, signal_number: i32) -> zbus::Result<()>;

    /// KillUser method
    fn kill_user(&self, uid: u32, signal_number: i32) -> zbus::Result<()>;

    /// ListInhibitors method
    fn list_inhibitors(&self) -> zbus::Result<Vec<(String, String, String, String, u32, u32)>>;

    /// ListSeats method
    fn list_seats(&self) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;

    /// ListSessions method
    fn list_sessions(
        &self,
    ) -> zbus::Result<Vec<(String, u32, String, String, zbus::zvariant::OwnedObjectPath)>>;

    /// ListUsers method
    fn list_users(&self) -> zbus::Result<Vec<(u32, String, zbus::zvariant::OwnedObjectPath)>>;

    /// LockSession method
    fn lock_session(&self, session_id: &str) -> zbus::Result<()>;

    /// LockSessions method
    fn lock_sessions(&self) -> zbus::Result<()>;

    /// PowerOff method
    fn power_off(&self, interactive: bool) -> zbus::Result<()>;

    /// PowerOffWithFlags method
    fn power_off_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// Reboot method
    fn reboot(&self, interactive: bool) -> zbus::Result<()>;

    /// RebootWithFlags method
    fn reboot_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// ReleaseSession method
    fn release_session(&self, session_id: &str) -> zbus::Result<()>;

    /// ScheduleShutdown method
    fn schedule_shutdown(&self, type_: &str, usec: u64) -> zbus::Result<()>;

    /// SetRebootParameter method
    fn set_reboot_parameter(&self, parameter: &str) -> zbus::Result<()>;

    /// SetRebootToBootLoaderEntry method
    fn set_reboot_to_boot_loader_entry(&self, boot_loader_entry: &str) -> zbus::Result<()>;

    /// SetRebootToBootLoaderMenu method
    fn set_reboot_to_boot_loader_menu(&self, timeout: u64) -> zbus::Result<()>;

    /// SetRebootToFirmwareSetup method
    fn set_reboot_to_firmware_setup(&self, enable: bool) -> zbus::Result<()>;

    /// SetUserLinger method
    fn set_user_linger(&self, uid: u32, enable: bool, interactive: bool) -> zbus::Result<()>;

    /// SetWallMessage method
    fn set_wall_message(&self, wall_message: &str, enable: bool) -> zbus::Result<()>;

    /// Suspend method
    fn suspend(&self, interactive: bool) -> zbus::Result<()>;

    /// SuspendThenHibernate method
    fn suspend_then_hibernate(&self, interactive: bool) -> zbus::Result<()>;

    /// SuspendThenHibernateWithFlags method
    fn suspend_then_hibernate_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// SuspendWithFlags method
    fn suspend_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// TerminateSeat method
    fn terminate_seat(&self, seat_id: &str) -> zbus::Result<()>;

    /// TerminateSession method
    fn terminate_session(&self, session_id: &str) -> zbus::Result<()>;

    /// TerminateUser method
    fn terminate_user(&self, uid: u32) -> zbus::Result<()>;

    /// UnlockSession method
    fn unlock_session(&self, session_id: &str) -> zbus::Result<()>;

    /// UnlockSessions method
    fn unlock_sessions(&self) -> zbus::Result<()>;

    /// PrepareForShutdown signal
    #[zbus(signal)]
    fn prepare_for_shutdown(&self, start: bool) -> zbus::Result<()>;

    /// PrepareForShutdownWithMetadata signal
    #[zbus(signal)]
    fn prepare_for_shutdown_with_metadata(
        &self,
        start: bool,
        metadata: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// PrepareForSleep signal
    #[zbus(signal)]
    fn prepare_for_sleep(&self, start: bool) -> zbus::Result<()>;

    /// SeatNew signal
    #[zbus(signal)]
    fn seat_new(
        &self,
        seat_id: &str,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// SeatRemoved signal
    #[zbus(signal)]
    fn seat_removed(
        &self,
        seat_id: &str,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// SessionNew signal
    #[zbus(signal)]
    fn session_new(
        &self,
        session_id: &str,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// SessionRemoved signal
    #[zbus(signal)]
    fn session_removed(
        &self,
        session_id: &str,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// UserNew signal
    #[zbus(signal)]
    fn user_new(&self, uid: u32, object_path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// UserRemoved signal
    #[zbus(signal)]
    fn user_removed(
        &self,
        uid: u32,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// BlockInhibited property
    #[zbus(property)]
    fn block_inhibited(&self) -> zbus::Result<String>;

    /// BootLoaderEntries property
    #[zbus(property)]
    fn boot_loader_entries(&self) -> zbus::Result<Vec<String>>;

    /// DelayInhibited property
    #[zbus(property)]
    fn delay_inhibited(&self) -> zbus::Result<String>;

    /// Docked property
    #[zbus(property)]
    fn docked(&self) -> zbus::Result<bool>;

    /// EnableWallMessages property
    #[zbus(property)]
    fn enable_wall_messages(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_enable_wall_messages(&self, value: bool) -> zbus::Result<()>;

    /// HandleHibernateKey property
    #[zbus(property)]
    fn handle_hibernate_key(&self) -> zbus::Result<String>;

    /// HandleHibernateKeyLongPress property
    #[zbus(property)]
    fn handle_hibernate_key_long_press(&self) -> zbus::Result<String>;

    /// HandleLidSwitch property
    #[zbus(property)]
    fn handle_lid_switch(&self) -> zbus::Result<String>;

    /// HandleLidSwitchDocked property
    #[zbus(property)]
    fn handle_lid_switch_docked(&self) -> zbus::Result<String>;

    /// HandleLidSwitchExternalPower property
    #[zbus(property)]
    fn handle_lid_switch_external_power(&self) -> zbus::Result<String>;

    /// HandlePowerKey property
    #[zbus(property)]
    fn handle_power_key(&self) -> zbus::Result<String>;

    /// HandlePowerKeyLongPress property
    #[zbus(property)]
    fn handle_power_key_long_press(&self) -> zbus::Result<String>;

    /// HandleRebootKey property
    #[zbus(property)]
    fn handle_reboot_key(&self) -> zbus::Result<String>;

    /// HandleRebootKeyLongPress property
    #[zbus(property)]
    fn handle_reboot_key_long_press(&self) -> zbus::Result<String>;

    /// HandleSuspendKey property
    #[zbus(property)]
    fn handle_suspend_key(&self) -> zbus::Result<String>;

    /// HandleSuspendKeyLongPress property
    #[zbus(property)]
    fn handle_suspend_key_long_press(&self) -> zbus::Result<String>;

    /// HoldoffTimeoutUSec property
    #[zbus(property, name = "HoldoffTimeoutUSec")]
    fn holdoff_timeout_usec(&self) -> zbus::Result<u64>;

    /// IdleAction property
    #[zbus(property)]
    fn idle_action(&self) -> zbus::Result<String>;

    /// IdleActionUSec property
    #[zbus(property, name = "IdleActionUSec")]
    fn idle_action_usec(&self) -> zbus::Result<u64>;

    /// IdleHint property
    #[zbus(property)]
    fn idle_hint(&self) -> zbus::Result<bool>;

    /// IdleSinceHint property
    #[zbus(property)]
    fn idle_since_hint(&self) -> zbus::Result<u64>;

    /// IdleSinceHintMonotonic property
    #[zbus(property)]
    fn idle_since_hint_monotonic(&self) -> zbus::Result<u64>;

    /// InhibitDelayMaxUSec property
    #[zbus(property, name = "InhibitDelayMaxUSec")]
    fn inhibit_delay_max_usec(&self) -> zbus::Result<u64>;

    /// InhibitorsMax property
    #[zbus(property)]
    fn inhibitors_max(&self) -> zbus::Result<u64>;

    /// KillExcludeUsers property
    #[zbus(property)]
    fn kill_exclude_users(&self) -> zbus::Result<Vec<String>>;

    /// KillOnlyUsers property
    #[zbus(property)]
    fn kill_only_users(&self) -> zbus::Result<Vec<String>>;

    /// KillUserProcesses property
    #[zbus(property)]
    fn kill_user_processes(&self) -> zbus::Result<bool>;

    /// LidClosed property
    #[zbus(property)]
    fn lid_closed(&self) -> zbus::Result<bool>;

    /// NAutoVTs property
    #[zbus(property, name = "NAutoVTs")]
    fn nauto_vts(&self) -> zbus::Result<u32>;

    /// NCurrentInhibitors property
    #[zbus(property, name = "NCurrentInhibitors")]
    fn ncurrent_inhibitors(&self) -> zbus::Result<u64>;

    /// NCurrentSessions property
    #[zbus(property, name = "NCurrentSessions")]
    fn ncurrent_sessions(&self) -> zbus::Result<u64>;

    /// OnExternalPower property
    #[zbus(property)]
    fn on_external_power(&self) -> zbus::Result<bool>;

    /// PreparingForShutdown property
    #[zbus(property)]
    fn preparing_for_shutdown(&self) -> zbus::Result<bool>;

    /// PreparingForSleep property
    #[zbus(property)]
    fn preparing_for_sleep(&self) -> zbus::Result<bool>;

    /// RebootParameter property
    #[zbus(property)]
    fn reboot_parameter(&self) -> zbus::Result<String>;

    /// RebootToBootLoaderEntry property
    #[zbus(property)]
    fn reboot_to_boot_loader_entry(&self) -> zbus::Result<String>;

    /// RebootToBootLoaderMenu property
    #[zbus(property)]
    fn reboot_to_boot_loader_menu(&self) -> zbus::Result<u64>;

    /// RebootToFirmwareSetup property
    #[zbus(property)]
    fn reboot_to_firmware_setup(&self) -> zbus::Result<bool>;

    /// RemoveIPC property
    #[zbus(property, name = "RemoveIPC")]
    fn remove_ipc(&self) -> zbus::Result<bool>;

    /// RuntimeDirectoryInodesMax property
    #[zbus(property)]
    fn runtime_directory_inodes_max(&self) -> zbus::Result<u64>;

    /// RuntimeDirectorySize property
    #[zbus(property)]
    fn runtime_directory_size(&self) -> zbus::Result<u64>;

    /// ScheduledShutdown property
    #[zbus(property)]
    fn scheduled_shutdown(&self) -> zbus::Result<(String, u64)>;

    /// SessionsMax property
    #[zbus(property)]
    fn sessions_max(&self) -> zbus::Result<u64>;

    /// StopIdleSessionUSec property
    #[zbus(property, name = "StopIdleSessionUSec")]
    fn stop_idle_session_usec(&self) -> zbus::Result<u64>;

    /// UserStopDelayUSec property
    #[zbus(property, name = "UserStopDelayUSec")]
    fn user_stop_delay_usec(&self) -> zbus::Result<u64>;

    /// WallMessage property
    #[zbus(property)]
    fn wall_message(&self) -> zbus::Result<String>;
}
