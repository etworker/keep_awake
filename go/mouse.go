package main

import (
	"sync/atomic"
	"time"
)

// MouseJiggler periodically moves the mouse slightly.
type MouseJiggler struct {
	stop chan struct{}
}

// StartMouseJiggler creates and starts a mouse jiggler goroutine.
// Uses platform-specific code (not robotgo) to avoid heavy dependencies.
func StartMouseJiggler(intervalSecs int) MouseJiggler {
	j := MouseJiggler{stop: make(chan struct{})}
	var paused int32

	go func() {
		// Helper that moves mouse by +/-1 px
		jiggle := jiggleFunc()

		for {
			select {
			case <-j.stop:
				return
			default:
				if atomic.LoadInt32(&paused) == 0 {
					jiggle()
				}
				time.Sleep(time.Duration(intervalSecs) * time.Second)
			}
		}
	}()
	return j
}

func (j *MouseJiggler) Stop() {
	close(j.stop)
}

// jiggleFunc returns a platform-specific jiggle function.
func jiggleFunc() func() {
	return platformJiggle()
}
