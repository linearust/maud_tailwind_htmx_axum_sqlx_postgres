use crate::db::DB;

pub async fn init_schema() {
    DB.query(SCHEMA)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to initialize database schema: {}", e);
            eprintln!("\nThis usually means there's an issue with the schema definitions.");
            std::process::exit(1);
        });
}

const SCHEMA: &str = r#"
-- Users
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON user TYPE datetime DEFAULT time::now();
DEFINE INDEX email_idx ON user FIELDS email UNIQUE;

-- Magic Links
DEFINE TABLE magic_link SCHEMAFULL;
DEFINE FIELD email ON magic_link TYPE string;
DEFINE FIELD expires_at ON magic_link TYPE datetime;
DEFINE FIELD created_at ON magic_link TYPE datetime DEFAULT time::now();
DEFINE INDEX email_idx ON magic_link FIELDS email;

-- Todos
DEFINE TABLE todo SCHEMAFULL;
DEFINE FIELD task ON todo TYPE string;
DEFINE FIELD is_done ON todo TYPE bool DEFAULT false;
DEFINE FIELD created_at ON todo TYPE datetime DEFAULT time::now();
DEFINE FIELD author ON todo TYPE record<user>;
DEFINE INDEX author_idx ON todo FIELDS author;

-- Orders
DEFINE TABLE order SCHEMAFULL;
DEFINE FIELD user ON order TYPE record<user>;
DEFINE FIELD user_email ON order TYPE string;
DEFINE FIELD filename ON order TYPE string;
DEFINE FIELD file_size ON order TYPE int;
DEFINE FIELD text_content ON order TYPE string;
DEFINE FIELD text_length ON order TYPE int;
DEFINE FIELD price_amount ON order TYPE int;
DEFINE FIELD payment_status ON order TYPE string ASSERT $value IN ['pending', 'paid', 'failed', 'cancelled'];
DEFINE FIELD payment_key ON order TYPE option<string>;
DEFINE FIELD order_number ON order TYPE string;
DEFINE FIELD created_at ON order TYPE datetime DEFAULT time::now();
DEFINE FIELD paid_at ON order TYPE option<datetime>;
DEFINE INDEX order_number_idx ON order FIELDS order_number UNIQUE;
DEFINE INDEX user_idx ON order FIELDS user;

-- User Roles
DEFINE TABLE user_role SCHEMAFULL;
DEFINE FIELD user ON user_role TYPE record<user>;
DEFINE FIELD role ON user_role TYPE string;
DEFINE FIELD granted_at ON user_role TYPE datetime DEFAULT time::now();
DEFINE FIELD granted_by ON user_role TYPE option<record<user>>;
DEFINE INDEX user_role_idx ON user_role FIELDS user, role UNIQUE;

-- Sessions
DEFINE TABLE session SCHEMAFULL;
DEFINE FIELD data ON session TYPE bytes;
DEFINE FIELD expires_at ON session TYPE datetime;
"#;
