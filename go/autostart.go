package main

import (
	"os"
	"os/exec"
	"path/filepath"
)

func enableAutostart() {
	exe, err := os.Executable()
	if err != nil {
		return
	}
	enableAutostartForOS(exe)
}

func disableAutostart() {
	disableAutostartForOS()
}

func enableAutostartForOS(exe string) {
	switch {
	case isWindows():
		exec.Command("reg", "add",
			"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
			"/v", "keep-awake",
			"/t", "REG_SZ",
			"/d", exe,
			"/f",
		).Run()
	case isMacOS():
		plist := `<?xml version="1.0"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>Label</key><string>com.keep-awake</string>
<key>ProgramArguments</key><array><string>` + exe + `</string></array>
<key>RunAtLoad</key><true/>
</dict></plist>`
		home, _ := os.UserHomeDir()
		path := filepath.Join(home, "Library", "LaunchAgents", "com.keep-awake.plist")
		os.WriteFile(path, []byte(plist), 0644)
	case isLinux():
		desktop := `[Desktop Entry]
Type=Application
Name=Keep Awake
Exec=` + exe + `
X-GNOME-Autostart-enabled=true
`
		config, _ := os.UserConfigDir()
		path := filepath.Join(config, "autostart", "keep-awake.desktop")
		os.MkdirAll(filepath.Dir(path), 0755)
		os.WriteFile(path, []byte(desktop), 0644)
	}
}

func disableAutostartForOS() {
	switch {
	case isWindows():
		exec.Command("reg", "delete",
			"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
			"/v", "keep-awake",
			"/f",
		).Run()
	case isMacOS():
		home, _ := os.UserHomeDir()
		os.Remove(filepath.Join(home, "Library", "LaunchAgents", "com.keep-awake.plist"))
	case isLinux():
		config, _ := os.UserConfigDir()
		os.Remove(filepath.Join(config, "autostart", "keep-awake.desktop"))
	}
}
