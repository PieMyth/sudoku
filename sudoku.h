//William Haugen
//Sudoku Solver
//See LICENSE for license information.
//
//
#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<stdbool.h>
#include<string.h>

#define SIZE 9

typedef struct {
	int b[SIZE][SIZE];
}board;


int* col(int, board *);
void display(board *);
bool dups(board *);
void initialize(board *);
void input(board *);
int* possible(board*, int);
int* row(int, board *);
bool solve(board *);
bool solve_index(board *, int);
int* square(int, board *);
bool valid(board *, bool initialize);
