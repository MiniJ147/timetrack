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

	//[NOTE]: does not handle udpating the elapsed_time this is because
	//        updating the elapsed is varyed depending on the sitution ie) the session is paused and you stop it
	//        so it is up to the service to call the update_elapsed function before ending.
	QUERY_END = `
    UPDATE sessions
    SET active=FALSE, time_ended=CAST(extract(epoch from now()) AS bigint)
    WHERE account_id = $1
    `

	QUERY_GET_ACTIVE = `
    SELECT * FROM sessions
    WHERE account_id=$1 AND active=TRUE
    ORDER BY time_current DESC
    LIMIT 1;
    `

	QUERY_UPDATE_ELAPSED = `
    WITH time_now AS (SELECT * FROM CAST(extract(epoch from now()) AS bigint) AS time_now)
    UPDATE sessions
    SET time_elapsed = time_elapsed + (time_now - time_current), time_current = time_now
    FROM time_now
    WHERE active = TRUE AND account_id = $1;
    `
)

//basic CRUD

func (r *repo) Create(ctx context.Context, accountID int64, name string) error {
	_, err := r.Conn.Exec(ctx, QUERY_CREATE, name, accountID)
	return err
}

func (r *repo) End(ctx context.Context, accountID int64) error {
	_, err := r.Conn.Exec(ctx, QUERY_END, accountID)
	return err
}

//OTHERS

func (r *repo) GetActive(ctx context.Context, accountId int64) (EntitySession, error) {
	rows, _ := r.Conn.Query(ctx, QUERY_GET_ACTIVE, accountId)
	res, err := pgx.CollectOneRow(rows, pgx.RowToStructByPos[EntitySession])
	if err != nil {
		fmt.Printf("failed to collect %v", err)
	}

	return res, err
}

func (r *repo) UpdateElapsed(ctx context.Context, accountID int64) error {
	_, err := r.Conn.Exec(ctx, QUERY_UPDATE_ELAPSED, accountID)
	return err
}

func NewPostgres(conn *pgxpool.Pool) Repository {
	return &repo{
		Conn: conn,
	}
}
