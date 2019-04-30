#[cfg(test)]
mod users {
  use cognite::*;

  #[test]
  fn create_and_delete_user() {
    let cognite_client = CogniteClient::new().unwrap();
    let new_user : User = User::new("test-user-should-be-deleted-after-creation", &vec!());
    match cognite_client.users.create(&vec!(new_user)) {
      Ok(users) => {
        let user_ids : Vec<u64> = users.iter().map(| u | u.id).collect();
        
        match cognite_client.users.delete(&user_ids) {
          Ok(_) => assert!(true),
          Err(e) => panic!("{:?}", e)
        }
      },
      Err(e) => panic!("{:?}", e)
    }
  }
}