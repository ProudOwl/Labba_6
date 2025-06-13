#include <iostream>
#include <vector>
#include <iomanip>
#include <cmath>
using namespace std;

const double EPSILON = 1e-6;

// метод Гаусса
vector<double> GaussianElimination(vector<vector<double>> A, vector<double> b) {
    int n = A.size();
    
    for (int i = 0; i < n; i++) {
        int maxRow = i;
        for (int k = i + 1; k < n; k++)
            if (abs(A[k][i]) > abs(A[maxRow][i]))
                maxRow = k;

        swap(A[i], A[maxRow]);
        swap(b[i], b[maxRow]);
        
        for (int k = i + 1; k < n; k++) {
            double factor = A[k][i] / A[i][i];
            for (int j = i; j < n; j++)
                A[k][j] -= factor * A[i][j];
            b[k] -= factor * b[i];
        }
    }
    
    vector<double> x(n, 0);
    for (int i = n - 1; i >= 0; i--) {
        x[i] = b[i];
        for (int j = i + 1; j < n; j++)
            x[i] -= A[i][j] * x[j];
        x[i] /= A[i][i];
    }
    
    return x;
}

// метод Зейделя
vector<double> SeidelMethod(const vector<vector<double>>& A, const vector<double>& b, double epsilon = EPSILON, int maxIterations = 100) {
    int n = A.size();
    vector<double> x(n, 0.0);
    vector<double> x_prev(n, 0.0);
    int iteration = 0;
    double error;

    do {
        x_prev = x;
        for (int i = 0; i < n; i++) {
            double sum = 0.0;
            for (int j = 0; j < n; j++) {
                if (j != i) {
                    sum += A[i][j] * (j < i ? x[j] : x_prev[j]);
                }
            }
            x[i] = (b[i] - sum) / A[i][i];
        }

        error = 0.0;
        for (int i = 0; i < n; i++) {
            error = max(error, abs(x[i] - x_prev[i]));
        }
        iteration++;
    } while (error > epsilon && iteration < maxIterations);

    if (iteration == maxIterations) {
        cout << "Метод Зейделя не сходится" << endl;
        return x;
    }
    else {
        cout << "Метод Зейделя сошелся за " << iteration << " итераций\n";
        return x;
    }
}

// Вывод результатов
void printResults(const vector<double>& x, const string& methodName) {
    cout << "\nРезультаты решения методом " << methodName << ":\n";
    cout << "+-----+-------------+-------------+-------------+-------------+\n";
    cout << "|  N  |     x1      |     x2      |     x3      |     x4      |\n";
    cout << "+-----+-------------+-------------+-------------+-------------+\n";
    cout << "|  1  | " << setw(11) << fixed << setprecision(8) << x[0] 
         << " | " << setw(11) << x[1] 
         << " | " << setw(11) << x[2] 
         << " | " << setw(11) << x[3] << " |\n";
    cout << "+-----+-------------+-------------+-------------+-------------+\n\n";
}

int main() {
    vector<vector<double>> A = {
        {-18.0,  -0.04,  0.21,  0.91},
        {-0.09,  -1.23,  -0.23, 0.25},
        {-0.13, -0.23,  0.8,   -0.21},
        {-1.04,  -1.31,  0.06,  0.15}
    };

    vector<double> b = {-1.24, -1.04, 2.56, 0.91};

    // Проверка диагонального преобладания
    bool isDominant = true;
    for (int i = 0; i < 4; i++) {
        double sum = 0.0;
        for (int j = 0; j < 4; j++) {
            if (j != i) sum += abs(A[i][j]);
        }
        if (abs(A[i][i]) <= sum) {
            isDominant = false;
            cout << "Строка " << i+1 << " не удовлетворяет условию диагонального преобладания\n";
        }
    }
    
    if (!isDominant) {
        cout << "Внимание: матрица не имеет диагонального преобладания!\n";
    }

    // метод Гаусса
    vector<double> x_gauss = GaussianElimination(A, b);
    printResults(x_gauss, "Гаусса");

    // метод Зейделя
    vector<double> x_seidel = SeidelMethod(A, b);
    printResults(x_seidel, "Зейделя");

    return 0;
}