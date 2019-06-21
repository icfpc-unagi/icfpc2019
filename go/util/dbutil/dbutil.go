package dbutil

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"google.golang.org/appengine/log"

	"github.com/jmoiron/sqlx"
	"github.com/pkg/errors"
)

// Database manages a database connection.
type Database struct {
	db *sqlx.DB
}

// NewConnection returns a connection to a database.
func NewConnection(ctx context.Context) (*Database, error) {
	password := os.Getenv("SQL_PASSWORD")
	if password != "" {
		password = ":" + password
	}
	dsn := fmt.Sprintf("%s%s@%s(%s)/%s",
		getEnvOrDefault("SQL_USER", "root"), password,
		getEnvOrDefault("SQL_PROTOCOL", "tcp"),
		getEnvOrDefault("SQL_ADDRESS", "127.0.0.1:3306"),
		getEnvOrDefault("SQL_DATABASE", getEnvOrDefault("SQL_USER", "root")))
	db, err := sqlx.Open(getEnvOrDefault("SQL_DRIVER", "mysql"), dsn)
	if err != nil {
		return nil, errors.Errorf(
			"failed to open a new SQL connection: %s", err)
	}
	return &Database{
		db: db,
	}, nil
}

// Cell runs a query which should return one value.
func (db *Database) Cell(
	ctx context.Context, dest interface{}, query string, args ...interface{},
) error {
	rows, err := db.db.QueryContext(ctx, query, args...)
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
func (db *Database) CellString(
	ctx context.Context, query string, args ...interface{},
) (string, error) {
	dest := ""
	if err := db.Cell(ctx, &dest, query, args...); err != nil {
		return "", err
	}
	return dest, nil
}

// MustCellString returns a string.
func (db *Database) MustCellString(
	ctx context.Context, query string, args ...interface{},
) string {
	v, err := db.CellString(ctx, query, args...)
	if err != nil {
		log.Errorf(ctx, "query failed: %v", err)
	}
	return v
}

// Row runs a query which should return one row.
func (db *Database) Row(
	ctx context.Context, dest interface{}, query string, args ...interface{},
) error {
	return db.db.GetContext(ctx, dest, query, args...)
}

// Select runs a query which may return multiple rows.
func (db *Database) Select(
	ctx context.Context, dest interface{}, query string, args ...interface{},
) error {
	return db.db.SelectContext(ctx, dest, query, args...)
}

// Execute rusn a query which do not return values.
func (db *Database) Execute(
	ctx context.Context, query string, args ...interface{},
) (sql.Result, error) {
	return db.db.ExecContext(ctx, query, args...)
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
