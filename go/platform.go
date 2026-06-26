package main

import "runtime"

func isWindows() bool {
	return runtime.GOOS == "windows"
}

func isMacOS() bool {
	return runtime.GOOS == "darwin"
}

func isLinux() bool {
	return runtime.GOOS == "linux"
}
