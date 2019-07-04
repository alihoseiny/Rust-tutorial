#include <stdio.h>
struct Course {
	char *name;
	short int passed;
	void (*pass) (struct Course * c);
};

void pass(struct Course *c) {
	c->passed = 1;
}

int main() {
	struct Course course;
	course.name = "We can not use Persian Letters simply";
	course.passed = 0;
	course.pass = pass;	// Assigning pass function to pass value of the struct
	course.pass(&course);	// We should send instance of the struct to the function expressly
	if (course.passed) {
		printf("Worked. But we can not print the whole struct simply like what we did in rust.");
	} else {
		printf("Didn't work");
	}
	return 0;
}
