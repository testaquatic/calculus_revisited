package cal

import (
	"fmt"
	"math"
)

func ExampleLogByBisect() {
	fmt.Printf("Log(2) ~ %v \t (Bisect)\n", LogByBisect(2))
	fmt.Printf("Log(2) ~ %v \t (math.Log)\n", math.Log(2))

	// Output:
	// Log(2) ~ 0.6931477785110474 	 (Bisect)
	// Log(2) ~ 0.6931471805599453 	 (math.Log)
}
