use cognite::time_series::*;
use prost::Message;
use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};

mod common;
use common::{get_client_for_mocking, get_path};

#[test]
fn state_datapoints_use_upstream_wire_tags() {
    let request = DataPointInsertionRequest::from(vec![AddDatapoints {
        id: cognite::models::instances::InstanceId {
            space: "s".to_owned(),
            external_id: "e".to_owned(),
        }
        .into(),
        datapoints: vec![DatapointState {
            timestamp: 1,
            numeric_value: Some(0),
            string_value: Some(String::new()),
            status: Some(StatusCode::Good),
        }]
        .into(),
    }]);

    let expected = [
        0x0a, 0x14, 0x2a, 0x06, 0x0a, 0x01, b's', 0x12, 0x01, b'e', 0x32, 0x0a, 0x0a, 0x08, 0x08,
        0x01, 0x10, 0x00, 0x1a, 0x00, 0x22, 0x00,
    ];
    assert_eq!(request.encode_to_vec(), expected);
    assert_eq!(
        DataPointInsertionRequest::decode(&expected[..]).unwrap(),
        request
    );
}

#[tokio::test]
async fn insert_mixed_datapoints_preserves_state_values() {
    let server = MockServer::start().await;
    let project = "my_project";
    Mock::given(method("POST"))
        .and(path(get_path("", project, "timeseries/data")))
        .and(header("content-type", "application/protobuf"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .expect(2)
        .mount(&server)
        .await;

    let values = [
        (Some(0), Some(""), None),
        (Some(-1), Some("stopped"), Some(StatusCode::Good)),
        (Some(i64::MAX), None, None),
        (Some(i64::MIN), Some("minimum"), None),
        (None, Some("running"), None),
        (None, None, Some(StatusCode::Bad)),
    ];
    let state_points: Vec<_> = values
        .iter()
        .enumerate()
        .map(
            |(i, (numeric_value, string_value, status))| DatapointState {
                timestamp: 1_700_000_000_000 + i as i64,
                numeric_value: *numeric_value,
                string_value: string_value.map(str::to_owned),
                status: *status,
            },
        )
        .collect();
    let batches = vec![
        AddDatapoints::new(
            123,
            vec![DatapointDouble {
                timestamp: 1,
                value: Some(1.5),
                status: None,
            }]
            .into(),
        ),
        AddDatapoints::new_external_id(
            "string-series",
            vec![DatapointString {
                timestamp: 2,
                value: Some("text".to_owned()),
                status: None,
            }]
            .into(),
        ),
        AddDatapoints {
            id: cognite::models::instances::InstanceId {
                space: "space".to_owned(),
                external_id: "state-series".to_owned(),
            }
            .into(),
            datapoints: state_points.into(),
        },
    ];
    let proto = DataPointInsertionRequest::from(batches.clone());
    let client = get_client_for_mocking(&server.uri(), project);
    client.time_series.insert_datapoints(batches).await.unwrap();
    client
        .time_series
        .insert_datapoints_proto(&proto)
        .await
        .unwrap();

    for request in server.received_requests().await.unwrap() {
        let decoded = DataPointInsertionRequest::decode(request.body.as_slice()).unwrap();
        assert_eq!(decoded.items.len(), 3);
        assert!(matches!(
            &decoded.items[0].datapoint_type,
            Some(InsertDatapointType::NumericDatapoints(dps)) if dps.datapoints[0].value == 1.5
        ));
        assert!(matches!(
            &decoded.items[1].datapoint_type,
            Some(InsertDatapointType::StringDatapoints(dps)) if dps.datapoints[0].value == "text"
        ));
        assert_eq!(
            decoded.items[2].time_series_reference,
            Some(TimeSeriesReference::InstanceId(InstanceId {
                space: "space".to_owned(),
                external_id: "state-series".to_owned(),
            }))
        );
        let Some(InsertDatapointType::StateDatapoints(dps)) = &decoded.items[2].datapoint_type
        else {
            panic!("Expected state datapoints");
        };
        assert_eq!(dps.datapoints.len(), values.len());
        for (i, (dp, (numeric_value, string_value, status))) in
            dps.datapoints.iter().zip(values).enumerate()
        {
            assert_eq!(dp.timestamp, 1_700_000_000_000 + i as i64);
            assert_eq!(dp.numeric_value, numeric_value);
            assert_eq!(dp.string_value.as_deref(), string_value);
            assert_eq!(dp.status, status.map(Status::from));
        }
    }
}
