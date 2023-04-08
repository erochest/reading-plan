pub mod error;

use chrono::NaiveDate;

pub fn get_pages(
    start_date: NaiveDate,
    start_page: u32,
    end_date: NaiveDate,
    page_count: u32,
) -> impl Iterator<Item = u32> {
    if start_date > end_date || start_page > page_count {
        return (1..1).into_iter();
    }
    (start_page..page_count).into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    #[test]
    fn when_starts_past_page_count_then_returns_empty_iter() {
        let page_count = 10;
        let start_page = 11;
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 1, 2).unwrap();

        let result = get_pages(start_date, start_page, end_date, page_count);

        assert_eq!(result.count(), 0);
    }

    #[test]
    fn when_start_date_is_after_end_date_then_returns_empty_iter() {
        let page_count = 10;
        let start_page = 5;
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 2).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();

        let result = get_pages(start_date, start_page, end_date, page_count);

        assert_eq!(result.count(), 0);
    }
}
