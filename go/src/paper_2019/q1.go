package main

import (
	"fmt"
	"strconv"
	"log"
)

func isPalindrome(num string) bool {
	for i := 0; i < len(num)/2;i++{
		if num[i] != num[len(num) - i - 1] {
			return false
		}
	}
	return true
}

func nextPalindrome(inputNum int) int {
	n := inputNum + 1
	for {
		if isPalindrome(fmt.Sprint(n)) {
			return n
		}
		n ++			
	}
}

func main() {
	var input string
	fmt.Println("Enter number: ")
	fmt.Scanln(&input)

	inputNum, err := strconv.Atoi(input)
	if err != nil {
		log.Fatal(err)
	}

	nextPalindrome := nextPalindrome(inputNum)
	fmt.Printf("next palindrome is %d", nextPalindrome)
}