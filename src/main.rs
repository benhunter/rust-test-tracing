use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("✅ Hello, world!");
}

#[cfg(test)]
mod tests {
    use test_log::test;
    use tracing::info;

    #[test]
    fn test_info_unit() {
        info!("❌ info in unit test");
        assert!(false);
    }
}
