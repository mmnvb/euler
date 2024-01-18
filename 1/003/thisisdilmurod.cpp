#include <iostream>
#include <cmath>
#include <vector>
#include <algorithm>

#define ll long long

using namespace std;

int main() {
    ll num = 600851475143;
    vector<int> primes; // Vectors are dynamic arrays

    while (num % 2 == 0) {
        primes.push_back(2);
        num /= 2;
    }

    for (int i = 3; i <= sqrt(num); i += 2) {
        while (num % i == 0) {
            primes.push_back(i);
            num /= i;
        }
    }

    if (num > 2 && primes.empty()) {
        cout << num << endl;
    }

    cout << *max_element(primes.begin(), primes.end()) << endl; // Cout the ptr to the max
}