#[macro_export]

macro_rules! get_element {
    // $c -- section
    // $i -- parameter
    // $v -- default value
    ($c:ident, $i:expr, $v:expr) => (
        $c.get($i).unwrap_or(&($v.to_string())).parse().unwrap();
    )
}

macro_rules! get_section {
    // $c -- config
    // $s -- section name
    ($c:ident, $s:expr) => (
        match $c.section(Some($s.to_owned())) {
            Some(value) => value,
            None => panic!("Section `[{}]` not found!", $s)
        }
    )
}
