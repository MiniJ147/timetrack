package env

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

var loaded bool = false

func Load(filenames ...string) {
	if godotenv.Load(filenames...) != nil {
		log.Fatalf("Error loading %v files", filenames)
	}

	loaded = true
}

func Get(name string) string {
	if !loaded {
		log.Fatal("attempting to grab from unloaded .env")
	}

	return os.Getenv(name)
}
