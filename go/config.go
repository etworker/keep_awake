package main

import (
	"encoding/json"
	"log"
	"os"
	"path/filepath"
)

type Mode int

const (
	ModeAPI   Mode = 0
	ModeMouse Mode = 1
)

func (m Mode) String() string {
	switch m {
	case ModeAPI:
		return "api"
	case ModeMouse:
		return "mouse"
	}
	return "api"
}

func modeFromString(s string) Mode {
	switch s {
	case "mouse":
		return ModeMouse
	default:
		return ModeAPI
	}
}

type Config struct {
	Enabled      bool `json:"enabled"`
	Mode         Mode `json:"mode"`
	IntervalSecs int  `json:"interval_secs"`
	Autostart    bool `json:"autostart"`
}

func defaultConfig() Config {
	return Config{
		Enabled:      true,
		Mode:         ModeAPI,
		IntervalSecs: 30,
		Autostart:    false,
	}
}

func configPath() string {
	dir, err := os.UserConfigDir()
	if err != nil {
		dir = "."
	}
	p := filepath.Join(dir, "keep-awake")
	os.MkdirAll(p, 0755)
	return filepath.Join(p, "config.json")
}

func loadConfig() Config {
	cfg := defaultConfig()
	data, err := os.ReadFile(configPath())
	if err != nil {
		return cfg
	}
	var raw struct {
		Enabled      *bool   `json:"enabled"`
		Mode         *string `json:"mode"`
		IntervalSecs *int    `json:"interval_secs"`
		Autostart    *bool   `json:"autostart"`
	}
	if err := json.Unmarshal(data, &raw); err != nil {
		return cfg
	}
	if raw.Enabled != nil {
		cfg.Enabled = *raw.Enabled
	}
	if raw.Mode != nil {
		cfg.Mode = modeFromString(*raw.Mode)
	}
	if raw.IntervalSecs != nil {
		cfg.IntervalSecs = *raw.IntervalSecs
	}
	if raw.Autostart != nil {
		cfg.Autostart = *raw.Autostart
	}
	return cfg
}

func (c Config) Save() {
	data, err := json.MarshalIndent(c, "", "  ")
	if err != nil {
		log.Println("Failed to marshal config:", err)
		return
	}
	if err := os.WriteFile(configPath(), data, 0644); err != nil {
		log.Println("Failed to write config:", err)
	}
}
