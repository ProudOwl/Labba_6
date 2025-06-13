#include <iostream>
#include <vector>
#include <cstdlib>
#include <ctime>
#include <unistd.h>
using namespace std;

const int WIDTH = 60;
const int HEIGHT = 30;

void clearScreen() {
    cout << "\033[2J\033[1;1H";
}

void printGrid(const vector<vector<bool>>& grid) {
    for (int i = 0; i < HEIGHT; ++i) {
        for (int j = 0; j < WIDTH; ++j) {
            cout << (grid[i][j] ? "0" : " ");
        }
        cout << endl;
    }
}

int countNeighbors(const vector<vector<bool>>& grid, int x, int y) {
    int count = 0;
    for (int i = -1; i <= 1; ++i) {
        for (int j = -1; j <= 1; ++j) {
            if (i == 0 && j == 0) continue;
            int nx = (x + i + HEIGHT) % HEIGHT;
            int ny = (y + j + WIDTH) % WIDTH;
            if (grid[nx][ny]) count++;
        }
    }
    return count;
}

vector<vector<bool>> nextGeneration(const vector<vector<bool>>& grid) {
    vector<vector<bool>> newGrid(HEIGHT, vector<bool>(WIDTH, false));
    
    for (int i = 0; i < HEIGHT; ++i) {
        for (int j = 0; j < WIDTH; ++j) {
            int neighbors = countNeighbors(grid, i, j);
            
            if (grid[i][j]) {
                newGrid[i][j] = (neighbors == 2 || neighbors == 3);
            } else {
                newGrid[i][j] = (neighbors == 3);
            }
        }
    }
    
    return newGrid;
}

void setupRandom(vector<vector<bool>>& grid) {
    for (int i = 0; i < HEIGHT; ++i) {
        for (int j = 0; j < WIDTH; ++j) {
            grid[i][j] = rand() % 4 == 0;
        }
    }
}

void setupGosperGliderGun(vector<vector<bool>>& grid) {
    // Координаты для ружья Госпера
    vector<pair<int, int>> gun_coords = {
        {5, 1}, {5, 2}, {6, 1}, {6, 2},
        {5, 11}, {6, 11}, {7, 11},
        {4, 12}, {8, 12},
        {3, 13}, {9, 13},
        {3, 14}, {9, 14},
        {6, 15},
        {4, 16}, {8, 16},
        {5, 17}, {6, 17}, {7, 17},
        {6, 18},
        {3, 21}, {4, 21}, {5, 21},
        {3, 22}, {4, 22}, {5, 22},
        {2, 23}, {6, 23},
        {1, 25}, {2, 25}, {6, 25}, {7, 25},
        {3, 35}, {4, 35}, {3, 36}, {4, 36}
    };
    
    for (const auto& coord : gun_coords) {
        int x = coord.first;
        int y = coord.second;
        if (x >= 0 && x < HEIGHT && y >= 0 && y < WIDTH) {
            grid[x][y] = true;
        }
    }
}

int main() {
    srand(time(0));
    vector<vector<bool>> grid(HEIGHT, vector<bool>(WIDTH, false));
    
    cout << "1. Случайная генерация" << endl;
    cout << "2. Ружьё Госпера (Gosper Glider Gun)" << endl;
    cout << "Номер режима (1 или 2): ";
    
    int choice;
    cin >> choice;
    
    switch(choice) {
        case 1:
            setupRandom(grid);
            break;
        case 2:
            setupGosperGliderGun(grid);
            break;
        default:
            setupRandom(grid);
    }
    
    int generation = 0;
    while (true) {
	    clearScreen();
        printGrid(grid);
        grid = nextGeneration(grid);
        usleep(150000);
    }
    return 0;
}