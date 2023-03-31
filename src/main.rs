fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rusty Puzzle Solver",
        native_options,
        Box::new(|cc| Box::new(rusting_puzzle::RustyPuzzle::new(cc))),
    )
}
