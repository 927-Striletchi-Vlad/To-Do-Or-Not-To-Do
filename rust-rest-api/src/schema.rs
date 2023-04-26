// @generated automatically by Diesel CLI.

diesel::table! {
    todolists (tlid) {
        tlid -> Varchar,
        uid -> Varchar,
        title -> Varchar,
        priority -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    todos (tid) {
        tid -> Varchar,
        title -> Varchar,
        content -> Varchar,
        completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    todostodolists (tlid, tid) {
        tid -> Varchar,
        tlid -> Varchar,
    }
}

diesel::table! {
    users (uid) {
        uid -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(todolists -> users (uid));
diesel::joinable!(todostodolists -> todolists (tlid));
diesel::joinable!(todostodolists -> todos (tid));

diesel::allow_tables_to_appear_in_same_query!(
    todolists,
    todos,
    todostodolists,
    users,
);
