package sessions

import (
	"context"
	"fmt"

	"github.com/jackc/pgx/v5"
)

type Service interface {
	Create(ctx context.Context, name string, accountId int64) error
}

type service struct {
	repo Repository
}

func (s *service) Create(ctx context.Context, name string, accountId int64) error {
	res, err := s.repo.GetActive(ctx, accountId)
	if err == nil || err != pgx.ErrNoRows {
		fmt.Println("failed because session already active on account", res)
		return fmt.Errorf("Session already active on this account")

	}
	return s.repo.Create(ctx, accountId, name)
}

func NewService(repo Repository) Service {
	return &service{repo}
}
