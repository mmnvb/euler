#include <iostream>
#include <cstdint>

#define _3DIGIT_MAX 999
#define _3DIGIT_MIN 100

bool isPalindrome(std::int32_t num) {
	int digit[10];
	int n = 0;

	while(num) {
		digit[n++] = num % 10;
		num /= 10;
	}

	for(std::uint8_t i = 0; i < n; i++) {
		if(digit[i] != digit[n-i-1]) return false;
	}

	return true;
}

int main() {
	
	int result = 0;
	volatile bool run = true;

	for(std::uint32_t i = _3DIGIT_MAX; i >= _3DIGIT_MIN && run; i--) {
	for(std::uint32_t j = _3DIGIT_MAX; j >= _3DIGIT_MIN; j--) {
		result = i * j;
		if(isPalindrome(result)) {
			std::cout << i << " x " << j << " = " << result << '\n';
			run = false;
			break;
		}
	}
	}


	return 0;
}
