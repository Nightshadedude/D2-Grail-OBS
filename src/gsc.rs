extern crate dotenv;
extern crate google_sheets4 as sheets4;
extern crate hyper;
extern crate hyper_native_tls;
extern crate yup_oauth2;

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::path::Path;

pub enum ValueOption {
    Raw,         //RAW
    UserEntered, //USER_ENTERED
}

impl ValueOption {
    fn to_string(&self) -> String {
        match self {
            Self::Raw => String::from("RAW"),
            Self::UserEntered => String::from("USER_ENTERED"),
        }
    }
}

#[derive(Debug)]
pub struct GenericError {
    details: String,
}

impl GenericError {
    fn new(msg: &str) -> Self {
        GenericError {
            details: String::from(msg),
        }
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GenericError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct Connection {
    pub sheet_id: Option<String>,         //url of google sheet
    pub credentials_file: Option<String>, //file that has the connection json
}

impl Connection {
    pub fn new() -> Self {
        dotenv::from_filename("d2_grail_obs.env").expect("Unable to read env file");

        let client_secret_file = match dotenv::var("CREDENTIALS_FILE_NAME") {
            Ok(file) => Some(file),
            Err(err) => {
                println!("{:?}", err);
                println!("using default credentials file");
                Some(String::from("cred.json"))
            }
        };
        let google_sheet_url = dotenv::var("GOOGLE_SHEET_URL").unwrap();
        let start_substr = "/spreadsheets/d/";
        let end_substr = "/edit";
        let spreadsheet_id = get_string_between(&google_sheet_url, start_substr, end_substr);
        println!("New connection created");
        Self {
            sheet_id: spreadsheet_id,
            credentials_file: client_secret_file,
        }
    }

    pub fn get_hub(
        self,
    ) -> (
        sheets4::Sheets<
            hyper::client::Client,
            yup_oauth2::Authenticator<
                yup_oauth2::DefaultAuthenticatorDelegate,
                yup_oauth2::DiskTokenStorage,
                hyper::client::Client,
            >,
        >,
        String,
    ) {
        let sheet_id = self.sheet_id.unwrap();
        let ret_sheet_id = String::from(&sheet_id);
        let secret =
            yup_oauth2::read_application_secret(Path::new(&self.credentials_file.unwrap()))
                .unwrap();
        let client = hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_native_tls::NativeTlsClient::new().unwrap(),
        ));
        let authenticator = yup_oauth2::Authenticator::new(
            &secret,
            yup_oauth2::DefaultAuthenticatorDelegate,
            client,
            yup_oauth2::DiskTokenStorage::new(&"token_store.json".to_string()).unwrap(),
            Some(yup_oauth2::FlowType::InstalledInteractive),
        );
        let ret_client = hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_native_tls::NativeTlsClient::new().unwrap(),
        ));
        
        println!("New hub created from connection");
        (
            sheets4::Sheets::new(ret_client, authenticator),
            ret_sheet_id,
        )
    }
}

pub struct Reader {
    hub: sheets4::Sheets<
        hyper::client::Client,
        yup_oauth2::Authenticator<
            yup_oauth2::DefaultAuthenticatorDelegate,
            yup_oauth2::DiskTokenStorage,
            hyper::client::Client,
        >,
    >,
    sheet_id: String,
    tab_name: Option<String>,
    start_column: Option<i32>,
    start_row: Option<i32>,
    end_column: Option<i32>,
    end_row: Option<i32>,
}

impl Reader {
    pub fn new(
        hub: sheets4::Sheets<
            hyper::client::Client,
            yup_oauth2::Authenticator<
                yup_oauth2::DefaultAuthenticatorDelegate,
                yup_oauth2::DiskTokenStorage,
                hyper::client::Client,
            >,
        >,
        sheet_id: &str,
        tab: &str,
        start_column: i32,
        start_row: i32,
        end_column: i32,
        end_row: i32,
    ) -> Self {
        println!("New reader created");
        Self {
            hub,
            sheet_id: String::from(sheet_id),
            tab_name: match tab.len() {
                0 => None,
                _ => Some(String::from(tab)),
            },
            start_column: match start_column {
                0 => None,
                _ => Some(start_column),
            },
            start_row: match start_row {
                0 => None,
                _ => Some(start_row),
            },
            end_column: match end_column {
                0 => None,
                _ => Some(end_column),
            },
            end_row: match end_row {
                0 => None,
                _ => Some(end_row),
            },
        }
    }

