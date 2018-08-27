//William Haugen
//Sudoku Solver
//See LICENSE for license information.
//
//
#include "sudoku.h"

//Outputs the current board state
//It is formatted to do the boxes
//ie. 9x9 would have 3 then a space to seperate to
//another box
void display(board * b) {
	for (int i= 0; i<SIZE; ++i) {
		for (int j = 0; j<SIZE; ++j) {
			printf("%d  ",b->b[i][j]);
			if ((j+1) %(int) sqrt((double) SIZE) == 0) {
				printf("   ");
			}
		}
		printf("\n");
		if((i+1) %(int) sqrt((double) SIZE) == 0) {
			printf("\n");
		}
	}
}

//Set all the values in the board to 0
//Then get user input
void initialize(board * b) {
	memset(b,0, SIZE*SIZE*sizeof(int));
	input(b);
}

//BUG: not set to do double digits
//Sets the board up, row by row.
//Only numbers, does not acount for whitespace.
void input(board * b) {
	//str needs to be null terminated
	char str[SIZE+1];
	//Grab a user's confirmation for that row
	char response[2];
	int c = 0;
	//initialize str to 0
	memset(str, 0, SIZE*sizeof(str[0]));
	//Do while the numbers put in where between
	//0 and SIZE && there are no duplicates
	do{
		//Go through each row
		for(int i = 0; i< SIZE; ++i) {
		  do {	
			printf("\nEnter for row %d: ", i);
			scanf("%s",str);
			str[SIZE] = '\0';
			printf("%s ", str);
			printf("\nIs this the correct input?[y/n] ");
			scanf("%s", response);
		  }while((response[0] != 'y' && response[0] != 'Y')
				  || strlen(str) != SIZE);
			for(int j = 0; j < SIZE; ++j) {
				c = (int) str[j] -48;
				b->b[i][j] = c;
			}
		}
	}
	while(!valid(b, true) && !dups(b));
	printf("\n");
}

//Get a row, starting at 0 to SIZE -1
//Does NOT check if greater than or equal to SIZE!!!
int* row(int row, board * b) {
	int *r = malloc(sizeof(int) *SIZE);
	for (int i = 0; i< SIZE; ++i) {
		r[i] = b->b[row][i];
	}
	return r;
}

//Much like row, gets col's from 0 to SIZE-1
//Again, does NOT check if out of bounds!!!
int* col(int col, board * b) {
	int *c = malloc(sizeof(int) *SIZE);
	for (int i = 0; i< SIZE; ++i) {
		c[i] = b->b[i][col];
	}
	return c;
}

//Get the boxe/square from 0 to SIZE-1
//ie for a 9x9 it would be as follows:
//0 1 2
//3 4 5
//6 7 8
int* square(int square, board * b) {
	//allocate a new array to be returned
	int *box = malloc(sizeof(int) *SIZE);
	//Get the sqroot, this is going to be the bounds for
	//the array
	int sqroot = (int) sqrt((double) SIZE);
	//Start of the box indexes
	int start_col = (square%sqroot)*sqroot;
	//Needs to be whole integer number, that's why it
	//does divide then multiply by sqroot
	int start_row = (square/sqroot) * sqroot;
	int index = 0;
	//Get the numbers from the box by the provided
	//start, up till the box width and height.
	for (int i = start_row; i<start_row+sqroot; ++i) {
		for (int j = start_col; j< start_col+sqroot; ++j) {
			box[index] = b->b[i][j];
			++index;
		}
	}
	return box;
}

//Checks for duplicates for all non-zero numbers.
//Creates an already array and just increments the
//index-1 for col, row, and boxes.
//If an index is greater than 1, there is a duplicate.
bool dups(board * b) {
	int already[SIZE];
	memset(already, 0, SIZE* sizeof(already[0]));
	int *arr = 0;
	bool dup = false;
	//Goes until all rows, cols, and boxes have been
	//checked or until there is a duplicate
	for (int i = 0; i < SIZE && !dup; ++i) {
		if(!dup) {
			//Check row
			arr = row(i, b);
			for (int j = 0; j < SIZE; ++j) {
				if (arr[j] > 0) {
					already[arr[j]-1] += 1;
				}
			}
			for (int j = 0; j < SIZE; ++j) {
				if (already[j] > 1) {
					dup = true;
				}
			}
			//Set already back to 0 in all indexes
			//free arr to get new array
			memset(already, 0, SIZE* sizeof(int));
			free(arr);

			//Check col
			arr = col(i, b);
			for (int j = 0; j < SIZE; ++j) {
				if (arr[j] > 0) {
					already[arr[j]-1] += 1;
				}
			}
			for (int j = 0; j < SIZE; ++j) {
				if (already[j] > 1) {
					dup = true;
				}
			}
			//Set already back to 0 in all indexes
			////free arr to get new array
			memset(already, 0, SIZE* sizeof(int));
			free(arr);

			//Check squares
			arr = square(i, b);
			for (int j = 0; j < SIZE; ++j) {
				if (arr[j] > 0) {
					already[arr[j]-1] += 1;
				}
			}
			for (int j = 0; j < SIZE; ++j) {
				if (already[j] > 1) {
					dup = true;
				}
			}
			//Set already to 0 in all indexes.
			memset(already, 0, SIZE* sizeof(int));
			//Free the arr for the next call.
			free(arr);
		}
	}

	//Return if there is a duplicate or not
	return dup;
}

