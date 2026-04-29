#[macro_export]
macro_rules! children {
    ($($child:expr),* $(,)?) => {
        vec![
            $($child.into()),*
        ]
    };
}

#[macro_export]
macro_rules! properties {
    ($(($prop:expr, $value:expr)),* $(,)?) => {
        vec![$(Property::new($prop, $value)),*]
    };
}
