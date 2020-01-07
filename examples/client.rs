use cognite::{
    Asset, AssetFilter, AssetSearch, CogniteClient, Event, EventFilter, EventSearch, FileFilter,
    FileMetadata, FileSearch, TimeSerie, TimeSerieFilter, TimeSerieSearch,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cognite_client = CogniteClient::new("TestApp").unwrap();
    // List all assets
    let mut filter: AssetFilter = AssetFilter::new();
    filter.name.replace("Aker".to_string());
    match cognite_client.assets.filter_all(filter).await {
        Ok(assets) => println!("{} assets retrieved.", assets.len()),
        Err(e) => println!("{:?}", e),
    }
    // Retrieve asset
    match cognite_client
        .assets
        .retrieve(&vec![6687602007296940_u64])
        .await
    {
        Ok(asset) => println!("{:?}", asset),
        Err(e) => println!("{:?}", e),
    }
    // Search asset
    let mut asset_search: AssetSearch = AssetSearch::new();
    asset_search.description.replace("Aker".to_string());
    let asset_filter: AssetFilter = AssetFilter::new();
    let assets_search_result: Vec<Asset> = cognite_client
        .assets
        .search(asset_filter, asset_search)
        .await
        .unwrap();
    println!("Search found: {:?} assets", assets_search_result.len());
    // List all events
    let event_filter: EventFilter = EventFilter::new();
    let events: Vec<Event> = cognite_client
        .events
        .filter_all(event_filter)
        .await
        .unwrap();
    println!("{} events retrieved.", events.len());
    // Search events
    let event_filter_2: EventFilter = EventFilter::new();
    let event_search: EventSearch = EventSearch::new();
    let event_search_result: Vec<Event> = cognite_client
        .events
        .search(event_filter_2, event_search)
        .await
        .unwrap();
    println!("Search found {:?} events", event_search_result.len());
    // List all events
    let time_series: Vec<TimeSerie> = cognite_client.time_series.list_all(None).await.unwrap();
    println!("{} time series retrieved.", time_series.len());
    // Search time serie
    let time_serie_search: TimeSerieSearch = TimeSerieSearch::new();
    let time_serie_filter: TimeSerieFilter = TimeSerieFilter::new();
    let time_series_search_result: Vec<TimeSerie> = cognite_client
        .time_series
        .search(time_serie_filter, time_serie_search)
        .await
        .unwrap();
    println!(
        "Search found {:?} time series",
        time_series_search_result.len()
    );
    // List all files
    let file_filter: FileFilter = FileFilter::new();
    let files: Vec<FileMetadata> = cognite_client.files.filter_all(file_filter).await.unwrap();
    println!("{} files retrieved.", files.len());
    // Search files
    let file_search: FileSearch = FileSearch::new();
    let file_filter_2: FileFilter = FileFilter::new();
    let files_search_result: Vec<FileMetadata> = cognite_client
        .files
        .search(file_filter_2, file_search)
        .await
        .unwrap();
    println!("Search found {:?} files", files_search_result.len());
    // List all service accounts
    match cognite_client.service_accounts.list_all(None).await {
        Ok(service_accounts) => println!("{} service accounts retrieved.", service_accounts.len()),
        Err(e) => println!("{:?}", e),
    }
    // List all api keys
    match cognite_client.api_keys.list_all(None).await {
        Ok(api_keys) => println!("{} api keys retrieved.", api_keys.len()),
        Err(e) => println!("{:?}", e),
    }
    // List all groups
    match cognite_client.groups.list_all(None).await {
        Ok(mut groups) => {
            println!("{} groups retrieved.", groups.len());
            if groups.len() > 0 {
                let group_id = groups.pop().unwrap().id;
                match cognite_client.groups.list_service_accounts(group_id).await {
                    Ok(service_accounts) => println!(
                        "{} service accounts in group {} retrieved.",
                        service_accounts.len(),
                        group_id
                    ),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }
    Ok(())
}
