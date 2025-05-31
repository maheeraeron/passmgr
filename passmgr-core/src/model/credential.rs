use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

/// Represents a stored credential for a website or service.
#[derive(Debug, Clone)]
pub struct Credential {
    /// Unique identifier for this credential.
    pub id: Uuid,
    /// Associated service or website URL.
    pub url: String,
    /// Optional description or label.
    pub description: Option<String>,
    /// Username for this credential.
    pub username: String,
    /// Password for this credential (unencrypted for now).
    pub password: String,
    /// When this credential was created.
    pub created_at: DateTime<Utc>,
    /// When this credential was last updated.
    pub updated_at: DateTime<Utc>,
}
