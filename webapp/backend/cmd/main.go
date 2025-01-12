package main

import (
	"context"
	"log"
	"net/http"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/minij147/timetrack/webapp/backend/pkg/env"
)

func main() {
	env.Load()

	//connecting to postgres
	dbpool, err := pgxpool.New(context.Background(), env.Get("DB_URL"))
	if err != nil {
		log.Fatalf("Failed to create database connection with url: %v", env.Get("DB_URL"))
	}
	defer dbpool.Close()

	e := echo.New()

	//middlewares
	e.Use(middleware.CORS())
	e.Use(middleware.Logger())

	//routes
	e.GET("/api/v1/test", func(c echo.Context) error {
		return c.JSON(http.StatusOK, map[string]interface{}{
			"msg": "hello from timetrack",
		})
	})

	e.Logger.Fatal(e.Start(":" + env.Get("PORT")))
}
