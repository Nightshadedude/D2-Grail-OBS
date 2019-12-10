package gsconnector

//this is the intermediary between Google Sheets and the app.

import (
	lib "D2-Grail-OBS/gsconnector/gsclib"
	"D2-Grail-OBS/types"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"reflect"
	"strconv"

	"golang.org/x/net/context"
	"golang.org/x/oauth2"
	"golang.org/x/oauth2/google"
	"google.golang.org/api/sheets/v4"
)

// Retrieve a token, saves the token, then returns the generated client.
func getClient(config *oauth2.Config) *http.Client {
	// The file token.json stores the user's access and refresh tokens, and is
	// created automatically when the authorization flow completes for the first
	// time.
	tokFile := "token.json"
	tok, err := tokenFromFile(tokFile)
	if err != nil {
		tok = getTokenFromWeb(config)
		saveToken(tokFile, tok)
	}
	return config.Client(context.Background(), tok)
}

// Request a token from the web, then returns the retrieved token.
func getTokenFromWeb(config *oauth2.Config) *oauth2.Token {
	authURL := config.AuthCodeURL("state-token", oauth2.AccessTypeOffline)
	fmt.Printf("Go to the following link in your browser then type the "+
		"authorization code: \n%v\n", authURL)

	var authCode string
	if _, err := fmt.Scan(&authCode); err != nil {
		log.Fatalf("Unable to read authorization code: %v", err)
	}

	tok, err := config.Exchange(context.TODO(), authCode)
	if err != nil {
		log.Fatalf("Unable to retrieve token from web: %v", err)
	}
	return tok
}

// Retrieves a token from a local file.
func tokenFromFile(file string) (*oauth2.Token, error) {
	f, err := os.Open(file)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	tok := &oauth2.Token{}
	err = json.NewDecoder(f).Decode(tok)
	return tok, err
}

// Saves a token to a file path.
func saveToken(path string, token *oauth2.Token) {
	fmt.Printf("Saving credential file to: %s\n", path)
	f, err := os.OpenFile(path, os.O_RDWR|os.O_CREATE|os.O_TRUNC, 0600)
	if err != nil {
		log.Fatalf("Unable to cache oauth token: %v", err)
	}
	defer f.Close()
	json.NewEncoder(f).Encode(token)
}

//GSReader returns an array of data based on the sheet, tab, and range provided.  Use 0 for endRow to return all rows
func GSReader(gscdata types.GSCRead) types.GSSheet {
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
