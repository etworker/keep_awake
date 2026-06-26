package main

import (
	"github.com/getlantern/systray"
)

var (
	toggleItem    *systray.MenuItem
	modeItem      *systray.MenuItem
	autostartItem *systray.MenuItem
	quitItem      *systray.MenuItem
)

func setupTray(cfg Config) {
	iconBytes := sunIconICO(cfg.Enabled)
	systray.SetTemplateIcon(iconBytes, iconBytes)
	systray.SetTooltip(tooltip(cfg.Enabled))

	toggleItem = systray.AddMenuItemCheckbox(MenuToggle(), "", cfg.Enabled)
	systray.AddSeparator()

	modeItem = systray.AddMenuItem(modeLabel(cfg.Mode), "")
	systray.AddSeparator()

	autostartItem = systray.AddMenuItemCheckbox(MenuAutostart(), "", cfg.Autostart)
	systray.AddSeparator()

	quitItem = systray.AddMenuItem(MenuQuit(), "")

	go func() {
		for {
			select {
			case <-toggleItem.ClickedCh:
				toggleEnabled()
			case <-modeItem.ClickedCh:
				switchMode()
			case <-autostartItem.ClickedCh:
				toggleAutostart()
			case <-quitItem.ClickedCh:
				systray.Quit()
			}
		}
	}()
}

func updateTray(cfg Config) {
	setChecked(toggleItem, cfg.Enabled)
	modeItem.SetTitle(modeLabel(cfg.Mode))
	systray.SetTooltip(tooltip(cfg.Enabled))
	iconBytes := sunIconICO(cfg.Enabled)
	systray.SetTemplateIcon(iconBytes, iconBytes)
}

func toggleAutostart() {
	cfg := loadConfig()
	cfg.Autostart = !cfg.Autostart
	cfg.Save()
	setChecked(autostartItem, cfg.Autostart)
	if cfg.Autostart {
		enableAutostart()
	} else {
		disableAutostart()
	}
}

func setChecked(item *systray.MenuItem, checked bool) {
	if checked {
		item.Check()
	} else {
		item.Uncheck()
	}
}

func tooltip(enabled bool) string {
	if enabled {
		return Tr(TitleActive, TitleActiveZh)
	}
	return Tr(TitleStopped, TitleStoppedZh)
}

func modeLabel(m Mode) string {
	switch m {
	case ModeAPI:
		return MenuSwitchMouse()
	case ModeMouse:
		return MenuSwitchAPI()
	}
	return ""
}
