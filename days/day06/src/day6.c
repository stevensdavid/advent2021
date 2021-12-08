#include <stdio.h>
#include <stdlib.h>

const char COMMA = 0x2C;

int main() {
  // Parse input. ASSUMES ONLY SINGLE DIGITS AND COMMAS IN STDIN
  long state[9] = {0};
  char c = getchar();
  while (c != EOF) {
    // If it's a comma, we skip
    if (c != COMMA) {
      int val = atoi(&c);
      state[val] += 1;
    }
    c = getchar();
  }

  // Simulate
  int n_days = 256;
  for (int day = 0; day < n_days; day++) {
    long new_state[9];
    for (int time = 8; time > 0; time--) {
      new_state[time - 1] = state[time];
    }
    new_state[6] += state[0];
    // New births
    new_state[8] = state[0];

    // Transfer new state
    for (int idx = 0; idx < 9; idx++) {
      state[idx] = new_state[idx];
    }
  }
  // Sum number of fish after simulation ends
  long sum = 0;
  for (int idx = 0; idx < 9; idx++) {
    sum += state[idx];
  }
  printf("The final number of fish is: %ld\n", sum);
}
