use sqlx::{
    Pool,
    sqlite::{Sqlite, SqliteConnectOptions, SqlitePoolOptions},
};
use std::{env, fs};

pub fn get() -> Pool<Sqlite> {
    let db_path = if cfg!(debug_assertions) {
        "./chai/chai.db".into()
    } else {
        env::home_dir()
            .expect("has a home directory")
            .join("chai")
            .join("chai.db")
    };

    let opts = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);

    fs::create_dir_all(db_path.parent().expect("has parent")).expect("creates parent dirs");
    SqlitePoolOptions::new().connect_lazy_with(opts)
}
