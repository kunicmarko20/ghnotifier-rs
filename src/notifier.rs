pub struct Notifier;

impl Notifier {
    pub const MAX_NOTIFICATIONS: usize = 10;

    pub fn error(body: &str) {
        Self::send(
            "Something went wrong",
            body,
            "error"
        )
    }

    pub fn success(title: &str, body: &str) {
        Self::send(
            title,
            body,
            "emblem-new"
        )
    }

    pub fn send(title: &str, body: &str, icon: &str) {
        notify_rust::Notification::new()
            .summary(title)
            .body(body)
            .icon(icon)
            .show()
            .unwrap();
    }
}