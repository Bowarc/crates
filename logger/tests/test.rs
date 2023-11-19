use logger;

#[test]
fn file_and_stdout_test() {
    let cfg = logger::LoggerConfig::new().set_level(log::LevelFilter::Debug);
    logger::init(cfg, Some("log_file.log"));
    logger::test()
}
