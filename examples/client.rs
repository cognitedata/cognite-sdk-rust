extern crate cognite;

use cognite::{
  CogniteClient,
  Asset,
  Event,
  TimeSerie,
  FileMetadata,
  AssetFilter,
  AssetSearch,
  TimeSerieSearch,
  TimeSerieFilter,
  EventFilter,
  EventSearch,
  FileSearch,
  FileFilter,
};

fn main() {
  let cognite_client = CogniteClient::new().unwrap();

  // List all assets

  let filter : AssetFilter = AssetFilter::new();
  let assets : Vec<Asset> = cognite_client.assets.filter_all(filter).unwrap();
  println!("{} assets retrieved.", assets.len());
  
  // Retrieve asset
  match cognite_client.assets.retrieve(&vec!(6687602007296940_u64)) {
    Ok(asset) => println!("{:?}", asset),
    Err(e) => println!("{:?}", e)
  }

  // Search asset
  let asset_search : AssetSearch = AssetSearch::new();
  let asset_filter : AssetFilter = AssetFilter::new();
  let asset_search : Vec<Asset> = cognite_client.assets.search(asset_filter, asset_search).unwrap();
  println!("Search found: {:?} assets", asset_search.len());

  // List all events
  let event_filter : EventFilter = EventFilter::new();
  let events : Vec<Event> = cognite_client.events.filter_all(event_filter).unwrap();
  println!("{} events retrieved.", events.len());

  // Search events
  let event_filter_2 : EventFilter = EventFilter::new();
  let event_search : EventSearch = EventSearch::new();
  let event_search_result : Vec<Event> = cognite_client.events.search(event_filter_2, event_search).unwrap();
  println!("Search found {:?} events", event_search_result.len());

  // List all events
  let time_series : Vec<TimeSerie> = cognite_client.time_series.list_all(None).unwrap();
  println!("{} time series retrieved.", time_series.len());

  // Search time serie
  let time_serie_search : TimeSerieSearch = TimeSerieSearch::new();
  let time_serie_filter : TimeSerieFilter = TimeSerieFilter::new();
  let time_series_search_result : Vec<TimeSerie> = cognite_client.time_series.search(time_serie_filter, time_serie_search).unwrap();
  println!("Search found {:?} time series", time_series_search_result.len());

  // List all files
  let file_filter : FileFilter = FileFilter::new();
  let files : Vec<FileMetadata> = cognite_client.files.filter_all(file_filter).unwrap();
  println!("{} files retrieved.", files.len());

  // Search files
  let file_search : FileSearch = FileSearch::new();
  let file_filter_2 : FileFilter = FileFilter::new();
  let files_search_result : Vec<FileMetadata> = cognite_client.files.search(file_filter_2, file_search).unwrap();
  println!("Search found {:?} files", files_search_result.len());

  // List all users
  match cognite_client.users.list_all(None) {
    Ok(users) => println!("{} users retrieved.", users.len()),
    Err(e) => println!("{:?}", e)
  }
}