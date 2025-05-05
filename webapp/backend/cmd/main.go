package main

import (
	"context"
	"log"
	"net/http"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/minij147/timetrack/webapp/backend/internal/sessions"
	"github.com/minij147/timetrack/webapp/backend/pkg/env"
)

func main() {
	log.Println("Server starting...")
	env.Load()

	//connecting to postgres
	dbpool, err := pgxpool.New(context.Background(), env.Get("DB_URL"))
	if err != nil {
		log.Fatalf("failed to initalize connection with %v | err: %v", env.Get("DB_URL"), err)
	}

	log.Println("Pinging Database...")
	if err := dbpool.Ping(context.Background()); err != nil {
		log.Fatalf("failed to ping database check connection: %v | err: %v", env.Get("DB_URL"), err)
	}
	defer dbpool.Close()
	log.Println("database connected and ready start server...")

	e := echo.New()

	//middlewares
	e.Use(middleware.CORS())
	// e.Use(middleware.Logger())

	// handlers
	handlerSession := sessions.New(sessions.NewPostgres(dbpool))

	//routes
	e.GET("/api/v1/test", func(c echo.Context) error {
		return c.JSON(http.StatusOK, map[string]interface{}{
			"msg": "hello from timetrack",
		})
	})

	e.GET("/api/v1/sessions/new", handlerSession.PostNew)

	log.Println("Server Online.")
	e.Logger.Fatal(e.Start(":" + env.Get("PORT")))
}
