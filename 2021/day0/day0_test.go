package main

import "testing"

func TestGetWhoToHello(t *testing.T) {
	if GetWhoToHello() != "world" {
		t.Errorf("GetWhoToHello returned %s not world", GetWhoToHello())
	}
}
