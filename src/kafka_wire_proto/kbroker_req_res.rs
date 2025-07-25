#[allow(unused)]
pub async fn broker_req(bytes: &[u8]) -> Vec<u8> {

    let mut resp_bytes = vec![];

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

    let error_code = {
        if api_ver_bytes[0] == 0 && api_ver_bytes[1] <= 4 {
            [0 as u8, 0]
        } else {    
            [0 as u8, 35]
        }
    };

    // c. Correlation ID 
    start = end;
    // println!("Error code: {:?}", error_code);
    end += 4;
    let corr_id = &bytes[start..end];

    // d. Client ID
    // d.1 client id length
    start = end;
    end += 2;
    let client_id_size = &bytes[start..end];
    let cis = (256 * client_id_size[0] as usize) + client_id_size[1] as usize;
    
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


    // add everything to the response bytes
    resp_bytes.extend_from_slice(&corr_id);       // correlation Id
    resp_bytes.extend_from_slice(&error_code);    // Error code
    resp_bytes.extend_from_slice(&[02]);          // API version array len (size + 1)
    resp_bytes.extend_from_slice(&api_key_bytes); // API key
    resp_bytes.extend_from_slice(&[0,0]);         // Min supported API version
    resp_bytes.extend_from_slice(&[0,4]);         // Max supported API version
    resp_bytes.extend_from_slice(&[0]);           // Tag buff of api element of array
    resp_bytes.extend_from_slice(&[0,0,0,0]);     // API vers throttle time
    resp_bytes.extend_from_slice(&[0]);           // API vers resp body tag buffer
    
    let resp_msg_size = compute_response_size(resp_bytes.len()).await;
    // println!("resp_msg_size: {:?}",resp_msg_size);
    let mut new_resp_bytes = vec![];
    new_resp_bytes.extend_from_slice(&resp_msg_size);
    new_resp_bytes.extend_from_slice(&resp_bytes);
    new_resp_bytes
}

pub async fn compute_response_size(mut n: usize) -> Vec<u8> {
    
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