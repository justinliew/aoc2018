package main

import (
	"bufio"
	"log"
	"os"
	"strings"
	"sort"
)

func SliceUniqMap(s []string) []string {
	seen := make(map[string]struct{}, len(s))
	j := 0
	for _, v := range s {
		if _, ok := seen[v]; ok {
			continue
		}
		seen[v] = struct{}{}
		s[j] = v
		j++
	}
	return s[:j]
}

func main() {
	file, err := os.Open("day7.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	graph := make(map[string][]string)
	deps := make(map[string][]string)
	all := make(map[string]bool)
	for scanner.Scan() {
		// Step Q must be finished before step O can begin.
		text := scanner.Text()
		tokens := strings.Split(text, " ")
		graph[tokens[1]] = append(graph[tokens[1]], tokens[7])
		deps[tokens[7]] = append(deps[tokens[7]], tokens[1])
		all[tokens[1]] = true
		all[tokens[7]] = true
	}

	log.Printf("Graph")
	for k,v := range graph {
		log.Printf("%s -> %s", k,v)
	}

	var frontier []string

	for k := range all {
		_,ok := deps[k]
		if !ok {
			frontier = append(frontier,k)
		}
	}
	
	var output []string
	for ;len(frontier) > 0;{
		frontier = SliceUniqMap(frontier)
		sort.Strings(frontier)
		log.Printf("Frontier is %v", frontier)
		found := false
		for findIndex := 0; !found;findIndex = findIndex + 1 {
			candidate := frontier[findIndex]
			d,ok := deps[candidate]
			if !ok || len(d) == 0 {
				log.Printf("Adding %s", candidate)
				output = append(output,candidate)
				for _,removeDep := range graph[candidate] {
					for dIndex := range deps[removeDep] {
						if deps[removeDep][dIndex] == candidate {
							deps[removeDep] = append(deps[removeDep][:dIndex],deps[removeDep][dIndex+1:]...)
							break
						}
					}
				}
				frontier = append(frontier[:findIndex],frontier[findIndex+1:]...)
				frontier = append(frontier,graph[candidate]...)
				findIndex = 0
				found = true
			}
		}
	}
	log.Printf("%v", output)
}
