use chrono::Utc;
use passmgr_core::Credential;
use passmgr_core::db::init_db;
use passmgr_core::store::get_all_credentials;
use passmgr_core::store::insert_credential;
use rusqlite::Connection;
use uuid::Uuid;

#[test]
fn test_insert_one_credential() {
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

#[test]
fn test_insert_multiple_credential() {
    let conn = Connection::open_in_memory().expect("Failed to open in-memory DB");
    init_db(&conn).expect("DB init failed");

    let now = Utc::now();
    
    // Create multiple test credentials
    let test_creds = vec![
        Credential {
            id: Uuid::new_v4(),
            url: "https://example1.com".to_string(),
            description: Some("First entry".to_string()),
            username: "user1".to_string(),
            password: "pass1".to_string(),
            created_at: now,
            updated_at: now,
        },
        Credential {
            id: Uuid::new_v4(),
            url: "https://example2.com".to_string(),
            description: Some("Second entry".to_string()),
            username: "user2".to_string(),
            password: "pass2".to_string(),
            created_at: now,
            updated_at: now,
        },
        Credential {
            id: Uuid::new_v4(),
            url: "https://example3.com".to_string(),
            description: None,
            username: "user3".to_string(),
            password: "pass3".to_string(),
            created_at: now,
            updated_at: now,
        },
    ];
    
    // Insert all test credentials
    for cred in &test_creds {
        insert_credential(&conn, cred).expect("Insert failed");
    }
    
    // Retrieve all credentials
    let retrieved_creds = get_all_credentials(&conn).expect("Fetch failed");
    
    // Verify correct number of credentials
    assert_eq!(retrieved_creds.len(), test_creds.len());
    
    // Verify each credential was inserted correctly
    for test_cred in &test_creds {
        let found = retrieved_creds.iter().any(|c| 
            c.id == test_cred.id && 
            c.url == test_cred.url &&
            c.description == test_cred.description &&
            c.username == test_cred.username &&
            c.password == test_cred.password
        );
        assert!(found, "Could not find matching credential for {}", test_cred.url);
    }
}
