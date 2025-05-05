package sessions

import (
	"context"
	"fmt"

	"github.com/jackc/pgx/v5"
)

type Service interface {
	Create(ctx context.Context, name string, accountID int64) error
	UpdateTimeElapsed(ctx context.Context, accountID int64) error
	End(ctx context.Context, accountID int64) error
}

type service struct {
	repo Repository
}

/*
Create will attempt to create a new active session under the users account.
Fails when an active session already exists or when there is an unexpected error
*/
func (s *service) Create(ctx context.Context, name string, accountID int64) error {
	res, err := s.repo.GetActive(ctx, accountID)
	if err == nil || err != pgx.ErrNoRows {
		fmt.Println("failed because session already active on account", res)

		return ErrActiveSession
	}
	return s.repo.Create(ctx, accountID, name)
}

func (s *service) UpdateTimeElapsed(ctx context.Context, accountID int64) error {
	return s.repo.UpdateElapsed(ctx, accountID)
}

func (s *service) End(ctx context.Context, accountID int64) error {
	err := s.UpdateTimeElapsed(ctx, accountID)
	if err != nil {
		return err
	}

	return s.repo.End(ctx, accountID)
}

func NewService(repo Repository) Service {
	return &service{repo}
}
