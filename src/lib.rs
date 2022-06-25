use clap::Parser;
use rusqlite::{params, Connection, Result};
use tabled::{Style, Table, Tabled};

pub const DONE: &str = "DONE";
pub const PENDING: &str = "PENDING";
pub const TABLE: &str = "tasks";
pub const SQL_FILE: &str = "db.db3";

/// To-do list with SQL
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
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
pub struct Task {
    pub id: u8,
    pub title: String,
    pub status: String,
    pub created_at: String,
}

///Creates table on database
pub fn create_database(conn: &Connection) -> Result<()> {
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
pub fn add_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        &format!("INSERT INTO {TABLE} (title, status, created_at) VALUES (?1, ?2, ?3)"),
        params![task.title, task.status, task.created_at],
    )?;
    Ok(())
}

///Shows all tasks in a table
pub fn show_tasks(conn: &Connection, status: &char) -> Result<()> {
    let mut records: Vec<Task> = Vec::new();
    let mut qtd_done: u8 = 0;
    let mut qtd_pending: u8 = 0;
    let mut message: String = String::from("");

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

    for i in result_iter {
        if &i.as_ref().unwrap().status == DONE {
            qtd_done += 1;
        } else {
            qtd_pending += 1;
        }
        records.push(i?);
    }
    let table = Table::new(records).with(Style::modern()).to_string();

    if qtd_done > 0 {
        message.push_str(&format!("\n{DONE}: {qtd_done}"));
    }
    if qtd_pending > 0 {
        message.push_str(&format!("\n{PENDING}: {qtd_pending}"));
    }

    println!("{table}");
    println!("{message}");

    Ok(())
}

///Deletes a task by id
pub fn delete_task(conn: &Connection, id: u8) -> Result<()> {
    conn.execute(&format!("DELETE FROM {TABLE} WHERE id = ?1"), params![id])?;
    Ok(())
}

///Deletes all tasks
pub fn purge_tasks(conn: &Connection, status: &char) -> Result<()> {
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
pub fn change_task_title(conn: &Connection, id: &u8, title: &str) -> Result<()> {
    conn.execute(
        &format!("UPDATE {TABLE} SET title = ?1 WHERE id = ?2"),
        params![title, id],
    )?;
    Ok(())
}

///Switch task status (PENDING or DONE)
pub fn change_status(conn: &Connection, id: &u8, status: &char) -> Result<()> {
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
