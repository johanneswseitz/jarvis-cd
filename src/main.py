from sandbox.docker import DockerContainer
from build import from_jarvisfile
import os

if __name__ == "__main__":
    jarvisfile = os.path.join(os.getcwd(), "Jarvisfile")
    docker_file = os.path.join(os.getcwd(), "Dockerfile")
    git_dir = os.getcwd()
    container = DockerContainer(docker_file, git_dir)
    container.rebuild_image_if_changed()
    build = from_jarvisfile(jarvisfile)
    build.execute_in(container)
    container.clean_containers()
