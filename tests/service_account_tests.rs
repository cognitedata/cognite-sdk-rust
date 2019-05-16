#[cfg(test)]
mod service_account_tests {
  use cognite::*;

  #[test]
  fn create_and_delete_user() {
    let cognite_client = CogniteClient::new().unwrap();
    let new_service_account : ServiceAccount = ServiceAccount::new("test-service_account-should-be-deleted-after-creation", &vec!());
    match cognite_client.service_accounts.create(&vec!(new_service_account)) {
      Ok(service_accounts) => {
        let service_account_ids : Vec<u64> = service_accounts.iter().map(| u | u.id).collect();
        
        match cognite_client.service_accounts.delete(&service_account_ids) {
          Ok(_) => assert!(true),
          Err(e) => panic!("{:?}", e)
        }
      },
      Err(e) => panic!("{:?}", e)
    }
  }
}