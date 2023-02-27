package main

import "fmt"

type Node struct {
	val   int
	right *Node
}

func newNode(val int) *Node {
	return &Node{val: val}
}

type List struct {
	head *Node
	tail *Node
}

func newList(vals []int) *List {
	l := List{}
	for _, val := range vals {
		l.insertEnd(val)
	}

	return &l
}

func (l *List) printList() {
	currNode := l.head
	for currNode != nil {
		fmt.Print(currNode.val, " ")
		currNode = currNode.right
	}

	fmt.Println()
}

func (l *List) insertStart(val int) {
	n := newNode(val)

	if l.head == nil {
		// Empty list
		l.head = n
		l.tail = n
		return
	}

	n.right = l.head
	l.head = n
}

func (l *List) insertEnd(val int) {
	n := newNode(val)

	if l.tail == nil {
		// Empty list
		l.head = n
		l.tail = n
		return
	}

	l.tail.right = n
	l.tail = n
}

func (l *List) removeStart() {
	if l.head == nil {
		return
	}

	l.head = l.head.right
}

func (l *List) removeAtIndex(index int) {
	var prevNode *Node
	currNode := l.head
	currIndex := 0
	for currIndex < index {
		if currNode == nil {
			// Don't raise error, just make it no op
			return
		}

		prevNode = currNode
		currNode = currNode.right
		currIndex += 1
	}

	if currNode == nil {
		// Don't raise error, just make it no op
		return
	}

	if prevNode == nil {
		// Index 0
		l.head = l.head.right
		return
	}

	prevNode.right = currNode.right
	if currNode.right == nil {
		// Tail of list
		l.tail = prevNode
	}
}

func (l *List) reverseList() {
	var prevNode *Node
	var nextNode *Node
	currNode := l.head

	l.head = l.tail
	l.tail = currNode

	for currNode != nil {
		nextNode = currNode.right
		currNode.right = prevNode
		prevNode = currNode
		currNode = nextNode
	}
}

func main() {
	l := newList([]int{1, 2, 3, 4, 5})
	l.printList()

	l.insertEnd(10)
	l.printList()

	l.insertStart(-5)
	l.printList()

	l.removeStart()
	l.printList()

	l.removeAtIndex(4)
	l.printList()

	l.removeAtIndex(20)
	l.printList()

	l.removeAtIndex(0)
	l.printList()

	l.removeAtIndex(3)
	l.printList()

	l.insertEnd(21)
	l.insertStart(-10)
	l.printList()

	l.reverseList()
	l.printList()
}
