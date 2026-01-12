package main

import (
	"fmt"
	"math"
	"math/rand"
	"os"
	"regexp"
	"strings"
)

const CORRECT_K = 2
const OUTPUT_FILE = "./results.md"

func main() {
	fmt.Println("Hello, World!")

	fileNames := []string{"./little-women.txt", "./also-rises.txt", "./huck-finn.txt", "./frankenstein.txt"}
	kValues := []int{1, 2, 3}
	elementsInFilter := []int{34, 35, 36, 119, 120, 121, 249, 250, 251, 399, 400, 401}
	searchTermLength := []int{3, 4, 5, 6, 7, 8, 9, 10, 16, 32}

	// keys: fileName, k, numElements, termLength
	// value: average distance
	results := make(map[string]map[int]map[int]map[int]int)

	for _, fileName := range fileNames {
		fmt.Println("Processing file:", fileName)
		sourceText := loadSourceText(fileName)
		articles := createArticles(sourceText)

		for _, k := range kValues {
			for _, numElements := range elementsInFilter {
				for i := range articles {
					articles[i].blossom(k, numElements)
				}
			}
		}

		for _, k := range kValues {
			for _, numElements := range elementsInFilter {
				for _, termLength := range searchTermLength {
					terms := pickSearchTerms(50, articles, termLength)

					distances := make([]int, 0, len(terms))
					for _, searchTerm := range terms {
						correctRanking := rankCorrectly(CORRECT_K, numElements, searchTerm, articles)
						hackyRanking := rankHackily(k, numElements, searchTerm, articles)
						// modify the '10' value below to only consider the top N articles in the ranking
						distance := kendallTauDistance(10, correctRanking, hackyRanking)
						distances = append(distances, distance)
					}

					totalDistance := 0
					for _, d := range distances {
						totalDistance += d
					}
					averageDistance := float64(totalDistance) / float64(len(distances))

					if results[fileName] == nil {
						results[fileName] = make(map[int]map[int]map[int]int)
					}
					if results[fileName][k] == nil {
						results[fileName][k] = make(map[int]map[int]int)
					}
					if results[fileName][k][numElements] == nil {
						results[fileName][k][numElements] = make(map[int]int)
					}
					results[fileName][k][numElements][termLength] = int(math.Round(averageDistance))
				}
			}
		}
	}

	saveResults(results, fileNames, kValues, elementsInFilter, searchTermLength)
}

type Article struct {
	Text         string
	Squashed     string
	BloomFilters map[int]map[int][]byte // first key = k, second key = elements in filter
}

type ArticleScore struct {
	Index ArticleIndex
	Score int
}

type ArticleIndex int
type Trigram string

func saveResults(results map[string]map[int]map[int]map[int]int, fileNames []string, kValues []int, elementsInFilter []int, searchTermLength []int) {
	// output is a markdown file
	// one table per k value
	// columns of the table should be term lengths
	// rows of the table should be numElements (in the bloom filters)
	// should average across all fileNames

	f, err := os.Create(OUTPUT_FILE)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	for _, k := range kValues {
		avgAvg := 0.0
		countAvg := 0
		fmt.Fprintf(f, "## k = %d\n\n", k)

		// header row
		fmt.Fprintf(f, "| Num Elements ")
		for _, termLength := range searchTermLength {
			fmt.Fprintf(f, "| %d ", termLength)
		}
		fmt.Fprintf(f, "|\n")

		// separator row
		fmt.Fprintf(f, "|--------------")
		for range searchTermLength {
			fmt.Fprintf(f, "|------")
		}
		fmt.Fprintf(f, "|\n")

		// data rows
		for _, numElements := range elementsInFilter {
			fmt.Fprintf(f, "| %d ", numElements)
			for _, termLength := range searchTermLength {
				// average across all fileNames
				total := 0
				count := 0
				for _, fileResults := range results {
					if fileResults[k] != nil && fileResults[k][numElements] != nil {
						if distance, ok := fileResults[k][numElements][termLength]; ok {
							total += distance
							count += 1
						}
					}
				}
				average := 0
				if count > 0 {
					average = int(math.Round(float64(total) / float64(count)))
				}
				avgAvg += float64(average)
				countAvg += 1
				fmt.Fprintf(f, "| %d ", average)
			}
			fmt.Fprintf(f, "|\n")
		}

		fmt.Fprintf(f, "\n")
		overallAvg := 0.0
		if countAvg > 0 {
			overallAvg = avgAvg / float64(countAvg)
		}
		fmt.Fprintf(f, "Average distance for k=%d: %.2f\n\n", k, overallAvg)
		fmt.Fprintf(f, "---\n\n")
	}
}

func loadSourceText(path string) string {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	return string(data)
}

