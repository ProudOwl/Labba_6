#include <iostream>
#include <vector>
#include <climits>
#include <algorithm>
#include <ctime>
#include <cstdlib>
using namespace std;

vector<vector<int>> generateRandomBoard(int rows, int cols) {
    vector<vector<int>> board(rows, vector<int>(cols));
    srand(time(0));
    
    for (int i = 0; i < rows; ++i) {
        for (int j = 0; j < cols; ++j) {
            board[i][j] = rand() % 2;
        }
    }
    return board;
}

void printBoard(const vector<vector<int>>& board) {
    for (const auto& row : board) {
        for (int coin : row) {
            cout << coin << " ";
        }
        cout << endl;
    }
}

int countMismatches(const vector<vector<int>>& board, bool startWith) {
    int mismatches = 0;
    bool expected = startWith;
    for (const auto& row : board) {
        bool currentExpected = expected;
        for (int coin : row) {
            if (coin != currentExpected) {
                mismatches++;
            }
            currentExpected = !currentExpected;
        }
        expected = !expected;
    }
    return mismatches;
}

int solveCoinBoard(const vector<vector<int>>& board) {
    // Подсчет несоответствий для двух возможных шахматных паттернов
    int mismatches1 = countMismatches(board, true);
    int mismatches2 = countMismatches(board, false);
    
    if (mismatches1 % 2 != 0 && mismatches2 % 2 != 0) {
        return -1;
    }
    
    // Минимальное количество ходов - половина от несоответствий (так как переворачиваем по 2 монеты)
    int minMoves1 = (mismatches1 % 2 == 0) ? mismatches1 / 2 : INT_MAX;
    int minMoves2 = (mismatches2 % 2 == 0) ? mismatches2 / 2 : INT_MAX;
    
    if (minMoves1 == INT_MAX && minMoves2 == INT_MAX) {
        return -1;
    }
    return min(minMoves1, minMoves2);
}

int main() {
    int rows, cols;
    cout << "Введите количество строк и столбцов доски: ";
    cin >> rows >> cols;
    
    if (rows <= 0 || cols <= 0) {
        cout << "Некорректные размеры доски!" << endl;
        return 1;
    }
    
    vector<vector<int>> board = generateRandomBoard(rows, cols);
    
    cout << "\nСгенерированная доска:\n";
    printBoard(board);
    
    int minMoves = solveCoinBoard(board);
    
    if (minMoves == -1) {
        cout << "\nНевозможно преобразовать доску в шахматный порядок";
    } else {
        cout << "\nМинимальное количество ходов: " << minMoves << endl;
    }
    return 0;
}