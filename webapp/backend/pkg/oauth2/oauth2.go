package oauth2

import (
	"context"

	"golang.org/x/oauth2"
)

var GithubCfg oauth2.Config

type Authorizer interface {
	// fetches email based off ctx and code given through the oauth provider.
	FetchEmail(ctx context.Context, code string) (string, error)

	// returns the link that is used to intilize the oauth2 request.
	Link() string

	// returns the oauth2 configuration data.
	Config() *oauth2.Config
}
