use rusqlite::{Connection, Result};
use std::env;

fn main() -> Result<()> {
    let path = "./tasks.db3";
    let con = Connection::open(path)?;
    let table_query = "
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            task TEXT NOT NULL,
            is_completed INTEGER DEFAULT 0
        );
        ";
    match con.execute(table_query, []) {
        Ok(updated) => println!("table created/found, {} rows changed", updated),
        Err(err) => println!("table could not found, new table was not created, error: {}", err),
    }
    let command = String::from(env::args().nth(1).expect("no command given"));
    if command == "add" {
        let task = String::from(env::args().nth(2).expect("no task given"));
        let insert_query = "
            INSERT INTO tasks (task)
            VALUES (?1);
            ";
        match con.execute(insert_query, [task]) {
            Ok(updated) => println!("added task, {} rows changed", updated),
            Err(err) => println!("failed to add task, error: {}", err),
        }
    }
    if command == "list" {
        let mut retrieve_query = con.prepare("SELECT * FROM tasks")?;
        let rows = retrieve_query.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let task: String = row.get(1)?;
            let is_completed: bool = if row.get::<_, i64>(2)? == 0 {false} else {true};
            Ok((id, task, is_completed))
        })?;
        for row in rows {
            if let Ok((id, task, is_completed)) = row {
                println!("{}) {} - completed: {}", id, task, is_completed)
            }
        }
    }
    if command == "complete" {
        let id = env::args().nth(2).expect("no id given");
        match con.execute("UPDATE tasks SET is_completed = 1 WHERE id = (?1)", [id]) {
            Ok(updated) => println!("marked task as completed, {} rows changed", updated),
            Err(err) => println!("failed to mark as complete, error: {}", err),
        }
    }
    if command == "delete" {
        let id = env::args().nth(2).expect("no id given");
        match con.execute("DELETE FROM tasks WHERE id = (?1)", [id]) {
            Ok(updated) => println!("deleted task, {} rows changed", updated),
            Err(err) => println!("failed to delete task, error: {}", err),
        }
    }
    Ok(())
}
