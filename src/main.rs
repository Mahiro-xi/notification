use clock::clock;
use notify_rust::*;
mod clock;
fn main() {
    // 入退出処理リマインダー
    let title = "入退出処理リマインダー";
    let time = clock();
    let body = time;

    notify_do(title, &body);

}

fn notify_do(title: &str, body: &str) {
    Notification::new()
        .summary(title)
        .body(body)
        .show()
        .unwrap();
}

fn notify_timing(){
    // 入室時間
    
}