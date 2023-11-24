/// This target is used exclusively to handle group events.
pub const GROUP_TARGET: &str = "codspeed::group";

#[macro_export]
/// Start a new log group. All logs between this and the next `end_group!` will be grouped together.
///
/// # Example
///
/// ```rust
/// start_group!("My group");
/// info!("This will be grouped");
/// end_group!();
/// ```
macro_rules! start_group {
    ($name:expr) => {
        log::log!(target: $crate::ci_provider::logger::GROUP_TARGET, log::Level::Info, "{}", $name);
    };
}

#[macro_export]
/// End the current log group.
/// See [`start_group!`] for more information.
macro_rules! end_group {
    () => {
        log::log!(target: $crate::ci_provider::logger::GROUP_TARGET, log::Level::Info, "");
    };
}

pub enum GroupEvent {
    Start(String),
    End,
}

/// Returns the group event if the record is a group event, otherwise returns `None`.
pub(super) fn get_group_event(record: &log::Record) -> Option<GroupEvent> {
    if record.target() == GROUP_TARGET {
        let args = record.args().to_string();
        if args.is_empty() {
            Some(GroupEvent::End)
        } else {
            Some(GroupEvent::Start(args))
        }
    } else {
        None
    }
}
