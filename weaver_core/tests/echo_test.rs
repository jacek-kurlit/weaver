use weaver_core::components::debug::echo::Echo;

#[test]
fn should_return_message_echo() {
    let echo = Echo {
        message: "Hello world!".to_string(),
    };
    let message = echo.execute().unwrap();
    assert_eq!(message, "Hello world!");
}
