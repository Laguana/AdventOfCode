package day0

import (
	"fmt"
)

func GetWhoToHello() string {
	return "world"
}

func main() {
	fmt.Printf("Hello %s!\n", GetWhoToHello())
}
