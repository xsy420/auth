use std::time::SystemTime;

pub fn get_notification_title(
    error_message: &Option<(String, SystemTime)>,
    copy_notification_time: Option<SystemTime>,
) -> String {
    if let Some(msg) = check_notification_time(
        error_message
            .as_ref()
            .map(|(msg, time)| (msg.as_str(), *time)),
    ) {
        return msg;
    }

    if let Some(msg) = check_notification_time(copy_notification_time.map(|time| ("Copied!", time)))
    {
        return msg;
    }

    " Auth ".to_string()
}

fn check_notification_time(notification: Option<(&str, SystemTime)>) -> Option<String> {
    let (message, time) = notification?;
    if time.elapsed().unwrap_or_default().as_secs() < 3 {
        Some(format!(" {} ", message))
    } else {
        None
    }
}
