use chrono::Utc;
use clap::Parser;
use rusqlite::{Connection, Result};
use tasks::*;

fn main() -> Result<()> {
    let conn = Connection::open(SQL_FILE)?;

    let cli = Cli::parse();

    match cli.action {
        Action::Show { status } => show_tasks(&conn, &status)?,
        Action::Clear { status } => purge_tasks(&conn, &status)?,
        Action::Edit { id, title } => change_task_title(&conn, &id, &title)?,
        Action::Status { id, status } => change_status(&conn, &id, &status)?,
        Action::Create => create_database(&conn)?,
        Action::Delete { id } => delete_task(&conn, id)?,
        Action::Add { title } => add_task(
            &conn,
            &Task {
                id: 0,
                title,
                status: PENDING.to_string(),
                created_at: Utc::now().to_string()[0..=18].to_string(),
            },
        )?,
    };
    conn.close().expect("Could not close SQL connection");
    Ok(())
}
