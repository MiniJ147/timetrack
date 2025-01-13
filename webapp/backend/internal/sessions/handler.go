package sessions

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
)

type JSON map[string]interface{}

type Handler interface {
	PostNew(c echo.Context) error
}
type handler struct {
	service Service
}

func (h *handler) PostNew(c echo.Context) error {
    err := h.service.Create(c.Request().Context(), "testtest2", 1)
    if err != nil{
        return c.JSON(http.StatusBadRequest,JSON{
            "err":"session already active or failed crate idk this is just basic error msg",
        })
    }
    fmt.Println(err)

    return c.JSON(http.StatusCreated,JSON{
        "msg": "new session made!",
    })
}

func New(repo Repository) Handler {
	return &handler{
		service: NewService(repo),
	}
}
