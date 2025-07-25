#[allow(unused)]
pub async fn broker_req(bytes: &[u8]) -> Vec<Vec<u8>> {

    let mut start = 0;
    let mut end = 4;

    // Message size (4 bytes)
    let msg_size_bytes = &bytes[start..end];

    // Request Header (v2)
    // a. API Key
    start = end;
    end += 2;
    let api_key_bytes = &bytes[start..end];

    // b. API Version
    start = end;
    end += 2;
    let api_ver_bytes = &bytes[start..end];
    println!("API ver: {:?}",&api_ver_bytes);

    let error_code = {
        if api_ver_bytes[0] == 0 && api_ver_bytes[1] <= 4 {
            [0 as u8, 0]
        } else {    
            [0 as u8, 35]
        }
    };
    println!("Error code: {:?}", error_code);
    // c. Correlation ID 
    start = end;
    end += 4;
    let corr_id = &bytes[start..end];

    // d. Client ID
    // d.1 client id length
    start = end;
    end += 2;
    let client_id_size = &bytes[start..end];
    let cis = (256 * client_id_size[0] as usize) + client_id_size[1] as usize;
    println!("Client ID size: {}", cis);
    // d.2 client id content
    start = end;
    end += cis;
    let clieint_id_content = &bytes[start..end];

    // e. Tag buffer
    start = end;
    end += 1;
    let req_header_tag_buf = &bytes[start..end];

    // API Version Request Body (v4)
    // ........

    vec![corr_id.to_vec(), error_code.to_vec()]
}