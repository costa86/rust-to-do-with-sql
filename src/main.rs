use chrono::Utc;
use clap::Parser;
use rusqlite::{params, Connection, Result};
use tabled::{Style, Table, Tabled};

const DONE: &str = "DONE";
const PENDING: &str = "PENDING";
const TABLE: &str = "tasks";
const SQL_FILE: &str = "db.db3";


/// To-do list with SQL
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    ///Add task <TITLE>
    Add { title: String },
    ///Delete task <ID>
    Delete { id: u8 },
    ///Show tasks <D=done, P=pending, A=all>
    Show { status: char },
    ///Create database
    Create,
    ///Update task status <ID> <D=done or P=pending>
    Status { id: u8, status: char },
    ///Change task title <ID> <NEW_TITLE>
    Edit { id: u8, title: String },
    ///Batch-delete tasks <D=done, P=pending, A=all>
    Clear { status: char },
}

#[derive(Tabled, Debug)]
struct Task {
    id: u8,
    title: String,
    status: String,
    created_at: String,
}

fn main() {
    let conn = Connection::open(SQL_FILE).unwrap();

    let cli = Cli::parse();

    match cli.action {
        Action::Show { status } => show_tasks(&conn, &status).expect("Could not show tasks"),
        Action::Clear { status } => purge_tasks(&conn, &status).expect("Could not delete tasks"),
        Action::Edit { id, title } => change_task_title(&conn, &id, &title).expect("Could not edit task"),
        Action::Status { id, status } => change_status(&conn, &id, &status).expect("Could not edit task"),
        Action::Create => create_database(&conn).expect("Could not create database table"),
        Action::Delete { id } => delete_task(&conn, id).expect("Could not delete task"),
        Action::Add { title } => add_task(
            &conn,
            &Task {
                id: 0,
                title,
                status: PENDING.to_string(),
                created_at: Utc::now().to_string()[0..=18].to_string(),
            },
        )
        .expect("e"),
    };
    conn.close().unwrap();
}


///Creates table on database
fn create_database(conn: &Connection) -> Result<()> {
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {TABLE} (
                  id              INTEGER PRIMARY KEY,
                  title           VARCHAR(255) NOT NULL,
                  status          VARCHAR(10) NOT NULL,
                  created_at      VARCHAR(20) NOT NULL
                  )"
        ),
        [],
    )?;
    Ok(())
}

///Creates a new task
fn add_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        &format!("INSERT INTO {TABLE} (title, status, created_at) VALUES (?1, ?2, ?3)"),
        params![task.title, task.status, task.created_at],
    )?;
    Ok(())
}

///Shows all tasks in a table
fn show_tasks(conn: &Connection, status: &char) -> Result<()> {
    let query = match status {
        'd' | 'D' => format!("SELECT * FROM {TABLE} WHERE status = '{DONE}'"),
        'p' | 'P' => format!("SELECT * FROM {TABLE} WHERE status = '{PENDING}'"),
        _ => format!("SELECT * FROM {TABLE}"),
    };

    let mut stmt = conn.prepare(&query)?;

    let result_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            status: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;

    let mut records: Vec<Task> = Vec::new();
    for i in result_iter {
        records.push(i.unwrap());
    }
    let table = Table::new(records).with(Style::modern()).to_string();
    println!("{}", table);
    Ok(())
}

///Deletes a task by id
fn delete_task(conn: &Connection, id: u8) -> Result<()> {
    conn.execute(&format!("DELETE FROM {TABLE} WHERE id = ?1"), params![id])?;
    Ok(())
}

fn purge_tasks(conn: &Connection, status: &char) -> Result<()> {
    let query = match status {
        'd' | 'D' => format!("DELETE FROM {TABLE} WHERE status = '{DONE}'"),
        'p' | 'P' => format!("DELETE FROM {TABLE} WHERE status = '{PENDING}'"),
        'A' | 'a' => format!("DELETE FROM {TABLE}"),
        _ => format!("DELETE FROM {TABLE}"),
    };
    conn.execute(&query, params![])?;
    Ok(())
}

///Edits task title
fn change_task_title(conn: &Connection, id: &u8, title: &str) -> Result<()> {
    conn.execute(
        &format!("UPDATE {TABLE} SET title = ?1 WHERE id = ?2"),
        params![title, id],
    )?;
    Ok(())
}

///Switch task status (PENDING or DONE)
fn change_status(conn: &Connection, id: &u8, status: &char) -> Result<()> {
    let status = match status {
        'd' | 'D' => DONE,
        'p' | 'P' => PENDING,
        _ => PENDING,
    };
    conn.execute(
        &format!("UPDATE {TABLE} SET status = ?1 WHERE id = ?2"),
        params![status, id],
    )?;
    Ok(())
}
