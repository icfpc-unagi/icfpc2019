package sqlutil

import (
	"context"
	"database/sql"
)

var db *sql.DB

func Cell(ctx context.Context, query string, args ...interface{}) (string, error) {
	rows, err := db.QueryContext(ctx, query, args...)
	if err != nil {
		return "", err
	}
	defer rows.Close()
	for rows.Next() {
		rows.Scan()
	}
	return "", nil
}
