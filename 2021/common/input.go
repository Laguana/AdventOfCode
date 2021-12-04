package common

import (
	"bufio"
	"io"
)

func ReadLinesToSlice(r io.Reader) []string {
	inputs := make([]string, 0)
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		text := scanner.Text()
		inputs = append(inputs, text)
	}

	return inputs
}
