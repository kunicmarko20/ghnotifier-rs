pub struct Notifier;

impl Notifier {
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

    fn send(title: &str, body: &str, icon: &str) {
        notify_rust::Notification::new()
            .summary(title)
            .body(body)
            .icon(icon)
            .show()
            .expect("Unable to send notification.");
    }
}