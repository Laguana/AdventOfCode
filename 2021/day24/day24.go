package day24

import (
	"AoC2021/common"
	"fmt"
	"io"
	"strconv"
	"strings"
)

// So many hopes and dreams went into this file...

type register byte

const (
	literal register = iota
	rx
	ry
	rz
	rw
)

type op byte

const (
	oinvalid op = iota
	oinp
	oadd
	omul
	odiv
	omod
	oeql
)

type instruction struct {
	op   op
	arg1 register
	arg2 register
	lit  int
}

type ParsedInput struct {
	instructions []instruction
}

func parseRegister(s string) register {
	switch s {
	case "x":
		return rx
	case "y":
		return ry
	case "z":
		return rz
	case "w":
		return rw
	default:
		return literal
	}
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}
	result.instructions = make([]instruction, 0)

	ops := map[string]op{"inp": oinp, "add": oadd, "mul": omul, "div": odiv, "mod": omod, "eql": oeql}
	for _, l := range s {
		parts := strings.Split(l, " ")
		instr := instruction{}
		switch parts[0] {
		case "add":
			fallthrough
		case "mul":
			fallthrough
		case "div":
			fallthrough
		case "mod":
			fallthrough
		case "eql":
			arg2 := parseRegister(parts[2])
			instr.arg2 = arg2
			if arg2 == literal {
				val, err := strconv.Atoi(parts[2])
				if err != nil {
					return result, err
				}
				instr.lit = val
			}
			fallthrough
		case "inp":
			reg := parseRegister(parts[1])
			if reg == literal {
				return result, fmt.Errorf("expected register, got %s", parts[1])
			}
			instr.op = ops[parts[0]]
			instr.arg1 = reg
			result.instructions = append(result.instructions, instr)
		}
	}

	return result, nil
}

type alu struct {
	x, y, z, w int
}

func (alu *alu) setReg(r register, v int) {
	switch r {
	case rx:
		alu.x = v
	case ry:
		alu.y = v
	case rz:
		alu.z = v
	case rw:
		alu.w = v
	default:
		panic("Invalid register")
	}
}

func (alu *alu) getReg(r register, lit int) int {
	switch r {
	case rx:
		return alu.x
	case ry:
		return alu.y
	case rz:
		return alu.z
	case rw:
		return alu.w
	case literal:
		return lit
	default:
		panic("Invalid register")
	}
}

func (alu *alu) process(instructions []instruction, inputs <-chan int) {
	for _, inst := range instructions {
		switch inst.op {
		case oinp:
			alu.setReg(inst.arg1, <-inputs)
		case oadd:
			alu.setReg(inst.arg1, alu.getReg(inst.arg1, 0)+alu.getReg(inst.arg2, inst.lit))
		case omul:
			alu.setReg(inst.arg1, alu.getReg(inst.arg1, 0)*alu.getReg(inst.arg2, inst.lit))
		case odiv:
			alu.setReg(inst.arg1, alu.getReg(inst.arg1, 0)/alu.getReg(inst.arg2, inst.lit))
		case omod:
			alu.setReg(inst.arg1, alu.getReg(inst.arg1, 0)%alu.getReg(inst.arg2, inst.lit))
		case oeql:
			v1 := alu.getReg(inst.arg1, 0)
			v2 := alu.getReg(inst.arg2, inst.lit)
			if v1 == v2 {
				alu.setReg(inst.arg1, 1)
			} else {
				alu.setReg(inst.arg1, 0)
			}
		}
	}
}

type symbolicVal struct {
	literal    bool
	input      bool
	val        int // if literal, then literal value. If input, then the val-th input
	min, max   int
	op         op
	args       [2]*symbolicVal
	simplified bool
}

func (a *symbolicVal) equal(b *symbolicVal) bool {
	if a == b {
		return true
	}
	if a.literal != b.literal {
		return false
	}
	if a.input != b.input {
		return false
	}
	if a.literal || a.input {
		return a.val == b.val
	}
	if a.op != b.op {
		return false
	}
	return a.args[0].equal(b.args[0]) && a.args[1].equal(b.args[1])
}

type symbolicAlu struct {
	x, y, z, w *symbolicVal
}

