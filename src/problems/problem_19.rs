use super::Result;

/*
Problem 19:
You are given the following information, but you may prefer to do some research for yourself.

- 1 Jan 1900 was a Monday.

- Thirty days has September,
  April, June and November.
  All the rest have thirty-one,
  Saving February alone,
  Which has twenty-eight, rain or shine.
  And on leap years, twenty-nine.

- A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

Answer: 171
*/

pub struct Options {
    // ISO8601 YYYY-MM-DD => (YYYY, MM, DD)
    from: Date,
    to: Date,
    day_of_week: DayOfWeek,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            from: Date::new(1901, 1, 1),
            to: Date::new(2000, 12, 31),
            day_of_week: DayOfWeek::Sunday,
        }
    }
}

const STARTING_DATE: Date = Date {
    year: 1900,
    month: 01,
    day: 01,
};
const STARTING_DAY_OF_WEEK: DayOfWeek = DayOfWeek::Monday;

pub fn solve(options: Options) -> Result<usize> {
    let mut date = STARTING_DATE;
    let mut day_of_week = STARTING_DAY_OF_WEEK;

    let mut num_of_weekdays = 0;

    while date != options.from {
        date.increment();
        day_of_week.increment();
    }

    while date != options.to {
        if day_of_week == options.day_of_week && date.day == 1 {
            num_of_weekdays += 1;
        }

        date.increment();
        day_of_week.increment();
    }

    if day_of_week == options.day_of_week && date.day == 1 {
        num_of_weekdays += 1;
    }

    Ok(num_of_weekdays)
}

#[derive(PartialEq, Eq)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    fn new(year: u16, month: u8, day: u8) -> Date {
        Date { year, month, day }
    }

    fn increment(&mut self) {
        const FEBRUARY: u8 = 2;
        const APRIL: u8 = 4;
        const JUNE: u8 = 6;
        const SEPTEMBER: u8 = 9;
        const NOVEMBER: u8 = 11;
        const DECEMBER: u8 = 12;

        if self.day < 28 {
            // Not end of any month yet
            self.naive_increment_day();
        } else if self.day == 28 {
            if self.month != FEBRUARY {
                // February is only month that can end in 28,
                // so not end of month yet
                self.naive_increment_day();
            } else if self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0) {
                // Leap year, so not end of month yet
                self.naive_increment_day();
            } else {
                // End of Feb. Go to March
                self.naive_increment_month();
            }
        } else if self.day == 29 {
            if self.month == FEBRUARY {
                // End of Feb on leap year. Go to March
                self.naive_increment_month();
            } else {
                // Otherwise, not end of month yet
                self.naive_increment_day();
            }
        } else if self.day == 30 {
            if self.month == APRIL
                || self.month == JUNE
                || self.month == SEPTEMBER
                || self.month == NOVEMBER
            {
                self.naive_increment_month();
            } else {
                self.naive_increment_day();
            }
        } else {
            // self.day == 31

            if self.month == DECEMBER {
                self.naive_increment_year();
            } else {
                self.naive_increment_month();
            }
        }
    }

    fn naive_increment_day(&mut self) {
        self.day += 1;
    }

    fn naive_increment_month(&mut self) {
        self.day = 1;
        self.month += 1;
    }

    fn naive_increment_year(&mut self) {
        const JANUARY: u8 = 1;

        self.day = 1;
        self.month = JANUARY;
        self.year += 1;
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    fn increment(&mut self) {
        // Prefer match over an array-lookup,
        //  as array with lookup requires an integer (with a max), constantly increasing, while calling modulus on the new value
        // Seems unnecessary to me
        *self = match self {
            Self::Monday => Self::Tuesday,
            Self::Tuesday => Self::Wednesday,
            Self::Wednesday => Self::Thursday,
            Self::Thursday => Self::Friday,
            Self::Friday => Self::Saturday,
            Self::Saturday => Self::Sunday,
            Self::Sunday => Self::Monday,
        };
    }
}
