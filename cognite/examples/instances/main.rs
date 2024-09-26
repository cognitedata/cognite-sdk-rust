use std::collections::HashMap;

use cognite::CogniteClient;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let client = CogniteClient::new_oidc("testing_instances", None).unwrap();
    let external_id = Uuid::new_v4().to_string();
    // let col = CogniteExtractorFile::<HashMap<String, String>>::new(
    //     "core-dm-test".to_string(),
    //     external_id,
    //     "random".to_string(),
    //     None,
    // );
    // println!("{:?}", col.view());
    // let res = client
    //     .models
    //     .files
    //     .upsert_extended(vec![col], None, None, None, None, None)
    //     .await
    //     .unwrap();
    // println!("{res:#?}");
}
