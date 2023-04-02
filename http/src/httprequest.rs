use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method{
    GET,
    POST,
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Version{
    V1_1,
    V2_0,
    Uninitialized,
} 




impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized,
        }
    }
}

impl From<&str> for Version{
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::GET);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }
}

pub enum Resource {
    path(String),
}

pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
} 
impl From<String> for HttpRequest {
    fn from(value: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";
        for line in req.lines(){
            if line.contains("HTTP"){
                // 方法，资源，版本
                let (method, resource, version) = process_request_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            }else if line.contains(":"){
                // 头部
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            }else if line.len() == 0{ // 处理空行
                // 消息体
                parsed_msg_body = process_msg_body(req);
                 
            }else{
                parsed_msg_body = line;
            }
        }
    }

    HttpRequest {
        method: parsed_method,
        version: parsed_version,
        resource: parsed_resource,
        headers: parsed_headers,
        msg_body: parsed_msg_body.to_string(),
    }
}

// 写两个process函数，分别处理请求行和头部行
// 传入的是字符串切片，返回的是一个元组
fn process_request_line(req: &str) -> (Method, Resource, Version){
    let mut words = s.split_whitespace();
    let mut method = Method::Uninitialized;
    let mut resource = Resource::path("".to_string());
    let mut version = Version::Uninitialized;
    (method.into(), Resource::path("".to_string()), version.into())

}


fn process_header_line(line: &str) -> (String, String){
    let mut header_items = line.split(":");
    let mut key = words.next().unwrap().to_string();
    let mut value = words.next().unwrap().to_string();
    // 如果能成功取得下一个元素，就把key赋值为下一个元素
    if let Some(k) = header_items.next(){
        key = k.to_string();
    }
    if let Some(v) = header_items.next(){
        value = v.to_string();
    }
    (key, value)
}

// [test]
// fn test_process_request_line(){
//     let s: String = String::from("GET /greeting HTTP")
//     let (method, resource, version) = process_request_line("GET / HTTP/1.1");
//     assert_eq!(method, Method::GET);
//     assert_eq!(resource, Resource::path("/".to_string()));
//     assert_eq!(version, Version::V1_1);
// }