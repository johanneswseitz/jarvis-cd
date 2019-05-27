# creating unbuffered print for faster user feedback
import functools
import termcolor

print = functools.partial(print, flush=True)


def debug(message):
    print("Jarvis DEBUG: " + message)


def log(message, colour=None):
    if colour:
        print("Jarvis: " + termcolor.colored(message, colour))
    else:
        print("Jarvis: " + message)


def verbatim(message, colour=None):
    if colour:
        print(termcolor.colored(message, colour))
    else:
        print(message)


def error(message):
    print("Jarvis: " + termcolor.colored(message, "red"))


def warn(message):
    print("Jarvis: " + termcolor.colored(message, "yellow"))
