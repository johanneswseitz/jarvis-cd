import docker
from docker.errors import ImageNotFound
import os
import hashlib


class DockerContainer:
    def __init__(self, docker_file, git_dir):
        self.client = docker.from_env()
        if not os.path.exists(git_dir):
            raise Exception("Git directory does not exist! " + git_dir)
        if not os.path.isfile(docker_file):
            raise Exception("File does not exist or is not a file! " + docker_file)
        self.dockerfile = os.path.basename(docker_file)
        self.dockerfile_dir = os.path.dirname(docker_file)
        self.dockerfile_hash = get_hash_value_for_file(docker_file)
        self.git_dir = git_dir
        self.container = None

    def __enter__(self):
        pass

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.clean_containers()

    def jarvis_directory(self) -> str:
        return "/jarvis/"

    def rebuild_image_if_changed(self):
        try:
            self.client.images.get(self.image_name())
            print("Found previously built container image " + self.image_name())
        except ImageNotFound:
            print("No image found. Rebuilding container. This may take a while.")
            self.client.images.build(path=self.dockerfile_dir, dockerfile=self.dockerfile, tag=self.image_name())
            print("Container built: " + self.image_name())

    def run_command(self, command):
        if not self.container:
            self.container = self.client.containers.run(self.image_name(), "sleep 1d",
                                                volumes={self.git_dir: {"bind":self.jarvis_directory(), 'mode': 'ro'}},
                                                working_dir=self.jarvis_directory(), detach=True)
        output = self.container.exec_run(command)
        print(output.output.decode("utf-8"))

    def clean_containers(self):
        self.container.stop(timeout=1)
        self.client.containers.prune()

    def image_name(self) -> str:
        return "jarvis-image-" + self.dockerfile_hash


def get_hash_value_for_file(file) -> str:
    digest = hashlib.sha1()
    with open(file, 'rb') as some_file:
        buf = some_file.read()
        digest.update(buf)
    return digest.hexdigest()
