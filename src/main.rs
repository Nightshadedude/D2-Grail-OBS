//check here: https://github.com/dermesser/yup-oauth2/blob/7e26db39ca4a17f0057c2b106bcd6fd654eb986c/examples/drive_example/src/main.rs
mod google_sheets_connector;

extern crate hyper;
extern crate hyper_native_tls;
extern crate yup_oauth2;
extern crate google_sheets4 as sheets4;
extern crate dotenv;

use std::path::Path;
use google_sheets_connector as gsc;
use google_sheets4::ValueRange;
use google_sheets4::{Result, Error};
use google_sheets4::Sheets;
use yup_oauth2::{Authenticator, FlowType, ApplicationSecret, DiskTokenStorage,
    DefaultAuthenticatorDelegate, read_application_secret};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;


fn main(){
	let read_data = gsc::get_gsc_read(
		gsc::get_gsc_connection("TestSheet"),
		1,
        1,
        4,
        0
	);

    let mut test_data = gsc::gsc_reader(read_data);

	println!("{:#?}", &test_data);

	let write_data = gsc::get_gsc_write(
		gsc::get_gsc_connection("TestSheet"),
		1,
		9,
		test_data,
		"RAW".to_string());
	
	let mut test_write = gsc::gsc_writer(write_data);
	println!("Response: {:#?}", test_write);
}

/*
// reads the provided example client secret, the quick and dirty way.
fn read_client_secret(file: String) -> ApplicationSecret {
    read_application_secret(Path::new(&file)).unwrap()
}

//google_sheets4::Sheets<hyper::client::Client,yup_oauth2::authenticator::Authenticator<yup_oauth2::authenticator_delegate::DefaultAuthenticatorDelegate, yup_oauth2::storage::DiskTokenStorage, hyper::client::Client>>

fn get_sheet_hub() -> Vec<Vec<String>> {
//google_sheets4::Sheets<hyper::client::Client,yup_oauth2::authenticator::Authenticator<yup_oauth2::authenticator_delegate::DefaultAuthenticatorDelegate, yup_oauth2::storage::DiskTokenStorage, hyper::client::Client>>> {
	dotenv::from_filename("d2_grail_obs.env").ok();

    let client_secret_file = dotenv::var("CREDENTIALS_FILE_NAME").unwrap();
    let google_sheet_url = dotenv::var("GOOGLE_SHEET_URL").unwrap();
    let tab_name_test = dotenv::var("TAB_NAME_TEST").unwrap();

    let start_substr = "/spreadsheets/d/";
	let end_substr = "/edit";
    let spreadsheet_id = get_string_between(&google_sheet_url, start_substr, end_substr);

	let secret = read_client_secret(client_secret_file.to_string());

	let client = hyper::Client::with_connector(
        HttpsConnector::new(NativeTlsClient::new().unwrap()));
	
	let authenticator = Authenticator::new(&secret,
		DefaultAuthenticatorDelegate,
		client,
		DiskTokenStorage::new(&"token_store.json".to_string())
			.unwrap(),
		Some(FlowType::InstalledInteractive));

	let client = hyper::Client::with_connector(
		HttpsConnector::new(NativeTlsClient::new().unwrap()));
	
	//return the authenticated "hub"
    let hub = Sheets::new(client, authenticator);
        //println!("{}", lib::type_of(&hub));
    
    let mut range = "TestSheet".to_string();
    
    range.push_str("!A1:B");

    //result retuns (Response, ValueRange) tuple
    let result = hub.spreadsheets().values_get(&spreadsheet_id, &range)
                .major_dimension("ROWS")
                .doit();

    //handle errors
    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
             Error::HttpError(_)
            |Error::MissingAPIKey
            |Error::MissingToken(_)
            |Error::Cancelled
            |Error::UploadSizeLimitExceeded(_, _)
            |Error::Failure(_)
            |Error::BadRequest(_)
            |Error::FieldClash(_)
            |Error::JsonDecodeError(_, _) => {
                println!("{} -- returning blank", e);
                let vec_str =vec![String::from("")];
                let vec_vec = vec![vec_str];
                vec_vec
            }
        },
        //Ok(res) effectively "unwraps" the ok option of res
        //res.1 to access the ValueRange part of the tuple
        //res.1.values to access the values option
        //res.1.values.unwrap() to unwrap the actual values
        //this is a Vec<Vec<String>> type,
        //a vector of a vector of strings
        Ok(res) => res.1.values.unwrap(),
    }
}
/*
//gsc_reader returns an array of data based on the sheet, tab, and range provided.  Use 0 for endRow to return all rows
pub fn gsc_reader(gscdata types.GSCRead) types.GSSheet {
	//fmt.Println(binary.Size(b))
	startSubstr := "/spreadsheets/d/"
	endSubstr := "/edit"
	spreadsheetID := lib.GetStringBetween(gscdata.SheetURL, startSubstr, endSubstr)
	readRange := gscdata.TabName + "!"
	readRange = readRange + lib.IntToCharStrArr(gscdata.StartColumn) + strconv.Itoa(gscdata.StartRow)
	readRange = readRange + ":" + lib.IntToCharStrArr(gscdata.EndColumn)
	if gscdata.EndRow != 0 {
		readRange = readRange + string(gscdata.EndRow)
	}

	fmt.Println(readRange)

	// If modifying these scopes, delete your previously saved token.json.
	config, err := google.ConfigFromJSON(gscdata.B, "https://www.googleapis.com/auth/spreadsheets")
	if err != nil {
		log.Fatalf("Unable to parse client secret file to config: %v", err)
	}
	client := getClient(config)

	srv, err := sheets.New(client)
	if err != nil {
		log.Fatalf("Unable to retrieve Sheets client: %v", err)
	}

	resp, err := srv.Spreadsheets.Values.Get(spreadsheetID, readRange).Do()
	if err != nil {
		log.Fatalf("Unable to retrieve data from sheet: %v", err)
	}

	var readSheet types.GSSheet
	if len(resp.Values) == 0 {
		log.Println("No data found.")
		return readSheet
	} else {
		for _, row := range resp.Values {
			var readRow types.GSRow
			for _, cellVal := range row {
				var readCell types.GSCell
				if str, ok := cellVal.(string); ok {
					readCell.DataType = reflect.TypeOf(cellVal)
					readCell.Cell = str
				} else {
					/* not string .. do nothing */
				}
				readRow.Row = append(readRow.Row, readCell)
			}
			readSheet.Sheet = append(readSheet.Sheet, readRow)
		}

		return readSheet
		//fmt.Println("Name, Major:")
		//for _, row := range resp.Values {
		// Print columns A to D, which correspond to indices 0 to 3.
		//	fmt.Printf("%s\n", row[0:4])
	}
}

