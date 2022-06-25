## 1. Description

Manage tasks in a a CLI to-do list, with tasks saved to SQLite

## 2. Executable

|Linux|Windows|
|--|--|
|[tasks](./tasks)|-|

## 3. Usage


    USAGE:
        sqlite <SUBCOMMAND>

    OPTIONS:
        -h, --help       Print help information
        -V, --version    Print version information

    SUBCOMMANDS:
        add       Add task <TITLE>
        clear     Batch-delete tasks <D=done, P=pending, A=all>
        create    Create database
        delete    Delete task <ID>
        edit      Change task title <ID> <NEW_TITLE>
        help      Print this message or the help of the given subcommand(s)
        show      Show tasks <D=done, P=pending, A=all>
        status    Update task status <ID> <D=done or P=pending>


## 4. Sample list

    ┌────┬───────────────────┬─────────┬─────────────────────┐
    │ id │       title       │ status  │     created_at      │
    ├────┼───────────────────┼─────────┼─────────────────────┤
    │ 2  │    learn rust     │ PENDING │ 2022-06-25 15:23:28 │
    ├────┼───────────────────┼─────────┼─────────────────────┤
    │ 4  │ conquer the world │ PENDING │ 2022-06-25 15:23:29 │
    ├────┼───────────────────┼─────────┼─────────────────────┤
    │ 6  │     buy shoes     │  DONE   │ 2022-06-25 17:32:42 │
    └────┴───────────────────┴─────────┴─────────────────────┘