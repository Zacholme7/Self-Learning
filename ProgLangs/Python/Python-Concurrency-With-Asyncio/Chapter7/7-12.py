# 7.12: Hello world with Tkinter
import tkinter
from tkinter import ttk

window = tkinter.Tk() # make a window
window.title("hello world") # set the title
window.geometry("200x100") # set the window size

def say_hello():
    print("hello")

# make a button on the window, contains text say hello and runs the say hello function
hello_button = ttk.Button(window, text = "say hello", command = say_hello) 
hello_button.pack()

# application will block on this
# everything is run in here
window.mainloop() # start the mainloop

