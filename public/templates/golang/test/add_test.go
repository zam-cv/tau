package test

import (
    "testing"
    "github.com/yourusername/yourprojectname/pkg/mathops"
)

func TestAdd(t *testing.T) {
    got := mathops.Add(1, 2)
    want := 3

    if got != want {
        t.Errorf("Add(1, 2) = %d; want %d", got, want)
    }
}