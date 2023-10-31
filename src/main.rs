use notify_rust::*;

const TITLE: &str = "入退室リマインダー[beta]";
const IN_TIME: [&str; 4] = ["10:55", "13:10", "14:50", "16:30"];
const OUT_TIME: [&str; 4] = ["12:25", "14:40", "16:20", "17:30"];
fn main() {
    let mut click_status = false;
    let body = "test";

    click_status = notify_do(&body);
    println!("{}, {}", click_status, get_date());

}

fn get_date() -> String{
    let date = chrono::Local::now();
    let date_str = date.format("%Y-%m-%d %H:%M").to_string();
    return date_str;
}


fn notify_do(body: &str) -> bool{
    let mut status = false;
    Notification::new()
        .summary(TITLE)
        .body(body)
        .action("click", "Ok")
        .hint(Hint::Resident(true))
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "click" | _ => {
                status = true;
            }
        });
    return status;
    }
