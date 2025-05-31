use chrono::Utc;
use passmgr_core::Credential;
use passmgr_core::db::init_db;
use passmgr_core::store::get_all_credentials;
use passmgr_core::store::insert_credential;
use rusqlite::Connection;
use uuid::Uuid;

#[test]
fn test_insert_and_fetch_credential() {
    let conn = Connection::open_in_memory().expect("Failed to open in-memory DB");
    init_db(&conn).expect("DB init failed");

    let now = Utc::now();
    let test_cred = Credential {
        id: Uuid::new_v4(),
        url: "https://example.com".to_string(),
        description: Some("Test entry".to_string()),
        username: "testuser".to_string(),
        password: "hunter2".to_string(),
        created_at: now,
        updated_at: now,
    };

    insert_credential(&conn, &test_cred).expect("Insert failed");

    let creds = get_all_credentials(&conn).expect("Fetch failed");
    assert_eq!(creds.len(), 1);

    let retrieved = &creds[0];
    assert_eq!(retrieved.url, test_cred.url);
    assert_eq!(retrieved.username, test_cred.username);
    assert_eq!(retrieved.password, test_cred.password);
    assert_eq!(retrieved.description, test_cred.description);
}
