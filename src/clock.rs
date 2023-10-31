// MEMO: ntpサーバーにポーリングレート制限があるため、1時間に最大20回までしかリクエストしない。また、リクエストは1日最大480回までを厳守する。
// TODO: request_history.txtにリクエストした日付・時刻を都度保存する。
// TODO: ntpサーバーにリクエストする前に、request_history.txtを読み込み、リクエスト制限を超えないようにする。
// MEMO: 制限を超えた場合はシステムの時刻を取得するようにする。
// MEMO: request_history.txtがなければ作成し、現在時刻を書き込む。
// MEMO: request_history.txtの書式は、YYYY-MM-DD HH:MM:SSにする。

use rsntp::*;
use chrono::{DateTime, Local};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

const REQUEST_DAY_MAX:i32 = 480;
const REQUEST_HOUR_MAX:i32 = 20;


pub fn clock() -> String{
    let client = SntpClient::new();
    let result = client.synchronize("ntp.nict.jp").unwrap();
    
    let local_time: DateTime<Local> =
        DateTime::from(result.datetime().into_chrono_datetime().unwrap());
    return local_time.format("%H:%M:%S").to_string();
}


