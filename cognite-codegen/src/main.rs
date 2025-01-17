fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("cognite/src/dto/core/datapoint/generated")?;
    prost_build::Config::new()
        .out_dir("cognite/src/dto/core/datapoint/generated")
        .compile_protos(
            &[
                "cognite-codegen/proto/data_point_insertion_request.proto",
                "cognite-codegen/proto/data_point_list_response.proto",
                "cognite-codegen/proto/data_points.proto",
            ],
            &["cognite-codegen/proto/"],
        )?;

    Ok(())
}
