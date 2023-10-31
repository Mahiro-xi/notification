use notify_rust::*;

const TITLE: &str = "入退室リマインダー[beta]";
const IN_TIME: [&str; 4] = ["10:55", "13:10", "14:50", "16:30"];
const OUT_TIME: [&str; 4] = ["12:25", "14:40", "16:20", "17:30"];
fn main() {
    let mut click_status = false;
    Notification::new().summary(TITLE).body("起動しました").show().unwrap();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let date = chrono::Local::now();
        let date_str = date.format("%H:%M").to_string();
        if IN_TIME.contains(&&*date_str) {
            let body = "入室時間です";
            click_status = notify_do(&body,&"入室");
            println!("{}, {}", click_status, get_date());
        } else if OUT_TIME.contains(&&*date_str) {
            let body = "退室時間です";
            click_status = notify_do(&body,&"退室");
            println!("{}, {}", click_status, get_date());
        }
    }

}

fn get_date() -> String{
    let date = chrono::Local::now();
    let date_str = date.format("%Y-%m-%d %H:%M").to_string();
    return date_str;
}


fn notify_do(body: &str,inout: &str) -> bool{
    let mut status = false;
    Notification::new()
        .summary(TITLE)
        .body(body)
        .action("click", inout)
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
