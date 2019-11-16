package types

type GSConnectorData {
	sheetURL string //url of google sheet
	tabName string //name of tab that is being accessed
	startColumn int //start data read at this column
	startRow int //start data read at this row
	endColumn int // stop reading data at this column
	endRow int // stop reading data at this row
	b []byte //byte array of credential file "b, err := ioutil.ReadFile(credentialsFileName)"
}

type GSSheet struct {

}

type GSRow struct {

}

type GCCell struct {

}