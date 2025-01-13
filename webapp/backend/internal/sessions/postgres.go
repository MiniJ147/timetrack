package sessions

import (
	"context"

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
)

func (r *repo) Create(ctx context.Context, accountID int64, name string) error {
    _, err := r.Conn.Exec(ctx, QUERY_CREATE, name, accountID)
    return err
}

func NewPostgres(conn *pgxpool.Pool) Repository {
	return &repo{
		Conn: conn,
	}
}