//GSWriter takes a sheet struct and writes it into the sheet. Returns true on success and false on failure.
func GSWriter(gscdata types.GSCWrite) bool {
	//fmt.Println(binary.Size(b))
	startSubstr := "/spreadsheets/d/"
	endSubstr := "/edit"
	spreadsheetID := lib.GetStringBetween(gscdata.SheetURL, startSubstr, endSubstr)
	dataRowCount, dataColumnCount := lib.CalcRangeBasedOnSheet(gscdata.WriteSheet)

	writeRange := gscdata.TabName + "!"
	writeRange = writeRange + lib.IntToCharStrArr(gscdata.StartColumn) + strconv.Itoa(gscdata.StartRow)
	writeRange = writeRange + ":" + lib.IntToCharStrArr(gscdata.StartColumn+dataColumnCount) + strconv.Itoa(gscdata.StartRow+dataRowCount)
	fmt.Println(writeRange)

	// If modifying these scopes, delete your previously saved token.json.
	config, err := google.ConfigFromJSON(gscdata.B, "https://www.googleapis.com/auth/spreadsheets")
	if err != nil {
		log.Fatalf("Unable to parse client secret file to config: %v", err)
	}
	client := getClient(config)

	srv, err := sheets.New(client)
	if err != nil {
		log.Fatalf("Unable to retrieve Sheets client: %v", err)
	}

	//	var interfacedStringArr types.SheetInterface
	//	interfacedStringArr = gscdata.WriteSheet

	requestBody := &sheets.ValueRange{}
	requestBody.Range = writeRange
	requestBody.MajorDimension = "ROWS"
	requestBody.Values = gscdata.WriteSheet.Sheet2D()

	resp, err := srv.Spreadsheets.Values.Update(spreadsheetID, writeRange, requestBody).ValueInputOption(gscdata.ValueOption).Do()
	//.Context(ctx).Do()
	if err != nil {
		log.Fatalf("Unable to write data to sheet: %v", err)
		return false
	}
	log.Printf("Write Response: %v", resp)
	return true
}

