package metadata

import (
	"io/ioutil"
	"testing"
)

func Test1(t *testing.T) {
	task, err := ioutil.ReadFile("../../../data/part-1-initial/prob-002.desc")
	if err != nil {
		t.Fatal(err)
	}
	md, err := GetTaskMetadata(string(task))
	if err != nil {
		t.Fatal(err)
	}
	if md != (TaskMetadata{
		MaxX:     42,
		MaxY:     43,
		Boosters: "XLFFFBB",
	}) {
		t.Fatalf("actual: %v", md)
	}
}
func Test2(t *testing.T) {
	task, err := ioutil.ReadFile("../../../data/part-3-clones/prob-300.desc")
	if err != nil {
		t.Fatal(err)
	}
	md, err := GetTaskMetadata(string(task))
	if err != nil {
		t.Fatal(err)
	}
	if md != (TaskMetadata{
		MaxX:     390,
		MaxY:     399,
		Boosters: "XXXXXXCCCCCRRRLLLLLLLLLLLFFFFFFFFFFFFFFFFFFBBBBBBBBBBBBBBB",
	}) {
		t.Fatalf("%v", md)
	}
}
