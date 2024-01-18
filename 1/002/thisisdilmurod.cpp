#include <iostream>

#define ll long long

using namespace std;

int main() {
    ll ans = 0;
    int n1 = 1, n2 = 2;

    while (n2 < 4000000) {
        if (n2 % 2 == 0) {
            ans += n2;
        }
        int temp = n2;
        n2 += n1;
        n1 = temp;
    }
    
    cout << ans << endl;
}