import yaml
import os
from sandbox.docker import DockerContainer


class Build:
    def __init__(self):
        self.stages = []

    def execute_in(self, container: DockerContainer):
        for stage in self.stages:
            stage.execute_in(container)

    def add_stage(self, param):
        self.stages.append(param)


class Stage:
    def __init__(self, name, script):
        self.name = name
        self.script = script

    def execute_in(self, container:DockerContainer):
        print("Executing Stage: " + self.name)
        print("=" * 80)
        container.run_command(self.script)


class JarvisFile:
    def __init__(self, jarvisfile):
        if not os.path.exists(jarvisfile):
            raise Exception("No such Jarvisfile: " + jarvisfile)
        with open(jarvisfile, "r") as file:
            self.content = yaml.load(file, Loader=yaml.SafeLoader)

    def stages(self) -> list:
        return [entry for entry in self.content if entry["stage"] is not None]


def from_jarvisfile(file) -> Build:
    jarvisfile = JarvisFile(file)
    build = Build()
    for stage in jarvisfile.stages():
        build.add_stage(Stage(name=stage["stage"], script=stage["script"]))
    return build

