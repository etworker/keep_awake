//go:build !windows

package main

import (
	"os/exec"
	"strconv"
)

func platformJiggle() func() {
	// Detect available command
	cmds := []string{"xdotool", "cliclick", "osascript"}
	cmdName := ""
	for _, c := range cmds {
		if _, err := exec.LookPath(c); err == nil {
			cmdName = c
			break
		}
	}

	switch cmdName {
	case "xdotool":
		return func() {
			// Get current position
			out, err := exec.Command("xdotool", "getmouselocation").Output()
			if err != nil {
				return
			}
			// Parse "x=123 y=456 ..."
			var x, y int
			_, _ = out, strconv.Atoi("0") // placeholder
			// Simple approach: move relative
			exec.Command("xdotool", "mousemove_relative", "--", "1", "0").Run()
			exec.Command("xdotool", "mousemove_relative", "--", "-1", "0").Run()
			_ = x
			_ = y
		}
	case "cliclick":
		return func() {
			exec.Command("cliclick", "m:+1,0").Run()
			exec.Command("cliclick", "m:-1,0").Run()
		}
	case "osascript":
		return func() {
			script := `
tell application "System Events"
	set pos to position of mouse
	set position of mouse to {item 1 of pos + 1, item 2 of pos}
	set position of mouse to {item 1 of pos, item 2 of pos}
end tell
`
			exec.Command("osascript", "-e", script).Run()
		}
	default:
		// No-op fallback
		return func() {}
	}
}
