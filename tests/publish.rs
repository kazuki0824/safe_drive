pub mod common;

use common::msgs::example_msg::msg::Num;
use safe_drive::{self, context::Context};
use std::error::Error;

#[test]
fn test_publish() -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    let ctx = Context::new()?;
    let node = ctx
        .create_node("test_publish_node", None, Default::default())
        .unwrap();

    #[cfg(any(feature = "humble", feature = "galactic"))]
    let publisher = node.create_publisher::<Num>("test_publish", Default::default())?;

    #[cfg(not(any(feature = "humble", feature = "galactic")))]
    let publisher = node.create_publisher::<Num>("test_publish", Default::default(), true)?;

    let msg = Num { num: 100 };
    publisher.send(&msg)?;

    Ok(())
}
