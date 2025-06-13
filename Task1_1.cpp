#include <iostream>
#include <vector>
#include <cstdlib>
#include <ctime>
using namespace std;

int main() {
    srand(time(0));
    
    int N;
    cout << "Введите размер матрицы N: ";
    cin >> N;
    
    // Инициализация матрицы
    vector<vector<int>> matrix(N, vector<int>(N));
    for (int i = 0; i < N; ++i) {
        for (int j = 0; j < N; ++j) {
            matrix[i][j] = rand() % 51 - 10; // [-10, 40]
        }
    }
    
    // Вывод матрицы
    cout << "Исходная матрица:" << endl;
    for (const auto& row : matrix) {
        for (int num : row) {
            cout << num << "\t";
        }
        cout << endl;
    }
    
    // Поиск строк с 3+ отрицательными элементами
    vector<int> negativeRows;
    vector<int> negativeElements;
    vector<vector<int>> newMatrix;
    
    for (int i = 0; i < N; ++i) {
        int count = 0;
        vector<int> currentNegatives;
        
        for (int j = 0; j < N; ++j) {
            if (matrix[i][j] < 0) {
                count++;
                currentNegatives.push_back(matrix[i][j]);
            }
        }
        
        if (count >= 3) {
            negativeRows.push_back(i);
            negativeElements.insert(negativeElements.end(), currentNegatives.begin(), currentNegatives.end());
            newMatrix.push_back(matrix[i]);
        }
    }
    
    // Вывод результатов
    cout << "\nНомера строк с 3+ отрицательными элементами: ";
    for (int row : negativeRows) {
        cout << row << " ";
    }
    
    cout << "\nОтрицательные элементы из этих строк: ";
    for (int num : negativeElements) {
        cout << num << " ";
    }
    
    cout << "\n\nНовая матрица из найденных строк:" << endl;
    for (const auto& row : newMatrix) {
        for (int num : row) {
            cout << num << "\t";
        }
        cout << endl;
    }
    return 0;
}