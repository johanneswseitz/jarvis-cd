from buildlog import error, warn
from sandbox.docker import DockerContainer
from build import from_jarvisfile
import os
import sys


def ensure_utf8_terminal_encoding():
    print(sys.stdout.encoding)
    if sys.stdout.encoding.lower() != "utf-8":
        warn("Your systems terminal encoding is set to " + sys.stdout.encoding)
        warn("To avoid headaches Jarvis requires terminals to use utf-8 encoding. "
             "Please configure your system to use utf-8 encoding.")
        warn("You can do so by setting the environment variable LC_ALL=en_US.UTF-8")
        sys.exit(1)


if __name__ == "__main__":
    ensure_utf8_terminal_encoding()
    jarvisfile = os.path.join(os.getcwd(), "Jarvisfile")
    docker_file = os.path.join(os.getcwd(), "Dockerfile")
    git_dir = os.getcwd()
    container = DockerContainer(docker_file, git_dir)
    container.rebuild_image_if_changed()
    build = from_jarvisfile(jarvisfile)
    build.execute_in(container)
    container.clean_containers()
