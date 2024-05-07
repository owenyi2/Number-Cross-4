from sympy import primerange

import sys

sequence = sys.argv[1]

MAX = 99_999_999_999

def print_sequence(seq):
    for n in seq:
        print(n, len(str(n)))


def square(nMax):
    mlist = []
    i = 0
    while i**2 < nMax:
        mlist.append(i**2)
        i+= 1

    return mlist

# Function to print decimal palindromic number 
def palindrome(nMax): 
    mlist = []
    def createPalindrome(inp, b, isOdd): 
        n = inp 
        palin = inp 
       
        # checks if number of digits is odd or even 
        # if odd then neglect the last digit of input in 
        # finding reverse as in case of odd number of 
        # digits middle element occur once 
        if (isOdd): 
            n = n // b 
       
        # Creates palindrome by just appending reverse 
        # of number to itself 
        while (n > 0): 
            palin = palin * b + (n % b) 
            n = n // b 
        return palin 
      
    # Run two times for odd and even length palindromes 
    for j in range(2): 
        # Creates palindrome numbers with first half as i.  
        # Value of j decided whether we need an odd length 
        # of even length palindrome. 
        i = 1
        while (createPalindrome(i, 10, j % 2) < nMax): 
            mlist.append(createPalindrome(i, 10, j % 2)) 
            i = i + 1
    mlist = sorted(mlist)
    return mlist
   

def prime_power_prime(nMax):
    primes = list(primerange(2, int(nMax**0.5)))
    mlist = []

    for p in primes:
        i = 0
        power = p ** primes[i]
        while power < nMax:
            mlist.append(power)
            i+= 1
            power = p ** primes[i]

    mlist = sorted(mlist)
    return mlist

def fib(n):
    fib = [0,1]
    
    result = fib[-1] + fib[-2]
    while result < n:
        fib.append(result)
        result = fib[-1] + fib[-2]
    
    return fib

def multiple(m, nMax):
    return range(m, nMax, m)

match sequence:
    case "square":
        print_sequence(square(MAX))
    case "palindrome_plus_one":
        print_sequence(map(lambda x: x+1, palindrome(MAX-1)))
    case "prime_power":
        print_sequence(prime_power_prime(MAX))
    case "digit_sum_seven":
        print("ERROR: Unimplemented")
    case "fib":
        print_sequence(fib(MAX))
    case "multiple_37":
        print("ERROR: don't even try")
        # print_sequence(multiple(37, MAX))
    case "palindrome_multiple_23":
        print_sequence(filter(lambda x: x % 23 == 0, palindrome(MAX)))
    case "digit_prod_one":
        print("ERROR: Unimplemented")
    case "multiple_88": 
        print("ERROR: don't even try")
        # print_sequence(multiple(88, MAX))
    case "palindrome_less_one":
        print_sequence(map(lambda x: x-1, palindrome(MAX+2)))
    case _:
        print("square", "palindrome_plus_one", "prime_power", "digit_sum_seven", "fib", "multiple_37", "palindrome_multiple_23", "digit_prod_one", "multiple_88", "palindrome_less_one")
