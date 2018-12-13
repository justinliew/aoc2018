package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strings"
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

func partOne(frontier []string, graph map[string][]string, deps map[string][]string) {
	var output []string
	for len(frontier) > 0 {
		frontier = SliceUniqMap(frontier)
		sort.Strings(frontier)
		log.Printf("Frontier is %v", frontier)
		found := false
		for findIndex := 0; !found; findIndex = findIndex + 1 {
			candidate := frontier[findIndex]
			d, ok := deps[candidate]
			if !ok || len(d) == 0 {
				log.Printf("Adding %s", candidate)
				output = append(output, candidate)
				for _, removeDep := range graph[candidate] {
					for dIndex := range deps[removeDep] {
						if deps[removeDep][dIndex] == candidate {
							deps[removeDep] = append(deps[removeDep][:dIndex], deps[removeDep][dIndex+1:]...)
							break
						}
					}
				}
				frontier = append(frontier[:findIndex], frontier[findIndex+1:]...)
				frontier = append(frontier, graph[candidate]...)
				findIndex = 0
				found = true
			}
		}
	}
	log.Printf("%v", output)
}

func findNextCandidates(frontier []string, graph map[string][]string, deps map[string][]string) ([]string, []string) {
	var candidates []string
	var candidateidx []int
	for i := range frontier {
		f := frontier[i]
		d, ok := deps[f]
		if !ok || len(d) == 0 {
			candidateidx = append(candidateidx, i)
		}
	}

	for _, v := range candidateidx {
		f := frontier[v]
		frontier = append(frontier[:v], frontier[v+1:]...)
		// todo this has to go elsewhere
		//		frontier = append(frontier, graph[f]...)
		candidates = append(candidates, frontier[v])
		log.Printf("After add: %v", frontier)
	}

	return candidates, frontier
}

type runningData struct {
	node string
	time int
}

func runUntilNextCandidateFinishes(running []runningData) (string, []runningData) {
	for {
		foundIndex := -1
		for i := range running {
			log.Printf("Checking %s, %d", running[i].node, running[i].time)
			running[i].time -= 1
			if running[i].time <= 0 && foundIndex == -1 {
				foundIndex = i
			}
		}
		if foundIndex != -1 {
			ret := running[foundIndex].node
			running = append(running[:foundIndex], running[foundIndex+1:]...)
			return ret, running
		}
	}
}

func partTwo(frontier []string, graph map[string][]string, deps map[string][]string) {
	var output []string
	var running []runningData
	for len(frontier) > 0 {
		frontier = SliceUniqMap(frontier)
		sort.Strings(frontier)
		log.Printf("Frontier is %v", frontier)
		var candidates []string
		candidates, frontier = findNextCandidates(frontier, graph, deps)
		for _, c := range candidates {
			t := int(c[0]) - 64
			log.Printf("Found %s with time %d", c, t)
			running = append(running, runningData{node: c, time: t})
		}
		var candidate string
		candidate, running = runUntilNextCandidateFinishes(running)
		log.Printf("Adding %s", candidate)
		output = append(output, candidate)
		for _, removeDep := range graph[candidate] {
			for dIndex := range deps[removeDep] {
				if deps[removeDep][dIndex] == candidate {
					deps[removeDep] = append(deps[removeDep][:dIndex], deps[removeDep][dIndex+1:]...)
					break
				}
			}
		}
	}
	log.Printf("%v", output)
}

func main() {
	file, err := os.Open("day7-2.txt")
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
	for k, v := range graph {
		log.Printf("%s -> %s", k, v)
	}

	var frontier []string

	for k := range all {
		_, ok := deps[k]
		if !ok {
			frontier = append(frontier, k)
		}
	}
	//partOne(frontier, graph, deps)
	partTwo(frontier, graph, deps)
}
