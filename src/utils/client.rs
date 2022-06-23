
use reqwest;



pub fn http_get(url:String)->Vec<u8> {
    reqwest::blocking::get(url.as_str()).unwrap().bytes().unwrap().to_vec()
}

#[allow(dead_code)]
fn test_client() {
    let url = "http://jsonplaceholder.typicode.com/users";
    println!("{:?}",http_get(url.to_string()));

}
