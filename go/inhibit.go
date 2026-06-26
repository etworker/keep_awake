package main

// InhibitGuard is a platform-specific sleep inhibition handle.
type InhibitGuard interface {
	Release()
}

func AcquireInhibit() InhibitGuard {
	return acquireInhibit()
}
