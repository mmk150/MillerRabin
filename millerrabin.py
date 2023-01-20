import numpy as np
import math
import random




def MillerRabin(number,trial_num):
    count=0
    def two_fac(num):
        s=0
        while num %2==0:
            num=num//2
            s+=1
        return s,num
    s,d=two_fac(number-1)
    while count<trial_num:
        witness=random.randint(2,number-2)
        x=pow(witness,d,number)
        for j in range(s):
            y=pow(x,2,number)
            if y==1 and x!=1 and x!=number-1:
                return "composite"
            x=y
        if y!=1:
            return "composite"
        count+=1
    return "probably prime"

def log4(x):
    return np.log(x)/np.log(4)

file=open('input.txt','r')
read=file.readlines()

for i in range(len(read)):
    read[i]=read[i].strip()

test_candidates=[]
prob_is_prime=float(read[0].strip().split(":")[-1])

trials= log4(1/(1-prob_is_prime))
trials= math.ceil(trials)

test_ints=[]
for i in range(1,len(read)):
    number=int(read[i])
    test_ints.append(number)

for input in test_ints:
    res=MillerRabin(input,trials)
    res="input: " + str(input) + " is " + res
    print(res)

    








#pow(base,exponent,modulus)
