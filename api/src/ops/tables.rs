//! `diesel` table definitions, copied over from [`doc/`](../../doc/)umentation.


table! {
    /// See [`doc/session.md`](../../doc/session/)
    sessions {
        /// Nullable wrapper so we can pass NULL to SQLite so it assigns new id
        id              -> Nullable<Integer>,
        expiry          -> Timestamp,
        is_admin        -> Bool,
        user_id         -> Nullable<Integer>,
        sudoku_board_id -> Nullable<Integer>,
        board_skeleton  -> Nullable<Text>,
        solve_start     -> Nullable<Timestamp>,
    }
}

table! {
    /// See [`doc/user.md`](../../doc/user/)
    users {
        /// Nullable wrapper so we can pass NULL to SQLite so it assigns new id
        id           -> Nullable<Integer>,
        username     -> Text,
        password     -> Text,
        email        -> Text,
        created_at   -> Timestamp,
        is_admin     -> Bool,
        points_total -> Integer,
    }
}
