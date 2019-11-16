package gsconnector

//this is the intermediary between Google Sheets and the app.

import (
	"encoding/json"
	"fmt"
	"lib"
	"log"
	"net/http"
	"os"
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

//GoogleSheetsConnector returns an array of data based on the sheet, tab, and range provided.  Use 0 for endRow to return all rows
func GoogleSheetsConnector(sheetURL string, tabName string, startColumn int, startRow int, endColumn int, endRow int, b []byte) [][]string {
	//fmt.Println(binary.Size(b))
	startSubstr := "/spreadsheets/d/"
	endSubstr := "/edit"
	spreadsheetID := lib.GetStringBetween(sheetURL, startSubstr, endSubstr)
	readRange := tabName + "!"
	readRange = readRange + lib.IntToCharStrArr(startColumn) + strconv.Itoa(startRow)
	readRange = readRange + ":" + lib.IntToCharStrArr(endColumn)
	if endRow != 0 {
		readRange = readRange + string(endRow)
	}
	fmt.Println(readRange)

	// If modifying these scopes, delete your previously saved token.json.
	config, err := google.ConfigFromJSON(b, "https://www.googleapis.com/auth/spreadsheets.readonly")
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

	var stringData2D [][]string
	if len(resp.Values) == 0 {
		log.Println("No data found.")
		return stringData2D
	} else {

		for _, row := range resp.Values {
			var stringData1D []string
			for _, cellVal := range row {
				tempCellVal := "<null>"
				if str, ok := cellVal.(string); ok {
					tempCellVal = str
				} else {
					/* not string .. do nothing */
				}
				stringData1D = append(stringData1D, tempCellVal)
			}
			stringData2D = append(stringData2D, stringData1D)
		}

		return stringData2D
		//fmt.Println("Name, Major:")
		//for _, row := range resp.Values {
		// Print columns A to D, which correspond to indices 0 to 3.
		//	fmt.Printf("%s\n", row[0:4])
	}
}
