package tasks

type Handler interface{}
type handler struct{}

func New() Handler {
	return &handler{}
}
