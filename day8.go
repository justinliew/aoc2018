package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

func run(strValues []string) {

	var values []int

	for _, s := range strValues {
		v, _ := strconv.Atoi(s)
		values = append(values, v)
	}

}

func main() {
	file, err := os.Open("day8.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		s := strings.Split(scanner.Text(), " ")
		run(s)
	}
}
