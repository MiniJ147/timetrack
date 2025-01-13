package sessions

import (
	"context"
	"fmt"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

type repo struct {
	Conn *pgxpool.Pool
}

const (
	QUERY_CREATE = `
    INSERT INTO sessions (name, time_current, active, account_id) 
    VALUES ($1, CAST(extract(epoch from now()) AS bigint),TRUE, $2);
    `

	QUERY_GET_ACTIVE = `
    SELECT * FROM sessions
    WHERE account_id=$1 AND active=TRUE
    ORDER BY time_current DESC
    LIMIT 1;
    `
)

func (r *repo) Create(ctx context.Context, accountID int64, name string) error {
	_, err := r.Conn.Exec(ctx, QUERY_CREATE, name, accountID)
	return err
}

func (r *repo) GetActive(ctx context.Context, accountId int64) (EntitySession, error) {
	rows, _ := r.Conn.Query(ctx, QUERY_GET_ACTIVE, accountId)
	res, err := pgx.CollectOneRow(rows, pgx.RowToStructByPos[EntitySession])
	if err != nil {
		fmt.Printf("failed to collect %v", err)
	}

	return res, err
}

func NewPostgres(conn *pgxpool.Pool) Repository {
	return &repo{
		Conn: conn,
	}
}
