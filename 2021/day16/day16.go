package day16

import (
	"AoC2021/common"
	"fmt"
	"io"
)

const (
	Sum     byte = 0
	Product byte = 1
	Minimum byte = 2
	Maximum byte = 3
	Literal byte = 4
	Greater byte = 5
	Less    byte = 6
	Equal   byte = 7
)

type Packet interface {
	Version() byte
	Kind() byte
}

type PacketHeader struct {
	version byte
	kind    byte
}

type LiteralPacket struct {
	PacketHeader
	value int64
}

func (p LiteralPacket) Version() byte {
	return p.version
}
func (p LiteralPacket) Kind() byte {
	return p.kind
}

type OperatorPacket struct {
	PacketHeader
	lengthType bool
	length     int16
	packets    []Packet
}

func (p OperatorPacket) Version() byte {
	return p.version
}
func (p OperatorPacket) Kind() byte {
	return p.kind
}

type ParsedInput struct {
	input <-chan bool
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}

	hexmap := make(map[rune]byte)
	hexmap['0'] = 0
	hexmap['1'] = 1
	hexmap['2'] = 2
	hexmap['3'] = 3
	hexmap['4'] = 4
	hexmap['5'] = 5
	hexmap['6'] = 6
	hexmap['7'] = 7
	hexmap['8'] = 8
	hexmap['9'] = 9
	hexmap['A'] = 10
	hexmap['B'] = 11
	hexmap['C'] = 12
	hexmap['D'] = 13
	hexmap['E'] = 14
	hexmap['F'] = 15

	ch := make(chan bool)
	go func() {
		for _, r := range s[0] {
			nibble := hexmap[r]
			for i := 0; i < 4; i++ {
				ch <- (nibble & 0x8) == 0x8
				nibble <<= 1
			}
		}
		close(ch)
	}()
	result.input = ch

	return result, nil
}

func readBit(stream <-chan bool) byte {
	var bit byte = 0
	if <-stream {
		bit = 1
	}
	return bit
}
func readBits(stream <-chan bool, n int) int64 {
	var result int64 = 0
	for ; n > 0; n-- {
		result <<= 1
		if <-stream {
			result |= 1
		}
	}
	return result
}

func streamBits(stream <-chan bool, n int) <-chan bool {
	result := make(chan bool)
	go func() {
		for i := 0; i < n; i++ {
			b, ok := <-stream
			if !ok {
				panic("Stream ran out early")
			}
			result <- b
		}
		close(result)
	}()
	return result
}

func readVersion(stream <-chan bool) (byte, error) {
	var version byte = 0
	for i := 0; i < 3; i++ {
		version <<= 1
		bit, ok := <-stream
		if !ok {
			return 0, fmt.Errorf("end of channel")
		}
		if bit {
			version |= 1
		}
	}
	return version, nil
}

func readKind(stream <-chan bool) byte {
	var kind byte = 0
	for i := 0; i < 3; i++ {
		kind <<= 1
		bit := <-stream
		if bit {
			kind |= 1
		}
	}
	return kind
}

func parseLiteral(ch <-chan bool, ph PacketHeader) LiteralPacket {
	var value int64 = 0
	debug := ""
	for nibbles := 0; ; nibbles++ {
		if nibbles >= 16 {
			fmt.Printf("%s %d %d\n", debug, nibbles, value)
			panic("Too large a literal!")
		}
		value <<= 4
		nibble := readBits(ch, 5)
		dnibble := nibble
		for i := 0; i < 5; i++ {
			if dnibble&0x10 == 0 {
				debug += "0"
			} else {
				debug += "1"
			}
			dnibble <<= 1
		}
		if nibble&0x10 == 0 {
			// last nibble
			value |= nibble
			break
		} else {
			// prefix nibble
			value |= nibble & 0xF
		}
	}
	return LiteralPacket{ph, value}
}

func parseOperator(ch <-chan bool, ph PacketHeader) OperatorPacket {
	lengthType := readBit(ch) == 1
	result := OperatorPacket{ph, lengthType, 0, nil}
	if lengthType {
		result.length = int16(readBits(ch, 11))
		for i := int16(0); i < result.length; i++ {
			packet, err := parsePacket(ch)
			if err != nil {
				panic(err)
			}
			result.packets = append(result.packets, packet)
		}
	} else {
		result.length = int16(readBits(ch, 15))
		subch := streamBits(ch, int(result.length))
		for {
			packet, err := parsePacket(subch)
			if err != nil {
				break
			}
			result.packets = append(result.packets, packet)
		}
	}

	return result
}

func parsePacket(ch <-chan bool) (Packet, error) {
	ph := PacketHeader{}

	ver, err := readVersion(ch)
	if err != nil {
		return nil, err
	}
	ph.version = ver
	ph.kind = readKind(ch)
	if ph.kind == Literal {
		p := parseLiteral(ch, ph)
		//fmt.Println(p)
		return p, nil
	} else {
		p := parseOperator(ch, ph)
		//fmt.Println(p)
		return p, nil

	}
}

func visitHeaders(f func(Packet), p Packet) {
	f(p)
	switch p.Kind() {
	case Literal:
		return
	default:
		op := p.(OperatorPacket)
		for _, cp := range op.packets {
			visitHeaders(f, cp)
		}
	}
}

func eval(p Packet) int64 {
	kind := p.Kind()
	if kind == Literal {
		return p.(LiteralPacket).value
	}
	op := p.(OperatorPacket)
	switch p.Kind() {
	case Sum:
		sum := int64(0)
		for _, cp := range op.packets {
			sum += eval(cp)
		}
		return sum
	case Product:
		prod := int64(1)
		for _, cp := range op.packets {
			prod *= eval(cp)
		}
		return prod
	case Minimum:
		min := int64(1) << 62
		for _, cp := range op.packets {
			v := eval(cp)
			if v < min {
				min = v
			}
		}
		return min
	case Maximum:
		max := -(int64(1) << 62)
		for _, cp := range op.packets {
			v := eval(cp)
			if v > max {
				max = v
			}
		}
		return max
	case Greater:
		v0 := eval(op.packets[0])
		v1 := eval(op.packets[1])
		if v0 > v1 {
			return 1
		} else {
			return 0
		}
	case Less:
		v0 := eval(op.packets[0])
		v1 := eval(op.packets[1])
		if v0 < v1 {
			return 1
		} else {
			return 0
		}
	case Equal:
		v0 := eval(op.packets[0])
		v1 := eval(op.packets[1])
		if v0 == v1 {
			return 1
		} else {
			return 0
		}
	default:
		panic("Invalid operator")

	}
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	packet, err := parsePacket(pi.input)
	if err != nil {
		return 0, err
	}

	versionSum := 0
	visitHeaders(func(p Packet) { versionSum += int(p.Version()) }, packet)

	return versionSum, nil
}

func Part2(r io.Reader) (int64, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	p, err := parsePacket(pi.input)
	if err != nil {
		return 0, err
	}
	v := eval(p)

	return v, nil
}
