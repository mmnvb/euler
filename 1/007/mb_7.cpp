#include <iostream>
// Sieve of Eratosthenes
bool is_prime(int number){
	if (number <= 3){
		return true;
	}

	if (number%2==0 || number%3==0){
		return false;
	}

	int i=5;
    while (i*i<=number){
        if (number%i==0 || number%(i+2)==0){
            return false;
        }
        i += 6;
    }
    return true;
}

int main() {
	long i=2; int counter=0;
	while (counter!=10001){
		if (is_prime(i)){counter+=1;}
		i+=1;
	}
	std::cout<<"Answer: "<<i-1; // i-1 due to the past increment
}