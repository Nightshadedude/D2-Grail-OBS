package main

import (
	gsc "D2-Grail-OBS/gsconnector"
	"D2-Grail-OBS/types"
	"io/ioutil"
	"log"
	"os"
	"reflect"

	"github.com/joho/godotenv"
)

func init() {
	// loads values from .env into the system
	if err := godotenv.Load("d2GrailOBS.env"); err != nil {
		log.Print("No .env file found")
	}
}

func main() {
	credentialsFileName := os.Getenv("CREDENTIALS_FILE_NAME")
	sheetURL := os.Getenv("GOOGLE_SHEET_URL")
	tabName := os.Getenv("TAB_NAME_TEST")
	b, err := ioutil.ReadFile(credentialsFileName)
	if err != nil {
		log.Fatalf("Unable to read client secret file: %v", err)
	}

	var readStruct types.GSCRead
	readStruct.TabName = tabName
	readStruct.SheetURL = sheetURL
	readStruct.StartColumn = 1
	readStruct.StartRow = 1
	readStruct.EndColumn = 4
	readStruct.EndRow = 0
	readStruct.B = b

	var testWriteSheet types.GSSheet
	var tempRow types.GSRow
	var tempCell types.GSCell

	tempCell.Cell = 8
	tempCell.DataType = reflect.TypeOf(tempCell.Cell)

	tempRow.Row = append(tempRow.Row, tempCell)
	testWriteSheet.Sheet = append(testWriteSheet.Sheet, tempRow)

	var writeStruct types.GSCWrite
	writeStruct.SheetURL = sheetURL
	writeStruct.TabName = tabName
	writeStruct.StartColumn = 3
	writeStruct.StartRow = 8
	writeStruct.WriteSheet = testWriteSheet
	writeStruct.ValueOption = "RAW"
	writeStruct.B = b

	var deleteStruct types.GSCDelete
	deleteStruct.SheetURL = sheetURL
	deleteStruct.TabName = tabName
	deleteStruct.StartColumn = 3
	deleteStruct.StartRow = 8
	deleteStruct.EndColumn = 3
	deleteStruct.EndRow = 8
	deleteStruct.B = b

	log.Printf("read sheet: %v", gsc.GSReader(readStruct))
	log.Printf("write status: %v", gsc.GSWriter(writeStruct))
	log.Printf("delete status: %v", gsc.GSDeleter(deleteStruct))
}
