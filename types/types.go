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
	ValueOption string  //value option type...RAW or USER_ENTERED
	B           []byte  //byte array of credential file "b, err := ioutil.ReadFile(credentialsFileName)"
}

//GSCDelete delets a range of data
type GSCDelete struct {
	SheetURL    string //url of google sheet
	TabName     string //name of tab that is being accessed
	StartColumn int    //start data delete at this column
	StartRow    int    //start data delet at this row
	EndColumn   int    //end of data delete at this column
	EndRow      int    //end of data delete at this row
	B           []byte //byte array of credential file "b, err := ioutil.ReadFile(credentialsFileName)"
}

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
	Cell     interface{}
}

//Sheet2D takes the GSSheet struct and flattens it to a 2D string array
func (sheet GSSheet) Sheet2D() [][]interface{} {
	var Arr2D [][]interface{}
	for _, row := range sheet.Sheet {
		var Arr1D []interface{}
		for _, cell := range row.Row {
			Arr1D = append(Arr1D, cell.Cell)
		}
		Arr2D = append(Arr2D, Arr1D)
	}
	return Arr2D
}
