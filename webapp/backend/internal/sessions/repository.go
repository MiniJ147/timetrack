package sessions

import "context"

type Repository interface {
	Create(ctx context.Context, accountID int64, name string) error
	GetActive(ctx context.Context, accountId int64) (EntitySession, error)
}
