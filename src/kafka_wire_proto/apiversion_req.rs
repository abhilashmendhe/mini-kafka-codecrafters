use crate::kafka_wire_proto::kbroker_req_res::compute_response_size_to_vec;

#[allow(unused)]
pub async fn parse_api_version_req(
    mut start: usize, 
    mut end: usize,
    api_key_bytes: &[u8],
    bytes: &[u8]
) -> Vec<u8> {

    let mut resp_bytes = vec![];

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
    let _clieint_id_content = &bytes[start..end];

    // e. Tag buffer
    start = end;
    end += 1;
    let _req_header_tag_buf = &bytes[start..end];

    // add everything to the response bytes
    resp_bytes.extend_from_slice(&corr_id);       // correlation Id
    resp_bytes.extend_from_slice(&error_code);    // Error code
    resp_bytes.extend_from_slice(&[03]);          // API version array len (size + 1)

    resp_bytes.extend_from_slice(&api_key_bytes); // API key
    resp_bytes.extend_from_slice(&[0,0]);         // Min supported API version
    resp_bytes.extend_from_slice(&[0,4]);         // Max supported API version
    resp_bytes.extend_from_slice(&[0]);           // Tag buff of API version element of array

    resp_bytes.extend_from_slice(&[0,75]);        // API key
    resp_bytes.extend_from_slice(&[0,0]);         // Min supported API version
    resp_bytes.extend_from_slice(&[0,0]);         // Max supported API version
    resp_bytes.extend_from_slice(&[0]);           // Tag buff of API version element of array

    resp_bytes.extend_from_slice(&[0,0,0,0]);     // API vers throttle time
    resp_bytes.extend_from_slice(&[0]);           // API vers resp body tag buffer
    
    let resp_msg_size = compute_response_size_to_vec(resp_bytes.len()).await;
    // println!("resp_msg_size: {:?}",resp_msg_size);
    let mut new_resp_bytes = vec![];
    new_resp_bytes.extend_from_slice(&resp_msg_size);
    new_resp_bytes.extend_from_slice(&resp_bytes);

    new_resp_bytes
}