//GSDeleter takes a sheet struct and writes it into the sheet. Returns true on success and false on failure.
func GSDeleter(gscdata types.GSCDelete) bool {
	//fmt.Println(binary.Size(b))
	startSubstr := "/spreadsheets/d/"
	endSubstr := "/edit"
	spreadsheetID := lib.GetStringBetween(gscdata.SheetURL, startSubstr, endSubstr)

	deleteRange := gscdata.TabName + "!"
	deleteRange = deleteRange + lib.IntToCharStrArr(gscdata.StartColumn) + strconv.Itoa(gscdata.StartRow)
	deleteRange = deleteRange + ":" + lib.IntToCharStrArr(gscdata.EndColumn) + strconv.Itoa(gscdata.EndRow)
	fmt.Println(deleteRange)

	// If modifying these scopes, delete your previously saved token.json.
	config, err := google.ConfigFromJSON(gscdata.B, "https://www.googleapis.com/auth/spreadsheets")
	if err != nil {
		log.Fatalf("Unable to parse client secret file to config: %v", err)
	}
	client := getClient(config)

	srv, err := sheets.New(client)
	if err != nil {
		log.Fatalf("Unable to retrieve Sheets client: %v", err)
	}

	requestBody := &sheets.ClearValuesRequest{} // requestBody is empty for delete/clear call

	resp, err := srv.Spreadsheets.Values.Clear(spreadsheetID, deleteRange, requestBody).Do()
	//.Context(ctx).Do()
	if err != nil {
		log.Fatalf("Unable to write data to sheet: %v", err)
		return false
	}
	log.Printf("Delete Response: %v", resp)
	return true
}
*/


//get_string_between returns empty string if no start string found
fn get_string_between(full_str: &str , start: &str, end: &str) -> String {    
    let start_i = full_str.find(start).unwrap()+start.len();
	if start_i < 1 {
		String::from("")
	} else {
        let end_i = full_str.find(end).unwrap();
        String::from(&full_str[start_i..end_i])
    }
}

