


#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>



#define MAX_BUFFER_SIZE 1024
#define MAX_GRID_DIM 100 

typedef int Num;

typedef struct 
{
  Num grid[MAX_GRID_DIM][MAX_GRID_DIM];
  size_t xdim;
  size_t ydim;
} Solution;

const int DIRECTIONS[4][2] = {
  {1, 0}, {-1, 0},
  {0, 1}, {0, -1},
};

void read_file(const char* path, char* buffer);
Solution* construct_solution(const char* buffer);
Num solve_one(Solution* solution);
Num solve_two(Solution* solution);
void populate_trailheads(Solution* solution, size_t* trailheads, size_t* count);
void get_directions(int directions[4][2]);
void recursive_search_trailhead(Solution* solution, size_t x, size_t y, Num* found, Num count, Num target, int seen[MAX_GRID_DIM][MAX_GRID_DIM]);
void recursive_search_trailhead_path(Solution* solution, size_t x, size_t y, Num* found, Num curr, Num target);
int idx(Solution* solution, size_t x, size_t y, int dx, int dy, size_t* nx, size_t* ny);

int main(int argc, char* argv[])
{
  const char* file_path = (argc > 1) ? argv[1] : "testing.txt";
  char buffer[MAX_BUFFER_SIZE];

  read_file(file_path, buffer);

  Solution* solution = construct_solution(buffer);

  clock_t start = clock();
  Num part_one = solve_one(solution);
  clock_t end = clock();
  double time_one = ((double)(end - start)) / CLOCKS_PER_SEC;

  start = clock();
  Num part_two = solve_two(solution);
  end = clock();
  double time_two = ((double)(end - start)) / CLOCKS_PER_SEC;

  printf("\n__--__--__--__--__--__--__--__--__--\n");
  printf("part one: %d\ntime one: %f\n", part_one, time_one);
  printf("part one: %d\ntime two: %f\n", part_two, time_two);

  free(solution);

  return 0;
}

void read_file(const char* path, char* buffer)
{
  FILE* file = fopen(path, "r");
  if (!file) {
    perror("buffer read error");
    exit(3);
  }
  fread(buffer, sizeof(char), MAX_BUFFER_SIZE-1, file);
  fclose(file);
}

Solution* construct_solution(const char* buffer)
{
  Solution* solution = (Solution*)malloc(sizeof(Solution));
  size_t xdim = 0;
  size_t ydim = 0;
  char line[MAX_BUFFER_SIZE];

  const char* ptr = buffer;
  while (*ptr) {
    size_t x = 0;
    while (*ptr >= '0' && *ptr <= 9) {
      solution->grid[ydim][xdim] = *ptr - '0';
      ptr++;
    }
    ydim++;
    ptr++;
    xdim = 0;
  }

  solution->xdim = xdim;
  solution->ydim = ydim;

  return solution;
}

Num solve_one(Solution* solution)
{
  size_t trailheads[MAX_GRID_DIM * MAX_GRID_DIM];
  size_t count = 0;
  populate_trailheads(solution, trailheads, &count);
  
  Num scores = 0;
  for (size_t i = 0; i < count; i++) {
    size_t x = trailheads[i] % solution->xdim;
    size_t y = trailheads[i] / solution->xdim;
    Num curr = 0;
    Num target = 9;
    int seen[MAX_GRID_DIM][MAX_GRID_DIM] = {0};

    recursive_search_trailhead(solution, x, y, &scores, curr, target, seen);
  }

  return scores;
}

Num solve_two(Solution* solution)
{
  size_t trailheads[MAX_GRID_DIM * MAX_GRID_DIM];
  size_t count = 0;
  populate_trailheads(solution, trailheads, &count);

  Num ratings = 0;
  for (size_t i = 0; i < count; i++) {
    size_t x = trailheads[i] % solution->xdim;
    size_t y = trailheads[i] / solution-> xdim;
    Num curr = 0;
    Num target = 9;

    recursive_search_trailhead_path(solution, x, y, &ratings, curr, target);
  }

  return ratings;
}

void recursive_search_trailhead(Solution* solution, size_t x, size_t y, Num* found, Num curr, Num target, int seen[MAX_GRID_DIM][MAX_GRID_DIM])
{
  if (solution->grid[y][x] == target && !seen[y][x]) {
    seen[y][x] = 1;
    (*found)++;
    return;
  }

  size_t nx, ny;
  for (int i = 0; i < 4; i++) {
    if (idx(solution, x, y, DIRECTIONS[i][0], DIRECTIONS[i][1], &nx, &ny) && solution->grid[y][x] == curr + 1) {
      recursive_search_trailhead(solution, nx, ny, found, curr + 1, target ,seen);
    }
  }
}

void recursive_search_trailhead_path(Solution* solution, size_t x, size_t y, Num* found, Num curr, Num target)
{
  if (solution->grid[y][x] == target) {
    (*found)++;
    return;
  }

  size_t nx, ny;
  for (int i = 0; i < 4; i++) {
    if (idx(solution, x, y, DIRECTIONS[i][0], DIRECTIONS[i][1], &nx, &ny) && solution->grid[ny][nx] == curr + 1) {
      recursive_search_trailhead_path(solution, nx, ny, found, curr + 1, target);
    }
  }
}

int idx(Solution* solution, size_t x, size_t y, int dx, int dy, size_t* nx, size_t* ny) {
  *nx = x + dx;
  *ny = y + dy;
  return *nx < solution->xdim && *ny < solution->ydim;
}

void populate_trailheads(Solution* solution, size_t* trailheads, size_t* count)
{
  for (size_t y = 0; y < solution->ydim; y++) {
    for (size_t x = 0; x < solution-> xdim; x++) {
      if (solution->grid[y][x] == 0) {
        trailheads[*count] = y * solution->xdim + x;
        (*count)++;
      }
    }
  }
}

void get_directions(int directions[4][2])
{
  for (int i = 0; i < 4; i++) {
    directions[i][0] = DIRECTIONS[i][0];
    directions[i][1] = DIRECTIONS[i][1];
  }
}
