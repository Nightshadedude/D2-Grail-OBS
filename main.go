package main

import (
	"D2-Grail-OBS/types"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	gc "solid-octo-lamp/googlesheetsconnector"

	"github.com/joho/godotenv"
)

func init() {
	// loads values from .env into the system
	if err := godotenv.Load(); err != nil {
		log.Print("No .env file found")
	}
}

func main() {
	credentialsFileName, _ := os.LookupEnv("CREDENTIALS_FILE_NAME")
	sheetURL, _ := os.LookupEnv("GOOGLE_SHEET_URL")
	tabName, _ := os.LookupEnv("TAB_NAME")
	b, err := ioutil.ReadFile(credentialsFileName)
	if err != nil {
		log.Fatalf("Unable to read client secret file: %v", err)
	}

	var readStruct types.GSCRead
	readStruct.TabName = tabName
	readStruct.StartColumn = 1
	readStruct.StartRow = 1
	readStruct.EndColumn = 4
	readStruct.EndRow = 0
	readStruct.B = b

	fmt.Println(gc.GSReader(readStruct))
}
