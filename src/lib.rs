use chrono::prelude::*;
use chrono::Duration;
use chrono::LocalResult;
use chrono::Utc;
use num_traits::cast::FromPrimitive;
use num_traits::cast::ToPrimitive;
use num_traits::NumOps;

pub trait Dotago {
    fn second(self) -> Self;
    fn seconds(self) -> Self;
    fn minute(self) -> Self;
    fn minutes(self) -> Self;
    fn hour(self) -> Self;
    fn hours(self) -> Self;
    fn day(self) -> Self;
    fn days(self) -> Self;
    fn week(self) -> Self;
    fn weeks(self) -> Self;
    fn ago(&self) -> i64;
    fn from_now(&self) -> i64;
    fn as_date(self) -> Option<DateTime<Utc>>;
}

impl<T> Dotago for T
where
    T: NumOps + FromPrimitive + ToPrimitive,
{
    fn second(self) -> Self {
        self * T::from_u32(1000).unwrap()
    }

    fn seconds(self) -> Self {
        self * T::from_u32(1000).unwrap()
    }

    fn minute(self) -> Self {
        self.seconds() * T::from_u32(60).unwrap()
    }

    fn minutes(self) -> Self {
        self.seconds() * T::from_u32(60).unwrap()
    }

    fn hour(self) -> Self {
        self.minutes() * T::from_u32(60).unwrap()
    }

    fn hours(self) -> Self {
        self.minutes() * T::from_u32(60).unwrap()
    }

    fn day(self) -> Self {
        self.hours() * T::from_u32(24).unwrap()
    }

    fn days(self) -> Self {
        self.hours() * T::from_u32(24).unwrap()
    }

    fn week(self) -> Self {
        self.days() * T::from_u32(7).unwrap()
    }

    fn weeks(self) -> Self {
        self.days() * T::from_u32(7).unwrap()
    }

    fn ago(&self) -> i64 {
        Utc::now()
            .checked_sub_signed(Duration::milliseconds(self.to_i64().unwrap()))
            .unwrap()
            .timestamp_millis()
    }

    fn from_now(&self) -> i64 {
        Utc::now()
            .checked_add_signed(Duration::milliseconds(self.to_i64().unwrap()))
            .unwrap()
            .timestamp_millis()
    }

    fn as_date(self) -> Option<DateTime<Utc>> {
        match Utc.timestamp_millis_opt(self.to_i64().unwrap()) {
            LocalResult::Single(dt) => Some(dt),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::assert_ge;
    use claims::assert_le;

    #[test]
    fn second() {
        assert_eq!(1.second(), 1000);
    }

    #[test]
    fn seconds() {
        assert_eq!(2.seconds(), 2 * 1000);
    }

    #[test]
    fn minute() {
        assert_eq!(1.minute(), 1000 * 60);
    }

    #[test]
    fn minutes() {
        assert_eq!(2.minutes(), 2 * 1000 * 60);
    }

    #[test]
    fn hour() {
        assert_eq!(1.hour(), 1000 * 60 * 60);
    }

    #[test]
    fn hours() {
        assert_eq!(2.hours(), 1000 * 60 * 60 * 2);
    }

    #[test]
    fn day() {
        assert_eq!(1.day(), 1000 * 60 * 60 * 24);
    }

    #[test]
    fn days() {
        assert_eq!(2.days(), 1000 * 60 * 60 * 24 * 2);
    }

    #[test]
    fn week() {
        assert_eq!(1.week(), 1000 * 60 * 60 * 24 * 7);
    }

    #[test]
    fn weeks() {
        assert_eq!(2.weeks(), 1000 * 60 * 60 * 24 * 7 * 2);
    }

    #[test]
    fn floating_numbers() {
        assert_eq!(1.5.minutes(), 1000.0 * 60.0 * 1.5);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!((-2.0).minutes(), 1000.0 * 60.0 * -2.0);
    }

    #[test]
    fn five_minutes_ago() {
        let ts = 5.minutes().ago().as_date().unwrap().timestamp();

        let expected = Utc::now()
            .checked_sub_signed(Duration::minutes(5))
            .unwrap()
            .timestamp();
        assert_le!(ts, expected);
        assert_ge!(ts, expected - 10);
    }
}
