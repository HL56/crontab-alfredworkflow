use std::env;
use std::error::Error;

use cron::Schedule;
use chrono::Local;
use std::str::FromStr;
use powerpack::Item;

fn main() -> Result<(), Box<dyn Error>> {
    let times = 5;
    let query = env::args()
        .nth(1)
        .as_deref()
        .map(str::trim)
        .unwrap_or("")
        .to_string();

    let mut vec = Vec::new();
    if query == "" {
        let item = Item::new("请输入cron表达式").valid(false);
        vec.push(item);
    } else {
        if query == "help" {
            vec.push(Item::new("f1 f2 f3 f4 f5").valid(false));
            vec.push(Item::new("f1 表示分钟(0-59)").valid(false));
            vec.push(Item::new("f2 表示小时(0-23)").valid(false));
            vec.push(Item::new("f3 表示一个月份中的第几日(1-31)").valid(false));
            vec.push(Item::new("f4 表示月份(1-12)").valid(false));
            vec.push(Item::new("f5 表示一个星期中的第几天(0-6)").valid(false));
        } else {
            // use linux crontab expression, first default 0
            let expression = format!("{} {}", "0", query);
            match Schedule::from_str(&*expression) {
                Ok(schedule) => {
                    vec.push(Item::new(format!("接下来{}次的执行时间：", times)).valid(true));
                    for datetime in schedule.upcoming(Local).take(times) {
                        let datetime_format = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                        vec.push(Item::new(datetime_format).valid(true));
                    }
                }
                Err(_) => {
                    let item;
                    if query.contains("h") {
                        item = Item::new("帮助")
                            .autocomplete("help")
                            .valid(false);
                    } else {
                        item = Item::new("请输入正确的cron表达式")
                            .autocomplete(query.trim().to_owned() + " *")
                            .valid(false);
                    }
                    vec.push(item);
                }
            }
        }
    }

    powerpack::output(vec.into_iter())?;
    Ok(())
}
