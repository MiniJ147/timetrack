package env

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

// tracks if env has been loaded
var loaded bool = false

// loads env file with optional filenames as given by the godotenv.Load() docs
// Fatalf if it fails to Load
// keeps loaded boolean check to ensure incorrect calls aren't made
func Load(filenames ...string) {
	if godotenv.Load(filenames...) != nil {
		log.Fatalf("Error loading %v files", filenames)
	}

	loaded = true
}

// get via name with os.Getenv()
// Fatal if .env is not loaded
func Get(name string) string {
	if !loaded {
		log.Fatal("attempting to grab from unloaded .env")
	}

	return os.Getenv(name)
}
