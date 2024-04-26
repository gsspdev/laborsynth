#include <stdio.h>
#include <stdlib.h>
#include "lib.h"

int main(int argc, char *argv[]) {

  if (argc < 3) {
    printf("Please pass two numbers as command line arguments.\n");
    return 1;
  }

  int y = atoi(argv[1]);  // convert string to int
  int x = atoi(argv[2]);  // convert string to int
  int c = cadder(x, y);

  // Your code
  printf("The sum of x and y is %d\n", x + y); // Change a and b to x and y.
  printf("c is %d\n", c); // You were missing a semicolon here

  return 0;
}
