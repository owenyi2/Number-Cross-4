from sympy import primerange

def square(nMax):
    mlist = []
    i = 0
    while i**2 < nMax:
        mlist.append(i**2)
        i+= 1

    return mlist

def palindrome(nMax):
    mlist=[]
    for n in range(nMax+1):
        mstr=str(n)
        if mstr==mstr[::-1]:
            mlist.append(n)
    return(mlist)

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

# def prime_power_prime(nMax):
#     primes = list(primerange(2, 1000))
#     mlist = []
#     for p in primes:
#         for q in primes:
#             mlist.append(p**q)
#     mlist = sorted(filter(lambda x: x < nMax, mlist))
#     return mlist

def fib(n, F=[0, 1]):
    F.extend(sum(F[-2:])for _ in range(n-len(F)+1)); return F

def m37(n):
    return [37*i for i in range(n)]

# == Section 1 == #

# print(square(100))
# print()
# print(palindrome(int(1e3)))
# print()
# print(prime_power_prime(int(1e11)))
# print()
# print(fib(40))
# print()
# print(list(filter(lambda x: x % 23 == 0, palindrome(int(1e7)))))
# print()

# print(prime_power_prime(int(1e9)))

# == Section 2: fibonacci == #

# for f in fib(54):
#    print(f, len(str(f)))

# == Section 3: multiples of 37 == #

# for n in m37(100000):
#     print(n, len(str(n))) 

# == Section 4: squares == #

# for n in square(99999999999):
#     print(n, len(str(n)))

# == Section 5: primes powers == #

for p in prime_power_prime(int(1e11)):
    print(p, len(str(p)))

^([0-9])\1{2}([0-9])\2{2}([0-9])\3{1}([0-9])\4{2}
