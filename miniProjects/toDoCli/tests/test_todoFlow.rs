use toDoCli::todo::TodoList;
#[test]
fn add_complete_remove_flow() {
    let mut list =TodoList::new();
    list.add("Test adding todo title".to_string());
    assert_eq!(list.complete(1).is_ok(), true);
    assert_eq!(list.remove(1).is_ok(), true);
    //assert_eq!(list.todos.len(), 0);
}