package main

import (
	"fmt"
	"strings"
)

type Node struct {
	columnName string
	children   []*Node
}

func newNode(columnName string) *Node {
	n := Node{columnName: columnName}
	return &n
}

func (node *Node) printNode(level int) {
	fmt.Println(strings.Repeat("--", level), node.columnName)
	for _, childNode := range node.children {
		childNode.printNode(level + 1)
	}
}

func getJsonTree(allKnownFields []string) *[]*Node {
	var result []*Node
	existingNodesMap := make(map[string]*Node)

	// Input is going to be like ["a", "a.b", "b.c", "b", "d.e.f"]
	// Output should be
	// [*Node(a, [*Node(a.b)]), *Node(b, [*Node(b.c)]), *Node(d, [*Node(d.e, [(*Node(d.e.f))])])]

	for _, field := range allKnownFields {
		currFields := strings.Split(field, ".")
		parentList := &result
		for index := range currFields {
			currFullName := strings.Join(currFields[:index+1], ".")
			if currNode, exists := existingNodesMap[currFullName]; exists {
				parentList = &currNode.children
				continue
			}

			// Need to create a new node and add it.
			newNode := newNode(currFullName)
			existingNodesMap[currFullName] = newNode
			*parentList = append(*parentList, newNode)
			parentList = &newNode.children
		}
	}

	return &result
}

func printResults(result *[]*Node) {
	for _, node := range *result {
		node.printNode(0)
	}
}

func main() {
	// case 1
	result := getJsonTree([]string{"a", "a.b", "b.c", "b", "d.e.f"})
	printResults(result)

	fmt.Println("-----------------------------")

	// case 2
	result = getJsonTree([]string{"d.e.f.g", "f.g", "b.c.d"})
	printResults(result)
}
