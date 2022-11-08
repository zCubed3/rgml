#include <prism_math.h>

#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(int argc, char** argv) {
	vec3_t v3 = vec3_new(0.0f, 0.0f, 0.0f);

	printf("V3: <%f, %f, %f>\n", v3.backing[0], v3.backing[1], v3.backing[2]);

	return 0;
}