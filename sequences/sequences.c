#include "stdio.h"
#include "math.h"

int digit_sum(int n) {
    int num_digits = log10(n) + 1;
    
    int digit_sum = 0;

    for (int i = 0; i < num_digits; i++) {
        digit_sum += n % 10;
        n /= 10;
    }
    return digit_sum;
}

int main() {
    for (int i = 0; i < 99999999; i++) {     
        if (digit_sum(i) % 10 == 1) {
            int len = log10(i) + 1;
            printf("%d %d\n", i, len);
        }
    }
    return 0;
}

// shit this is slow as fuck as well. So we can't pre-generate stuff. Shit

// We should just try backtracking