    pub fn read(self) -> Result<Vec<Vec<String>>, GenericError> {
        let read_range = to_a1_notation(
            &self.tab_name.unwrap(),
            self.start_column,
            self.start_row,
            self.end_column,
            self.end_row,
        );

        println!("Reading from range: \"{:?}\"", &read_range);
        match read_range {
            Ok(range) => {
                let result = self
                    .hub
                    .spreadsheets()
                    .values_get(&self.sheet_id, &range)
                    .major_dimension("ROWS")
                    .doit()
                    .unwrap().1
                    .values
                    .unwrap();
                println!("Data read: {:?}", &result);
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }
}

pub struct Writer {
    hub: sheets4::Sheets<
        hyper::client::Client,
        yup_oauth2::Authenticator<
            yup_oauth2::DefaultAuthenticatorDelegate,
            yup_oauth2::DiskTokenStorage,
            hyper::client::Client,
        >,
    >,
    sheet_id: String,
    data: sheets4::ValueRange,
    range: Option<String>,
    val_option: ValueOption,
}

impl Writer {
    pub fn new(
        hub: sheets4::Sheets<
            hyper::client::Client,
            yup_oauth2::Authenticator<
                yup_oauth2::DefaultAuthenticatorDelegate,
                yup_oauth2::DiskTokenStorage,
                hyper::client::Client,
            >,
        >,
        sheet_id: &str,
        tab: &str,
        start_column: i32,
        start_row: i32,
        data: Vec<Vec<String>>,
        val_opt: ValueOption,
    ) -> Self {
        let (mut end_row, mut end_column) = row_col_size_from_vec_vec(&data);
        end_row += start_row - 1;
        end_column += start_column - 1;
        let s_col = match start_column {
            0 => None,
            _ => Some(start_column),
        };
        let s_row = match start_row {
            0 => None,
            _ => Some(start_row),
       };
        let e_col = match end_column {
            0 => None,
            _ => Some(end_column),
        };
        let e_row = match end_row {
            0 => None,
            _ => Some(end_row),
        };

        let range_result = to_a1_notation(&tab, s_col, s_row, e_col, e_row);
        let write_range = match range_result {
            Err(e) => {
                println!("Write range error: {}", e);
                None
            }
            Ok(r) => Some(r),
        };
        println!("New writer created");
        Self {
            hub,
            sheet_id: String::from(sheet_id),
            data: value_range_new(&write_range.clone().unwrap(), data),
            range: write_range,
            val_option: val_opt,
        }
    }

    pub fn write(self) -> Result<String, GenericError> {
        let result = self
            .hub
            .spreadsheets()
            .values_update(self.data, &self.sheet_id, &self.range.unwrap())
            .value_input_option(&self.val_option.to_string())
            .include_values_in_response(false)
            .doit();
        match result {
            //TODO: pull success message from result
            Ok(_) => {
                println!("Write successful");
                Ok(String::from("Write success"))
            },
            Err(e) => {
                println!("Write unsuccessful: {}", &e.to_string());
                Err(GenericError::new(&e.to_string()))}
            ,
        }
    }
}

pub struct Deleter {
    hub: sheets4::Sheets<
        hyper::client::Client,
        yup_oauth2::Authenticator<
            yup_oauth2::DefaultAuthenticatorDelegate,
            yup_oauth2::DiskTokenStorage,
            hyper::client::Client,
        >,
    >,
    sheet_id: String,
    delete_request: sheets4::ClearValuesRequest,
    range: Option<String>,
}

impl Deleter {
    pub fn new(
        hub: sheets4::Sheets<
            hyper::client::Client,
            yup_oauth2::Authenticator<
                yup_oauth2::DefaultAuthenticatorDelegate,
                yup_oauth2::DiskTokenStorage,
                hyper::client::Client,
            >,
        >,
        sheet_id: &str,
        tab: &str,
        start_column: i32,
        start_row: i32,
        end_column: i32,
        end_row: i32,
    ) -> Self {
        let s_col = match start_column {
            0 => None,
            _ => Some(start_column),
        };
        let s_row = match start_row {
            0 => None,
            _ => Some(start_row),
        };
        let e_col = match end_column {
            0 => None,
            _ => Some(end_column),
        };
        let e_row = match end_row {
            0 => None,
            _ => Some(end_row),
        };

        let range_result = to_a1_notation(tab, s_col, s_row, e_col, e_row);
        let delete_range = match range_result {
            Err(e) => {
                println!("Write range error: {}", e);
                None
            }
            Ok(r) => Some(r),
        };

        println!("New deleter created");

        Self {
            hub,
            sheet_id: String::from(sheet_id),
            delete_request: sheets4::ClearValuesRequest::default(),
            range: delete_range,
        }
    }

    pub fn delete(self) -> Result<String, GenericError> {
        let result = self
            .hub
            .spreadsheets()
            .values_clear(self.delete_request, &self.sheet_id, &self.range.unwrap())
            .doit();
        match result {
            //TODO: pull success message from result
            Ok(_) => {
                println!("Delete successful");
                Ok(String::from("Delete success"))}
            ,
            Err(e) => {
                println!("Delete unsuccessful: {}", &e.to_string());
                Err(GenericError::new(&e.to_string()))
            },
        }
    }
}

fn value_range_new(rng: &str, data: Vec<Vec<String>>) -> sheets4::ValueRange {
    sheets4::ValueRange {
        range: match rng.len() {
            rlen if rlen > 0 => Some(String::from(rng)),
            _ => None,
        },
        values: match data.len() {
            rlen if rlen > 0 => Some(data),
            _ => None,
        },
        major_dimension: Some(String::from("ROWS")),
    }
}

fn row_col_size_from_vec_vec(data: &Vec<Vec<String>>) -> (i32, i32) {
    let row = match i32::try_from(data.len()) {
        Err(_) => 0,
        Ok(sz) => sz,
    };

    let col = match i32::try_from(data[0].len()) {
        Err(_) => 0,
        Ok(sz) => sz,
    };

    (row, col)
}

fn get_string_between(full_str: &str, start: &str, end: &str) -> Option<String> {
    let start_i = full_str.find(start).unwrap() + start.len();
    match start_i {
        0 | 1 => None,
        _ => match full_str.find(end) {
            None => Some(String::from(&full_str[start_i..])),
            Some(_) => {
                let end_i = full_str.find(end).unwrap();
                Some(String::from(&full_str[start_i..end_i]))
            }
        },
    }
}

fn to_a1_notation(
    tab: &str,
    start_column: Option<i32>,
    start_row: Option<i32>,
    end_column: Option<i32>,
    end_row: Option<i32>,
) -> Result<String, GenericError> {
    let mut a1_notation = String::from("'");
    a1_notation.push_str(tab);
    a1_notation.push_str("'");
    match (start_column, start_row, end_column, end_row) {
        (None, None, None, None) => Ok(a1_notation),
        (Some(s_col), Some(s_row), Some(e_col), None) => {
            a1_notation.push_str("!");
            a1_notation.push_str(&int_to_char_string(s_col).unwrap());
            a1_notation.push_str(&s_row.to_string());
            a1_notation.push_str(":");
            a1_notation.push_str(&int_to_char_string(e_col).unwrap());
            Ok(a1_notation)
        }
        (Some(s_col), Some(s_row), Some(e_col), Some(e_row)) => {
            a1_notation.push_str("!");
            a1_notation.push_str(&int_to_char_string(s_col).unwrap());
            a1_notation.push_str(&s_row.to_string());
            a1_notation.push_str(":");
            a1_notation.push_str(&int_to_char_string(e_col).unwrap());
            a1_notation.push_str(&e_row.to_string());
            Ok(a1_notation)
        }
        _ => {
            let err_result = format!(
                "Unable to parse to a1 notation -
                                         \ntab:{:?}
                                         \nstart_col: {:?}
                                         \nstart_row: {:?}
                                         \nend_column: {:?}
                                         \nend_row: {:?}",
                tab, start_column, start_row, end_column, end_row
            );
            Err(GenericError::new(&err_result))
        }
    }
}

//int_to_char_string returns the alpha variant of a numeric column...allows for far more options than currently
//available in google sheets
fn int_to_char_string(i: i32) -> Result<String, GenericError> {
    let string_arr = vec![
        ".", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
        "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "AA", "AB", "AC", "AD", "AE", "AF", "AG",
        "AH", "AI", "AJ", "AK", "AL", "AM", "AN", "AO", "AP", "AQ", "AR", "AS", "AT", "AU", "AV",
        "AW", "AX", "AY", "AZ", "BA", "BB", "BC", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BK",
        "BL", "BM", "BN", "BO", "BP", "BQ", "BR", "BS", "BT", "BU", "BV", "BW", "BX", "BY", "BZ",
        "CA", "CB", "CC", "CD", "CE", "CF", "CG", "CH", "CI", "CJ", "CK", "CL", "CM", "CN", "CO",
        "CP", "CQ", "CR", "CS", "CT", "CU", "CV", "CW", "CX", "CY", "CZ", "DA", "DB", "DC", "DD",
        "DE", "DF", "DG", "DH", "DI", "DJ", "DK", "DL", "DM", "DN", "DO", "DP", "DQ", "DR", "DS",
        "DT", "DU", "DV", "DW", "DX", "DY", "DZ", "EA", "EB", "EC", "ED", "EE", "EF", "EG", "EH",
        "EI", "EJ", "EK", "EL", "EM", "EN", "EO", "EP", "EQ", "ER", "ES", "ET", "EU", "EV", "EW",
        "EX", "EY", "EZ", "FA", "FB", "FC", "FD", "FE", "FF", "FG", "FH", "FI", "FJ", "FK", "FL",
        "FM", "FN", "FO", "FP", "FQ", "FR", "FS", "FT", "FU", "FV", "FW", "FX", "FY", "FZ", "GA",
        "GB", "GC", "GD", "GE", "GF", "GG", "GH", "GI", "GJ", "GK", "GL", "GM", "GN", "GO", "GP",
        "GQ", "GR", "GS", "GT", "GU", "GV", "GW", "GX", "GY", "GZ", "HA", "HB", "HC", "HD", "HE",
        "HF", "HG", "HH", "HI", "HJ", "HK", "HL", "HM", "HN", "HO", "HP", "HQ", "HR", "HS", "HT",
        "HU", "HV", "HW", "HX", "HY", "HZ", "IA", "IB", "IC", "ID", "IE", "IF", "IG", "IH", "II",
        "IJ", "IK", "IL", "IM", "IN", "IO", "IP", "IQ", "IR", "IS", "IT", "IU", "IV", "IW", "IX",
        "IY", "IZ", "JA", "JB", "JC", "JD", "JE", "JF", "JG", "JH", "JI", "JJ", "JK", "JL", "JM",
        "JN", "JO", "JP", "JQ", "JR", "JS", "JT", "JU", "JV", "JW", "JX", "JY", "JZ", "KA", "KB",
        "KC", "KD", "KE", "KF", "KG", "KH", "KI", "KJ", "KK", "KL", "KM", "KN", "KO", "KP", "KQ",
        "KR", "KS", "KT", "KU", "KV", "KW", "KX", "KY", "KZ", "LA", "LB", "LC", "LD", "LE", "LF",
        "LG", "LH", "LI", "LJ", "LK", "LL", "LM", "LN", "LO", "LP", "LQ", "LR", "LS", "LT", "LU",
        "LV", "LW", "LX", "LY", "LZ", "MA", "MB", "MC", "MD", "ME", "MF", "MG", "MH", "MI", "MJ",
        "MK", "ML", "MM", "MN", "MO", "MP", "MQ", "MR", "MS", "MT", "MU", "MV", "MW", "MX", "MY",
        "MZ", "NA", "NB", "NC", "ND", "NE", "NF", "NG", "NH", "NI", "NJ", "NK", "NL", "NM", "NN",
        "NO", "NP", "NQ", "NR", "NS", "NT", "NU", "NV", "NW", "NX", "NY", "NZ", "OA", "OB", "OC",
        "OD", "OE", "OF", "OG", "OH", "OI", "OJ", "OK", "OL", "OM", "ON", "OO", "OP", "OQ", "OR",
        "OS", "OT", "OU", "OV", "OW", "OX", "OY", "OZ", "PA", "PB", "PC", "PD", "PE", "PF", "PG",
        "PH", "PI", "PJ", "PK", "PL", "PM", "PN", "PO", "PP", "PQ", "PR", "PS", "PT", "PU", "PV",
        "PW", "PX", "PY", "PZ", "QA", "QB", "QC", "QD", "QE", "QF", "QG", "QH", "QI", "QJ", "QK",
        "QL", "QM", "QN", "QO", "QP", "QQ", "QR", "QS", "QT", "QU", "QV", "QW", "QX", "QY", "QZ",
        "RA", "RB", "RC", "RD", "RE", "RF", "RG", "RH", "RI", "RJ", "RK", "RL", "RM", "RN", "RO",
        "RP", "RQ", "RR", "RS", "RT", "RU", "RV", "RW", "RX", "RY", "RZ", "SA", "SB", "SC", "SD",
        "SE", "SF", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SP", "SQ", "SR", "SS",
        "ST", "SU", "SV", "SW", "SX", "SY", "SZ", "TA", "TB", "TC", "TD", "TE", "TF", "TG", "TH",
        "TI", "TJ", "TK", "TL", "TM", "TN", "TO", "TP", "TQ", "TR", "TS", "TT", "TU", "TV", "TW",
        "TX", "TY", "TZ", "UA", "UB", "UC", "UD", "UE", "UF", "UG", "UH", "UI", "UJ", "UK", "UL",
        "UM", "UN", "UO", "UP", "UQ", "UR", "US", "UT", "UU", "UV", "UW", "UX", "UY", "UZ", "VA",
        "VB", "VC", "VD", "VE", "VF", "VG", "VH", "VI", "VJ", "VK", "VL", "VM", "VN", "VO", "VP",
        "VQ", "VR", "VS", "VT", "VU", "VV", "VW", "VX", "VY", "VZ", "WA", "WB", "WC", "WD", "WE",
        "WF", "WG", "WH", "WI", "WJ", "WK", "WL", "WM", "WN", "WO", "WP", "WQ", "WR", "WS", "WT",
        "WU", "WV", "WW", "WX", "WY", "WZ", "XA", "XB", "XC", "XD", "XE", "XF", "XG", "XH", "XI",
        "XJ", "XK", "XL", "XM", "XN", "XO", "XP", "XQ", "XR", "XS", "XT", "XU", "XV", "XW", "XX",
        "XY", "XZ", "YA", "YB", "YC", "YD", "YE", "YF", "YG", "YH", "YI", "YJ", "YK", "YL", "YM",
        "YN", "YO", "YP", "YQ", "YR", "YS", "YT", "YU", "YV", "YW", "YX", "YY", "YZ", "ZA", "ZB",
        "ZC", "ZD", "ZE", "ZF", "ZG", "ZH", "ZI", "ZJ", "ZK", "ZL", "ZM", "ZN", "ZO", "ZP", "ZQ",
        "ZR", "ZS", "ZT", "ZU", "ZV", "ZW", "ZX", "ZY", "ZZ",
    ];

    let vec_len: i32 = string_arr.len() as i32 - 1;
    if 1 <= i && i <= vec_len {
        Ok(string_arr[i as usize].to_string())
    } else {
        Err(GenericError::new("Error parsing int into alpha column"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_option_check() {
        assert_eq!(ValueOption::Raw.to_string(), "RAW");
        assert_eq!(ValueOption::UserEntered.to_string(), "USER_ENTERED");
    }

    #[test]
    fn int_to_col() {
        assert_eq!(int_to_char_string(0).is_ok(), false);

        assert_eq!(int_to_char_string(1).unwrap(), String::from("A"));
        assert_eq!(int_to_char_string(1).is_ok(), true);

        assert_eq!(int_to_char_string(26).unwrap(), String::from("Z"));
        assert_eq!(int_to_char_string(26).is_ok(), true);

        assert_eq!(int_to_char_string(27).unwrap(), String::from("AA"));
        assert_eq!(int_to_char_string(27).is_ok(), true);

        assert_eq!(int_to_char_string(702).unwrap(), String::from("ZZ"));
        assert_eq!(int_to_char_string(702).is_ok(), true);

        assert_eq!(int_to_char_string(703).is_ok(), false);
    }
}
