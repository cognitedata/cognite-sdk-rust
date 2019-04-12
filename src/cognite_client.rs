use std::env;

use super::{ 
  Assets,
  Datapoints,
  Events,
  Files,
  Login,
  TimeSeries,

  ApiClient,
};

pub struct CogniteClient {
  api_key : String,
  project : String,
  base_url : String,
  pub api_client : ApiClient,

  pub assets : Assets,
  pub datapoints : Datapoints,
  pub events : Events,
  pub files : Files,
  pub time_series : TimeSeries
}

static COGNITE_API_KEY : &'static str = "COGNITE_API_KEY";
static COGNITE_BASE_URL : &'static str = "COGNITE_BASE_URL";

impl CogniteClient {

  pub fn new() -> Self {
    let api_key : String = match env::var(COGNITE_API_KEY) {
      Ok(api_key) => {
        println!("API KEY is set");
        api_key
      },
      Err(e) => panic!("{} is not defined in the environment. Error: {}", COGNITE_API_KEY, e)
    };

    let api_base_url : String = match env::var(COGNITE_BASE_URL) {
      Ok(base_url) => {
        println!("API BASE URL: {}", base_url);
        base_url
      },
      Err(e) => panic!("{} is not defined in the environment. Error: {}", COGNITE_BASE_URL, e)
    };
    let api_client = ApiClient::new(api_base_url.clone(), api_key.clone());

    // Get project name associated to API KEY
    let login_api_client = ApiClient::new(api_base_url.clone(), api_key.clone());
    let login = Login::new(login_api_client);
    let login_status = login.status();

    let project = login_status.project;
    println!("API PROJECT: {}", project);
    
    let api_version = "0.5".to_string();
    let api_base_path = format!("{}/api/{}/projects/{}", api_base_url, api_version, project);
    let assets_api_client = ApiClient::new(api_base_path.clone(), api_key.clone());
    let datapoints_api_client = ApiClient::new(api_base_path.clone(), api_key.clone());
    let events_api_client = ApiClient::new(api_base_path.clone(), api_key.clone());
    let files_api_client = ApiClient::new(api_base_path.clone(), api_key.clone());
    let time_series_api_client = ApiClient::new(api_base_path.clone(), api_key.clone());

    CogniteClient { 
      api_key : api_key,
      base_url : api_base_url,
      project : project,
      api_client : api_client,

      assets : Assets::new(assets_api_client),
      datapoints : Datapoints::new(datapoints_api_client),
      events : Events::new(events_api_client),
      files : Files::new(files_api_client),
      time_series : TimeSeries::new(time_series_api_client),
    }
  }
}