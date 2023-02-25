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
	n.children = []*Node{}
	return &n
}

func getJsonTree(allKnownFields []string) *[]*Node {
	result := []*Node{}
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

func printResults(result *[]*Node, level int) {
	for _, node := range *result {
		fmt.Println(strings.Repeat("--", level), node.columnName)
		if len(node.children) > 0 {
			printResults(&node.children, level+1)
		}
	}
}

func main() {
	// case 1
	result := getJsonTree([]string{"a", "a.b", "b.c", "b", "d.e.f"})
	printResults(result, 0)

	// case 2
	result = getJsonTree([]string{"d.e.f.g", "f.g", "b.c.d"})
	printResults(result, 0)
}
