package main

import "fmt"

func getClusterLengths(carSpeeds []int) []int {
	var clusterLengths []int

	minSpeed := carSpeeds[0]
	currentClusterLength := 1

	for index, currSpeed := range carSpeeds {
		if index == 0 {
			continue
		}

		if currSpeed > minSpeed {
			currentClusterLength += 1
			continue
		}

		clusterLengths = append(clusterLengths, currentClusterLength)
		currentClusterLength = 1
		minSpeed = currSpeed
	}

	clusterLengths = append(clusterLengths, currentClusterLength)
	return clusterLengths
}

func getClustersWithFastestCar(carSpeeds []int) [][]int {
	initialClusters := getClusterLengths(carSpeeds)
	firstVal := append([]int{1}, initialClusters...)
	returnVal := [][]int{firstVal}

	for index, currClusterLength := range initialClusters {
		var clusterCopy []int
		clusterCopy = append(clusterCopy, initialClusters...)
		clusterCopy[index] += 1

		for j := 0; j < currClusterLength; j++ {
			returnVal = append(returnVal, clusterCopy)
		}
	}

	return returnVal
}

func main() {
	fmt.Println(getClusterLengths([]int{2, 4, 1, 3}))
	fmt.Println(getClusterLengths([]int{2, 5, 4, 3, 1}))

	fmt.Println(getClustersWithFastestCar([]int{2, 4, 1, 3}))
	fmt.Println(getClustersWithFastestCar([]int{2, 5, 4, 3, 1}))
}
