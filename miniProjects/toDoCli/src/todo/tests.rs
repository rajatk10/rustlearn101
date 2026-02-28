use super::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
#[test]
fn new_list_is_empty_and_next_id_1() {
    let list = TodoList::new();
    assert_eq!(list.todos.len(), 0);
    assert_eq!(list.next_id, 1);
}

#[test]
fn add_todo_with_incrementing_id_status_incomplete() {
    let mut list = TodoList::new();
    list.add("Test adding todo title".to_string());
    assert_eq!(list.todos.len(), 1);
    assert_eq!(list.todos[0].id, 1);
    assert_eq!(list.todos[0].title, "Test adding todo title");
    assert_eq!(list.todos[0].completed, false);
    assert_eq!(list.next_id, 2);
}

#[test]
fn mark_todo_as_completed() {
    let mut list = TodoList::new();
    list.add("Test adding todo title".to_string());
    list.complete(1).unwrap();
    assert_eq!(list.todos[0].completed, true);
}

#[test]
fn show_todo_list() {
    let mut list = TodoList::new();
    list.add("Test adding todo title".to_string());
    list.list();
    assert_eq!(list.todos.len(), 1);
    assert_eq!(list.todos[0].title, "Test adding todo title");
    assert_eq!(list.todos[0].completed, false);
}

#[test]
fn list_empty_list() {
    let list = TodoList::new();
    list.list();
    assert_eq!(list.todos.is_empty(), true);
}

#[test]
fn todo_mark_status_complete_returns_error_if_todo_not_found() {
    let mut list = TodoList::new();
    list.add("Test adding todo title".to_string());
    assert_eq!(list.complete(2).is_err(), true);
}

#[test]
fn remove_todo_from_list() {
    let mut list = TodoList::new();
    list.add("Test adding todo title".to_string());
    list.add("Test adding todo title 2".to_string());
    let result = list.remove(1);
    assert!(result.is_ok());
    assert_eq!(list.todos.len(), 1);
    assert_eq!(list.todos[0].id, 2);
    assert_eq!(list.todos[0].title, "Test adding todo title 2");
    assert_eq!(list.todos[0].completed, false);
    assert_eq!(list.next_id, 3);
}

#[test]
fn remove_returns_err_for_missing_id() {
    let mut list = TodoList::new();
    list.add("Test adding todo title".to_string());
    let result = list.remove(123);
    assert!(result.is_err());
    //assert_eq!(list.remove(2).is_err(), true);
}

fn temp_file_for_load_save(name: &str) -> String {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    //let path = std::env::temp_dir();
    format!("{}-{}.json", name, ts)
}

#[test]
fn save_and_load() {
    let mut list = TodoList::new();
    list.add("Test-add 1".to_string());
    list.add("Test-add 2".to_string());
    list.add("Test-add 3".to_string());

    let path = temp_file_for_load_save("test-save-and-load");
    //Create a temp file now
    fs::write(&path, "{}".as_bytes()).unwrap();
    list.save_to(&path).unwrap();
    let loaded = TodoList::load_from(&path).unwrap();
    assert_eq!(loaded.todos.len(), 3);
    assert_eq!(loaded.todos[0].title, "Test-add 1");
    assert_eq!(loaded.todos[1].title, "Test-add 2");
    assert_eq!(loaded.todos[2].title, "Test-add 3");
    assert_eq!(loaded.next_id, 4);
    fs::remove_file(&path).unwrap(); //If needed comment this out for testing purposes.

}