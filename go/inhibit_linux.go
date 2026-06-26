//go:build linux

package main

import (
	"os/exec"
	"strings"
)

type linuxGuard struct {
	cookie string
}

func acquireInhibit() InhibitGuard {
	// Try loginctl first (systemd-based)
	if out, err := exec.Command("loginctl", "lock-session").Output(); err == nil {
		// Actually, we want inhibit, not lock
	}

	// Try dbus-send for freedesktop inhibition
	app := "keep-awake"
	reason := "Keep Awake"
	cmd := exec.Command("dbus-send",
		"--session",
		"--dest=org.freedesktop.ScreenSaver",
		"--type=method_call",
		"--print-reply=literal",
		"/org/freedesktop/ScreenSaver",
		"org.freedesktop.ScreenSaver.Inhibit",
		"string:"+app,
		"string:"+reason,
	)
	out, err := cmd.Output()
	if err != nil {
		return nil
	}
	cookie := strings.TrimSpace(string(out))
	return &linuxGuard{cookie: cookie}
}

func (g *linuxGuard) Release() {
	exec.Command("dbus-send",
		"--session",
		"--dest=org.freedesktop.ScreenSaver",
		"--type=method_call",
		"/org/freedesktop/ScreenSaver",
		"org.freedesktop.ScreenSaver.UnInhibit",
		"uint32:"+g.cookie,
	).Run()
}
