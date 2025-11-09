#[test]
fn test_add() {
    use std::path::PathBuf;
    // TODO:  and we would manually create our own Config and pass it in?
    assert_eq!(std::env::current_dir().unwrap(), PathBuf::new());
}
