use crate::kafka_wire_proto::{apiversion_req::parse_api_version_req, describe_topic_part_req::parse_describe_topic_req};

#[allow(unused)]
pub async fn parse_broker_req(bytes: &[u8]) -> Vec<u8> {

    let mut start = 0;
    let mut end = 4;

    // Message size (4 bytes)
    let msg_size_bytes = &bytes[start..end];

    // Request Header (v2)
    // a. API Key
    start = end;
    end += 2;
    let api_key_bytes = &bytes[start..end];

    let resp_bytes = if api_key_bytes == [0, 18] { // API Version Request Body (v4)
        parse_api_version_req(start, end, &api_key_bytes, &bytes).await
    } else if api_key_bytes == [0, 75] {
        parse_describe_topic_req(start, end, api_key_bytes, &bytes).await
    } else {
        vec![]
    };
    resp_bytes
}

pub async fn compute_response_size_to_vec(mut n: usize) -> Vec<u8> {
    
    let mut msg_size = [0 as u8; 5];
    let mut i = 0;
    let mut tfs = 256*256*256*256 as usize;
    while i < 4 {
        let quot = n / tfs;
        
        msg_size[i] = quot as u8;
        if quot != 0 {
            n = n % tfs;
        }
        tfs /= 256;
        i += 1;
    }
    msg_size[i] = n as u8;
    msg_size[1..].to_vec()
}