//Checks to make sure that the numbers entered were valid
//ie. from 0(if initialize was set to true, otherwise it
//looks from 1) to SIZE. If something is lower or greater
//than those bounds, return false.
bool valid(board *b, bool initialize) {
	bool valid = true;

	//Checks all elements until there is a number out
	//of the bounds.
	for(int i = 0; i < SIZE && valid; ++i) {
		for(int j = 0; j < SIZE && valid; ++j) {
			//If the number is greater than the
			//SIZE, set valid to false
			if(b->b[i][j] > SIZE) {
				valid = false;
			}
			else {
				//If initializing the values
				//and less than 0, set valid to
				//false
				if (initialize 
				      && b->b[i][j] < 0) {
					valid = false;
				}
				else {
				  //If not initialize and value
				  //is less than 1, set valid to
				  //false
				  if (!initialize &&
				        b->b[i][j] < 1) {
					  valid = false;
				  }
				}
			}
		}
	}
	return valid;
}

//Get the possible numbers to put in the element at the current
//board state.
int* possible(board *b, int index) {
	//To be returned from function.
	int * arr = malloc(SIZE*sizeof(int));
	//To be used to see the possibilities.
	int temp[SIZE];
	//Set the arrays to 0
	memset(temp, 0, SIZE*sizeof(temp[0]));
	memset(arr, 0, SIZE*sizeof(arr[0]));
	//Get the squareroot of SIZE
	int sqroot = (int) sqrt((double) SIZE);
	//Index to be used for copying from temp
	//to arr
	int count = 0;
	//Get the row, col, and box/square
	int * r = row(index/SIZE, b);
	int * c = col(index%SIZE, b);
	int box = 0;
	box += index/SIZE/sqroot*sqroot;
	box += index%SIZE/sqroot;
	int * s = square(box, b);

	//Increment the non-zero numbers from row, col, and box
	//Values are -1 so it fits the array SIZE
	for(int i = 0; i < SIZE; ++i) {
		if(r[i] > 0) {
			temp[r[i]-1] += 1;
		}
		if(c[i] > 0) {
			temp[c[i]-1] += 1;
		}
		if(s[i] > 0) {
			temp[s[i]-1] += 1;
		}
	}

	//Free the extra pointers
	free(r);
	free(c);
	free(s);
	//Copy anything temp if there wasn't anything found
	//in the row, col, or square
	for(int i = 0; i < SIZE; ++i) {
		if(temp[i] <1) {
			arr[count] = i+1;
			++count;
		}
	}
	return arr;
}

//Wrapper function for solve with the start index
//Skips over the non-zero numbers as those are set.
bool solve(board * b) {
	int start_index = 0;
	bool found = false;
	for(int i = 0; i< SIZE && !found; ++i) {
		for(int j = 0; j<SIZE && !found; ++j) {
			if(b->b[i][j] == 0) {
				start_index = i*SIZE + j;
				found = true;
			}
		}
	}

	return solve_index(b, start_index);;
}

//Recursively goes through each index.
//Base case: Already over the size limit: returns valid && !dups
//If the number is non-zero, leave it and go to next index
//Do while the return value is not solved.
bool solve_index(board *b, int index) {
	if(index >= SIZE*SIZE) {
		return valid(b, false) && !dups(b);
	}
	bool r = false;
	if(b->b[index/SIZE][index%SIZE] != 0) {
		r = solve_index(b, index+1);
	}
	else {
	do{
	int * p = possible(b, index);
	//Goes while less than the number of possible elements
	//or a solution has been found
	for(int i = 0; i< SIZE && !r; ++i) {
		//No moves, return false so previous elements
		//will try other combos before continuing.
		//free the memory before returning.
		if(p[i] == 0) {
			free(p);
			return false;
		}
			//Set the index to the possible value
			b->b[index/SIZE][index%SIZE] = p[i];
			r = solve_index(b, index+1);
			//If the return is false, set the value
			//back to 0 before trying again.
			//This prevents the second conditions
			//from just skipping back over
			if(!r) {
				b->b[index/SIZE][index%SIZE] = 0;
			}
	}
	//Free the memory before looping again.
	free(p);
	}while(!r);
	}
	//Return if true or not.
	return r;
}
