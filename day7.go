package main

import (
	"bufio"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("day7.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	graph := make(map[string][]string)
	deps := make(map[string][]string)
	for scanner.Scan() {
		// Step Q must be finished before step O can begin.
		text := scanner.Text()
		tokens := strings.Split(text, " ")
		graph[tokens[0]] = append(graph[tokens[0]], tokens[7])
		deps[tokens[7]] = append(deps[tokens[7]], tokens[0])
	}
}
