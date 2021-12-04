package common

import (
	"strings"
	"testing"
)

func TestReadLinesToSliceBasic(t *testing.T) {
	r := strings.NewReader("1\n2\n3\n")
	result := ReadLinesToSlice(r)
	if len(result) != 3 {
		t.Errorf("Expected 3 lines, got %d", len(result))
	}
}

func TestReadLinesToSliceWithNewlines(t *testing.T) {
	r := strings.NewReader("1\n2\n\n3\n")
	result := ReadLinesToSlice(r)
	if len(result) != 4 {
		t.Errorf("Expected 4 lines, got %d", len(result))
	}
}
