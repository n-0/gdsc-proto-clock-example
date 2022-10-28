fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(
            "action.Action",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "action.ActionState",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "clock.Empty",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "clock.Time",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "clock.Waiting",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .out_dir("src/protos")
        .compile(
            &[
                "../../schema/action.proto",
                "../../schema/clock.proto",
            ],
            &["../../schema/"],
        )?;
    Ok(())
}
