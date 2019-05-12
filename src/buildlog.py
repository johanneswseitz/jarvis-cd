# creating unbuffered print for faster user feedback
import functools
import termcolor

print = functools.partial(print, flush=True)


def debug(message):
    print("Jarvis DEBUG: " + message)


def log(message, color=None):
    if color:
        print("Jarvis: " + termcolor.colored(message, color))
    else:
        print("Jarvis: " + message)


def error(message):
    print("Jarvis: " + termcolor.colored(message, "red"))


def warn(message):
    print("Jarvis: " + termcolor.colored(message, "yellow"))
