package sessions

import "context"

type Repository interface {
	//Basic CRUD
	Create(ctx context.Context, accountID int64, name string) error
	End(ctx context.Context, accountID int64) error

    // fetches the active session if it exists
	GetActive(ctx context.Context, accountId int64) (EntitySession, error)

    // update the elapsed time
	UpdateElapsed(ctx context.Context, accountID int64) error
}
