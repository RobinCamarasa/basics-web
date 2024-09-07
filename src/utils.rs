#[macro_export]
macro_rules! html {
    ($template:expr) => {
        Html($template.render().unwrap())
    };
}
