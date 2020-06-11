package main

import (
	"fmt"
	"github.com/gorilla/mux"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"text/template"
)

type Context struct {
	Title   string
	Header  string
	Kittens []string
}

func Increment(i int) int {
	return i + 1
}

func Modulo(i1 int, i2 int) int {
	return i1 % i2
}

func HomeHandler(w http.ResponseWriter, req *http.Request) {
	templates := template.New("templates")

	templates.
		New("home").
		Funcs(template.FuncMap{"inc": Increment, "mod": Modulo}).
		Parse(home)

	kittens := getFilenamesFromDir("./images")

	fmt.Println(kittens)

	context := Context{
		Title:   "Kittens",
		Header:  "Welcome, please browse our selection of cat pictures",
		Kittens: kittens,
	}

	templates.Lookup("home").Execute(w, context)
}

func LogHandler(w http.ResponseWriter, req *http.Request) {
	body, err := ioutil.ReadAll(req.Body)
	if err != nil {
		log.Printf("Error reading body: %v", err)
		http.Error(w, "can't read body", http.StatusBadRequest)
		return
	}

	f, err := os.OpenFile("text.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	defer f.Close()

	if err != nil {
		log.Printf("Error opening log file")
		http.Error(w, "Error opening log file", http.StatusBadRequest)
		return
	}

	if _, err := f.Write(body); err != nil {
		log.Printf("Error writing to log: %s", err)
		http.Error(w, "Could not write to log file", http.StatusBadRequest)
		return
	}
}

func ShowLogHandler(w http.ResponseWriter, req *http.Request) {
	dat, err := ioutil.ReadFile("text.log")
	if err != nil {
		log.Printf("Error reading log file")
		http.Error(w, "Error reading log file", http.StatusBadRequest)
		return
	}

	fmt.Fprintf(w, string(dat))
}

func main() {
	r := mux.NewRouter().StrictSlash(true)
	r.HandleFunc("/", HomeHandler).Methods("GET")
	r.
		PathPrefix("/images/").
		Handler(http.StripPrefix("/images/", http.FileServer(http.Dir("./images"))))
	r.
		PathPrefix("/js/").
		Handler(http.StripPrefix("/js/", http.FileServer(http.Dir("./js"))))

	r.HandleFunc("/log", LogHandler).Methods("POST")
	r.HandleFunc("/showlog", ShowLogHandler).Methods("GET")

	server := &http.Server{Addr: ":80", Handler: r}

	server.SetKeepAlivesEnabled(false)

	log.Fatal(server.ListenAndServe())
}

// Read filenames from given directory
func getFilenamesFromDir(dir string) []string {
	res := make([]string, 0, 10)
	files, err := ioutil.ReadDir(dir)

	fmt.Println(files)

	if err != nil {
		log.Fatal(err)
	}

	for _, f := range files {
		if !f.IsDir() {
			res = append(res, fmt.Sprintf("%s/%s", dir, f.Name()))
		}
	}

	return res
}
