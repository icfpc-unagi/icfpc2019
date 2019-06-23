package metadata

import (
	"errors"
	"strconv"
	"strings"
	"unicode"
)

type TaskMetadata struct {
	MaxX     int64
	MaxY     int64
	Boosters string
}

func GetTaskMetadata(task string) (md TaskMetadata, err error) {
	sp := strings.Split(strings.TrimSpace(task), "#")
	if len(sp) != 4 {
		err = errors.New("bad format")
		return
	}
	m := sp[0]
	for _, p := range strings.Split(m[1:len(m)-1], "),(") {
		xy := strings.SplitN(p, ",", 2)
		if len(xy) != 2 {
			err = errors.New("bad format")
			return
		}
		var x int64
		if x, err = strconv.ParseInt(xy[0], 10, 64); err != nil {
			return
		}
		if md.MaxX < x {
			md.MaxX = x
		}
		var y int64
		if y, err = strconv.ParseInt(xy[1], 10, 64); err != nil {
			return
		}
		if md.MaxY < y {
			md.MaxY = y
		}
	}
	sb := &strings.Builder{}
	for _, c := range sp[3] {
		if unicode.IsUpper(c) {
			sb.WriteRune(c)
		}
	}
	md.Boosters = sb.String()
	return
}
