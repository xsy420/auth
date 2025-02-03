use crate::constants::{AUTH_TITLE, COPIED_MSG};
use std::time::SystemTime;

pub fn get_notification_title(
    error_message: &Option<(String, SystemTime)>,
    copy_notification_time: Option<SystemTime>,
) -> String {
    let error_msg = get_error_message(error_message);
    if let Some(msg) = error_msg {
        return msg;
    }

    let copy_msg = get_copy_message(copy_notification_time);
    if let Some(msg) = copy_msg {
        return msg;
    }

    AUTH_TITLE.to_string()
}

fn get_error_message(error_message: &Option<(String, SystemTime)>) -> Option<String> {
    let (msg, time) = error_message.as_ref()?;
    if time.elapsed().unwrap_or_default().as_secs() < 3 {
        Some(format!(" {} ", msg))
    } else {
        None
    }
}

fn get_copy_message(copy_notification_time: Option<SystemTime>) -> Option<String> {
    let notify_time = copy_notification_time?;
    if notify_time.elapsed().unwrap_or_default().as_secs() < 3 {
        Some(COPIED_MSG.to_string())
    } else {
        None
    }
}
