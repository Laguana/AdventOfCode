package day16

import (
	"os"
	"strings"
	"testing"
)

var sampleInput []string = strings.Split("8A004A801A8002F478", "\n")

func TestDay16Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
	packet, err := parsePacket(pi.input)
	if err != nil {
		t.Error(err)
	}
	if packet.Version() != 4 {
		t.Errorf("Should be version 4, got %v", packet.Version())
	}
	if packet.Kind() == Literal {
		t.Errorf("Should not be a literal, got %v", packet.Kind())
	}

	versionSum := 0
	visitHeaders(func(p Packet) { versionSum += int(p.Version()) }, packet)

	if versionSum != 16 {
		t.Errorf("Should have summed to 16, got %d", versionSum)
	}

}

func TestDay16Part1(t *testing.T) {
	d16i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d16i.Close()
	result, err := Part1(d16i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 906
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay16Part2Sample(t *testing.T) {
	pi, err := parseInput(strings.Split("9C0141080250320F1802104A08", "\n"))
	if err != nil {
		t.Error(err)
	}

	p, err := parsePacket(pi.input)
	if err != nil {
		t.Error(err)
	}
	v := eval(p)
	if v != 1 {
		t.Errorf("Expected 1, got %d", v)
	}
}

func TestDay16Part2(t *testing.T) {
	d16i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d16i.Close()
	result, err := Part2(d16i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := int64(819324480368)
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
