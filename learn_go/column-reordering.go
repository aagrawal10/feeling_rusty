package main

import "fmt"

func getFinalColumnOrder(currColumns []string, sourceColumns []string) []int {
	var result []int

	// Create the corresponding maps
	numColumnsByName := make(map[string]int)
	allIndicesOfColumn := make(map[string][]int)
	for index, column_name := range sourceColumns {
		numColumnsByName[column_name] += 1
		allIndicesOfColumn[column_name] = append(allIndicesOfColumn[column_name], index)
	}

	numColumnsInHeaderByName := make(map[string]int)
	for _, column_name := range currColumns {
		if _, ok := numColumnsByName[column_name]; !ok {
			continue
		}

		numColumnsAlready, _ := numColumnsInHeaderByName[column_name]
		if numColumnsAlready >= numColumnsByName[column_name] {
			continue
		}

		allIndicesofCurrColumn, _ := allIndicesOfColumn[column_name]
		result = append(result, allIndicesofCurrColumn[numColumnsAlready])
		numColumnsInHeaderByName[column_name] += 1
	}

	for index, column_name := range sourceColumns {
		found := false
		for _, item := range result {
			if item == index {
				found = true
				break
			}
		}

		if found || len(column_name) == 0 {
			continue
		}

		result = append(result, index)
	}

	return result
}

func main() {
	currColumns := []string{"A", "B", "C", "D", "E", "F", "F", "G", "G", "H"}
	sourceColumns := []string{"Z", "A", "B", "B", "C", "F", "G", "H", "H"}
	fmt.Println("Expected -> ", []int{1, 2, 4, 5, 6, 7, 0, 3, 8})
	fmt.Println("Got -> ", getFinalColumnOrder(currColumns, sourceColumns))
}
