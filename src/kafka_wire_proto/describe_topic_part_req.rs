use crate::kafka_wire_proto::kbroker_req_res::compute_response_size_to_vec;

#[allow(unused)]
pub async fn parse_describe_topic_req(
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
    // println!("client_id_size: {:?}",client_id_size);
    let cis = (256 * client_id_size[0] as usize) + client_id_size[1] as usize;
    // println!("{}",cis);
    // d.2 client id content
    start = end;
    end += cis;
    let _clieint_id_content = &bytes[start..end];

    // e. Tag buffer
    start = end;
    end += 1;
    let _req_header_tag_buf = &bytes[start..end];
    
    // 3. parse topics array
    start = end;
    end += 1;
    let array_bytes = &bytes[start..end];
    
    // add everything to the response bytes
    // 1. response header (v1)
    resp_bytes.extend_from_slice(&corr_id);       // correlation Id
    resp_bytes.extend_from_slice(&[0]);           // Tag buffer

    // 2.Describe topic partition body (v0)
    resp_bytes.extend_from_slice(&[0,0,0,0]);     // Throttle time

    let mut array_len = if array_bytes[0] == 0 { 0 } else { array_bytes[0] - 1 };
    
    // 3. Topics Array
    // 3.1 Add 1 byte as array len
    resp_bytes.extend_from_slice(&[array_len + 1]);
    
    while array_len > 0 {

        resp_bytes.extend_from_slice(&[0,3]);     // Error code (UNKNOWN_TOPIC)
    
        start = end;
        end += 1;
        let topic_name_len_bytes = &bytes[start..end];
        let topic_name_len = if topic_name_len_bytes[0] == 0 { 0 } else { topic_name_len_bytes[0] - 1 };

        // Add topic name (Length, and content)
        // Add topic name length as byte
        resp_bytes.extend_from_slice(&[topic_name_len+1]);

        start = end;
        end += topic_name_len as usize;
        let topic_name_bytes = &bytes[start..end];

        // Add topic name content as bytes
        resp_bytes.extend_from_slice(topic_name_bytes);

        // Add topic id (16 bytes)
        resp_bytes.extend_from_slice(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

        // Add topic byte (is internal) defualt is false
        resp_bytes.extend_from_slice(&[0]);

        // Partition array () if empty the byte is 01
        resp_bytes.extend_from_slice(&[0]);

        // Add topic authorized ops
        resp_bytes.extend_from_slice(&[0,0,13,248]);

        start = end;
        end += 1;
        let topci_tag_buf_bytes = &bytes[start..end];

        // Add topic tag buffer
        resp_bytes.extend_from_slice(&[0]);

        array_len -= 1;
    }

    start = end;
    end += 4;
    let resp_partition_limit_bytes = &bytes[start..end];

    start = end;
    end += 1;
    let cursor_bytes = &bytes[start..end];

    start = end;
    end += 1;
    let tag_bffer = &bytes[start..end];

    // 4. 
    resp_bytes.extend_from_slice(cursor_bytes);   // DescribePartition Next Cursor
    resp_bytes.extend_from_slice(tag_bffer);      // DescribePartition tag buffer

    let resp_msg_size = compute_response_size_to_vec(resp_bytes.len()).await;
    // println!("resp_msg_size: {:?}",resp_msg_size);
    let mut new_resp_bytes = vec![];
    new_resp_bytes.extend_from_slice(&resp_msg_size);
    new_resp_bytes.extend_from_slice(&resp_bytes);

    new_resp_bytes
}