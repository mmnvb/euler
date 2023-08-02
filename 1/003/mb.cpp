// feel like genius
// 100% written by m_bob

#include <iostream>
bool is_prime(int number){
	for (int i=2;i<10;i++){
		if (number%i==0 && number!=i){
			return false;
		}
	}
	return true;
}

int main(){
    long long num = 600851475143;
    int i = 2;

    while (true){
        if (num%i==0){
            num = num/i;
            i = 2;
            continue;
        }
        i += 1;

        if (num == i){
            break;
        }
    }
    std::cout<<"Answer: "<<num<<std::endl;
    std::cout<<"Is prime: "<<is_prime(num);
}