//go:build windows

package main

import (
	"syscall"
	"unsafe"
)

const (
	nifInfo  = 0x00000010
	niModify = 0x00000001
)

type notifyIconDataW struct {
	cbSize           uint32
	hWnd             uintptr
	uID              uint32
	uFlags           uint32
	uCallbackMessage uint32
	hIcon            uintptr
	szTip            [128]uint16
	dwState          uint32
	dwStateMask      uint32
	szInfo           [256]uint16
	uVersion         uint32
	szInfoTitle      [64]uint16
	dwInfoFlags      uint32
	guidItem         [16]byte
	hBalloonIcon     uintptr
}

func showNotification(title, body string) {
	class, _ := syscall.UTF16PtrFromString("SystrayClass")
	hwnd, _, _ := procFindWindowW.Call(uintptr(unsafe.Pointer(class)), 0)
	if hwnd == 0 {
		return
	}

	var nid notifyIconDataW
	nid.cbSize = uint32(unsafe.Sizeof(nid))
	nid.hWnd = hwnd
	nid.uID = 100
	nid.uFlags = nifInfo

	for i, c := range utf16FromString(title) {
		if i >= 63 {
			break
		}
		nid.szInfoTitle[i] = c
	}
	for i, c := range utf16FromString(body) {
		if i >= 255 {
			break
		}
		nid.szInfo[i] = c
	}

	procShellNotifyIconW.Call(niModify, uintptr(unsafe.Pointer(&nid)))
}

func utf16FromString(s string) []uint16 {
	u, _ := syscall.UTF16FromString(s)
	return u
}
