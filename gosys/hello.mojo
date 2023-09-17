from Pointer import DTypePointer
from DType import DType
from Buffer import NDBuffer
from List import DimList
from Random import randint
from List import VariadicList
from Math import sqrt
from Numerics import FPUtils


struct Board[grid_size: Int]:
    var data: DTypePointer[DType.uint8]
    var sub_size: Int
    alias elements = grid_size**2

    fn __init__(inout self, *values: Int) raises:
        let args_list = VariadicList(values)
        if len(args_list) != elements:
            raise Error("The amount of elements must be equal to the grid_size parameter squared")

        let sub_size = sqrt(Float64(grid_size))
        if sub_size - sub_size.cast[DType.int64]().cast[DType.float64]() > 0:
            raise Error("The square root of the grid grid_size must be a whole number 9 = 3, 16 = 4")
        self.sub_size = sub_size.cast[DType.int64]().to_int()


        self.data = DTypePointer[DType.uint8].alloc(grid_size**2)
        for i in range(len(args_list)):
            self.data.simd_store[1](i, args_list[i])

    fn __getitem__(self, row: Int, col: Int) -> UInt8:
        return self.data.simd_load[1](row * grid_size + col)

    fn __setitem__(self, row: Int, col: Int, data: UInt8):
        self.data.simd_store[1](row * grid_size + col, data)

    fn print_board(inout self):
        for i in range(grid_size):
            print(self.data.simd_load[grid_size](i * grid_size))

    fn is_valid(self, row: Int, col: Int, num: Int) -> Bool:
        # Check the given number in the row
        for x in range(grid_size):
            if self[row, x] == num:
                return False

        # Check the given number in the col
        for x in range(grid_size):
            if self[x, col] == num:
                return False

        # Check the given number in the box
        let start_row = row - row % self.sub_size
        let start_col = col - col % self.sub_size
        for i in range(self.sub_size):
            for j in range(self.sub_size):
                if self[i+start_row, j+start_col] == num:
                    return False
        return True

    fn solve(self) -> Bool:
        for i in range(grid_size):
            for j in range(grid_size):
                if self[i, j] == 0:
                    for num in range(1, 10):
                        if self.is_valid(i, j, num):
                            self[i, j] = num
                            if self.solve():
                                return True
                            # If this number leads to no solution, then undo it
                            self[i, j] = 0
                    return False
        return True


let board = Board[9](
    5, 3, 0, 0, 7, 0, 0, 0, 0,
    6, 0, 0, 1, 9, 5, 0, 0, 0,
    0, 9, 8, 0, 0, 0, 0, 6, 0,
    8, 0, 0, 0, 6, 0, 0, 0, 3,
    4, 0, 0, 8, 0, 3, 0, 0, 1,
    7, 0, 0, 0, 2, 0, 0, 0, 6,
    0, 6, 0, 0, 0, 0, 2, 8, 0,
    0, 0, 0, 4, 1, 9, 0, 0, 5,
    0, 0, 0, 0, 8, 0, 0, 7, 9
)

print("Solved:", board.solve())
board.print_board()