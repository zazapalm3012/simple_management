// @generated automatically by Diesel CLI.

diesel::table! {
    tickets (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        subject -> Varchar,
        #[max_length = 30]
        issue_type -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 1000]
        ticket_file -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        #[max_length = 20]
        priority -> Nullable<Varchar>,
        reject_message -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 1000]
        user_address -> Nullable<Varchar>,
        #[max_length = 255]
        organization_name -> Nullable<Varchar>,
        #[max_length = 1000]
        user_img -> Nullable<Varchar>,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(tickets -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tickets,
    users,
);
