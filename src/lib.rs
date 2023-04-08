pub mod error;

pub fn get_pages(start_page: u32, page_count: u32) -> impl Iterator<Item = u32> {
    (start_page..page_count).into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn when_starts_past_page_count_then_returns_empty_iter() {
        let page_count = 10;
        let start_page = 11;

        let result = get_pages(start_page, page_count);

        assert_eq!(result.count(), 0);
    }
}