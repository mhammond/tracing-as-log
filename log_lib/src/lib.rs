pub fn inner(i: u32) {
    log::debug!("inner {i}, via `log::debug!`");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app() {
        env_logger::try_init().ok();
        inner(2);
        panic!("test panic");
    }
}
