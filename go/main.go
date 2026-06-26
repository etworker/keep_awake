package main

import (
	"log"
	"sync"

	"github.com/getlantern/systray"
)

var (
	guard   InhibitGuard
	jiggler *MouseJiggler
	mu      sync.Mutex
)

func main() {
	if !ensureSingleInstance() {
		return
	}
	detectLang()

	cfg := loadConfig()
	enabled = cfg.Enabled
	currentMode = cfg.Mode

	systray.Run(onReady, onExit)
}

var (
	enabled     bool
	currentMode Mode
)

func onReady() {
	cfg := loadConfig()
	setupTray(cfg)
	showNotification(Tr(TitleRunning, TitleRunningZh), Tr(TextRunning, TextRunningZh))
	if cfg.Enabled {
		activate(cfg)
	}
}

func onExit() {
	mu.Lock()
	defer mu.Unlock()
	if guard != nil {
		guard.Release()
		guard = nil
	}
	if jiggler != nil {
		jiggler.Stop()
		jiggler = nil
	}
}

func toggleEnabled() {
	mu.Lock()
	defer mu.Unlock()

	cfg := loadConfig()
	cfg.Enabled = !cfg.Enabled
	cfg.Save()
	enabled = cfg.Enabled

	if cfg.Enabled {
		activate(cfg)
	} else {
		if guard != nil {
			guard.Release()
			guard = nil
		}
		if jiggler != nil {
			jiggler.Stop()
			jiggler = nil
		}
	}
	updateTray(cfg)
}

func switchMode() {
	mu.Lock()
	defer mu.Unlock()

	cfg := loadConfig()
	switch cfg.Mode {
	case ModeAPI:
		cfg.Mode = ModeMouse
	case ModeMouse:
		cfg.Mode = ModeAPI
	}
	cfg.Save()
	currentMode = cfg.Mode

	if cfg.Enabled {
		if guard != nil {
			guard.Release()
			guard = nil
		}
		if jiggler != nil {
			jiggler.Stop()
			jiggler = nil
		}
		activate(cfg)
	}
	updateTray(cfg)
}

func activate(cfg Config) bool {
	switch cfg.Mode {
	case ModeAPI:
		if guard != nil {
			guard.Release()
		}
		g := AcquireInhibit()
		if g == nil {
			log.Println(Tr("Failed to acquire sleep inhibition", "获取睡眠抑制失败"))
			return false
		}
		guard = g
		return true
	case ModeMouse:
		if jiggler != nil {
			jiggler.Stop()
		}
		j := StartMouseJiggler(cfg.IntervalSecs)
		jiggler = &j
		return true
	}
	return false
}
