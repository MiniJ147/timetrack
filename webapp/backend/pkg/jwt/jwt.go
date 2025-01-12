package jwt

import (
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

var (
	ErrTokenExpired = errors.New("token has invalid claims: token is expired")
)

// creates JWT token.
// lifetime specficed by param.
// data will be put into the "data": field param
func Create(data interface{}, lifetime time.Duration, secret string) (string, error) {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256,
		jwt.MapClaims{
			"data": data,
			"exp":  time.Now().Add(lifetime).Unix(),
		})

	return token.SignedString([]byte(secret))
}

/*
Given token string, it will parse the jwt token.
Returns the token object, claims map, and error.
Claims: is the data felid.
*/
func ParseToClaims(tokenString string, secret string) (interface{}, jwt.MapClaims, error) {
	claims := jwt.MapClaims{}
	token, err := jwt.ParseWithClaims(tokenString, claims, func(t *jwt.Token) (interface{}, error) {
		return []byte(secret), nil
	})

	return token, claims, err
}

/*
parses out jwt given a jwtstring.
unloads data field into destation interface.
Returns error if failed.
*/
func ParseToJSON(tokenString string, destination interface{}, secret string) error {
	_, claims, err := ParseToClaims(tokenString, secret)
	if err != nil {
		return err
	}
	bytes, err := json.Marshal(claims["data"])
	if err != nil {
		return err
	}
	return json.Unmarshal(bytes, &destination)
}

// Checks to see if JWT is still valid given experation.
// Returns Error if its no longer vaild.
func Verify(tokenString string, secret string) error {
	token, err := jwt.Parse(tokenString, func(t *jwt.Token) (interface{}, error) {
		return []byte(secret), nil
	})

	if err != nil {
		return err
	}

	if !token.Valid {
		return fmt.Errorf("invalid token")
	}

	return nil
}

// calls the jwt.Verify func and checks if we hit an error.
func IsValid(tokenString string, secret string) bool {
	return Verify(tokenString, secret) == nil
}
