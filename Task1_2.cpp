#include <iostream>
#include <vector>
#include <cstdlib>
#include <ctime>
using namespace std;

int main() {
    srand(time(0));
    
    int N;
    do {
        cout << "Введите размер матрицы: ";
        cin >> N;
    } while (N % 2 != 0);
    
    // Инициализация матрицы
    vector<vector<int>> matrix(N, vector<int>(N));
    for (int i = 0; i < N; ++i) {
        for (int j = 0; j < N; ++j) {
            matrix[i][j] = rand() % 21 + 10; // [10, 30]
        }
    }
    
    // Вывод исходной матрицы
    cout << "Исходная матрица:" << endl;
    for (const auto& row : matrix) {
        for (int num : row) {
            cout << num << "\t";
        }
        cout << endl;
    }
    
    // Размер четверти
    int quarter = N / 2;
    
    // Левая нижняя четверть
    for (int i = quarter; i < N; ++i) {
        for (int j = 0; j < quarter; ++j) {
            matrix[i][j] = 0;
        }
    }
    
    // Правая нижняя четверть
    for (int i = quarter; i < N; ++i) {
        for (int j = quarter; j < N; ++j) {
            matrix[i][j] = 10;
        }
    }
    
    // Меняем местами оставшиеся четверти
    for (int i = 0; i < quarter; ++i) {
        for (int j = 0; j < quarter; ++j) {
            swap(matrix[i][j], matrix[i][j + quarter]);
        }
    }
    
    // Вывод измененной матрицы
    cout << "\nИзмененная матрица:" << endl;
    for (const auto& row : matrix) {
        for (int num : row) {
            cout << num << "\t";
        }
        cout << endl;
    }
    return 0;
}