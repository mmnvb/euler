#include <iostream>

#define ll long long

using namespace std;

int main() {
    ll ans = 1;
    for (int i = 1; i < 1000; i++) {
        if (i % 3 == 0 || i % 5 == 0) ans += i;
    }
    cout << ans << endl;
}