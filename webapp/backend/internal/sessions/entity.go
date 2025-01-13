package sessions

type EntitySession struct {
    Id uint32
    Name string
    TimeElapsed int64
    TimeCurrent int64
    TimeEnded int64
    Active bool 
    AccountId uint32 
}
