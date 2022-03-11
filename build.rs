fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .format(true)
        .compile(
            &[
                "contracts-repo/src/docs/contracts/common.proto",
                "contracts-repo/src/docs/contracts/instruments.proto",
                "contracts-repo/src/docs/contracts/marketdata.proto",
                "contracts-repo/src/docs/contracts/operations.proto",
                "contracts-repo/src/docs/contracts/orders.proto",
                "contracts-repo/src/docs/contracts/sandbox.proto",
                "contracts-repo/src/docs/contracts/stoporders.proto",
                "contracts-repo/src/docs/contracts/users.proto",
            ],
            &["contracts-repo/src/docs/contracts"],
        )
        .unwrap();

    Ok(())
}