// read the sourceText 1,200 characters at a time
// create 100 articles from this
func createArticles(sourceText string) []Article {
	result := make([]Article, 0, 100)

	for i := 0; i < 100; i++ {
		start := i * 1600
		end := start + 1600
		if end > len(sourceText) {
			panic("sourceText too short")
		}
		articleText := sourceText[start:end]

		article := Article{
			Text: articleText,
		}
		article.squash()
		result = append(result, article)
	}

	return result
}

func pickSearchTerms(numTerms int, articles []Article, termLength int) [][]Trigram {
	result := make([][]Trigram, 0, numTerms)

	for i := 0; i < numTerms; i++ {
		article := articles[rand.Intn(len(articles))]
		startPos := rand.Intn(len(article.Squashed) - termLength)
		term := article.Squashed[startPos : startPos+termLength]
		trigrams := parseTrigrams(term)
		result = append(result, trigrams)
	}

	return result
}

func (a *Article) squash() {
	a.Squashed = squashText(a.Text)
	if len(a.Squashed) < 403 {
		println("squashed text too short:", len(a.Squashed))
		println(a.Squashed)
		panic("panic")
	}
}

func (a *Article) blossom(k int, elementCount int) {
	if k < 1 || k > 3 {
		panic("k must be 1, 2, or 3")
	}

	// filter size is alway 2048 bits (256 bytes)

	// take the first charCount characters of the squashed text
	// form into trigrams
	// for each trigram:
	//   get hash from djb2tri
	//   extract two bit positions
	//   set those bits in the bloom filter

	filter := make([]byte, 256)

	trigrams := parseTrigrams(a.Squashed[:elementCount+2]) // +2 because we want that many trigrams, and there are (N-2) trigrams in N characters
	for _, tri := range trigrams {
		hash := djb2tri(tri)

		for i := 0; i < k; i++ {
			bitPos := (hash >> (11 * i)) % 2048
			filter[bitPos/8] |= (1 << (bitPos % 8))
		}
	}

	if a.BloomFilters == nil {
		a.BloomFilters = make(map[int]map[int][]byte)
	}
	if a.BloomFilters[k] == nil {
		a.BloomFilters[k] = make(map[int][]byte)
	}
	a.BloomFilters[k][elementCount] = filter
}

// returns 20 terms, already squashed but not split into trigrams
func (a *Article) extractSearchTerms() []string {
	terms := make([]string, 0, 20)

	// from consecutive characters
	{
		consecutive := []int{3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 32}
		for _, length := range consecutive {
			terms = append(terms, a.Squashed[:length])
		}
	}

	// from non-consecutive words
	{
		words := strings.Fields(a.Text[:200])
		wordCount := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}
		for _, count := range wordCount {
			var selectedWords []string
			for i := 0; i < count; i++ {
				oddIndex := i*2 + 1
				selectedWords = append(selectedWords, words[oddIndex%len(words)])
			}

			term := strings.Join(selectedWords, "")
			terms = append(terms, squashText(term))
		}
	}

	return terms
}

// 1. lowercase
// 2. replace all non-alphanumeric (excluding #) with space
// 3. remove all spaces
// example: "Hello, World! #This\nis a test." -> "helloworld#thisisatest"
func squashText(text string) string {
	result := strings.ToLower(text)
	result = regexp.MustCompile(`[^a-z0-9#]`).ReplaceAllString(result, " ")
	result = strings.ReplaceAll(result, " ", "")
	return result
}

// takes a string, returns all unique 3-letter chunks
// does not squash or otherwise preprocess the text
func parseTrigrams(text string) []Trigram {
	trigramSet := make(map[string]struct{})
	for i := 0; i < len(text)-2; i++ {
		trigram := text[i : i+3]
		trigramSet[trigram] = struct{}{}
	}
	trigrams := make([]Trigram, 0, len(trigramSet))
	for trigram := range trigramSet {
		trigrams = append(trigrams, Trigram(trigram))
	}
	return trigrams
}

// 'djb2' hash function, specifically for a single trigram
func djb2tri(tri Trigram) int64 {
	var h int64 = 5381
	for i := 0; i < 33; i++ {
		h = ((h << 5) + h) ^ int64(tri[i%3])
	}
	if h < 0 {
		return -h
	}
	return h
}

