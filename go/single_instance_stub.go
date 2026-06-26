//go:build !windows

package main

func ensureSingleInstance() bool {
	// TODO: implement for macOS/Linux (e.g., lock file)
	return true
}
