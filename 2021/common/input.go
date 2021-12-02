package common

import (
	"bufio"
	"io"
)

func ReadLinesToSlice(r io.Reader) []string {
	inputs := make([]string, 0)
	scanner := bufio.NewScanner(r)

	for {
		scanner.Scan()
		text := scanner.Text()
		if len(text) == 0 {
			break
		}

		inputs = append(inputs, text)
	}

	return inputs
}
