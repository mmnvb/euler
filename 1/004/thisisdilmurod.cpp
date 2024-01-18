#include <iostream>

using namespace std;

bool isPalindrome(int number) {
    int temp = number;
    int reverse = 0;
    while (temp != 0) {
        reverse = reverse * 10 + temp % 10;
        temp /= 10;
    }
    return number == reverse;
}

int main() {
    int max = 0;
    for (int i = 100; i < 1000; i++) {
        for (int j = 100; j < 1000; j++) {
            if (isPalindrome(i * j) && i * j > max) {
                max = i * j;
            } 
        }
    }
    cout << max << endl;
}