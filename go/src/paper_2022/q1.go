package main

import (
	"fmt"
	"strconv"
	"log"
)

func highestFibonacci(n int) int {
	currentFibonacci := 1
	previousFibonacci := 0

	for {
		temp := currentFibonacci
		currentFibonacci = currentFibonacci + previousFibonacci
		previousFibonacci = temp

		if currentFibonacci >= n {
			break
		}
	}

	return previousFibonacci
}

func getRepresentation(n int) []int {
	zackendorfRepresentation := []int{}
	subtracted := n

	for {
		highest := highestFibonacci(subtracted)
		subtracted = subtracted - highest
		zackendorfRepresentation = append(zackendorfRepresentation, highest)

		if subtracted == 0 {
			break
		}
	}

	return zackendorfRepresentation
}

func main(){
	var numToRepresent string
	fmt.Println("Enter number to represent: ")
	fmt.Scanln(&numToRepresent)

	n, err := strconv.Atoi(numToRepresent)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(getRepresentation(n))
}