// ranks articles from highest to lowest score, based on how many trigrams match
func rankCorrectly(k int, elementCount int, searchTerm []Trigram, articles []Article) []ArticleIndex {
	scoreRanking := make([]ArticleScore, 0, len(articles))

	for articleIndex, article := range articles {
		countMatching := 0
		seenTri := make(map[Trigram]struct{})
		for _, tri := range searchTerm {
			hash := djb2tri(tri)
			allBitsMatch := true
			for i := 0; i < k; i++ {
				bitPos := (hash >> (11 * i)) % 2048
				if (article.BloomFilters[k][elementCount][bitPos/8] & (1 << (bitPos % 8))) == 0 {
					allBitsMatch = false
					break
				}
			}
			if allBitsMatch {
				countMatching += 1
			}

			if _, alreadySeen := seenTri[tri]; !alreadySeen {
				seenTri[tri] = struct{}{}
			} else {
				fmt.Println("duplicate trigram in search term!!:", tri)
			}
		}

		newScore := ArticleScore{
			Index: ArticleIndex(articleIndex),
			Score: countMatching,
		}

		inserted := false
		for rank, existing := range scoreRanking {
			if newScore.Score > existing.Score {
				scoreRanking = append(scoreRanking[:rank], append([]ArticleScore{newScore}, scoreRanking[rank:]...)...)
				inserted = true
				break
			}
		}
		if !inserted {
			scoreRanking = append(scoreRanking, newScore)
		}
	}

	result := make([]ArticleIndex, len(scoreRanking))
	for i, articleScore := range scoreRanking {
		result[i] = articleScore.Index
	}
	return result
}

// ranks articles from highest to lowest score, based on how many bits match
func rankHackily(k int, elementCount int, searchTerm []Trigram, articles []Article) []ArticleIndex {
	// suppose that the bloom filters are fully saturated: every bit is set to 1
	// this means that every term will match every article perfectly -> score as high as possible
	// both rankCorrectly and rankHackily use insertion sort, and if they both iterate through articles in the same order, then both will produce the same ranking
	// this is just something to watch out for

	scoreRanking := make([]ArticleScore, 0, len(articles))

	searchBloom := make([]byte, 256)
	for _, tri := range searchTerm {
		hash := djb2tri(tri)
		for i := 0; i < k; i++ {
			bitPos := (hash >> (11 * i)) % 2048
			searchBloom[bitPos/8] |= (1 << (bitPos % 8))
		}
	}

	for articleIndex, article := range articles {
		matchingBits := 0
		for byteIndex := 0; byteIndex < 256; byteIndex++ {
			matchingBits += CountMatchingBits(searchBloom[byteIndex], article.BloomFilters[k][elementCount][byteIndex])
		}

		newScore := ArticleScore{
			Index: ArticleIndex(articleIndex),
			Score: matchingBits,
		}

		inserted := false
		for rank, existing := range scoreRanking {
			if newScore.Score > existing.Score {
				scoreRanking = append(scoreRanking[:rank], append([]ArticleScore{newScore}, scoreRanking[rank:]...)...)
				inserted = true
				break
			}
		}
		if !inserted {
			scoreRanking = append(scoreRanking, newScore)
		}
	}

	result := make([]ArticleIndex, len(scoreRanking))
	for i, articleScore := range scoreRanking {
		result[i] = articleScore.Index
	}
	return result
}

func CountMatchingBits(a, b byte) int {
	matchingBits := a & b // Perform bitwise AND to isolate matching bits
	count := 0

	// Count the number of 1s in the result
	for matchingBits > 0 {
		count += int(matchingBits & 1) // Check the least significant bit
		matchingBits >>= 1             // Shift right to check the next bit
	}

	return count
}

// cr = correctly ranked articles
// hr = hackily ranked articles
// returns a value in the range 0..1000, where 0 = perfect agreement, 1000 = complete disagreement
func kendallTauDistance(topN int, cr []ArticleIndex, hr []ArticleIndex) int {
	distance := 0

	// we only care about the top N correctly ranked articles
	if len(cr) > topN {
		cr = cr[:topN]
	}

	// convert hr into a map from article index to rank
	hrMap := make(map[ArticleIndex]int)
	for rank, articleIndex := range hr {
		hrMap[articleIndex] = rank
	}

	// compare every pair or articles (x,y) in sortedCS
	for x := 0; x < len(cr); x++ {
		for y := x + 1; y < len(cr); y++ {
			rankXinHR := hrMap[cr[x]]
			rankYinHR := hrMap[cr[y]]

			// x < y, and cr is a ranking of articles, so cr[x].Score >= cr[y].Score
			// if (the score of article cr[x] in hr is greater than the score of article cr[y] in hr), then cr and hr are in agreement and we don't increment distance
			// in hr, higher rank means lower score. So if cr and hr match then we expect rankXinHR <= rankYinHR

			if rankXinHR > rankYinHR {
				distance += 1
			}
		}
	}

	// normalize distance
	normalized := float64(distance) / float64(len(cr)*(len(cr)-1)/2)

	// three sig figs
	return int(math.Round(normalized * 1000))
}
