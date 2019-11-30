package types

import "reflect"

//GSCRead holds the necessary data to read from Google Sheets
type GSCRead struct {
	SheetURL    string //url of google sheet
	TabName     string //name of tab that is being accessed
	StartColumn int    //start data read at this column
	StartRow    int    //start data read at this row
	EndColumn   int    // stop reading data at this column
	EndRow      int    // stop reading data at this row
	B           []byte //byte array of credential file "b, err := ioutil.ReadFile(credentialsFileName)"
}

//GSCWrite holds the necessary data to write a block of data to Google Sheets
type GSCWrite struct {
	SheetURL    string  //url of google sheet
	TabName     string  //name of tab that is being accessed
	StartColumn int     //start data read at this column
	StartRow    int     //start data read at this row
	WriteSheet  GSSheet //data that will be written
	ValueOption string  //value option type...RAW or USER_ENTEREDD
	B           []byte  //byte array of credential file "b, err := ioutil.ReadFile(credentialsFileName)"
}

//TODO: GSCUpdate

//TODO: GSCDelete

//GSSheet holds sheet data for reading and writing (array of rows)
type GSSheet struct {
	Sheet []GSRow
}

//GSRow holds row data for reading and writing (array of cells)
type GSRow struct {
	Row []GSCell
}

//GSCell holds cell data for reading and writing as well as potential type data of
//the contents (as well as the stringed data)
type GSCell struct {
	DataType reflect.Type //data type before string conversion
	Cell     string
}

//Sheet2D takes the GSSheet struct and flattens it to a 2D string array
func (sheet GSSheet) Sheet2D() [][]string {
	Arr2D := [][]string{}
	for _, row := range sheet.Sheet {
		Arr1D := []string{}
		for _, cell := range row.Row {
			Arr1D = append(Arr1D, cell.Cell)
		}
		Arr2D = append(Arr2D, Arr1D)
	}
	return Arr2D
}

//SheetInterface takes the GSSheet struct and converts it to an interface
type SheetInterface interface {
	Sheet2D() [][]string
}

//ToInterface is the method to convert to an interface
func ToInterface(interfaceVar SheetInterface, sheet GSSheet) {
	return interfaceVar.Sheet2D(sheet)
}