//IntToCharStrArr Returns A
fn int_to_char_string(i: u32) -> String {
	 let string_arr = vec![".", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
		"N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "AA", "AB", "AC", "AD",
		"AE", "AF", "AG", "AH", "AI", "AJ", "AK", "AL", "AM", "AN", "AO", "AP", "AQ", "AR", "AS",
		"AT", "AU", "AV", "AW", "AX", "AY", "AZ", "BA", "BB", "BC", "BD", "BE", "BF", "BG", "BH",
		"BI", "BJ", "BK", "BL", "BM", "BN", "BO", "BP", "BQ", "BR", "BS", "BT", "BU", "BV", "BW",
		"BX", "BY", "BZ", "CA", "CB", "CC", "CD", "CE", "CF", "CG", "CH", "CI", "CJ", "CK", "CL",
		"CM", "CN", "CO", "CP", "CQ", "CR", "CS", "CT", "CU", "CV", "CW", "CX", "CY", "CZ", "DA",
		"DB", "DC", "DD", "DE", "DF", "DG", "DH", "DI", "DJ", "DK", "DL", "DM", "DN", "DO", "DP", 
		"DQ", "DR", "DS", "DT", "DU", "DV", "DW", "DX", "DY", "DZ", "EA", "EB", "EC", "ED", "EE",
		"EF", "EG", "EH", "EI", "EJ", "EK", "EL", "EM", "EN", "EO", "EP", "EQ", "ER", "ES", "ET",
		"EU", "EV", "EW", "EX", "EY", "EZ", "FA", "FB", "FC", "FD", "FE", "FF", "FG", "FH", "FI",
		"FJ", "FK", "FL", "FM", "FN", "FO", "FP", "FQ", "FR", "FS", "FT", "FU", "FV", "FW", "FX",
		"FY", "FZ", "GA", "GB", "GC", "GD", "GE", "GF", "GG", "GH", "GI", "GJ", "GK", "GL", "GM",
		"GN", "GO", "GP", "GQ", "GR", "GS", "GT", "GU", "GV", "GW", "GX", "GY", "GZ", "HA", "HB",
		"HC", "HD", "HE", "HF", "HG", "HH", "HI", "HJ", "HK", "HL", "HM", "HN", "HO", "HP", "HQ",
		"HR", "HS", "HT", "HU", "HV", "HW", "HX", "HY", "HZ", "IA", "IB", "IC", "ID", "IE", "IF",
		"IG", "IH", "II", "IJ", "IK", "IL", "IM", "IN", "IO", "IP", "IQ", "IR", "IS", "IT", "IU",
		"IV", "IW", "IX", "IY", "IZ", "JA", "JB", "JC", "JD", "JE", "JF", "JG", "JH", "JI", "JJ",
		"JK", "JL", "JM", "JN", "JO", "JP", "JQ", "JR", "JS", "JT", "JU", "JV", "JW", "JX", "JY",
		"JZ", "KA", "KB", "KC", "KD", "KE", "KF", "KG", "KH", "KI", "KJ", "KK", "KL", "KM", "KN",
		"KO", "KP", "KQ", "KR", "KS", "KT", "KU", "KV", "KW", "KX", "KY", "KZ", "LA", "LB", "LC",
		"LD", "LE", "LF", "LG", "LH", "LI", "LJ", "LK", "LL", "LM", "LN", "LO", "LP", "LQ", "LR",
		"LS", "LT", "LU", "LV", "LW", "LX", "LY", "LZ", "MA", "MB", "MC", "MD", "ME", "MF", "MG",
		"MH", "MI", "MJ", "MK", "ML", "MM", "MN", "MO", "MP", "MQ", "MR", "MS", "MT", "MU", "MV",
		"MW", "MX", "MY", "MZ", "NA", "NB", "NC", "ND", "NE", "NF", "NG", "NH", "NI", "NJ", "NK",
		"NL", "NM", "NN", "NO", "NP", "NQ", "NR", "NS", "NT", "NU", "NV", "NW", "NX", "NY", "NZ",
		"OA", "OB", "OC", "OD", "OE", "OF", "OG", "OH", "OI", "OJ", "OK", "OL", "OM", "ON", "OO",
		"OP", "OQ", "OR", "OS", "OT", "OU", "OV", "OW", "OX", "OY", "OZ", "PA", "PB", "PC", "PD",
		"PE", "PF", "PG", "PH", "PI", "PJ", "PK", "PL", "PM", "PN", "PO", "PP", "PQ", "PR", "PS",
		"PT", "PU", "PV", "PW", "PX", "PY", "PZ", "QA", "QB", "QC", "QD", "QE", "QF", "QG", "QH",
		"QI", "QJ", "QK", "QL", "QM", "QN", "QO", "QP", "QQ", "QR", "QS", "QT", "QU", "QV", "QW",
		"QX", "QY", "QZ", "RA", "RB", "RC", "RD", "RE", "RF", "RG", "RH", "RI", "RJ", "RK", "RL",
		"RM", "RN", "RO", "RP", "RQ", "RR", "RS", "RT", "RU", "RV", "RW", "RX", "RY", "RZ", "SA",
		"SB", "SC", "SD", "SE", "SF", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SP",
		"SQ", "SR", "SS", "ST", "SU", "SV", "SW", "SX", "SY", "SZ", "TA", "TB", "TC", "TD", "TE",
		"TF", "TG", "TH", "TI", "TJ", "TK", "TL", "TM", "TN", "TO", "TP", "TQ", "TR", "TS", "TT",
		"TU", "TV", "TW", "TX", "TY", "TZ", "UA", "UB", "UC", "UD", "UE", "UF", "UG", "UH", "UI",
		"UJ", "UK", "UL", "UM", "UN", "UO", "UP", "UQ", "UR", "US", "UT", "UU", "UV", "UW", "UX",
		"UY", "UZ", "VA", "VB", "VC", "VD", "VE", "VF", "VG", "VH", "VI", "VJ", "VK", "VL", "VM",
		"VN", "VO", "VP", "VQ", "VR", "VS", "VT", "VU", "VV", "VW", "VX", "VY", "VZ", "WA", "WB",
		"WC", "WD", "WE", "WF", "WG", "WH", "WI", "WJ", "WK", "WL", "WM", "WN", "WO", "WP", "WQ",
		"WR", "WS", "WT", "WU", "WV", "WW", "WX", "WY", "WZ", "XA", "XB", "XC", "XD", "XE", "XF",
		"XG", "XH", "XI", "XJ", "XK", "XL", "XM", "XN", "XO", "XP", "XQ", "XR", "XS", "XT", "XU",
		"XV", "XW", "XX", "XY", "XZ", "YA", "YB", "YC", "YD", "YE", "YF", "YG", "YH", "YI", "YJ",
		"YK", "YL", "YM", "YN", "YO", "YP", "YQ", "YR", "YS", "YT", "YU", "YV", "YW", "YX", "YY",
		"YZ", "ZA", "ZB", "ZC", "ZD", "ZE", "ZF", "ZG", "ZH", "ZI", "ZJ", "ZK", "ZL", "ZM", "ZN",
		"ZO", "ZP", "ZQ", "ZR", "ZS", "ZT", "ZU", "ZV", "ZW", "ZX", "ZY", "ZZ"];

	let vec_len:u32 = string_arr.len() as u32 - 1;
	if 1 <= i && i <= vec_len {
		string_arr[i as usize].to_string()
	} else {
		"ERROR".to_string()
	}
}


// calc_range_from_vec_vec takes a vector of vector of strings and returns the number of rows and columns (as a tuple)
pub fn calc_range_from_vec_vec(range: &[Vec<String>]) -> (u32, u32) {
	(range.len() as u32, range[0].len() as u32)
}






*/
