#include <iostream>
#include <utility>
#include <algorithm>
using namespace std;

// Функция для подсчета количества единиц (бананов) в двоичном представлении числа
int count_bananas(unsigned long long num) {
    int count = 0;
    while (num > 0) {
        count += num & 1;
        num >>= 1;
    }
    return count;
}

int main() {
    unsigned long long N;
    cin >> N;

    pair<unsigned long long, unsigned long long> best_pair = {0, N};
    int max_bananas = count_bananas(0) + count_bananas(N);
    unsigned long long max_diff = N;

    // Перебираем все возможные пары чисел (a, b), где a + b = N
    for (unsigned long long a = 1; a <= N / 2; ++a) {
        unsigned long long b = N - a;
        int current_bananas = count_bananas(a) + count_bananas(b);

        // Если нашли пару с большим количеством бананов
        if (current_bananas > max_bananas) {
            max_bananas = current_bananas;
            best_pair = {a, b};
            max_diff = b - a;
        }
        // Если количество бананов такое же, но разность больше
        else if (current_bananas == max_bananas && (b - a) > max_diff) {
            best_pair = {a, b};
            max_diff = b - a;
        }
    }

    cout << best_pair.first << " " << best_pair.second << endl;
    return 0;
}