#[cfg(test)]
mod service_account_tests {

    #[tokio::test]
    async fn create_and_delete_user() {
        use cognite::*;
        let cognite_client = CogniteClient::new("TestApp").unwrap();
        let new_service_account: ServiceAccount = ServiceAccount::new(
            "test-service_account-should-be-deleted-after-creation",
            &vec![],
        );
        match cognite_client
            .service_accounts
            .create(&vec![new_service_account])
            .await
        {
            Ok(service_accounts) => {
                let service_account_ids: Vec<u64> = service_accounts.iter().map(|u| u.id).collect();

                match cognite_client
                    .service_accounts
                    .delete(&service_account_ids)
                    .await
                {
                    Ok(_) => assert!(true),
                    Err(e) => panic!("{:?}", e),
                }
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
