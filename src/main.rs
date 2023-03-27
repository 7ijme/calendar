use bpaf::Bpaf;
use chrono::{Datelike, Local};
use colored::*;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Args {
    #[bpaf(short('b'), long("before-christ"))]
    /// Whether the year is before Christ.
    before_christ: bool,
    #[bpaf(positional("month"), optional)]
    /// The month to print the calendar for.
    month: Option<u32>,
    #[bpaf(positional("year"), optional)]
    /// The year to print the calendar for.
    year: Option<i32>,
}

fn main() {
    // Check for arguments
    let args: Args = args().run();

    let month = args.month.unwrap_or_else(|| Local::now().month());
    let year = args.year.unwrap_or_else(|| Local::now().year());

    let year = if args.before_christ { -year } else { year };

    print_calendar(month, year);
}

fn print_calendar(month: u32, year: i32) {
    let weekdays = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let now = Local::now()
        .with_month(month)
        .expect("Please give me a valid month.")
        .with_year(year)
        .expect("Please give me a valid year.");
    let month = now.month();
    let year = now.year();

    // Print the month and year in the center of the line
    let amount_of_padding = (20 - months[month as usize - 1].len() - year.to_string().len()) / 2;
    println!(
        "{}",
        format!(
            "{}{} {}",
            " ".repeat(amount_of_padding),
            months[month as usize - 1],
            year
        )
        .red()
        .to_string()
    );
    // Print the weekdays
    println!(
        "{}",
        weekdays
            .iter()
            .map(|day| day.yellow().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let first_day_of_month = now.with_day(1).unwrap();
    let weekday_of_first_day = first_day_of_month.weekday().num_days_from_monday();

    // Print the days
    for day in 1..=31 {
        let day_date = match now.with_day(day) {
            Some(v) => v,
            None => break,
        };

        // Add padding for days before the first weekday of the month
        if day == 1 {
            print!("{}", "   ".repeat(weekday_of_first_day as usize));
        }

        // Print the day number
        let day_str = if day == now.day() && day_date.month() == Local::now().month() {
            format!("{:2}", day).red().to_string()
        } else {
            format!("{:2}", day).cyan().to_string()
        };
        print!("{} ", day_str);

        // Add a newline after each Saturday
        let is_last_day_of_month = now.with_day(day + 1).is_none();
        if day_date.weekday() == chrono::Weekday::Sun && !is_last_day_of_month {
            println!();
        }
    }

    println!();
}
