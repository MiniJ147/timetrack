package main

import (
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/minij147/timetrack/webapp/backend/pkg/env"
)

func main() {
	env.Load()
	e := echo.New()

	e.GET("/", func(c echo.Context) error {
		return c.JSON(http.StatusOK, map[string]interface{}{
			"msg": "hello from timetrack",
		})
	})

	e.Logger.Fatal(e.Start(":" + env.Get("port")))
}
