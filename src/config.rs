#[macro_export]
macro_rules! get_element {
    ($c:ident, $i:expr) => ($c.get($i).unwrap().parse().unwrap();)
}
