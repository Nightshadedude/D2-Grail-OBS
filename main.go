package main

import (
	"fmt"
	"io/ioutil"
	"log"
	gc "solid-octo-lamp/googlesheetsconnector"
)

func main() {
	credentialsFileName := "cred.json"
	sheetURL := ""
	tabName := "TestSheet"
	startColumn := 1
	startRow := 1
	endColumn := 4
	endRow := 0
	b, err := ioutil.ReadFile(credentialsFileName)
	if err != nil {
		log.Fatalf("Unable to read client secret file: %v", err)
	}
	fmt.Println(gc.GoogleSheetsConnector(sheetURL, tabName, startColumn, startRow, endColumn, endRow, b))
}
