package db

import (
	"context"
	"database/sql"
	"fmt"
	"os"
	"sync"

	"google.golang.org/appengine/log"

	"github.com/jmoiron/sqlx"
	"github.com/pkg/errors"
)

var db *sqlx.DB
var once sync.Once

func getDatabase() *sqlx.DB {
	// if db != nil {
	// 	return db
	// }
	once.Do(func() {
		password := os.Getenv("SQL_PASSWORD")
		if password != "" {
			password = ":" + password
		}
		dsn := fmt.Sprintf("%s%s@%s(%s)/%s",
			getEnvOrDefault("SQL_USER", "root"), password,
			getEnvOrDefault("SQL_PROTOCOL", "tcp"),
			getEnvOrDefault("SQL_ADDRESS", "127.0.0.1:3306"),
			getEnvOrDefault("SQL_DATABASE", getEnvOrDefault("SQL_USER", "root")))
		var err error
		db, err = sqlx.Open(getEnvOrDefault("SQL_DRIVER", "mysql"), dsn)
		if err != nil {
			panic(fmt.Errorf("%+v", errors.Errorf(
				"failed to open a new SQL connection: %s", err)))
		}
	})
	return db
}

func DB() *sqlx.DB {
	return getDatabase()
}

// Cell runs a query which should return one value.
func Cell(
	ctx context.Context, dest interface{}, query string, args ...interface{},
) error {
	rows, err := getDatabase().QueryContext(ctx, query, args...)
	if err != nil {
		return errors.Errorf(
			"failed to query: %s: %s", err, formatQuery(query, args))
	}
	defer rows.Close()
	cols, err := rows.Columns()
	if err != nil {
		return errors.Errorf(
			"failed to get columns: %s: %s", err, formatQuery(query, args))
	}
	if len(cols) != 1 {
		return errors.Errorf(
			"query result must have exactly one column, but %d: %s",
			len(cols), formatQuery(query, args))
	}
	defer rows.Close()
	if !rows.Next() {
		return errors.Errorf("no row returned: %s", formatQuery(query, args))
	}
	if err := rows.Scan(dest); err != nil {
		return errors.Errorf(
			"scan failed: %s: %s", err, formatQuery(query, args))
	}
	return nil
}

// CellString returns a string.
func CellString(
	ctx context.Context, query string, args ...interface{},
) (string, error) {
	dest := ""
	if err := Cell(ctx, &dest, query, args...); err != nil {
		return "", err
	}
	return dest, nil
}

// MustCellString returns a string.
func MustCellString(
	ctx context.Context, query string, args ...interface{},
) string {
	v, err := CellString(ctx, query, args...)
	if err != nil {
		log.Errorf(ctx, "query failed: %v", err)
	}
	return v
}

// Row runs a query which should return one row.
func Row(
	ctx context.Context, dest interface{}, query string, args ...interface{},
) error {
	return errors.WithStack(
		getDatabase().GetContext(ctx, dest, query, args...))
}

// Select runs a query which may return multiple rows.
func Select(
	ctx context.Context, dest interface{}, query string, args ...interface{},
) error {
	return errors.WithStack(
		getDatabase().SelectContext(ctx, dest, query, args...))
}

// Execute rusn a query which do not return values.
func Execute(
	ctx context.Context, query string, args ...interface{},
) (sql.Result, error) {
	result, err := getDatabase().ExecContext(ctx, query, args...)
	return result, errors.WithStack(err)
}

func formatQuery(query string, args []interface{}) string {
	return fmt.Sprintf("query=%v, args=%v", query, args)
}

func getEnvOrDefault(key, defaultValue string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return defaultValue
}
