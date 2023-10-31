use rsntp::*;
use chrono::{DateTime, Local};


pub fn clock() -> String{
    let client = SntpClient::new();
    let result = client.synchronize("ntp.nict.jp").unwrap();
    
    let local_time: DateTime<Local> =
        DateTime::from(result.datetime().into_chrono_datetime().unwrap());
    return local_time.format("%H:%M:%S").to_string();
}
