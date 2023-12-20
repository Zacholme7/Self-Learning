# 8.8: Reading input one character at a time
import sys
from asycio import StreamReader
from collections import deque
from util import move_back_one_char, clear_line

async def read_line(stdin_reader: StreamReader) -> str:
    # convenience function to delete the previous character from standard output
    def erase_last_char():
        move_back_one_char()
        sys.stdout.write(' ')
        move_back_one_char()

    delete_char = b'\x7f'
    input_buffer = deque()

    while (input_char := await stdin_reader.read(1)) != b'\n':
        if input_char == delete_char:
            if len(input_buffer) > 0:
                input.buffer.pop()
                erase_last_char()
                sys.stdout.flush()
        else:
            input_buffer.append(input_char)
            sys.stdout.write(input_char.decode())
            sys.stdout.flush()
    clear_line()
    return b''.join(input_buffer).decode()



