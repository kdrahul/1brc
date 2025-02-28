package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"os"
	"runtime/pprof"
	"strconv"
	"strings"
)

const filename string = "measurements_10000.txt"

type Station struct {
	City  string
	Value float32
}

type Measurement struct {
	Min   float32
	Max   float32
	Sum   float32
	Count uint32
}

func (m *Measurement) calculate(station Station) {
	m.Sum += station.Value
	m.Min = min(station.Value, m.Min)
	m.Max = max(station.Value, m.Max)
	m.Count += 1
}

func main() {

	var (
		cpuProfile = flag.String("cpuprofile", "", "write CPU profile to file")
		// revision   = flag.Int("revision", len(revisionFuncs), "revision of solution to run")
		// goroutines = flag.Int("goroutines", 0, "num goroutines for parallel solutions (default NumCPU)")
		// benchAll   = flag.Bool("benchall", false, "benchmark all solutions")
	)
	flag.Parse()
	if *cpuProfile != "" {
		f, err := os.Create(*cpuProfile)
		if err != nil {
			fmt.Fprintf(os.Stderr, "error: %v\n", err)
			os.Exit(1)
		}
		pprof.StartCPUProfile(f)
		defer pprof.StopCPUProfile()
	}
	// filename := os.Args[1]
	cities := make(map[string]*Measurement)
	writerFile, err := os.Create("result")
	defer writerFile.Close()
	if err != nil {
		log.Fatalf("Error creating file : %s", err.Error())
	}
	file, err := os.Open(filename)
	defer file.Close()
	if err != nil {
		log.Fatalf("Error reading the file %s : %s", filename, err.Error())
	}

	data := bufio.NewReader(file)

	if err != nil {
		log.Fatalf("Error reading the file %s", filename)
	}

	w := bufio.NewWriter(writerFile)

	compute(data, cities)
	storeAndPrint(w, cities)
}

func storeAndPrint(w *bufio.Writer, cities map[string]*Measurement) {
	w.Write([]byte("{"))
	for name, result := range cities {

		// w.WriteString(fmt.Sprintf("%s: %v", name, result))
		w.WriteString(fmt.Sprintf(
			"%s=%.1f/%.1f/%.1f,\n",
			name,
			result.Min,
			result.Sum/float32(result.Count),
			result.Max))
	}
	w.Write([]byte("}"))
	w.Flush()
}

func compute(data *bufio.Reader, cities map[string]*Measurement) {
	for {
		// Read file line-by-line
		d, _, err := data.ReadLine()
		if err != nil {
			break
		}
		splitEnds := strings.Split(string(d), ";")
		// Parsing takes too much computation
		value, err := strconv.ParseFloat(splitEnds[1], 32) // Huge computation, requires optimization

		if err != nil {
			log.Fatalln("Error parsing a float: " + err.Error())
		}
		station := Station{splitEnds[0], float32(value)}

		city, ok := cities[station.City]

		if ok {
			city.calculate(station)
		} else {
			cities[station.City] = &Measurement{
				Count: 1,
				Min:   station.Value,
				Max:   station.Value,
				Sum:   station.Value,
			}
		}
	}
}
