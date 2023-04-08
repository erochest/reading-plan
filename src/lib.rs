pub mod error;

use std::iter::Iterator;

use chrono::NaiveDate;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ReadingDay {
    pub start_page: u32,
    pub raw_page_count: f32,
    pub page_count: u32,
    pub date: NaiveDate,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct ReadPages {
    start_date: NaiveDate,
    start_page: u32,
    end_date: NaiveDate,
    end_page: u32,
    current_page: u32,
    current_date: NaiveDate,
}

impl ReadPages {
    fn days_remaining(&self) -> u32 {
        // Needs to be +1 because the current day is not included in the count.
        ((self.end_date - self.current_date).num_days() as u32) + 1
    }
}

impl Iterator for ReadPages {
    type Item = ReadingDay;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page >= self.end_page || self.current_date > self.end_date {
            return None;
        }
        let page_count = self.end_page - self.current_page;
        let raw_page_count = page_count as f32 / self.days_remaining() as f32;
        let page_count = raw_page_count.round() as u32;
        let result = ReadingDay {
            start_page: self.current_page,
            raw_page_count,
            page_count,
            date: self.current_date,
        };
        self.current_page += page_count;
        self.current_date = self.current_date.succ_opt().unwrap();
        Some(result)
    }
}

pub fn read_pages(
    start_date: NaiveDate,
    start_page: u32,
    end_date: NaiveDate,
    end_page: u32,
) -> impl Iterator<Item = ReadingDay> {
    ReadPages {
        start_date,
        start_page,
        end_date,
        end_page,
        current_page: start_page,
        current_date: start_date,
    }
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

        let result = read_pages(start_date, start_page, end_date, page_count);

        assert_eq!(result.count(), 0);
    }

    #[test]
    fn when_start_date_is_after_end_date_then_returns_empty_iter() {
        let page_count = 10;
        let start_page = 5;
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 2).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();

        let result = read_pages(start_date, start_page, end_date, page_count);

        assert_eq!(result.count(), 0);
    }

    #[test]
    fn when_one_day_then_returns_one_page() {
        let page_count = 10;
        let start_page = 5;
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();

        let mut result = read_pages(start_date, start_page, end_date, page_count);
        let result = result.next();

        assert_eq!(
            result,
            Some(ReadingDay {
                start_page: 5,
                raw_page_count: 5.0,
                page_count: 5,
                date: start_date,
            })
        );
    }

    #[test]
    fn when_two_days_then_splits_the_reading_over_days() {
        let page_count = 10;
        let start_page = 4;
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 1, 2).unwrap();

        let mut result = read_pages(start_date, start_page, end_date, page_count);
        let first = result.next();
        let second = result.next();

        assert_eq!(
            first,
            Some(ReadingDay {
                start_page: 4,
                raw_page_count: 3.0,
                page_count: 3,
                date: start_date,
            })
        );
        assert_eq!(
            second,
            Some(ReadingDay {
                start_page: 7,
                raw_page_count: 3.0,
                page_count: 3,
                date: start_date.succ_opt().unwrap(),
            })
        );
    }

    #[test]
    fn when_two_days_with_odd_pages_then_unequally_spreads_reading_over_days() {
        let page_count = 11;
        let start_page = 4;
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 1, 2).unwrap();

        let mut result = read_pages(start_date, start_page, end_date, page_count);
        let first = result.next();
        let second = result.next();

        assert_eq!(
            first,
            Some(ReadingDay {
                start_page: 4,
                raw_page_count: 3.5,
                page_count: 4,
                date: start_date,
            })
        );
        assert_eq!(
            second,
            Some(ReadingDay {
                start_page: 8,
                raw_page_count: 3.0,
                page_count: 3,
                date: start_date.succ_opt().unwrap(),
            })
        );

    }
}
