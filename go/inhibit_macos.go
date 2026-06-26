//go:build darwin

package main

/*
#cgo LDFLAGS: -framework CoreFoundation -framework IOKit
#include <IOKit/pwr_mgt/IOPMLib.h>
#include <CoreFoundation/CoreFoundation.h>

static IOPMAssertionID _assertion = 0;

int acquire(char *reason) {
    CFStringRef r = CFStringCreateWithCString(kCFAllocatorDefault, reason, kCFStringEncodingUTF8);
    if (!r) return -1;
    IOReturn ret = IOPMAssertionCreateWithName(
        kIOPMAssertionTypeNoDisplaySleep,
        kIOPMAssertionLevelOn,
        r,
        &_assertion);
    CFRelease(r);
    return (int)ret;
}

void release() {
    if (_assertion) {
        IOPMAssertionRelease(_assertion);
        _assertion = 0;
    }
}
*/
import "C"
import "unsafe"

type macOSGuard struct{}

func acquireInhibit() InhibitGuard {
	reason := C.CString("Keep Awake")
	defer C.free(unsafe.Pointer(reason))
	if ret := C.acquire(reason); ret != 0 {
		return nil
	}
	return &macOSGuard{}
}

func (g *macOSGuard) Release() {
	C.release()
}
