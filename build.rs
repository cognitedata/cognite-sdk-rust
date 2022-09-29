use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "src/dto/core/datapoint/proto/data_point_insertion_request.proto",
            "src/dto/core/datapoint/proto/data_point_list_response.proto",
            "src/dto/core/datapoint/proto/data_points.proto",
        ],
        &["src/dto/core/datapoint/proto/"],
    )?;
    Ok(())
}
