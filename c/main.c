///William Haugen
//Sudoku Solver
//See LICENSE for license information.
//
//
#include "sudoku.h"

int main() {
	board b;
	initialize(&b);
	display(&b);	
	if(solve(&b)) {
		printf("\nSolved!\n");
	}
	else {
		printf("\nNot Solvable.. Double check input.\n");
	}
	display(&b);

	if (!dups(&b)) {
		printf("\nNo dups\n");
	}
	else {
		printf("\nDups\n");
	}

	if(valid(&b, false)) {
		printf("\nValid numbers\n");
	}
	else {
		printf("\nNot Valid numbers\n");
	}	
	return 0;
}
