package sessions_test

import (
	"context"
	"testing"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/minij147/timetrack/webapp/backend/internal/sessions"
	"github.com/minij147/timetrack/webapp/backend/pkg/env"
	"github.com/stretchr/testify/assert"
)

func Test(t *testing.T) {
	env.Load("../../.env")
	dbpool, err := pgxpool.New(context.Background(), env.Get("DB_URL"))
	assert.Nil(t, err, "databse failed to make")
	assert.Nil(t, dbpool.Ping(context.Background()), "failed database ping")

	repo := sessions.NewPostgres(dbpool)
	serv := sessions.NewService(repo)

	t.Run("End", func(t *testing.T) {
		a := assert.New(t)
		ctx, _ := context.WithTimeout(context.Background(), 10*time.Second)
		a.Nil(serv.End(ctx, 1), "failed to end session")
	})
}