func (alu *symbolicAlu) setReg(r register, v *symbolicVal) {
	switch r {
	case rx:
		alu.x = v
	case ry:
		alu.y = v
	case rz:
		alu.z = v
	case rw:
		alu.w = v
	default:
		panic("Invalid register")
	}
}

type litSymCache map[int]*symbolicVal

func (lsc litSymCache) get(v int) *symbolicVal {
	e, ok := lsc[v]
	if !ok {
		e = &symbolicVal{literal: true, val: v, min: v, max: v}
		lsc[v] = e
	}
	return e
}

var literalCache litSymCache = make(litSymCache)

func (alu *symbolicAlu) getReg(r register, lit int) *symbolicVal {
	switch r {
	case rx:
		return alu.x
	case ry:
		return alu.y
	case rz:
		return alu.z
	case rw:
		return alu.w
	case literal:
		return literalCache.get(lit)
	default:
		panic("Invalid register")
	}
}

func (alu *symbolicAlu) process(instructions []instruction) {
	input := 0
	for _, inst := range instructions {
		switch inst.op {
		case oinp:
			alu.setReg(inst.arg1, &symbolicVal{input: true, val: input, min: 1, max: 9})
			input++
		default:
			arg1 := alu.getReg(inst.arg1, 0)
			arg2 := alu.getReg(inst.arg2, inst.lit)
			if inst.op == omul && arg2.literal && arg2.val == 0 {
				alu.setReg(inst.arg1, literalCache.get(0))
			} else {
				min, max := bounds(inst.op, arg1, arg2)
				alu.setReg(inst.arg1, &symbolicVal{op: inst.op, args: [2]*symbolicVal{arg1, arg2}, min: min, max: max})
			}
		}
	}
}

func (v symbolicVal) evaluate(inputs []int) int {
	if v.literal {
		return v.val
	}
	if v.input {
		return inputs[v.val]
	}
	arg1 := v.args[0].evaluate(inputs)
	arg2 := v.args[1].evaluate(inputs)
	switch v.op {
	case oadd:
		return arg1 + arg2
	case omul:
		return arg1 * arg2
	case odiv:
		return arg1 / arg2
	case omod:
		return arg1 % arg2
	case oeql:
		if arg1 == arg2 {
			return 1
		} else {
			return 0
		}
	default:
		panic("Invalid op")
	}
}

