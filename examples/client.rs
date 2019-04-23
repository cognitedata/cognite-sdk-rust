extern crate cognite;

use cognite::{
  CogniteClient,
  Asset,
  Event,
  TimeSerie,
  File,
  Params,
};

fn main() {
  let cognite_client = CogniteClient::new();

  // List all assets
  let assets : Vec<Asset> = cognite_client.assets.list_all(None);
  println!("{} assets retrieved.", assets.len());
  
  // Retrieve asset
  let asset : Asset = cognite_client.assets.retrieve(6687602007296940);
  println!("{:?}", asset);

  // Search asset
  let params = Some(vec!(
    Params::AssetsSearch_Name("Aker".to_owned()), 
    Params::AssetsSearch_Description("Aker".to_owned())
  ));
  let asset_search : Vec<Asset> = cognite_client.assets.search(params);
  println!("{:?}", asset_search);

  // Retrieve multiple assets
  let assets_multiple : Vec<Asset> = cognite_client.assets.retrieve_multiple(vec!(6687602007296940));
  println!("{:?}", assets_multiple);

  // List all events
  let events : Vec<Event> = cognite_client.events.list_all(None);
  println!("{} events retrieved.", events.len());

  // Search events
  let event_search_params = Some(vec!(
    Params::EventsSearch_SubType("val".to_owned()),
  ));
  let event_search : Vec<Event> = cognite_client.events.search(event_search_params);
  println!("Search found {:?} events", event_search.len());

  // List all events
  let time_series : Vec<TimeSerie> = cognite_client.time_series.list_all(None);
  println!("{} time series retrieved.", time_series.len());

  // Search time serie
  let time_serie_search_params = Some(vec!(
    Params::TimeSeriesSearch_Name("val".to_owned()),
  ));
  let time_series_search : Vec<TimeSerie> = cognite_client.time_series.search(time_serie_search_params);
  println!("Search found {:?} time series", time_series_search.len());

  // List all files
  let files : Vec<File> = cognite_client.files.list_all(None);
  println!("{} files retrieved.", files.len());

  // Search files
  let files_search_params = Some(vec!(
    Params::FilesSearch_Name("ph".to_owned()),
  ));
  let files_search : Vec<File> = cognite_client.files.search(files_search_params);
  println!("Search found {:?} files", files_search.len());
}