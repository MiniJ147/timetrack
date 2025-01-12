package oauth2

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"os"

	"github.com/minij147/timetrack/webapp/backend/pkg/env"
	"golang.org/x/oauth2"
)

type githubImp struct {
	cfg *oauth2.Config
}

type GithubEmailData struct {
	Email    string `json:"email"`
	Primary  bool   `json:"primary"`
	Verified bool   `json:"verified"`
}

func (g *githubImp) FetchEmail(ctx context.Context, code string) (string, error) {
	token, err := g.cfg.Exchange(ctx, code)
	if err != nil {
		return "", err
	}

	// Store the access token and refresh token in in-memory session storage.
	fmt.Println(token.AccessToken)
	fmt.Println(token.RefreshToken)

	// making new request
	req, err := http.NewRequest(http.MethodGet, "https://api.github.com/user/emails", nil)
	if err != nil {
		return "", err
	}

	// setting headers to fetch data
	req.Header.Set("Authorization", fmt.Sprintf("Bearer %s", token.AccessToken))
	req.Header.Set("Accept", "application/vnd.github+json")

	// attempting to fetch data
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	// reading body data
	var userInfo []GithubEmailData
	err = json.NewDecoder(resp.Body).Decode(&userInfo)
	if err != nil {
		return "", err
	}

	// searching for valid user info
	for _, info := range userInfo {
		if info.Primary && info.Verified {
			return info.Email, nil
		}
	}

	return "", fmt.Errorf("could not find validate email from github")
}
func (g *githubImp) Link() string {
	return fmt.Sprintf("https://github.com/login/oauth/authorize?scope=user:email&client_id=%s", os.Getenv("GITHUB_CLIENT_ID"))
}
func (g *githubImp) Config() *oauth2.Config {
	return g.cfg
}

func NewGithub() Authorizer {
	// panic because I pulled this from copr projectt
	panic("warning Github oauth is not implemented please go and make it, missing tokens and redirect")
	return &githubImp{
		cfg: &oauth2.Config{
			ClientID:     env.Get("GITHUB_CLIENT_ID"),
			ClientSecret: env.Get("GITHUB_CLIENT_SECRET"),
			Endpoint: oauth2.Endpoint{
				AuthURL:  "https://github.com/login/oauth/authorize",
				TokenURL: "https://github.com/login/oauth/access_token",
			},
			RedirectURL: "",
			Scopes:      []string{"user:email"},
		},
	}
}
