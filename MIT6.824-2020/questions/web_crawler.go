package main

import (
	"fmt"
	"sync"
)

const CHANNEL_CAPACITY = 64

type Fetcher interface {
	// Fetch returns the body of URL and
	// a slice of URLs found on that page.
	Fetch(url string) (body string, urls []string, err error)
}

type UrlCache struct {
	mutex sync.Mutex
	cache map[string]bool
}

func newCache() *UrlCache {
	return &UrlCache{
		cache: make(map[string]bool),
	}
}

// Returns whether the cache already contains the supplied URL.
//
// The "check then insert" operation must be performed atomically, otherwise the user of the cache
// would be vulnerable to TOCTOU bugs.
func (cache *UrlCache) insert(url string) bool {
	cache.mutex.Lock()
	defer cache.mutex.Unlock()

	_, ok := cache.cache[url]
	if !ok {
		cache.cache[url] = true
	}
	return ok
}

type CrawlerContext struct {
	cache *UrlCache
	// channel used as a barrier
	cnt chan bool
}

// Crawl uses fetcher to recursively crawl
// pages starting with url, to a maximum of depth.
func Crawl(context *CrawlerContext, url string, depth int, fetcher Fetcher) {
	cnt := context.cnt
	defer func() { cnt <- true }()

	if depth <= 0 {
		return
	}

	if context.cache.insert(url) {
		return
	}

	body, urls, err := fetcher.Fetch(url)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Printf("found: %s %q\n", url, body)

	subCnt := make(chan bool, CHANNEL_CAPACITY)

	subContext := CrawlerContext{
		cache: context.cache,
		cnt:   subCnt,
	}

	for _, u := range urls {
		go Crawl(&subContext, u, depth-1, fetcher)
	}

	for i := 0; i < len(urls); i++ {
		<-subCnt
	}

	return
}

func main() {
	cache := newCache()
	context := CrawlerContext{
		cache: cache,
		cnt:   make(chan bool, 1),
	}

	Crawl(&context, "https://golang.org/", 4, fetcher)
}

// fakeFetcher is Fetcher that returns canned results.
type fakeFetcher map[string]*fakeResult

type fakeResult struct {
	body string
	urls []string
}

func (f fakeFetcher) Fetch(url string) (string, []string, error) {
	if res, ok := f[url]; ok {
		return res.body, res.urls, nil
	}
	return "", nil, fmt.Errorf("not found: %s", url)
}

// fetcher is a populated fakeFetcher.
var fetcher = fakeFetcher{
	"https://golang.org/": &fakeResult{
		"The Go Programming Language",
		[]string{
			"https://golang.org/pkg/",
			"https://golang.org/cmd/",
		},
	},
	"https://golang.org/pkg/": &fakeResult{
		"Packages",
		[]string{
			"https://golang.org/",
			"https://golang.org/cmd/",
			"https://golang.org/pkg/fmt/",
			"https://golang.org/pkg/os/",
		},
	},
	"https://golang.org/pkg/fmt/": &fakeResult{
		"Package fmt",
		[]string{
			"https://golang.org/",
			"https://golang.org/pkg/",
		},
	},
	"https://golang.org/pkg/os/": &fakeResult{
		"Package os",
		[]string{
			"https://golang.org/",
			"https://golang.org/pkg/",
		},
	},
}
