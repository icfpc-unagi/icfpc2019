package main

import (
	"encoding/json"
	"fmt"
	"net/http"

	_ "github.com/go-sql-driver/mysql"
	"google.golang.org/appengine"

	_ "github.com/imos/icfpc2019/go/dashboard/api"
	_ "github.com/imos/icfpc2019/go/dashboard/handler"
	"github.com/imos/icfpc2019/go/util/db"
)

type Response struct {
	Status  string `json:"status"`
	Message string `json:"message"`
}

func handle(w http.ResponseWriter, r *http.Request) {
	if r.URL.Path == "/" {
		http.Redirect(w, r, "/ranking", http.StatusFound)
		return
	}
	json.NewEncoder(w).Encode(Response{Status: "ok", Message: "Hello world."})
}

func sqlHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/plain")
	ctx := appengine.NewContext(r)
	out := struct {
		Name string `json:"name"`
	}{}
	if err := db.Row(ctx, &out, "SELECT ? AS `name`", "hogehoge"); err != nil {
		http.Error(w, fmt.Sprintf("failed to select: %s", err), 500)
		return
	}
	w.Write([]byte(fmt.Sprintf("%v\n", out)))
	w.Write([]byte(fmt.Sprintf("%v\n", db.MustCellString(ctx, "SELECT 1 + 1"))))
	w.Write([]byte(fmt.Sprintf("%v\n", db.MustCellString(ctx, `SELECT "hoge"`))))
}

func main() {
	http.HandleFunc("/", handle)
	http.HandleFunc("/db", sqlHandler)
	appengine.Main()
}
