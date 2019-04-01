#include "stdio.h"

/*
* This function gets userLevel integer and prints proper message according to the input value.
*/
void printMessage(int userLevel) {
	// Checking userLevel parameter for different probabilities
	switch (userLevel) {	
		case 0:		// Is userLevel value equals to 0?
			printf("Welcome dear admin.\n");	// userLevel is zero, so user is an admin.
			break;	// We found what we wanted, so we breaking decision structure.
		case 1:		// Is userLevel value equals to 1?
			printf("Welcome back our best member.\n");	// userLevel is one, so user is a member. 
			break;	// We found what we wanted, so we breaking decision structure.
		case 2:		// Is userLevel value equals to 2?
			printf("Hello. Please register first.\n");	// userLevel is two, so user is a guest. 
			break;	// We found what we wanted, so we breaking decision structure.
	}
}

/*
* This function gets an integer from user and returns it.
*/
int getUserInput() {
	int userValue;
	printf("Please enter your level. enter -1 for exit.\n");
	scanf("%d", &userValue);
	return userValue;
}

int main() {
	int userInput = getUserInput();	// Getting input from user for the first time.
	while (userInput != -1) {	// While user input is not equal to the -1, the program will repeat.
		printMessage(userInput);	// Showing proper message for user according to his/hers type.
		userInput = getUserInput();	// Getting new input from the user.
	}
	return 0;
}