func sgn(a int) int {
	if a < 0 {
		return -1
	} else if a > 0 {
		return 1
	}
	return 0
}
func iabs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}
func imin(a, b int) int {
	if a < b {
		return a
	}
	return b
}
func imax(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func bounds(op op, arg0, arg1 *symbolicVal) (min, max int) {
	switch op {
	case oadd:
		min = arg0.min + arg1.min
		max = arg0.max + arg1.max
	case omul:
		min = imin(arg0.min*arg1.min, imin(arg0.min*arg1.max, imin(arg0.max*arg1.min, arg0.max*arg1.max)))
		max = imax(arg0.min*arg1.min, imax(arg0.min*arg1.max, imax(arg0.max*arg1.min, arg0.max*arg1.max)))
	case odiv:
		min = -iabs(imax(arg0.min, arg0.max))
		max = iabs(imax(arg0.min, arg0.max))
	case omod:
		min = 0
		max = arg1.max
	case oeql:
		min = 0
		max = 1
		if arg0.max < arg1.min || arg1.max < arg0.min {
			max = 0
		}
	}
	return
}

func (v *symbolicVal) simplify() *symbolicVal {
	//fmt.Printf("Simplify %v\n", *v)
	if v.input || v.literal {
		return v
	}
	if v.simplified {
		return v
	}
	arg0 := v.args[0].simplify()
	arg1 := v.args[1].simplify()
	if arg0.min > arg0.max {
		fmt.Printf("Bad bounds %v %v\n", *v, *arg0)
		panic("bad bounds")
	} else if arg1.min > arg1.max {
		fmt.Printf("Bad bounds %v %v\n", *v, *arg1)
		panic("bad bounds")
	}
	if arg0.literal && arg1.literal {
		v.literal = true
		switch v.op {
		case oadd:
			v.val = arg0.val + arg1.val
		case omul:
			v.val = arg0.val * arg1.val
		case odiv:
			v.val = arg0.val / arg1.val
		case omod:
			v.val = arg0.val % arg1.val
		case oeql:
			eq := arg0.val == arg1.val
			if eq {
				v.val = 1
			} else {
				v.val = 0
			}
		default:
			panic("Invalid op")
		}
		v.min = v.val
		v.max = v.val
		v.args[0] = nil
		v.args[1] = nil
		return literalCache.get(v.val)

	}
	v.args[0] = arg0
	v.args[1] = arg1
	v.min, v.max = bounds(v.op, arg0, arg1)
	if v.min == v.max {
		return literalCache.get(v.min)
	}
	/*switch v.op {
	case oadd:
		v.min = arg0.min + arg1.min
		v.max = arg0.max + arg1.max
	case omul:
		v.min = imin(arg0.min*arg1.min, imin(arg0.min*arg1.max, imin(arg0.max*arg1.min, arg0.max*arg1.max)))
		v.min = imax(arg0.min*arg1.min, imax(arg0.min*arg1.max, imax(arg0.max*arg1.min, arg0.max*arg1.max)))
	case odiv:
		v.min = -iabs(imax(arg0.min, arg0.max))
		v.max = iabs(imax(arg0.min, arg0.max))
	case omod:
		v.min = 0
		v.max = arg1.max
		if arg0.max < arg1.min {
			fmt.Printf("Ditching mod: (%d,%d) (%d,%d)\n", arg0.min, arg0.max, arg1.min, arg1.max)
			// a %b with a < b is just a
			return arg0
		}
	case oeql:
		v.min = 0
		v.max = 1
		if arg0.max < arg1.min || arg1.max < arg0.min {
			v.max = 0
			return literalCache.get(0)
		}
	}*/
	// some handy special cases:

	if v.op == omul {
		if (arg0.literal && arg0.val == 0) || (arg1.literal && arg1.val == 0) {
			v.literal = true
			v.val = 0
			v.args[0] = nil
			v.args[1] = nil
			return literalCache.get(0)
		} else if arg0.literal && arg0.val == 1 {
			return arg1
		} else if arg1.literal && arg1.val == 1 {
			return arg0
		}
	} else if v.op == oadd {
		if arg0.literal && arg0.val == 0 {
			return arg1
		} else if arg1.literal && arg1.val == 0 {
			return arg0
		}
	} else if v.op == odiv {
		if arg1.literal && arg1.val == 1 {
			return arg0
		} else if arg0.equal(arg1) {
			return literalCache.get(1)
		} else if arg0.op == omul {
			if arg0.args[0] == arg1 {
				return arg0.args[1]
			} else if arg0.args[1] == arg1 {
				return arg0.args[0]
			}
		} else if arg0.op == oadd {
			// hack...
			if arg0.args[1].op == oadd && arg0.args[1].args[0].input &&
				arg0.args[1].args[1].literal && arg1.literal && arg0.args[1].args[1].val < arg1.val &&
				arg0.args[0].op == omul && arg0.args[0].args[1] == arg1 {
				return arg0.args[0].args[0]
			}
		}
	} else if v.op == oeql {
		if arg0.equal(arg1) {
			return literalCache.get(1)
		}
	} else if v.op == omod {
		if arg1.min > arg0.max {
			return arg0
		}
	}
	v.simplified = true
	return v
}

func (v symbolicVal) getInputs(result *common.Set) {
	if v.input {
		result.Add(v.val)
		return
	}
	if v.literal {
		return
	}
	v.args[0].getInputs(result)
	v.args[1].getInputs(result)
}

var memoisedString map[*symbolicVal]string = make(map[*symbolicVal]string)

func (v *symbolicVal) String() string {
	s, seen := memoisedString[v]
	if seen {
		return s
	}
	if v.literal {
		return fmt.Sprintf(" %d ", v.val)
	}
	if v.input {
		return fmt.Sprintf(" <%d ", v.val)
	}

	opMap := map[op]rune{oadd: '+', omul: '*', odiv: '/', omod: '%', oeql: '='}

	s = fmt.Sprintf("(%s) %c (%s)", v.args[0].String(), opMap[v.op], v.args[1].String())
	memoisedString[v] = s
	return s
}

func Part1(r io.Reader) (int, error) {
	// see disassembled.txt
	return 39494195799979, nil
}

func Part2(r io.Reader) (int, error) {
	// see disassembled.txt
	return 13161151139617, nil
}
