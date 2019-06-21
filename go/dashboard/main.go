package main

import (
	"bytes"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/imos/icfpc2019/go/util/dbutil"

	_ "github.com/go-sql-driver/mysql"
	"google.golang.org/appengine"
)

type Response struct {
	Status  string `json:"status"`
	Message string `json:"message"`
}

var db *sql.DB

func handle(w http.ResponseWriter, r *http.Request) {
	json.NewEncoder(w).Encode(Response{Status: "ok", Message: "Hello world."})
}

func handler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/plain")

	rows, err := db.Query("SHOW DATABASES")
	if err != nil {
		http.Error(w, fmt.Sprintf("Could not query db: %v", err), 500)
		return
	}
	defer rows.Close()

	buf := bytes.NewBufferString("Databases:\n")
	for rows.Next() {
		var dbName string
		if err := rows.Scan(&dbName); err != nil {
			http.Error(w, fmt.Sprintf("Could not scan result: %v", err), 500)
			return
		}
		fmt.Fprintf(buf, "- %s\n", dbName)
	}
	w.Write(buf.Bytes())
}

func sqlHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/plain")
	ctx := appengine.NewContext(r)
	db, err := dbutil.NewConnection(ctx)
	if err != nil {
		http.Error(w, fmt.Sprintf("failed to connect to db: %s", err), 500)
		return
	}
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
	var (
		connectionName = os.Getenv("CLOUDSQL_CONNECTION_NAME")
		user           = os.Getenv("CLOUDSQL_USER")
		password       = os.Getenv("CLOUDSQL_PASSWORD")
	)

	var err error
	if appengine.IsDevAppServer() {
		dsn := fmt.Sprintf("%s:%s@tcp(%s)/%s",
			user,
			password,
			"127.0.0.1:3306",
			"")
		db, err = sql.Open("mysql", dsn)
	} else {
		target := fmt.Sprintf(
			"%s:%s@unix(/cloudsql/%s)/", user, password, connectionName)
		db, err = sql.Open("mysql", target)
		if err != nil {
			log.Fatalf("Could not open db: %v", err)
		}
	}

	http.HandleFunc("/", handle)
	http.HandleFunc("/sql", handler)
	http.HandleFunc("/db", sqlHandler)
	appengine.Main()
}
