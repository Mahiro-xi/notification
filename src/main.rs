use notify_rust::*;

const TITLE: &str = "入退室リマインダー[beta]";
const IN_TIME: [&str; 4] = ["10:55", "13:10", "14:50", "16:30"];
const OUT_TIME: [&str; 4] = ["12:25", "14:40", "16:20", "17:30"];
fn main() {
    let mut click_status = false;
    let date = chrono::Local::now();
    let date_str = date.format("%a").to_string();
    if date_str != "Tue" {
        Notification::new()
            .summary(TITLE)
            .body("本日の勤務はありません")
            .show()
            .unwrap();
        std::process::exit(0);
    }
    loop {

        std::thread::sleep(std::time::Duration::from_secs(2));
        let date = chrono::Local::now();
        let date_str = date.format("%H:%M").to_string();
        
        // Debug
        println!("Debug: {}\nclick_status: {}\nin_time: {}\nout_time: {}", get_date(), click_status, IN_TIME.contains(&&*date_str), OUT_TIME.contains(&&*date_str));
        println!("");

        if click_status == true && !(IN_TIME.contains(&&*date_str) | OUT_TIME.contains(&&*date_str)){
            click_status = false;
        }

        if IN_TIME.contains(&&*date_str) && click_status == false{
            let body = "入室時間です";
            click_status = notify_do(&body,&"入室");
            println!("{}, {}", click_status, get_date());

        } else if OUT_TIME.contains(&&*date_str) && click_status == false {
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
        .timeout(Timeout::Never)
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "click"  => {
                status = true;
            },
            &_ => todo!()
        });
    return status;
    }
