pub mod notification_client;

#[derive(Deserialize)]
pub struct Notification {
    id: String,
    subject: Subject,
    repository: Repository,
}

impl Notification {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.repository.name
    }

    pub fn body(&self) -> &String {
        &self.subject.title
    }

    pub fn notification_type(&self) -> &NotificationType {
        &self.subject.notification_type
    }
}

#[derive(Deserialize)]
struct Subject {
    title: String,
    #[serde(rename = "type")]
    notification_type: NotificationType,
}

#[derive(Deserialize)]
struct Repository {
    name: String,
}

pub enum NotificationType {
    Issue,
    Release,
    PullRequest,
}

impl<'de> serde::Deserialize<'de> for NotificationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Issue" => NotificationType::Issue,
            "Release" => NotificationType::Release,
            "PullRequest" => NotificationType::PullRequest,
            _ => panic!("Unknown notification type."),
        })
    }
}

