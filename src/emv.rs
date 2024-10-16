use brcode::{crc16_ccitt, BrCode, Info, Label, MerchantInfo};



pub fn get_code_noccitt(valor: f64, location: &str, name: &str) -> BrCode {
    BrCode {
        payload_version: 1,
        amount: Some(valor),
        initiation_method: Some(12),
        merchant_category_code: 0000u32,
        country_code: "BR".to_string(),
        currency: "986".to_string(),
        merchant_name: name.to_string(),
        merchant_information: vec![
            MerchantInfo {
                id: 26,
                info: vec![
                    Info {
                        id: 0,
                        info: "BR.GOV.BCB.PIX".to_string()
                    },
                    Info {
                        id: 25,
                        info: location.to_string()
                    }
                ]
            }
        ],
        merchant_city: "SJBV".to_string(),
        convenience: None,
        templates: None,
        postal_code: None,
        field_template: vec![Label {
            reference_label: "RP123456789-2024".to_string()
        }],
        convenience_fee_fixed: None,
        convenience_fee_percentage: None,
        merchant_account_information: None,
        crc1610: "".to_string()
    }
}

pub fn get_qr_code_string(valor: f64, location: &str, name: &str) -> String {
    let original = get_code_noccitt(valor, location, name).to_string();

    let ccit = crc16_ccitt(&original);

    format!("{original}{ccit}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_string() {
        println!("{}", get_qr_code_string(12.03, "pix.example.com/qr/v2/2353c790eefb11eaadc10242ac120002", "VideoTranscribe"));
    }
}
